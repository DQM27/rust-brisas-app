// src/db/seed.rs

use crate::services::auth::hash_password;
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

/// Orquesta la ejecución de todos los seeds
pub async fn seed_db(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    seed_admin_user(pool).await?;
    Ok(())
}

async fn seed_admin_user(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    // Verificar si ya existe el admin
    let count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE email = ?")
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
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
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
