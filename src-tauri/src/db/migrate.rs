// src/db/migrate.rs

use sqlx::{migrate::Migrator, SqlitePool};

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

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
            Err(e.into())
        }
    }
}
