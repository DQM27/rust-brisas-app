// src/db/mod.rs
use sqlx::{sqlite::SqlitePoolOptions, migrate::Migrator};

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn init_db() -> Result<sqlx::SqlitePool, Box<dyn std::error::Error>> {
    // Usar directorio actual del proyecto
    let db_dir = std::path::PathBuf::from("./data");
    
    // Crear el directorio si no existe
    std::fs::create_dir_all(&db_dir)?;
    
    let db_path = db_dir.join("brisas.db");
    let db_url = format!("sqlite:{}", db_path.display());
    
    println!("💾 Base de datos: {}", db_path.display());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    println!("🔄 Ejecutando migraciones...");
    MIGRATOR.run(&pool).await?;

    println!("✅ Base de datos inicializada correctamente");
    Ok(pool)
}