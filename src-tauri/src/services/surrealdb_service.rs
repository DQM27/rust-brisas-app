// ==========================================
// SurrealDB Connection Service (EMBEDDED)
// ==========================================
// Maneja la conexión a SurrealDB embebido (sin servidor externo)

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use surrealdb::engine::local::{Db, SurrealKv};
use surrealdb::Surreal;
use thiserror::Error;
use tokio::sync::RwLock;

// ==========================================
// ERRORS
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
}

impl From<surrealdb::Error> for SurrealDbError {
    fn from(e: surrealdb::Error) -> Self {
        Self::Query(e.to_string())
    }
}

// ==========================================
// CONFIGURATION
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealDbConfig {
    /// Ruta donde se almacenará la base de datos (SurrealKV)
    pub data_path: PathBuf,
    /// Namespace a usar
    pub namespace: String,
    /// Database a usar
    pub database: String,
}

impl Default for SurrealDbConfig {
    fn default() -> Self {
        // Por defecto, usa el directorio de datos de la app
        let data_path =
            dirs::data_dir().unwrap_or_else(|| PathBuf::from(".")).join("Brisas").join("surrealdb");

        Self { data_path, namespace: "brisas".to_string(), database: "produccion".to_string() }
    }
}

impl SurrealDbConfig {
    pub fn demo() -> Self {
        let data_path = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Brisas")
            .join("surrealdb_demo");

        Self { data_path, namespace: "brisas".to_string(), database: "demo".to_string() }
    }
}

// ==========================================
// SERVICE
// ==========================================

pub struct SurrealDbService {
    client: Arc<RwLock<Option<Surreal<Db>>>>,
    config: SurrealDbConfig,
}

impl SurrealDbService {
    pub fn new(config: SurrealDbConfig) -> Self {
        Self { client: Arc::new(RwLock::new(None)), config }
    }

    /// Conecta a SurrealDB en modo embebido (SurrealKV - Pure Rust)
    pub async fn connect(&self) -> Result<(), SurrealDbError> {
        // Crear directorio si no existe
        if !self.config.data_path.exists() {
            std::fs::create_dir_all(&self.config.data_path)
                .map_err(|e| SurrealDbError::Init(format!("Error creando directorio: {}", e)))?;
        }

        log::info!("🔌 Conectando a SurrealDB embebido en: {:?}", self.config.data_path);

        // Crear conexión embebida con SurrealKV (Pure Rust, no C++ deps)
        let db = Surreal::new::<SurrealKv>(self.config.data_path.clone())
            .await
            .map_err(|e| SurrealDbError::Connection(e.to_string()))?;

        // Seleccionar namespace y database
        db.use_ns(&self.config.namespace).use_db(&self.config.database).await?;

        *self.client.write().await = Some(db);

        log::info!(
            "✅ SurrealDB embebido conectado: ns={}, db={}",
            self.config.namespace,
            self.config.database
        );

        Ok(())
    }

    /// Inicializa el esquema de la base de datos
    pub async fn init_schema(&self) -> Result<(), SurrealDbError> {
        let client = self.get_client().await?;

        // Definir tablas y esquema
        client
            .query(include_str!("../db/surrealdb_schema.surql"))
            .await
            .map_err(|e| SurrealDbError::Init(format!("Error inicializando esquema: {}", e)))?;

        log::info!("📋 Esquema SurrealDB inicializado");
        Ok(())
    }

    /// Obtiene referencia al cliente
    pub async fn get_client(&self) -> Result<Surreal<Db>, SurrealDbError> {
        self.client.read().await.clone().ok_or(SurrealDbError::NotConnected)
    }

    /// Verifica si está conectado
    pub async fn is_connected(&self) -> bool {
        self.client.read().await.is_some()
    }

    /// Desconecta
    pub async fn disconnect(&self) {
        *self.client.write().await = None;
        log::info!("🔌 Desconectado de SurrealDB");
    }

    /// Obtiene la ruta de datos
    pub fn data_path(&self) -> &PathBuf {
        &self.config.data_path
    }
}

// ==========================================
// SINGLETON GLOBAL (para Tauri State)
// ==========================================

use once_cell::sync::OnceCell;

static SURREAL_SERVICE: OnceCell<Arc<SurrealDbService>> = OnceCell::new();

/// Inicializa el servicio de SurrealDB
pub fn init_surrealdb(config: SurrealDbConfig) -> Arc<SurrealDbService> {
    SURREAL_SERVICE.get_or_init(|| Arc::new(SurrealDbService::new(config))).clone()
}

/// Obtiene el servicio de SurrealDB si está inicializado
pub fn get_surrealdb() -> Option<Arc<SurrealDbService>> {
    SURREAL_SERVICE.get().cloned()
}

/// Conecta e inicializa SurrealDB embebido
pub async fn setup_embedded_surrealdb(
    config: SurrealDbConfig,
) -> Result<Arc<SurrealDbService>, SurrealDbError> {
    let service = init_surrealdb(config);
    service.connect().await?;
    service.init_schema().await?;
    Ok(service)
}
