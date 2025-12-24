// src/db/mod.rs

use crate::config::AppConfig;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{ConnectOptions, SqlitePool}; // Import ConnectOptions trait
use std::str::FromStr;
use tokio::sync::RwLock;

pub struct DbPool(pub RwLock<SqlitePool>);

pub mod migrate;

pub mod alerta_gafete_queries;
pub mod cita_queries;
pub mod contratista_queries;
pub mod empresa_queries;
pub mod gafete_queries;

pub mod ingreso_contratista_queries; // Contratistas
pub mod ingreso_general_queries; // General (Logs, Historial completo)
pub mod ingreso_proveedor_queries; // Proveedores
pub mod ingreso_visita_queries; // Visitas

pub mod lista_negra_queries;
pub mod proveedor_queries;
pub mod user_queries;
pub mod vehiculo_queries;
pub mod visitante_queries;

pub mod audit_queries; // Tablas de auditoría

/// Inicializa la conexión a la base de datos (Pool) tomando en cuenta el modo Demo
pub async fn init_pool(config: &AppConfig) -> Result<SqlitePool, Box<dyn std::error::Error>> {
    // Verificar si estamos en modo Demo
    let db_path = if config.setup.show_demo_mode {
        log::info!("🧪 Arranque en Modo Demo detectado. Usando DB aislada.");
        crate::config::manager::get_demo_database_path()
    } else {
        crate::config::manager::get_database_path(config)
    };

    init_pool_by_path(&db_path).await
}

/// Inicializa un pool en una ruta específica (útil para Demo o restauraciones)
pub async fn init_pool_by_path(
    db_path: &std::path::Path,
) -> Result<SqlitePool, Box<dyn std::error::Error>> {
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

    let options = sqlx::sqlite::SqliteConnectOptions::from_str(&db_url)?
        .log_statements(log::LevelFilter::Off)
        .log_slow_statements(log::LevelFilter::Warn, std::time::Duration::from_secs(3));

    let pool = SqlitePoolOptions::new()
        .max_connections(1) // Desktop Optimization
        .min_connections(0)
        .idle_timeout(std::time::Duration::from_secs(60))
        .connect_with(options)
        .await?;

    // Optimizaciones de SQLite para mejor rendimiento y menor memoria
    sqlx::query(
        r#"
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;
        PRAGMA cache_size = -2000;      -- Reduce a ~2MB
        PRAGMA temp_store = MEMORY;
        PRAGMA mmap_size = 0;           -- Desactiva mmap de SQLite
        PRAGMA foreign_keys = ON;
        "#,
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}
