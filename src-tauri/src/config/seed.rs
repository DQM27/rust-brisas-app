// src/config/seed.rs
// ====================
// Seeds esenciales para SurrealDB
// ====================

use crate::domain::role::{
    ROLE_ADMIN_ID, ROLE_GUARDIA_ID, ROLE_SUPERVISOR_ID, SUPERUSER_EMAIL, SUPERUSER_ID,
};
use crate::models::role::{Action, Module};
use crate::services::auth::hash_password;
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use log::info;
use surrealdb::RecordId;

/// Orquesta la ejecución de todos los seeds
pub async fn seed_db() -> Result<(), Box<dyn std::error::Error>> {
    seed_roles().await?;
    seed_superuser().await?;
    seed_admin_user().await?;
    Ok(())
}

/// Seed de roles del sistema
async fn seed_roles() -> Result<(), SurrealDbError> {
    let db = get_db().await?;

    let roles = [
        (ROLE_ADMIN_ID, "Administrador", "Acceso completo al sistema"),
        (ROLE_SUPERVISOR_ID, "Supervisor", "Supervisión de operaciones"),
        (ROLE_GUARDIA_ID, "Guardia", "Registro de ingresos"),
    ];

    // Generar permisos para admin
    let all_permissions: Vec<String> = Module::all()
        .iter()
        .flat_map(|m| {
            Action::all()
                .iter()
                .map(|a| format!("{}:{}", m.as_str(), a.as_str()))
                .collect::<Vec<_>>()
        })
        .collect();

    // Permisos para supervisor
    let supervisor_perms = vec![
        "users:view",
        "users:read",
        "contratistas:view",
        "contratistas:create",
        "contratistas:read",
        "contratistas:update",
        "empresas:view",
        "empresas:create",
        "empresas:read",
        "empresas:update",
        "ingresos:view",
        "ingresos:create",
        "ingresos:read",
        "ingresos:update",
        "citas:view",
        "citas:create",
        "citas:read",
        "citas:update",
        "vehiculos:view",
        "vehiculos:create",
        "vehiculos:read",
        "vehiculos:update",
        "gafetes:view",
        "gafetes:create",
        "gafetes:read",
        "gafetes:update",
        "lista_negra:view",
        "lista_negra:create",
        "lista_negra:read",
        "config:view",
        "config:read",
        "export:view",
        "export:export",
        "proveedores:view",
        "proveedores:create",
        "proveedores:read",
        "proveedores:update",
        "visitantes:view",
        "visitantes:create",
        "visitantes:read",
        "visitantes:update",
    ];

    // Permisos para guardia
    let guardia_perms = vec![
        "contratistas:view",
        "contratistas:read",
        "ingresos:view",
        "ingresos:create",
        "ingresos:read",
        "citas:view",
        "citas:read",
        "lista_negra:view",
        "lista_negra:read",
        "proveedores:view",
        "proveedores:read",
        "visitantes:view",
        "visitantes:read",
    ];

    for (id, name, desc) in roles {
        let permissions: Vec<String> = if id == ROLE_ADMIN_ID {
            all_permissions.clone()
        } else if id == ROLE_SUPERVISOR_ID {
            supervisor_perms.iter().map(|s| s.to_string()).collect()
        } else {
            guardia_perms.iter().map(|s| s.to_string()).collect()
        };

        // Primero eliminar cualquier registro existente (puede estar corrupto)
        db.query("DELETE type::thing('role', $id)").bind(("id", id)).await?;

        // Luego crear el rol fresco - debemos incluir created_at explícitamente
        // porque READONLY + VALUE no funciona bien con CREATE CONTENT
        db.query(
            r#"
                CREATE type::thing('role', $id) CONTENT {
                    name: $name,
                    description: $desc,
                    is_system: true,
                    permissions: $permissions,
                    created_at: time::now(),
                    updated_at: time::now()
                }
                "#,
        )
        .bind(("id", id))
        .bind(("name", name))
        .bind(("desc", desc))
        .bind(("permissions", permissions.clone()))
        .await?
        .check()?;
    }

    info!("✅ Roles del sistema creados/actualizados");
    Ok(())
}

/// Seed del superuser oculto
async fn seed_superuser() -> Result<(), SurrealDbError> {
    let db = get_db().await?;

    // Verificar si ya existe usando SELECT VALUE id
    let existing: Vec<RecordId> = db
        .query("SELECT VALUE id FROM user WHERE id = type::thing('user', $id)")
        .bind(("id", SUPERUSER_ID))
        .await?
        .take(0)?;

    if !existing.is_empty() {
        info!("🔐 Superuser ya existe, saltando creación.");
        return Ok(());
    }

    let password = std::env::var("BRISAS_ROOT_PASSWORD").unwrap_or_else(|_| "desing27".to_string());
    let password_hash =
        hash_password(&password).map_err(|e| SurrealDbError::Query(e.to_string()))?;

    // No deserializamos el resultado para evitar el bug de serde_json::Value con SurrealDB 2.x
    db.query(
        r#"
            CREATE user CONTENT {
                id: type::thing('user', $id),
                email: $email,
                password_hash: $password_hash,
                nombre: "System",
                apellido: "Root",
                role: type::thing('role', $role_id),
                is_active: true,
                cedula: "0000000000",
                must_change_password: true,
                created_at: time::now(),
                updated_at: time::now()
            }
            "#,
    )
    .bind(("id", SUPERUSER_ID))
    .bind(("email", SUPERUSER_EMAIL))
    .bind(("password_hash", password_hash.clone()))
    .bind(("role_id", ROLE_ADMIN_ID))
    .await?
    .check()?; // Solo verificar errores, no deserializar

    info!("✅ Superuser creado");
    Ok(())
}

/// Seed del primer admin (SOLO DESARROLLO)
async fn seed_admin_user() -> Result<(), SurrealDbError> {
    let db = get_db().await?;

    // Verificar si ya existe usando SELECT VALUE id
    let existing: Vec<RecordId> = db
        .query("SELECT VALUE id FROM user WHERE email = $email")
        .bind(("email", "daniel.bleach1@gmail.com"))
        .await?
        .take(0)?;

    if !existing.is_empty() {
        info!("👤 Admin de desarrollo ya existe, saltando creación.");
        return Ok(());
    }

    let id = uuid::Uuid::new_v4().to_string();
    let password_hash =
        hash_password("desing27").map_err(|e| SurrealDbError::Query(e.to_string()))?;

    // No deserializamos el resultado para evitar el bug de serde_json::Value con SurrealDB 2.x
    db.query(
        r#"
            CREATE user CONTENT {
                id: type::thing('user', $id),
                email: "daniel.bleach1@gmail.com",
                password_hash: $password_hash,
                nombre: "Daniel",
                apellido: "Quintana",
                role: type::thing('role', $role_id),
                is_active: true,
                cedula: "155824395105",
                must_change_password: true,
                created_at: time::now(),
                updated_at: time::now()
            }
            "#,
    )
    .bind(("id", id.clone()))
    .bind(("password_hash", password_hash.clone()))
    .bind(("role_id", ROLE_ADMIN_ID))
    .await?
    .check()?; // Solo verificar errores, no deserializar

    info!("✅ Admin de desarrollo creado");
    Ok(())
}
