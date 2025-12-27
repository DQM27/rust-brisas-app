// ==========================================
// src/commands/role_commands.rs
// ==========================================
// Comandos Tauri para gestión de roles (SurrealDB-native)

use crate::domain::errors::RoleError;
use crate::models::role::{
    CreateRoleInput, Permission, RoleListResponse, RoleResponse, UpdateRoleInput, VisibleModule,
};
use crate::services::session::SessionState;
use crate::services::surrealdb_authorization;
use tauri::State;

// ==========================================
// CONSULTAS
// ==========================================

#[tauri::command]
pub async fn get_all_roles() -> Result<RoleListResponse, RoleError> {
    // TODO: Implementar en surrealdb_role_service
    Err(RoleError::NotFound)
}

#[tauri::command]
pub async fn get_role_by_id(id: String) -> Result<RoleResponse, RoleError> {
    let _ = id;
    // TODO: Implementar en surrealdb_role_service
    Err(RoleError::NotFound)
}

#[tauri::command]
pub async fn get_all_permissions() -> Result<Vec<Permission>, RoleError> {
    // TODO: Implementar para SurrealDB - por ahora retorna lista vacía
    Ok(vec![])
}

#[tauri::command]
pub async fn get_visible_modules(
    session: State<'_, SessionState>,
) -> Result<Vec<VisibleModule>, RoleError> {
    let user = session.get_user().ok_or(RoleError::Unauthorized("Sesión requerida".to_string()))?;

    let modules = surrealdb_authorization::get_visible_modules(&user.id, &user.role_id)
        .await
        .map_err(|e| RoleError::Database(sqlx::Error::Protocol(e.to_string())))?;

    // Obtener permisos para cada módulo
    let permissions = surrealdb_authorization::get_role_permissions(&user.role_id)
        .await
        .map_err(|e| RoleError::Database(sqlx::Error::Protocol(e.to_string())))?;

    let visible: Vec<VisibleModule> = modules
        .into_iter()
        .map(|m| VisibleModule {
            module: m.as_str().to_string(),
            display_name: m.display_name().to_string(),
            can_create: permissions.contains(&format!("{}:create", m.as_str())),
            can_read: permissions.contains(&format!("{}:read", m.as_str())),
            can_update: permissions.contains(&format!("{}:update", m.as_str())),
            can_delete: permissions.contains(&format!("{}:delete", m.as_str())),
            can_export: permissions.contains(&format!("{}:export", m.as_str())),
        })
        .collect();

    Ok(visible)
}

// ==========================================
// MUTACIONES
// ==========================================

#[tauri::command]
pub async fn create_role(
    session: State<'_, SessionState>,
    input: CreateRoleInput,
) -> Result<RoleResponse, RoleError> {
    let _user =
        session.get_user().ok_or(RoleError::Unauthorized("Sesión requerida".to_string()))?;
    let _ = input;
    // TODO: Implementar en surrealdb_role_service
    Err(RoleError::Validation("create_role no implementado para SurrealDB aún".to_string()))
}

#[tauri::command]
pub async fn update_role(
    session: State<'_, SessionState>,
    id: String,
    input: UpdateRoleInput,
) -> Result<RoleResponse, RoleError> {
    let _user =
        session.get_user().ok_or(RoleError::Unauthorized("Sesión requerida".to_string()))?;
    let _ = (id, input);
    // TODO: Implementar en surrealdb_role_service
    Err(RoleError::Validation("update_role no implementado para SurrealDB aún".to_string()))
}

#[tauri::command]
pub async fn delete_role(session: State<'_, SessionState>, id: String) -> Result<(), RoleError> {
    let _user =
        session.get_user().ok_or(RoleError::Unauthorized("Sesión requerida".to_string()))?;
    let _ = id;
    // TODO: Implementar en surrealdb_role_service
    Err(RoleError::Validation("delete_role no implementado para SurrealDB aún".to_string()))
}
