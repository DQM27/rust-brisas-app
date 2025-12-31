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
    use crate::domain::role::{GodModeGuard, ROLE_GUARDIA_SENIOR_ID};

    // Activar God Mode para el seed
    let _guard = GodModeGuard::activate();
    let db = get_db().await?;

    // (id, name, desc, inherits_from)
    let roles = [
        (ROLE_ADMIN_ID, "Administrador", "Acceso completo al sistema", None),
        (ROLE_SUPERVISOR_ID, "Supervisor", "Supervisión de operaciones", None),
        (
            ROLE_GUARDIA_SENIOR_ID,
            "Guardia Senior",
            "Supervisor de turno con permisos de edición",
            Some(ROLE_GUARDIA_ID),
        ),
        (ROLE_GUARDIA_ID, "Guardia", "Registro de ingresos", None),
    ];

    // Generar permisos para admin (todos)
    let all_permissions: Vec<String> = Module::all()
        .iter()
        .flat_map(|m| {
            Action::all()
                .iter()
                .map(|a| format!("{}:{}", m.as_str(), a.as_str()))
                .collect::<Vec<_>>()
        })
        .collect();

    // Permisos base para guardia
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

    // Permisos extra para guardia senior (edición de ingresos)
    let senior_perms = vec!["ingresos:update", "vehiculos:view", "vehiculos:read"];

    // Permisos para supervisor (gestión pero no borrado masivo)
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
        "settings_general:view",
        "settings_general:read",
        "settings_visual:view",
        "settings_visual:read",
        "settings_visual:update",
        "settings_security:view",
        "settings_security:read",
        "settings_sessions:view",
        "settings_sessions:read",
    ];

    for (id, name, desc, inherits) in roles {
        let permissions: Vec<String> = match id {
            ROLE_ADMIN_ID => all_permissions.clone(),
            ROLE_SUPERVISOR_ID => supervisor_perms.iter().map(|s| s.to_string()).collect(),
            ROLE_GUARDIA_SENIOR_ID => senior_perms.iter().map(|s| s.to_string()).collect(),
            ROLE_GUARDIA_ID => guardia_perms.iter().map(|s| s.to_string()).collect(),
            _ => vec![],
        };

        // Eliminar y re-crear
        db.query("DELETE type::thing('role', $id)").bind(("id", id)).await?;

        db.query(
            r#"
                CREATE type::thing('role', $id) CONTENT {
                    name: $name,
                    description: $desc,
                    is_system: true,
                    inherits_from: $inherits,
                    permissions: $permissions,
                    created_at: time::now(),
                    updated_at: time::now()
                }
                "#,
        )
        .bind(("id", id))
        .bind(("name", name))
        .bind(("desc", desc))
        .bind(("inherits", inherits.map(|i| RecordId::from_table_key("role", i))))
        .bind(("permissions", permissions))
        .await?
        .check()?;
    }

    info!("✅ Roles del sistema creados/actualizados con herencia");
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

    let password = std::env::var("BRISAS_ROOT_PASSWORD").unwrap_or_else(|_| "desing27".to_string());
    let password_hash =
        hash_password(&password).map_err(|e| SurrealDbError::Query(e.to_string()))?;

    if !existing.is_empty() {
        // Actualizar el superuser existente con los nuevos campos
        db.query(
            r#"
            UPDATE type::thing('user', $id) SET
                nombre = "DQM27",
                apellido = "",
                is_superuser = true,
                must_change_password = true,
                password_hash = $password_hash,
                updated_at = time::now()
            "#,
        )
        .bind(("id", SUPERUSER_ID))
        .bind(("password_hash", password_hash))
        .await?
        .check()?;

        info!("🔐 Superuser actualizado con nuevos campos");
        return Ok(());
    }

    // Crear nuevo superuser si no existe
    db.query(
        r#"
            CREATE user CONTENT {
                id: type::thing('user', $id),
                email: $email,
                password_hash: $password_hash,
                nombre: "DQM27",
                apellido: "",
                role: type::thing('role', $role_id),
                is_active: true,
                is_superuser: true,
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
    .check()?;

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
        info!("👤 Admin de desarrollo ya existe. Verificando rol...");
        // Auto-heal: Asegurar que tenga el rol de admin correcto
        // Esto corrige el problema donde el usuario pierde permisos si el frontend envía un rol incorrecto
        db.query("UPDATE user SET role = type::thing('role', $role_id) WHERE email = $email")
            .bind(("role_id", ROLE_ADMIN_ID))
            .bind(("email", "daniel.bleach1@gmail.com"))
            .await?;
        info!("🔧 Rol de admin verificado/restaurado");
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
