// ==========================================
// SurrealDB Connection Service
// ==========================================
// Maneja la conexión a SurrealDB (cloud o local)

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
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
    /// URL del servidor (ws://localhost:8000 o wss://cloud.surrealdb.com)
    pub url: String,
    /// Namespace a usar
    pub namespace: String,
    /// Database a usar
    pub database: String,
    /// Usuario root (solo para desarrollo/setup)
    pub username: Option<String>,
    /// Password root (solo para desarrollo/setup)
    pub password: Option<String>,
}

impl Default for SurrealDbConfig {
    fn default() -> Self {
        Self {
            url: "ws://localhost:8000".to_string(),
            namespace: "brisas".to_string(),
            database: "produccion".to_string(),
            username: Some("root".to_string()),
            password: Some("root".to_string()),
        }
    }
}

// ==========================================
// SERVICE
// ==========================================

pub struct SurrealDbService {
    client: Arc<RwLock<Option<Surreal<Client>>>>,
    config: SurrealDbConfig,
}

impl SurrealDbService {
    pub fn new(config: SurrealDbConfig) -> Self {
        Self { client: Arc::new(RwLock::new(None)), config }
    }

    /// Conecta a SurrealDB
    pub async fn connect(&self) -> Result<(), SurrealDbError> {
        let db = Surreal::new::<Ws>(&self.config.url)
            .await
            .map_err(|e| SurrealDbError::Connection(e.to_string()))?;

        // Autenticación root (para desarrollo)
        if let (Some(user), Some(pass)) = (&self.config.username, &self.config.password) {
            db.signin(Root { username: user, password: pass })
                .await
                .map_err(|e| SurrealDbError::Auth(e.to_string()))?;
        }

        // Seleccionar namespace y database
        db.use_ns(&self.config.namespace).use_db(&self.config.database).await?;

        *self.client.write().await = Some(db);

        log::info!("✅ Conectado a SurrealDB: {}", self.config.url);
        Ok(())
    }

    /// Obtiene referencia al cliente
    pub async fn get_client(&self) -> Result<Surreal<Client>, SurrealDbError> {
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
}

// ==========================================
// SINGLETON GLOBAL (para Tauri State)
// ==========================================

use once_cell::sync::OnceCell;

static SURREAL_SERVICE: OnceCell<Arc<SurrealDbService>> = OnceCell::new();

pub fn init_surrealdb(config: SurrealDbConfig) -> Arc<SurrealDbService> {
    SURREAL_SERVICE.get_or_init(|| Arc::new(SurrealDbService::new(config))).clone()
}

pub fn get_surrealdb() -> Option<Arc<SurrealDbService>> {
    SURREAL_SERVICE.get().cloned()
}
