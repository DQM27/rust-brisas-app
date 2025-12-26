// ==========================================
// src/config/surrealdb_seed.rs
// ==========================================
// Seeds para SurrealDB (separados de SQLite)

use crate::domain::role::{
    ROLE_ADMIN_ID, ROLE_GUARDIA_ID, ROLE_SUPERVISOR_ID, SUPERUSER_EMAIL, SUPERUSER_ID,
};
use crate::services::auth::hash_password;
use crate::services::surrealdb_service::{get_surrealdb, SurrealDbError};

/// Ejecuta todos los seeds para SurrealDB
pub async fn seed_surrealdb() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌱 [SURREALDB] Ejecutando seeds...");

    seed_roles().await?;
    seed_superuser().await?;
    seed_admin_user().await?;

    println!("✅ [SURREALDB] Seeds completados");
    Ok(())
}

/// Seed de roles del sistema
async fn seed_roles() -> Result<(), SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    let now = chrono::Utc::now().to_rfc3339();

    // Crear roles usando UPSERT para que sea idempotente
    client
        .query(
            r#"
            UPSERT roles:admin CONTENT {
                name: 'Administrador',
                description: 'Acceso completo al sistema',
                is_system: true,
                created_at: $now,
                updated_at: $now
            };
            
            UPSERT roles:supervisor CONTENT {
                name: 'Supervisor',
                description: 'Supervisión de operaciones',
                is_system: true,
                created_at: $now,
                updated_at: $now
            };
            
            UPSERT roles:guardia CONTENT {
                name: 'Guardia',
                description: 'Registro de ingresos',
                is_system: true,
                created_at: $now,
                updated_at: $now
            };
            "#,
        )
        .bind(("now", now))
        .await?;

    println!("  ✓ Roles creados");
    Ok(())
}

/// Seed del superuser
async fn seed_superuser() -> Result<(), SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    // Verificar si ya existe
    let mut result = client
        .query("SELECT count() FROM usuarios WHERE id = type::thing('usuarios', $id) GROUP ALL")
        .bind(("id", SUPERUSER_ID.to_string()))
        .await?;

    #[derive(serde::Deserialize)]
    struct CountResult {
        count: i32,
    }

    let counts: Vec<CountResult> = result.take(0)?;
    if counts.into_iter().next().map(|c| c.count).unwrap_or(0) > 0 {
        println!("  ✓ Superuser ya existe");
        return Ok(());
    }

    let password = std::env::var("BRISAS_ROOT_PASSWORD").unwrap_or_else(|_| "desing27".to_string());
    let password_hash =
        hash_password(&password).map_err(|e| SurrealDbError::Init(e.to_string()))?;
    let now = chrono::Utc::now().to_rfc3339();

    client
        .query(
            r#"
            CREATE type::thing('usuarios', $id) CONTENT {
                email: $email,
                password: $password,
                nombre: 'System',
                apellido: 'Root',
                role: roles:admin,
                is_active: true,
                created_at: $now,
                updated_at: $now,
                cedula: '0000000000',
                must_change_password: true,
                deleted_at: NONE
            }
            "#,
        )
        .bind(("id", SUPERUSER_ID.to_string()))
        .bind(("email", SUPERUSER_EMAIL.to_string()))
        .bind(("password", password_hash))
        .bind(("now", now))
        .await?;

    println!("  ✓ Superuser creado");
    Ok(())
}

/// Seed del admin de desarrollo
async fn seed_admin_user() -> Result<(), SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    // Verificar si ya existe
    let mut result = client
        .query("SELECT count() FROM usuarios WHERE email = $email AND deleted_at IS NONE GROUP ALL")
        .bind(("email", "daniel.bleach1@gmail.com".to_string()))
        .await?;

    #[derive(serde::Deserialize)]
    struct CountResult {
        count: i32,
    }

    let counts: Vec<CountResult> = result.take(0)?;
    if counts.into_iter().next().map(|c| c.count).unwrap_or(0) > 0 {
        println!("  ✓ Admin ya existe");
        return Ok(());
    }

    let id = uuid::Uuid::new_v4().to_string();
    let password_hash =
        hash_password("desing27").map_err(|e| SurrealDbError::Init(e.to_string()))?;
    let now = chrono::Utc::now().to_rfc3339();

    client
        .query(
            r#"
            CREATE type::thing('usuarios', $id) CONTENT {
                email: $email,
                password: $password,
                nombre: 'Daniel',
                apellido: 'Quintana',
                role: roles:admin,
                is_active: true,
                created_at: $now,
                updated_at: $now,
                cedula: '155824395105',
                must_change_password: true,
                deleted_at: NONE
            }
            "#,
        )
        .bind(("id", id))
        .bind(("email", "daniel.bleach1@gmail.com".to_string()))
        .bind(("password", password_hash))
        .bind(("now", now))
        .await?;

    println!("  ✓ Admin de desarrollo creado");
    Ok(())
}
