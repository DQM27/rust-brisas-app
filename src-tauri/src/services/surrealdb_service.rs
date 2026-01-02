//! # Servicio: Infraestructura SurrealDB (Modo Embebido)
//!
//! Este servicio es el núcleo de persistencia de Brisas APP. Gestiona el ciclo
//! de vida de la conexión a SurrealDB, la inicialización del esquema y provee
//! acceso thread-safe al cliente mediante un patrón Singleton.
//!
//! ## Características
//! - Persistencia local mediante `SurrealKv`.
//! - Inicialización declarativa del esquema (`.surql`).
//! - Acceso global optimizado con `Arc<RwLock>` y `OnceCell`.

use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use surrealdb::engine::local::{Db, SurrealKv};
use surrealdb::Surreal;
use thiserror::Error;
use tokio::sync::RwLock;

// ==========================================
// SINGLETON & HELPERS (Acceso Global)
// ==========================================

use once_cell::sync::OnceCell;
static SURREAL_SERVICE: OnceCell<Arc<SurrealDbService>> = OnceCell::new();

/// Inicializa una instancia global única del servicio de base de datos.
pub fn init_surrealdb(config: SurrealDbConfig) -> Arc<SurrealDbService> {
    SURREAL_SERVICE.get_or_init(|| Arc::new(SurrealDbService::new(config))).clone()
}

/// Recupera la instancia global, si existe.
pub fn get_surrealdb() -> Option<Arc<SurrealDbService>> {
    SURREAL_SERVICE.get().cloned()
}

/// Orquestador inicial: conecta a la DB e inmediatamente aplica el esquema.
pub async fn setup_embedded_surrealdb(
    config: SurrealDbConfig,
) -> Result<Arc<SurrealDbService>, SurrealDbError> {
    let service = init_surrealdb(config);
    service.connect().await?;
    service.init_schema().await?;
    Ok(service)
}

/// Función auxiliar de alto nivel para obtener el cliente de DB de forma rápida.
pub async fn get_db() -> Result<Surreal<Db>, SurrealDbError> {
    let service = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    service.get_client().await
}

// ==========================================
// MODELOS DE ERROR Y CONFIGURACIÓN
// ==========================================

#[derive(Debug, Error)]
pub enum SurrealDbError {
    #[error("Error de conexión: {0}")]
    Connection(String),
    #[error("Error de autenticación: {0}")]
    Auth(String),
    #[error("Error de query: {0}")]
    Query(String),
    #[error("No conectado a SurrealDB")]
    NotConnected,
    #[error("Error de inicialización: {0}")]
    Init(String),
    #[error("Error de transacción: {0}")]
    TransactionError(String),
}

impl From<surrealdb::Error> for SurrealDbError {
    fn from(e: surrealdb::Error) -> Self {
        Self::Query(e.to_string())
    }
}

/// Configuración física y lógica de la base de datos.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealDbConfig {
    /// Ruta en el disco donde se almacenarán los archivos K/V.
    pub data_path: PathBuf,
    /// Espacio de nombres para aislar entornos (ej. producción vs demo).
    pub namespace: String,
    /// Nombre de la base de datos lógica.
    pub database: String,
}

impl Default for SurrealDbConfig {
    fn default() -> Self {
        let data_path = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Brisas")
            .join("surrealdb");
        Self { data_path, namespace: "brisas".to_string(), database: "produccion".to_string() }
    }
}

impl SurrealDbConfig {
    /// Configuración aislada para pruebas o demostraciones sin afectar los datos reales.
    pub fn demo() -> Self {
        let data_path = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Brisas")
            .join("surrealdb_demo");
        Self { data_path, namespace: "brisas".to_string(), database: "demo".to_string() }
    }
}

// ==========================================
// SERVICIO PRINCIPAL
// ==========================================

/// Servicio principal para interactuar con SurrealDB.
pub struct SurrealDbService {
    client: Arc<RwLock<Option<Surreal<Db>>>>,
    config: SurrealDbConfig,
}

impl SurrealDbService {
    pub fn new(config: SurrealDbConfig) -> Self {
        Self { client: Arc::new(RwLock::new(None)), config }
    }

    /// Establece la conexión con el motor SurrealKv y selecciona el entorno de trabajo.
    pub async fn connect(&self) -> Result<(), SurrealDbError> {
        info!("🔌 Conectando a SurrealDB (Modo: Embebido)...");
        debug!("📂 Ruta de datos: {:?}", self.config.data_path);

        if !self.config.data_path.exists() {
            debug!("📁 Creando directorio de base de datos...");
            std::fs::create_dir_all(&self.config.data_path)
                .map_err(|e| SurrealDbError::Init(e.to_string()))?;
        }

        let db = Surreal::new::<SurrealKv>(self.config.data_path.clone())
            .await
            .map_err(|e| SurrealDbError::Connection(e.to_string()))?;

        db.use_ns(&self.config.namespace).use_db(&self.config.database).await?;

        *self.client.write().await = Some(db);
        info!(
            "✅ Conexión establecida con éxito [Namespace: {} | DB: {}]",
            self.config.namespace, self.config.database
        );
        Ok(())
    }

    pub async fn init_schema(&self) -> Result<(), SurrealDbError> {
        debug!("📜 Inicializando esquema de la base de datos...");
        let client = self.get_client().await?;

        client.query(include_str!("../db/surrealdb_schema.surql")).await.map_err(|e| {
            error!("❌ Error al aplicar el esquema: {}", e);
            SurrealDbError::Query(e.to_string())
        })?;

        info!("✨ Esquema de base de datos aplicado correctamente");
        Ok(())
    }

    pub async fn get_client(&self) -> Result<Surreal<Db>, SurrealDbError> {
        self.client.read().await.clone().ok_or(SurrealDbError::NotConnected)
    }

    pub async fn is_connected(&self) -> bool {
        self.client.read().await.is_some()
    }

    pub async fn disconnect(&self) {
        info!("🔌 Cerrando conexión con SurrealDB...");
        *self.client.write().await = None;
    }
}

// --------------------------------------------------------------------------
// PRUEBAS UNITARIAS
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_paths() {
        let default_conf = SurrealDbConfig::default();
        assert!(default_conf.data_path.to_string_lossy().contains("Brisas"));
        assert_eq!(default_conf.namespace, "brisas");

        let demo_conf = SurrealDbConfig::demo();
        assert!(demo_conf.data_path.to_string_lossy().contains("surrealdb_demo"));
    }

    #[tokio::test]
    async fn test_service_initial_state() {
        let config = SurrealDbConfig {
            data_path: PathBuf::from("./test_db"),
            namespace: "test".into(),
            database: "test".into(),
        };
        let service = SurrealDbService::new(config);
        assert!(!service.is_connected().await);
    }
}
