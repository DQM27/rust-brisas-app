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

use crate::config::manager::save_config;
use crate::config::settings::AppConfigState;

/// Orquesta la ejecución secuencial de todos los seeds del sistema.
/// Debe ser invocado durante el arranque de la aplicación si el estado es 'Setup'.
pub async fn seed_db(config_state: AppConfigState) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Verificación de estado previo
    {
        let config = config_state.read().unwrap();
        if config.setup.is_seeded {
            info!("🌱 Sistema ya sembrado previamente. Omitiendo regeneración completa.");
            return Ok(());
        }
    }

    info!("🚀 Iniciando proceso de seeding de base de datos...");

    // 2. Ejecución de semillas
    seed_roles().await?;
    seed_god_user().await?;
    seed_admin_user().await?;
    seed_modules().await?;

    // 3. Registrar éxito y persistir
    {
        let mut config = config_state.write().unwrap();
        config.setup.is_seeded = true;

        if let Err(e) = save_config(&config, &crate::config::manager::get_default_config_path()) {
            log::error!("❌ Error guardando estado de seed: {e}");
        } else {
            info!("✨ Estado de seed guardado en configuración.");
        }
    }

    info!("✨ Proceso de seeding completado satisfactoriamente");
    Ok(())
}

/// Inicializa el catálogo de módulos del sistema.
/// Solo crea los registros si no existen, para no sobrescribir estados personalizados (ej. 'maintenance').
async fn seed_modules() -> Result<(), SurrealDbError> {
    let db = get_db().await?;

    // Lista de módulos canónicos del sistema
    let modules = [
        ("users", "Gestión de Usuarios"),
        ("contractors", "Gestión de Contratistas"),
        ("access_control", "Control de Acceso"),
        ("providers", "Gestión de Proveedores"),
        ("visits", "Gestión de Visitas"),
        ("vehicles", "Gestión de Vehículos"),
        ("reports", "Reportes y Estadísticas"),
        ("settings", "Configuración del Sistema"),
    ];

    for (key, name) in modules {
        // Usamos UPDATE ... SET ... WHERE ... para asegurar existencia sin sobrescribir 'status' si ya existe
        // Pero si no existe, necesitamos CREARLO.
        // La estrategia robusta en SurrealDB para "Insert if not exists":
        // CREATE module CONTENT { ... } -- fallará si el ID ya existe.
        // O mejor:
        // define id based on key logic if possible, or query existence.

        let existing: Option<RecordId> = db
            .query("SELECT VALUE id FROM module WHERE key = $key")
            .bind(("key", key))
            .await?
            .take(0)?;

        if existing.is_none() {
            db.query(
                r"
                CREATE module CONTENT {
                    key: $key,
                    name: $name,
                    status: 'active',
                    created_at: time::now(),
                    updated_at: time::now()
                }
                ",
            )
            .bind(("key", key))
            .bind(("name", name))
            .await?
            .check()?;
            info!("📦 Módulo inicial registrado: {name} ({key})");
        }
    }

    info!("✅ Catálogo de módulos sincronizado");
    Ok(())
}

/// Provisiona los roles fundamentales del sistema.
///
/// Refactorizado para usar "Upsert" (Merge) en lugar de Delete/Create,
/// preservando así cualquier estado extra que no sea crítico.
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

        // UPSERT SEGURO: Actualiza permisos y metadata, pero no borra el record.
        db.query(
            r"
                UPDATE type::thing('role', $id) MERGE {
                    name: $name,
                    description: $desc,
                    is_system: true,
                    inherits_from: $inherits,
                    permissions: $permissions,
                    updated_at: time::now()
                }
                ",
        )
        .bind(("id", id))
        .bind(("name", name))
        .bind(("desc", desc))
        .bind(("inherits", inherits.map(|i| RecordId::from_table_key("role", i))))
        .bind(("permissions", permissions.clone()))
        .await?
        .check()?;

        // Si no existía, el UPDATE no hace nada, así que intentamos CREATE IF NOT EXISTS
        // Pero UPDATE con ID específico en SurrealDB debería crearlo si no existe?
        // No, UPDATE solo actualiza. CREATE crea.
        // Hacemos un CREATE pasivo por si acaso.

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
        .bind(("permissions", permissions)) // Variable reuse warning potential, but cloning above or copying is ok
        .await
        .ok(); // Ignoramos error si ya existe (el UPDATE ya se encargó)
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
        // En cada arranque, solo aseguramos que el usuario tenga el rol de Admin (Auto-healing de permisos)
        // PERO respetamos la contraseña y nombre que el usuario haya definido.
        // ACTUALIZACIÓN: Forzamos el email correcto para el usuario GOD.
        db.query("UPDATE type::thing('user', $id) SET role = type::thing('role', $role_id), email = $email, operacion = 'Mega Brisas', vencimiento_portacion = '10/12/2030', updated_at = time::now()")
            .bind(("id", GOD_ID))
            .bind(("role_id", ROLE_ADMIN_ID))
            .bind(("email", GOD_EMAIL))
            .await?
            .check()?;

        info!("🔐 Usuario raíz verificado (id={GOD_ID}) - Permisos sincronizados");
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
                operacion: "Mega Brisas",
                vencimiento_portacion: "10/12/2030",
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
        // Auto-heal: Aseguramos que Daniel tenga siempre el rol de administrador y campos obligatorios.
        db.query("UPDATE user SET role = type::thing('role', $role_id), operacion = 'Mega Brisas', vencimiento_portacion = '10/12/2030' WHERE email = $email")
            .bind(("role_id", ROLE_ADMIN_ID))
            .bind(("email", dev_email))
            .await?;
        return Ok(());
    }

    let id = uuid::Uuid::now_v7().to_string();
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
                operacion: "Mega Brisas",
                vencimiento_portacion: "10/12/2030",
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
