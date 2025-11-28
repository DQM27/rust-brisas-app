// src/db/mod.rs

use crate::config::AppConfig;
use sqlx::{migrate::Migrator, sqlite::SqlitePoolOptions, SqlitePool};

pub mod blacklist_import_queries;
pub mod contratista_queries;
pub mod lista_negra_queries;
pub mod seed;
pub mod user_queries;
pub mod vehiculo_queries;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

/// Inicializa la conexión a la base de datos (Pool)
pub async fn init_pool(config: &AppConfig) -> Result<SqlitePool, Box<dyn std::error::Error>> {
    // Obtener ruta de la DB desde la configuración
    let db_path = crate::config::manager::get_database_path(config);

    let db_exists = db_path.exists();
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

    println!("💾 Base de datos: {}", db_path.display());
    if !db_exists {
        println!("🆕 Creando nueva base de datos...");
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    // Configuración básica de SQLite
    sqlx::query("PRAGMA foreign_keys = ON;")
        .execute(&pool)
        .await?;

    Ok(pool)
}

/// Ejecuta las migraciones pendientes
pub async fn run_migrations(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Ejecutando migraciones...");

    match MIGRATOR.run(pool).await {
        Ok(_) => {
            println!("✅ Migraciones completadas");
            Ok(())
        }
        Err(e) => {
            eprintln!("❌ Error en migraciones: {}", e);
            // No retornamos error fatal aquí para permitir que la app arranque si es un error menor,
            // pero en producción quizás deberíamos. Por ahora mantenemos comportamiento similar.
            // O mejor, retornamos el error para que init falle explícitamente.
            Err(e.into())
        }
    }
}
