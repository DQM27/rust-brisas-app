// ==========================================
// src/services/role_service.rs
// ==========================================
// Capa de servicio: orquesta dominio, db y lógica

use crate::domain::role::{self as domain, is_superuser};
use crate::models::role::{
    CreateRoleInput, Module, Permission, Role, RoleListResponse, RoleResponse, UpdateRoleInput,
    VisibleModule,
};
use crate::services::authorization;
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

// ==========================================
// ERRORES
// ==========================================

#[derive(Debug, thiserror::Error)]
pub enum RoleError {
    #[error("Rol no encontrado")]
    NotFound,

    #[error("Ya existe un rol con ese nombre")]
    NameExists,

    #[error("No se puede eliminar un rol del sistema")]
    CannotDeleteSystemRole,

    #[error("Solo el superusuario puede modificar roles del sistema")]
    CannotModifySystemRole,

    #[error("Error de validación: {0}")]
    Validation(String),

    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),
}

// ==========================================
// CONSULTAS DE ROLES
// ==========================================

pub async fn get_all_roles(pool: &SqlitePool) -> Result<RoleListResponse, RoleError> {
    let roles: Vec<Role> = sqlx::query_as(
        r#"SELECT id, name, description, is_system, created_at, updated_at 
           FROM roles 
           ORDER BY is_system DESC, name ASC"#,
    )
    .fetch_all(pool)
    .await?;

    let mut responses = Vec::new();
    let mut system_count = 0;

    for role in roles {
        let permissions = get_role_permission_ids(pool, &role.id).await?;
        if role.is_system {
            system_count += 1;
        }
        responses.push(RoleResponse::from_role_with_permissions(role, permissions));
    }

    let total = responses.len();

    Ok(RoleListResponse {
        roles: responses,
        total,
        system_roles: system_count,
        custom_roles: total - system_count,
    })
}

