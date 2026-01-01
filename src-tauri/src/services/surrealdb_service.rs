/// Gestión de la conexión a SurrealDB en modo incrustado (embedded).
///
/// Elegimos SurrealDB por su capacidad de manejar modelos relacionales y de grafos
/// de forma nativa, y usamos el modo embebido (SurrealKv) para que la aplicación sea
/// totalmente autónoma, sin depender de servidores externos instalados en la máquina del cliente.
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use surrealdb::engine::local::{Db, SurrealKv};
use surrealdb::Surreal;
use thiserror::Error;
use tokio::sync::RwLock;

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
        // Por defecto, almacenamos los datos en la carpeta local de la aplicación (AppData en Windows).
        // Esto garantiza que los datos persistan entre reinicios y actualizaciones.
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

/// Servicio principal para interactuar con SurrealDB.
///
/// Usamos un Arc<RwLock> para el cliente para permitir que múltiples hilos lean de la DB
/// de forma segura y concurrente, mientras que las reconexiones o desconexiones
/// obtienen acceso exclusivo de escritura.
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
        // Aseguramos que la carpeta de destino existe antes de que Surreal intente abrirla.
        if !self.config.data_path.exists() {
            std::fs::create_dir_all(&self.config.data_path)
                .map_err(|e| SurrealDbError::Init(e.to_string()))?;
        }

        // Iniciamos el motor usando el almacenamiento persistente definido en la ruta.
        let db = Surreal::new::<SurrealKv>(self.config.data_path.clone())
            .await
            .map_err(|e| SurrealDbError::Connection(e.to_string()))?;

        // Configuramos el contexto lógigo. Es obligatorio antes de realizar cualquier consulta.
        db.use_ns(&self.config.namespace).use_db(&self.config.database).await?;

        *self.client.write().await = Some(db);
        Ok(())
    }

    /// Inicializa la estructura de tablas, índices y restricciones (schema).
    ///
    /// Leemos un archivo .surql embebido en el binario. Esto permite que la aplicación
    /// defina su propio esquema de forma declarativa sin necesidad de migraciones externas manuales.
    pub async fn init_schema(&self) -> Result<(), SurrealDbError> {
        let client = self.get_client().await?;
        client.query(include_str!("../db/surrealdb_schema.surql")).await?;
        Ok(())
    }

    /// Obtiene una instancia clonada del cliente.
    /// Devuelve error si el servicio no ha sido conectado previamente.
    pub async fn get_client(&self) -> Result<Surreal<Db>, SurrealDbError> {
        self.client.read().await.clone().ok_or(SurrealDbError::NotConnected)
    }

    pub async fn is_connected(&self) -> bool {
        self.client.read().await.is_some()
    }

    pub async fn disconnect(&self) {
        *self.client.write().await = None;
    }
}

// Patrón Singleton para acceso global simplificado.
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
/// Es llamado durante el setup de Tauri.
pub async fn setup_embedded_surrealdb(
    config: SurrealDbConfig,
) -> Result<Arc<SurrealDbService>, SurrealDbError> {
    let service = init_surrealdb(config);
    service.connect().await?;
    service.init_schema().await?;
    Ok(service)
}

/// Función auxiliar de alto nivel para obtener el cliente de DB de forma rápida.
/// Es la más utilizada en los controladores y servicios de la aplicación.
pub async fn get_db() -> Result<Surreal<Db>, SurrealDbError> {
    let service = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    service.get_client().await
}
