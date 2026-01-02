// ==========================================================================
// src/config/seed.rs
// ==========================================================================
// Gestión de Datos Iniciales (Seeds) para SurrealDB.
//
// Este módulo se encarga de garantizar que la base de datos tenga siempre los
// registros críticos necesarios para el funcionamiento del sistema:
// 1. Roles base (Administrador, Guardia).
// 2. Usuario raíz (God) con acceso total bypass.
// 3. Usuario administrador de desarrollo (para pruebas locales).
// ==========================================================================

use crate::domain::role::{GOD_EMAIL, GOD_ID, ROLE_ADMIN_ID, ROLE_GUARDIA_ID};
use crate::models::role::{Action, Module};
use crate::services::auth::hash_password;
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use log::info;
use surrealdb::RecordId;

/// Orquesta la ejecución secuencial de todos los seeds del sistema.
/// Debe ser invocado durante el arranque de la aplicación si el estado es 'Setup'.
pub async fn seed_db() -> Result<(), Box<dyn std::error::Error>> {
    info!("🚀 Iniciando proceso de seeding de base de datos...");
    seed_roles().await?;
    seed_god_user().await?;
    seed_admin_user().await?;
    info!("✨ Proceso de seeding completado satisfactoriamente");
    Ok(())
}

/// Provisiona los roles fundamentales del sistema.
///
/// Este proceso es destructivo para los roles de sistema (los elimina y re-crea)
/// para asegurar que los permisos estén siempre actualizados con la última
/// versión del código (Action/Module).
async fn seed_roles() -> Result<(), SurrealDbError> {
    use crate::domain::role::GodModeGuard;

    // Activamos el GodModeGuard para poder manipular roles protegidos
    // sin que el middleware de seguridad bloquee la operación.
    let _guard = GodModeGuard::activate();
    let db = get_db().await?;

    // Definición de la matriz de roles base (id, nombre, descripción, herencia)
    let roles = [
        (ROLE_ADMIN_ID, "Administrador", "Acceso completo al sistema", None::<&'static str>),
        (
            ROLE_GUARDIA_ID,
            "Guardia",
            "Registro de ingresos y consultas básicas",
            None::<&'static str>,
        ),
    ];

    // Recolectamos todos los permisos posibles definidos en el código.
    // El Administrador siempre recibe el set completo.
    let all_permissions: Vec<String> = Module::all()
        .iter()
        .flat_map(|m| {
            Action::all()
                .iter()
                .map(|a| format!("{}:{}", m.as_str(), a.as_str()))
                .collect::<Vec<_>>()
        })
        .collect();

    // Set restringido de permisos para el rol de Guardia.
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

    for (id, name, desc, inherits) in roles {
        let permissions: Vec<String> = match id {
            ROLE_ADMIN_ID => all_permissions.clone(),
            ROLE_GUARDIA_ID => guardia_perms.iter().map(|s| (*s).to_string()).collect(),
            _ => vec![],
        };

        // Limpieza previa para asegurar consistencia
        db.query("DELETE type::thing('role', $id)").bind(("id", id)).await?;

        // Inserción del rol con metadatos de sistema
        db.query(
            r"
                CREATE type::thing('role', $id) CONTENT {
                    name: $name,
                    description: $desc,
                    is_system: true,
                    inherits_from: $inherits,
                    permissions: $permissions,
                    created_at: time::now(),
                    updated_at: time::now()
                }
                ",
        )
        .bind(("id", id))
        .bind(("name", name))
        .bind(("desc", desc))
        .bind(("inherits", inherits.map(|i| RecordId::from_table_key("role", i))))
        .bind(("permissions", permissions))
        .await?
        .check()?;
    }

    info!("✅ Roles base de sistema sincronizados");
    Ok(())
}

/// Garantiza la existencia del usuario raíz ("God User").
///
/// Este usuario es especial porque posee un ID fijo referenciado en el código
/// para otorgar autoridad total (God Authority). No depende de flags en BD.
async fn seed_god_user() -> Result<(), SurrealDbError> {
    let db = get_db().await?;

    // Verificamos existencia por ID estricto
    let existing: Vec<RecordId> = db
        .query("SELECT VALUE id FROM user WHERE id = type::thing('user', $id)")
        .bind(("id", GOD_ID))
        .await?
        .take(0)?;

    // Password por defecto: 'desing27' (Debe ser cambiada inmediatamente)
    let password_hash =
        hash_password("desing27").map_err(|e| SurrealDbError::Query(e.to_string()))?;

    if !existing.is_empty() {
        // En cada arranque, refrescamos los datos críticos del God User (Auto-healing)
        db.query(
            r#"
            UPDATE type::thing('user', $id) SET
                nombre = "DQM27",
                apellido = "SYSTEM",
                must_change_password = true,
                password_hash = $password_hash,
                updated_at = time::now()
            "#,
        )
        .bind(("id", GOD_ID))
        .bind(("password_hash", password_hash))
        .await?
        .check()?;

        info!("🔐 Usuario raíz actualizado (id={GOD_ID})");
        return Ok(());
    }

    // Creación inicial si la base de datos está vacía
    db.query(
        r#"
            CREATE user CONTENT {
                id: type::thing('user', $id),
                email: $email,
                password_hash: $password_hash,
                nombre: "DQM27",
                apellido: "SYSTEM",
                role: type::thing('role', $role_id),
                is_active: true,
                cedula: "0000000000",
                must_change_password: true,
                created_at: time::now(),
                updated_at: time::now()
            }
            "#,
    )
    .bind(("id", GOD_ID))
    .bind(("email", GOD_EMAIL))
    .bind(("password_hash", password_hash.clone()))
    .bind(("role_id", ROLE_ADMIN_ID))
    .await?
    .check()?;

    info!("✅ Usuario raíz (God) creado con éxito");
    Ok(())
}

/// Seed para el administrador de Daniel Quintana (Entorno de Desarrollo).
///
/// Incluye lógica de 'Auto-healing' para restaurar el rol de admin si este
/// es modificado accidentalmente mediante la interfaz de usuario.
async fn seed_admin_user() -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    let dev_email = "daniel.bleach1@gmail.com";

    let existing: Vec<RecordId> = db
        .query("SELECT VALUE id FROM user WHERE email = $email")
        .bind(("email", dev_email))
        .await?
        .take(0)?;

    if !existing.is_empty() {
        // Auto-heal: Aseguramos que Daniel tenga siempre el rol de administrador.
        db.query("UPDATE user SET role = type::thing('role', $role_id) WHERE email = $email")
            .bind(("role_id", ROLE_ADMIN_ID))
            .bind(("email", dev_email))
            .await?;
        return Ok(());
    }

    let id = uuid::Uuid::new_v4().to_string();
    let password_hash =
        hash_password("desing27").map_err(|e| SurrealDbError::Query(e.to_string()))?;

    db.query(
        r#"
            CREATE user CONTENT {
                id: type::thing('user', $id),
                email: $email,
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
    .bind(("id", id))
    .bind(("email", dev_email))
    .bind(("password_hash", password_hash.clone()))
    .bind(("role_id", ROLE_ADMIN_ID))
    .await?
    .check()?;

    info!("✅ Administrador de desarrollo registrado");
    Ok(())
}
