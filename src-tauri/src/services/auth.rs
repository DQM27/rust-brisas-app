// ==========================================
// src/services/auth.rs
// ==========================================
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, Algorithm, Version, Params
};

/// Hashea una contraseña usando Argon2id
pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    
    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::default(),
    );
    
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| format!("Error al hashear contraseña: {}", e))
}

/// Verifica una contraseña contra un hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, String> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| format!("Hash inválido: {}", e))?;
    
    let argon2 = Argon2::default();
    
    Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
}

// ==========================================
// src/db/mod.rs
// ==========================================
use sqlx::{sqlite::SqlitePoolOptions, migrate::Migrator};
use std::{env, path::Path};

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn init_db() -> Result<sqlx::SqlitePool, Box<dyn std::error::Error>> {
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:./data/brisas.db".to_string());
    
    // Crea carpeta data/
    let db_path = db_url.strip_prefix("sqlite:").unwrap_or(&db_url);
    if let Some(parent) = Path::new(db_path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    MIGRATOR.run(&pool).await?;

    println!("✅ BD creada en: {}", db_path);
    Ok(pool)
}