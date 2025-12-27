// ==========================================
// SurrealDB Connection Service (EMBEDDED)
// ==========================================
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealDbConfig {
    pub data_path: PathBuf,
    pub namespace: String,
    pub database: String,
}

impl Default for SurrealDbConfig {
    fn default() -> Self {
        let data_path = dirs::data_dir().unwrap_or_else(|| PathBuf::from(".")).join("Brisas").join("surrealdb");
        Self { data_path, namespace: "brisas".to_string(), database: "produccion".to_string() }
    }
}

impl SurrealDbConfig {
    pub fn demo() -> Self {
        let data_path = dirs::data_dir().unwrap_or_else(|| PathBuf::from(".")).join("Brisas").join("surrealdb_demo");
        Self { data_path, namespace: "brisas".to_string(), database: "demo".to_string() }
    }
}

pub struct SurrealDbService {
    client: Arc<RwLock<Option<Surreal<Db>>>>,
    config: SurrealDbConfig,
}

impl SurrealDbService {
    pub fn new(config: SurrealDbConfig) -> Self {
        Self { client: Arc::new(RwLock::new(None)), config }
    }
    pub async fn connect(&self) -> Result<(), SurrealDbError> {
        if !self.config.data_path.exists() {
            std::fs::create_dir_all(&self.config.data_path).map_err(|e| SurrealDbError::Init(e.to_string()))?;
        }
        let db = Surreal::new::<SurrealKv>(self.config.data_path.clone()).await.map_err(|e| SurrealDbError::Connection(e.to_string()))?;
        db.use_ns(&self.config.namespace).use_db(&self.config.database).await?;
        *self.client.write().await = Some(db);
        Ok(())
    }
    pub async fn init_schema(&self) -> Result<(), SurrealDbError> {
        let client = self.get_client().await?;
        client.query(include_str!("../db/surrealdb_schema.surql")).await?;
        Ok(())
    }
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

use once_cell::sync::OnceCell;
static SURREAL_SERVICE: OnceCell<Arc<SurrealDbService>> = OnceCell::new();
pub fn init_surrealdb(config: SurrealDbConfig) -> Arc<SurrealDbService> {
    SURREAL_SERVICE.get_or_init(|| Arc::new(SurrealDbService::new(config))).clone()
}
pub fn get_surrealdb() -> Option<Arc<SurrealDbService>> {
    SURREAL_SERVICE.get().cloned()
}
pub async fn setup_embedded_surrealdb(config: SurrealDbConfig) -> Result<Arc<SurrealDbService>, SurrealDbError> {
    let service = init_surrealdb(config);
    service.connect().await?;
    service.init_schema().await?;
    Ok(service)
}
pub async fn get_db() -> Result<Surreal<Db>, SurrealDbError> {
    let service = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    service.get_client().await
}
