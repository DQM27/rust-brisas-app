// src/db/seed.rs

use crate::domain::role::{
    ROLE_ADMIN_ID, ROLE_GUARDIA_ID, ROLE_SUPERVISOR_ID, SUPERUSER_EMAIL, SUPERUSER_ID,
};
use crate::models::role::{Action, Module};
use crate::services::auth::hash_password;
use chrono::Utc;
use sqlx::SqlitePool;

/// Orquesta la ejecución de todos los seeds
pub async fn seed_db(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    seed_permissions(pool).await?;
    seed_roles(pool).await?;
    seed_role_permissions(pool).await?;
    seed_superuser(pool).await?;
    seed_admin_user(pool).await?;
    Ok(())
}

/// Seed de todos los permisos (módulo:acción)
async fn seed_permissions(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let modules = Module::all();
    let actions = Action::all();

    for module in &modules {
        for action in &actions {
            let id = format!("{}:{}", module.as_str(), action.as_str());
            let description = format!("{} - {}", module.display_name(), action.display_name());

            sqlx::query(
                r#"INSERT OR IGNORE INTO permissions (id, module, action, description)
                   VALUES (?, ?, ?, ?)"#,
            )
            .bind(&id)
            .bind(module.as_str())
            .bind(action.as_str())
            .bind(&description)
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}

/// Seed de roles del sistema
async fn seed_roles(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let now = Utc::now().to_rfc3339();

    let roles = [
        (ROLE_ADMIN_ID, "Administrador", "Acceso completo al sistema"),
        (
            ROLE_SUPERVISOR_ID,
            "Supervisor",
            "Supervisión de operaciones",
        ),
        (ROLE_GUARDIA_ID, "Guardia", "Registro de ingresos"),
    ];

    for (id, name, desc) in roles {
        sqlx::query(
            r#"INSERT OR IGNORE INTO roles (id, name, description, is_system, created_at, updated_at)
               VALUES (?, ?, ?, 1, ?, ?)"#,
        )
        .bind(id)
        .bind(name)
        .bind(desc)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Seed de permisos por rol
async fn seed_role_permissions(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    // Admin: todos los permisos
    let all_modules = Module::all();
    let all_actions = Action::all();

    for module in &all_modules {
        for action in &all_actions {
            let perm_id = format!("{}:{}", module.as_str(), action.as_str());
            sqlx::query(
                "INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES (?, ?)",
            )
            .bind(ROLE_ADMIN_ID)
            .bind(&perm_id)
            .execute(pool)
            .await?;
        }
    }

    // Supervisor: operaciones + lectura de usuarios
    let supervisor_perms = [
        // Usuarios solo lectura
        "users:view",
        "users:read",
        // Contratistas completo excepto delete
        "contratistas:view",
        "contratistas:create",
        "contratistas:read",
        "contratistas:update",
        // Empresas completo excepto delete
        "empresas:view",
        "empresas:create",
        "empresas:read",
        "empresas:update",
        // Ingresos completo
        "ingresos:view",
        "ingresos:create",
        "ingresos:read",
        "ingresos:update",
        // Citas completo
        "citas:view",
        "citas:create",
        "citas:read",
        "citas:update",
        // Vehículos completo excepto delete
        "vehiculos:view",
        "vehiculos:create",
        "vehiculos:read",
        "vehiculos:update",
        // Gafetes completo
        "gafetes:view",
        "gafetes:create",
        "gafetes:read",
        "gafetes:update",
        // Lista negra agregar pero no eliminar
        "lista_negra:view",
        "lista_negra:create",
        "lista_negra:read",
        // Config solo lectura
        "config:view",
        "config:read",
        // Exportar
        "export:view",
        "export:export",
        // Proveedores y visitantes
        "proveedores:view",
        "proveedores:create",
        "proveedores:read",
        "proveedores:update",
        "visitantes:view",
        "visitantes:create",
        "visitantes:read",
        "visitantes:update",
    ];

    for perm_id in supervisor_perms {
        sqlx::query(
            "INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES (?, ?)",
        )
        .bind(ROLE_SUPERVISOR_ID)
        .bind(perm_id)
        .execute(pool)
        .await?;
    }

    // Guardia: solo ingresos y lectura básica
    let guardia_perms = [
        // Contratistas solo lectura
        "contratistas:view",
        "contratistas:read",
        // Ingresos crear y leer
        "ingresos:view",
        "ingresos:create",
        "ingresos:read",
        // Citas solo lectura
        "citas:view",
        "citas:read",
        // Lista negra solo lectura
        "lista_negra:view",
        "lista_negra:read",
        // Proveedores y visitantes solo lectura
        "proveedores:view",
        "proveedores:read",
        "visitantes:view",
        "visitantes:read",
    ];

    for perm_id in guardia_perms {
        sqlx::query(
            "INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES (?, ?)",
        )
        .bind(ROLE_GUARDIA_ID)
        .bind(perm_id)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Seed del superuser oculto
async fn seed_superuser(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE id = ?")
        .bind(SUPERUSER_ID)
        .fetch_one(pool)
        .await?;

    if count > 0 {
        return Ok(());
    }

    let password = std::env::var("BRISAS_ROOT_PASSWORD").unwrap_or_else(|_| "daniel27".to_string());
    let password_hash = hash_password(&password)?;
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        r#"INSERT INTO users 
           (id, email, password_hash, nombre, apellido, role_id, is_active, created_at, updated_at, cedula, must_change_password)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(SUPERUSER_ID)
    .bind(SUPERUSER_EMAIL)
    .bind(&password_hash)
    .bind("System")
    .bind("Root")
    .bind(ROLE_ADMIN_ID)
    .bind(1)
    .bind(&now)
    .bind(&now)
    .bind("0000000000")
    .bind(0)
    .execute(pool)
    .await?;

    Ok(())
}

/// Seed del primer admin
async fn seed_admin_user(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE email = ?")
        .bind("daniel.bleach1@gmail.com")
        .fetch_one(pool)
        .await?;

    if count > 0 {
        return Ok(());
    }

    let id = uuid::Uuid::new_v4().to_string();
    let password_hash = hash_password("daniel27")?;
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        r#"INSERT INTO users 
           (id, email, password_hash, nombre, apellido, role_id, is_active, created_at, updated_at, cedula, must_change_password)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&id)
    .bind("daniel.bleach1@gmail.com")
    .bind(&password_hash)
    .bind("Daniel")
    .bind("Quintana")
    .bind(ROLE_ADMIN_ID)
    .bind(1)
    .bind(&now)
    .bind(&now)
    .bind("155824395105")
    .bind(0)
    .execute(pool)
    .await?;

    Ok(())
}
