// src/db/mod.rs

use sqlx::{sqlite::SqlitePoolOptions, migrate::Migrator, SqlitePool};
use crate::config::AppConfig;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn init_db(config: &AppConfig) -> Result<SqlitePool, Box<dyn std::error::Error>> {
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
    
    sqlx::query("PRAGMA foreign_keys = ON;").execute(&pool).await?;
    
    println!("🔄 Ejecutando migraciones...");
    
    match MIGRATOR.run(&pool).await {
        Ok(_) => println!("✅ Migraciones completadas"),
        Err(e) => {
            eprintln!("❌ Error en migraciones: {}", e);
            eprintln!("💡 Para empezar de cero, elimina: {}", db_path.display());
            return Err(e.into());
        }
    }
    
    // Crear usuario admin si no existe
    seed_admin_user(&pool).await?;
    
    println!("✅ Base de datos inicializada correctamente");
    
    Ok(pool)
}

async fn seed_admin_user(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    use crate::services::auth::hash_password;
    use uuid::Uuid;
    use chrono::Utc;
    
    // Verificar si ya existe el admin
    let count: i32 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM users WHERE email = ?"
    )
    .bind("daniel.bleach1@gmail.com")
    .fetch_one(pool)
    .await?;
    
    if count > 0 {
        println!("👤 Usuario admin ya existe");
        return Ok(());
    }
    
    println!("👤 Creando usuario admin inicial...");
    
    let id = Uuid::new_v4().to_string();
    let password_hash = hash_password("daniel27")?;
    let now = Utc::now().to_rfc3339();
    
    sqlx::query(
        r#"INSERT INTO users 
           (id, email, password_hash, nombre, apellido, role, is_active, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#
    )
    .bind(&id)
    .bind("daniel.bleach1@gmail.com")
    .bind(&password_hash)
    .bind("Daniel")
    .bind("Quintana")
    .bind("admin")
    .bind(1)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;
    
    println!("✅ Usuario admin creado exitosamente");
    println!("   📧 Email: daniel.bleach1@gmail.com");
    println!("   🔑 Contraseña: daniel27");
    
    Ok(())
}