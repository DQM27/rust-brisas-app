// ==========================================
// src/commands/role_commands.rs
// ==========================================
// Comandos Tauri para gestión de roles (SurrealDB-native)

use crate::domain::errors::RoleError;
use crate::models::role::{
    CreateRoleInput, Permission, RoleListResponse, RoleResponse, UpdateRoleInput, VisibleModule,
};
use crate::services::role_service; // Importamos el service correcto
use crate::services::session::SessionState;
use crate::services::surrealdb_authorization;
use tauri::State;

// ==========================================
// CONSULTAS
// ==========================================

#[tauri::command]
pub async fn get_all_roles(
    session: State<'_, SessionState>,
) -> Result<RoleListResponse, RoleError> {
    require_perm!(session, "roles:read")?;
    role_service::get_all_roles().await
}

#[tauri::command]
pub async fn get_role_by_id(
    session: State<'_, SessionState>,
    id: String,
) -> Result<RoleResponse, RoleError> {
    require_perm!(session, "roles:read")?;
    role_service::get_role_by_id(&id).await
}

#[tauri::command]
pub async fn get_all_permissions(
    session: State<'_, SessionState>,
) -> Result<Vec<Permission>, RoleError> {
    require_perm!(session, "roles:read")?;
    role_service::get_all_permissions().await
}

#[tauri::command]
pub async fn get_visible_modules(
    session: State<'_, SessionState>,
) -> Result<Vec<VisibleModule>, RoleError> {
    let user = session.get_user().ok_or(RoleError::Unauthorized("Sesión requerida".to_string()))?;

    let modules = surrealdb_authorization::get_visible_modules(&user.id, &user.role_id)
        .await
        .map_err(|e| RoleError::Database(e.to_string()))?;

    // Obtener permisos efectivos (propios + heredados) para cada módulo
    let permissions = surrealdb_authorization::get_effective_permissions(&user.role_id)
        .await
        .map_err(|e| RoleError::Database(e.to_string()))?;

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
    require_perm!(session, "roles:create", "Creando nuevo rol")?;
    role_service::create_role(input).await
}

#[tauri::command]
pub async fn update_role(
    session: State<'_, SessionState>,
    id: String,
    input: UpdateRoleInput,
) -> Result<RoleResponse, RoleError> {
    require_perm!(session, "roles:update", format!("Actualizando rol {}", id))?;
    let user = session.get_user().ok_or(RoleError::Unauthorized("Sesión requerida".to_string()))?;
    role_service::update_role(&id, input, &user.id).await
}

#[tauri::command]
pub async fn delete_role(session: State<'_, SessionState>, id: String) -> Result<(), RoleError> {
    require_perm!(session, "roles:delete", format!("Eliminando rol {}", id))?;
    role_service::delete_role(&id).await
}
