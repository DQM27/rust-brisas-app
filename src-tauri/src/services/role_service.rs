// ==========================================
// src/services/role_service.rs
// ==========================================
// Capa de servicio: orquesta dominio, db y lógica

use crate::db::surrealdb_role_queries as db;
use crate::domain::role::{self as domain, is_superuser};
use crate::models::role::{
    CreateRoleInput, Module, Permission, Role, RoleListResponse, RoleResponse, UpdateRoleInput,
    VisibleModule,
};
use crate::services::authorization;
use uuid::Uuid;

// ==========================================
// ERRORES
// ==========================================

use crate::domain::errors::RoleError;

// ==========================================
// CONSULTAS DE ROLES
// ==========================================

pub async fn get_all_roles() -> Result<RoleListResponse, RoleError> {
    let roles: Vec<Role> = db::find_all()
        .await
        .map_err(|e| RoleError::Database(sqlx::Error::Protocol(e.to_string())))?;

    let mut responses = Vec::new();
    let mut system_count = 0;

    for role in roles {
        // En Surreal guardamos permisos en el mismo rol
        // Pero el struct Role legacy no tiene 'permissions' field, los cargabamos aparte.
        // Si Role struct no ha cambiado, necesitamos mapear.
        // Voy a asumir que Role struct en models NO tiene permissions.
        // db::find_all retorna Role.
        // db::get_permissions retorna los permisos del rol.

        let permissions = db::get_permissions(&role.id)
            .await
            .map_err(|e| RoleError::Database(sqlx::Error::Protocol(e.to_string())))?;

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

pub async fn get_role_by_id(id: &str) -> Result<RoleResponse, RoleError> {
    let role = db::find_by_id(id)
        .await
        .map_err(|e| RoleError::Database(sqlx::Error::Protocol(e.to_string())))?
        .ok_or(RoleError::NotFound)?;

    let permissions = db::get_permissions(&role.id)
        .await
        .map_err(|e| RoleError::Database(sqlx::Error::Protocol(e.to_string())))?;
    Ok(RoleResponse::from_role_with_permissions(role, permissions))
}

// ==========================================
// CREAR ROL
// ==========================================

pub async fn create_role(input: CreateRoleInput) -> Result<RoleResponse, RoleError> {
    // 1. Validar input (dominio)
    domain::validar_create_input(&input)?;

    // 2. Verificar nombre único
    let exists = db::exists_by_name(&input.name)
        .await
        .map_err(|e| RoleError::Database(sqlx::Error::Protocol(e.to_string())))?;

    if exists {
        return Err(RoleError::NameExists);
    }

    // 3. Crear rol
    let id = Uuid::new_v4().to_string();
    let nombre = domain::normalizar_nombre(&input.name);

    db::create(
        &id,
        &nombre,
        input.description.as_deref(),
        &input.permissions,
        false, // is_system = false para roles creados
    )
    .await
    .map_err(|e| RoleError::Database(sqlx::Error::Protocol(e.to_string())))?;

    get_role_by_id(&id).await
}

// ==========================================
// ACTUALIZAR ROL
// ==========================================

pub async fn update_role(
    id: &str,
    input: UpdateRoleInput,
    requester_id: &str,
) -> Result<RoleResponse, RoleError> {
    // 1. Validar input
    domain::validar_update_input(&input)?;

    // 2. Obtener rol actual
    let role = get_role_by_id(id).await?;

    // 3. Solo root puede editar roles del sistema
    if role.is_system && !is_superuser(requester_id) {
        return Err(RoleError::CannotModifySystemRole);
    }

    // 4. Actualizar
    let nombre = input.name.as_ref().map(|n| domain::normalizar_nombre(n));
    let perms_ref = input.permissions.as_deref();

    db::update(id, nombre.as_deref(), input.description.as_deref(), perms_ref)
        .await
        .map_err(|e| RoleError::Database(sqlx::Error::Protocol(e.to_string())))?;

    get_role_by_id(id).await
}

// ==========================================
// ELIMINAR ROL
// ==========================================

pub async fn delete_role(id: &str) -> Result<(), RoleError> {
    let role = get_role_by_id(id).await?;

    if role.is_system {
        return Err(RoleError::CannotDeleteSystemRole);
    }

    db::delete(id).await.map_err(|e| RoleError::Database(sqlx::Error::Protocol(e.to_string())))?;

    Ok(())
}

// ==========================================
// MÓDULOS VISIBLES
// ==========================================

pub async fn get_user_visible_modules(
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

    // Usar surrealdb_authorization
    use crate::services::surrealdb_authorization;

    // Aquí invocamos la lógica migrada de autorización
    // que revisa los permisos guardados en el rol (array strings)
    let permissions = surrealdb_authorization::get_role_permissions(role_id)
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

pub async fn get_all_permissions() -> Result<Vec<Permission>, RoleError> {
    // Generar dinámicamente basado en enum Module
    // Esto evita depender de tabla permissions en DB
    let mut perms = Vec::new();

    for module in Module::all() {
        let actions = vec!["view", "create", "read", "update", "delete", "export"];
        for action in actions {
            perms.push(Permission {
                id: format!("{}:{}", module.as_str(), action),
                module: module.as_str().to_string(),
                action: action.to_string(),
                description: format!("{} {}", action, module.display_name()),
            });
        }
    }

    Ok(perms)
}