pub async fn get_role_by_id(pool: &SqlitePool, id: &str) -> Result<RoleResponse, RoleError> {
    let role: Role = sqlx::query_as(
        r#"SELECT id, name, description, is_system, created_at, updated_at 
           FROM roles WHERE id = ?"#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or(RoleError::NotFound)?;

    let permissions = get_role_permission_ids(pool, &role.id).await?;
    Ok(RoleResponse::from_role_with_permissions(role, permissions))
}

async fn get_role_permission_ids(
    pool: &SqlitePool,
    role_id: &str,
) -> Result<Vec<String>, RoleError> {
    let perms: Vec<(String,)> =
        sqlx::query_as("SELECT permission_id FROM role_permissions WHERE role_id = ?")
            .bind(role_id)
            .fetch_all(pool)
            .await?;

    Ok(perms.into_iter().map(|(id,)| id).collect())
}

// ==========================================
// CREAR ROL
// ==========================================

pub async fn create_role(
    pool: &SqlitePool,
    input: CreateRoleInput,
) -> Result<RoleResponse, RoleError> {
    // 1. Validar input (dominio)
    domain::validar_create_input(&input).map_err(RoleError::Validation)?;

    // 2. Verificar nombre único
    let count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM roles WHERE name = ?")
        .bind(&input.name)
        .fetch_one(pool)
        .await?;

    if count > 0 {
        return Err(RoleError::NameExists);
    }

    // 3. Crear rol
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let nombre = domain::normalizar_nombre_rol(&input.name);

    sqlx::query(
        r#"INSERT INTO roles (id, name, description, is_system, created_at, updated_at)
           VALUES (?, ?, ?, 0, ?, ?)"#,
    )
    .bind(&id)
    .bind(&nombre)
    .bind(&input.description)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    // 4. Asignar permisos
    for perm_id in &input.permissions {
        let _ = sqlx::query(
            "INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES (?, ?)",
        )
        .bind(&id)
        .bind(perm_id)
        .execute(pool)
        .await;
    }

    get_role_by_id(pool, &id).await
}

// ==========================================
// ACTUALIZAR ROL
// ==========================================

pub async fn update_role(
    pool: &SqlitePool,
    id: &str,
    input: UpdateRoleInput,
    requester_id: &str,
) -> Result<RoleResponse, RoleError> {
    // 1. Validar input
    domain::validar_update_input(&input).map_err(RoleError::Validation)?;

    // 2. Obtener rol actual
    let role = get_role_by_id(pool, id).await?;

    // 3. Solo root puede editar roles del sistema
    if role.is_system && !is_superuser(requester_id) {
        return Err(RoleError::CannotModifySystemRole);
    }

    let now = Utc::now().to_rfc3339();

    // 4. Actualizar campos básicos
    if input.name.is_some() || input.description.is_some() {
        let nombre = input
            .name
            .as_ref()
            .map(|n| domain::normalizar_nombre_rol(n));

        sqlx::query(
            r#"UPDATE roles SET 
               name = COALESCE(?, name),
               description = COALESCE(?, description),
               updated_at = ?
               WHERE id = ?"#,
        )
        .bind(&nombre)
        .bind(&input.description)
        .bind(&now)
        .bind(id)
        .execute(pool)
        .await?;
    }

    // 5. Actualizar permisos si vienen
    if let Some(permissions) = input.permissions {
        sqlx::query("DELETE FROM role_permissions WHERE role_id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        for perm_id in permissions {
            let _ = sqlx::query(
                "INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES (?, ?)",
            )
            .bind(id)
            .bind(&perm_id)
            .execute(pool)
            .await;
        }
    }

    get_role_by_id(pool, id).await
}

// ==========================================
// ELIMINAR ROL
// ==========================================

pub async fn delete_role(pool: &SqlitePool, id: &str) -> Result<(), RoleError> {
    let role = get_role_by_id(pool, id).await?;

    if role.is_system {
        return Err(RoleError::CannotDeleteSystemRole);
    }

    sqlx::query("DELETE FROM roles WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

// ==========================================
// MÓDULOS VISIBLES
// ==========================================

pub async fn get_user_visible_modules(
    pool: &SqlitePool,
    user_id: &str,
    role_id: &str,
) -> Result<Vec<VisibleModule>, RoleError> {
    // Superuser ve todo
    if is_superuser(user_id) {
        return Ok(Module::all()
            .into_iter()
            .map(|m| VisibleModule {
                module: m.as_str().to_string(),
                display_name: m.display_name().to_string(),
                can_create: true,
                can_read: true,
                can_update: true,
                can_delete: true,
                can_export: true,
            })
            .collect());
    }

    let permissions = authorization::get_role_permissions(pool, role_id)
        .await
        .map_err(|e| RoleError::Database(sqlx::Error::Protocol(e.to_string())))?;

    let mut modules = Vec::new();

    for module in Module::all() {
        let view_perm = format!("{}:view", module.as_str());

        if permissions.contains(&view_perm) {
            modules.push(VisibleModule {
                module: module.as_str().to_string(),
                display_name: module.display_name().to_string(),
                can_create: permissions.contains(&format!("{}:create", module.as_str())),
                can_read: permissions.contains(&format!("{}:read", module.as_str())),
                can_update: permissions.contains(&format!("{}:update", module.as_str())),
                can_delete: permissions.contains(&format!("{}:delete", module.as_str())),
                can_export: permissions.contains(&format!("{}:export", module.as_str())),
            });
        }
    }

    Ok(modules)
}

// ==========================================
// PERMISOS DISPONIBLES
// ==========================================

pub async fn get_all_permissions(pool: &SqlitePool) -> Result<Vec<Permission>, RoleError> {
    let perms: Vec<Permission> = sqlx::query_as(
        "SELECT id, module, action, description FROM permissions ORDER BY module, action",
    )
    .fetch_all(pool)
    .await?;

    Ok(perms)
}
