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
    seed_permissions().await?;
    seed_role_permissions().await?;
    seed_superuser().await?;
    seed_admin_user().await?;

    // Debug: listar todos los usuarios después de seeds
    list_all_users().await.ok();

    println!("✅ [SURREALDB] Seeds completados");
    Ok(())
}

async fn list_all_users() -> Result<(), SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    #[derive(serde::Deserialize, Debug)]
    struct UserDebug {
        email: String,
        is_active: bool,
    }

    let mut result = client.query("SELECT email, is_active FROM usuarios").await?;
    let users: Vec<UserDebug> = result.take(0)?;

    println!("  📋 Usuarios en DB: {}", users.len());
    for u in &users {
        println!("     - {} (active={})", u.email, u.is_active);
    }
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

/// Seed de todos los permisos del sistema (module:action)
async fn seed_permissions() -> Result<(), SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    // Módulos y acciones
    let modules = [
        "users",
        "roles",
        "contratistas",
        "empresas",
        "proveedores",
        "visitantes",
        "ingresos",
        "citas",
        "vehiculos",
        "gafetes",
        "lista_negra",
        "config",
        "backup",
        "export",
    ];
    let actions = ["view", "create", "read", "update", "delete", "export"];

    for module in &modules {
        for action in &actions {
            let perm_id = format!("{}:{}", module, action);
            let description = format!("Permiso para {} en {}", action, module);

            // UPSERT para que sea idempotente
            client
                .query(
                    r#"
                    UPSERT type::thing('permissions', $id) CONTENT {
                        module: $module,
                        action: $action,
                        description: $description
                    }
                    "#,
                )
                .bind(("id", perm_id))
                .bind(("module", *module))
                .bind(("action", *action))
                .bind(("description", description))
                .await?;
        }
    }

    println!(
        "  ✓ Permisos creados ({} x {} = {})",
        modules.len(),
        actions.len(),
        modules.len() * actions.len()
    );
    Ok(())
}

/// Seed de role_permissions - asigna todos los permisos al rol admin
async fn seed_role_permissions() -> Result<(), SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    // Admin tiene TODOS los permisos
    let modules = [
        "users",
        "roles",
        "contratistas",
        "empresas",
        "proveedores",
        "visitantes",
        "ingresos",
        "citas",
        "vehiculos",
        "gafetes",
        "lista_negra",
        "config",
        "backup",
        "export",
    ];
    let actions = ["view", "create", "read", "update", "delete", "export"];

    for module in &modules {
        for action in &actions {
            let perm_id = format!("{}:{}", module, action);
            let rp_id = format!("admin_{}", perm_id);

            client
                .query(
                    r#"
                    UPSERT type::thing('role_permissions', $rp_id) CONTENT {
                        role_id: 'admin',
                        permission_id: $perm_id
                    }
                    "#,
                )
                .bind(("rp_id", rp_id))
                .bind(("perm_id", perm_id))
                .await?;
        }
    }

    // Guardia solo tiene permisos de vista en módulos básicos + create/read ingresos
    let guardia_perms = [
        "contratistas:view",
        "contratistas:read",
        "empresas:view",
        "empresas:read",
        "ingresos:view",
        "ingresos:create",
        "ingresos:read",
        "ingresos:update",
        "gafetes:view",
        "gafetes:read",
        "citas:view",
        "citas:read",
    ];

    for perm_id in &guardia_perms {
        let rp_id = format!("guardia_{}", perm_id);
        client
            .query(
                r#"
                UPSERT type::thing('role_permissions', $rp_id) CONTENT {
                    role_id: 'guardia',
                    permission_id: $perm_id
                }
                "#,
            )
            .bind(("rp_id", rp_id))
            .bind(("perm_id", *perm_id))
            .await?;
    }

    // Supervisor tiene más permisos que guardia
    let supervisor_perms = [
        "contratistas:view",
        "contratistas:create",
        "contratistas:read",
        "contratistas:update",
        "empresas:view",
        "empresas:read",
        "proveedores:view",
        "proveedores:read",
        "visitantes:view",
        "visitantes:read",
        "ingresos:view",
        "ingresos:create",
        "ingresos:read",
        "ingresos:update",
        "citas:view",
        "citas:create",
        "citas:read",
        "citas:update",
        "gafetes:view",
        "gafetes:read",
        "gafetes:update",
        "lista_negra:view",
        "lista_negra:read",
        "export:view",
        "export:create",
    ];

    for perm_id in &supervisor_perms {
        let rp_id = format!("supervisor_{}", perm_id);
        client
            .query(
                r#"
                UPSERT type::thing('role_permissions', $rp_id) CONTENT {
                    role_id: 'supervisor',
                    permission_id: $perm_id
                }
                "#,
            )
            .bind(("rp_id", rp_id))
            .bind(("perm_id", *perm_id))
            .await?;
    }

    println!("  ✓ Permisos de roles asignados");
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
