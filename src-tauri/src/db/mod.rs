// ==========================================
// src/db/mod.rs
// ==========================================

use sqlx::{
    sqlite::{SqlitePoolOptions, SqliteConnectOptions},
    migrate::Migrator,
    SqlitePool,
};
use std::path::PathBuf;
use std::str::FromStr;

// ==========================================
// CONFIGURACIÓN
// ==========================================

/// Directorio donde se almacena la base de datos
const DB_DIR: &str = "./data";
/// Nombre del archivo de base de datos
const DB_FILE: &str = "brisas.db";
/// Número máximo de conexiones al pool
const MAX_CONNECTIONS: u32 = 5;

/// Migrator estático embebido
static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

// ==========================================
// INICIALIZACIÓN
// ==========================================

/// Inicializa la base de datos SQLite con pool de conexiones
/// 
/// # Errores
/// 
/// Retorna error si:
/// - No se puede crear el directorio de datos
/// - No se puede conectar a la base de datos
/// - Fallan las migraciones
/// - Falla el seeding inicial
pub async fn init_db() -> Result<SqlitePool, DbInitError> {
    log_section("Inicializando Base de Datos");
    
    // Crear pool de conexiones
    let pool = create_pool().await?;
    
    // Ejecutar migraciones
    run_migrations(&pool).await?;
    
    // Seed inicial
    seed_initial_data(&pool).await?;
    
    log_success("Base de datos inicializada correctamente");
    
    Ok(pool)
}

// ==========================================
// CREACIÓN DEL POOL
// ==========================================

async fn create_pool() -> Result<SqlitePool, DbInitError> {
    let db_path = get_db_path()?;
    let db_exists = db_path.exists();
    
    log_info(&format!("Base de datos: {}", db_path.display()));
    
    if !db_exists {
        log_info("Creando nueva base de datos...");
    }
    
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());
    
    let options = SqliteConnectOptions::from_str(&db_url)?
        .create_if_missing(true)
        .foreign_keys(true)  // ✅ Foreign keys activadas por defecto
        .busy_timeout(std::time::Duration::from_secs(30));
    
    let pool = SqlitePoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .connect_with(options)
        .await?;
    
    Ok(pool)
}

fn get_db_path() -> Result<PathBuf, DbInitError> {
    let db_dir = PathBuf::from(DB_DIR);
    
    // Crear directorio si no existe
    std::fs::create_dir_all(&db_dir)
        .map_err(|e| DbInitError::CreateDirError(e.to_string()))?;
    
    Ok(db_dir.join(DB_FILE))
}

// ==========================================
// MIGRACIONES
// ==========================================

async fn run_migrations(pool: &SqlitePool) -> Result<(), DbInitError> {
    log_info("Ejecutando migraciones...");
    
    MIGRATOR.run(pool).await.map_err(|e| {
        log_error(&format!("Error en migraciones: {}", e));
        log_error(&format!(
            "💡 Para reiniciar, elimina: {}",
            get_db_path().unwrap().display()
        ));
        DbInitError::MigrationError(e.to_string())
    })?;
    
    log_success("Migraciones completadas");
    
    Ok(())
}

// ==========================================
// SEEDING
// ==========================================

async fn seed_initial_data(pool: &SqlitePool) -> Result<(), DbInitError> {
    seed_admin_user(pool).await?;
    // Aquí puedes agregar más seeds:
    // seed_gafete_sin_gafete(pool).await?;
    // seed_empresas_default(pool).await?;
    Ok(())
}

/// Crea el usuario administrador por defecto si no existe
async fn seed_admin_user(pool: &SqlitePool) -> Result<(), DbInitError> {
    const ADMIN_EMAIL: &str = "daniel.bleach1@gmail.com";
    const ADMIN_PASSWORD: &str = "daniel27";
    
    // Verificar si ya existe
    let count: i32 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM users WHERE email = ?"
    )
    .bind(ADMIN_EMAIL)
    .fetch_one(pool)
    .await?;
    
    if count > 0 {
        log_info("Usuario admin ya existe");
        return Ok(());
    }
    
    log_info("Creando usuario admin inicial...");
    
    // Crear usuario
    let id = uuid::Uuid::new_v4().to_string();
    let password_hash = hash_password(ADMIN_PASSWORD)?;
    let now = chrono::Utc::now().to_rfc3339();
    
    sqlx::query(
        r#"INSERT INTO users 
           (id, email, password_hash, nombre, apellido, role, is_active, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#
    )
    .bind(&id)
    .bind(ADMIN_EMAIL)
    .bind(&password_hash)
    .bind("Daniel")
    .bind("Quintana")
    .bind("admin")
    .bind(true)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;
    
    log_success("Usuario admin creado exitosamente");
    log_info(&format!("   📧 Email: {}", ADMIN_EMAIL));
    log_info(&format!("   🔑 Contraseña: {}", ADMIN_PASSWORD));
    
    Ok(())
}

// ==========================================
// HELPERS
// ==========================================

/// Hash de contraseña usando bcrypt
fn hash_password(password: &str) -> Result<String, DbInitError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|e| DbInitError::HashError(e.to_string()))
}

// ==========================================
// ERRORES
// ==========================================

#[derive(Debug, thiserror::Error)]
pub enum DbInitError {
    #[error("Error creando directorio: {0}")]
    CreateDirError(String),
    
    #[error("Error de conexión a la base de datos: {0}")]
    ConnectionError(#[from] sqlx::Error),
    
    #[error("Error en migraciones: {0}")]
    MigrationError(String),
    
    #[error("Error al generar hash: {0}")]
    HashError(String),
    
    #[error("Error al parsear opciones: {0}")]
    ParseError(#[from] std::str::ParseError),
}

// ==========================================
// LOGGING HELPERS
// ==========================================

fn log_section(msg: &str) {
    println!("\n{'═':<50}", "");
    println!("  {}", msg);
    println!("{'═':<50}\n", "");
}

fn log_info(msg: &str) {
    println!("ℹ️  {}", msg);
}

fn log_success(msg: &str) {
    println!("✅ {}", msg);
}

fn log_error(msg: &str) {
    eprintln!("❌ {}", msg);
}

// ==========================================
// UTILIDADES PÚBLICAS
// ==========================================

/// Ejecuta un query de verificación de salud de la base de datos
pub async fn health_check(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1")
        .execute(pool)
        .await?;
    
    Ok(())
}

/// Obtiene estadísticas del pool de conexiones
pub fn pool_stats(pool: &SqlitePool) -> PoolStats {
    PoolStats {
        size: pool.size(),
        idle: pool.num_idle(),
    }
}

#[derive(Debug)]
pub struct PoolStats {
    pub size: u32,
    pub idle: usize,
}

// ==========================================
// TESTS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_init_db() {
        let pool = init_db().await;
        assert!(pool.is_ok());
    }
    
    #[tokio::test]
    async fn test_health_check() {
        let pool = init_db().await.unwrap();
        assert!(health_check(&pool).await.is_ok());
    }
    
    #[test]
    fn test_hash_password() {
        let hash = hash_password("test123");
        assert!(hash.is_ok());
        assert_ne!(hash.unwrap(), "test123");
    }
}