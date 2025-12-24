// ==========================================
// src/commands/role_commands.rs
// ==========================================
// Comandos Tauri para gestión de roles

use crate::db::DbPool;
use crate::domain::errors::RoleError;

use crate::models::role::{
    CreateRoleInput, Permission, RoleListResponse, RoleResponse, UpdateRoleInput, VisibleModule,
};
use crate::services::{role_service, session::SessionState};
use tauri::State;

// ==========================================
// CONSULTAS
// ==========================================

/// Obtiene todos los roles
#[tauri::command]
pub async fn get_all_roles(pool_state: State<'_, DbPool>) -> Result<RoleListResponse, RoleError> {
    let pool = pool_state.0.read().await;
    role_service::get_all_roles(&pool).await
}

/// Obtiene un rol por ID
#[tauri::command]
pub async fn get_role_by_id(
    pool_state: State<'_, DbPool>,
    id: String,
) -> Result<RoleResponse, RoleError> {
    let pool = pool_state.0.read().await;
    role_service::get_role_by_id(&pool, &id).await
}

/// Obtiene todos los permisos disponibles
#[tauri::command]
pub async fn get_all_permissions(
    pool_state: State<'_, DbPool>,
) -> Result<Vec<Permission>, RoleError> {
    let pool = pool_state.0.read().await;
    role_service::get_all_permissions(&pool).await
}

/// Obtiene los módulos visibles para el usuario actual
#[tauri::command]
pub async fn get_visible_modules(
    pool_state: State<'_, DbPool>,
    session: State<'_, SessionState>,
) -> Result<Vec<VisibleModule>, RoleError> {
    let user = session.get_user().ok_or(RoleError::Unauthorized("Sesión requerida".to_string()))?;
    let pool = pool_state.0.read().await;
    role_service::get_user_visible_modules(&pool, &user.id, &user.role_id).await
}

// ==========================================
// MUTACIONES
// ==========================================

/// Crea un nuevo rol (solo admin)
#[tauri::command]
pub async fn create_role(
    pool_state: State<'_, DbPool>,
    session: State<'_, SessionState>,
    input: CreateRoleInput,
) -> Result<RoleResponse, RoleError> {
    // Verificar sesión
    let _user =
        session.get_user().ok_or(RoleError::Unauthorized("Sesión requerida".to_string()))?;

    // Por ahora permitimos a cualquier admin crear roles
    // TODO: Verificar permiso roles:create

    let pool = pool_state.0.read().await;
    role_service::create_role(&pool, input).await
}

/// Actualiza un rol existente
#[tauri::command]
pub async fn update_role(
    pool_state: State<'_, DbPool>,
    session: State<'_, SessionState>,
    id: String,
    input: UpdateRoleInput,
) -> Result<RoleResponse, RoleError> {
    let user = session.get_user().ok_or(RoleError::Unauthorized("Sesión requerida".to_string()))?;
    let pool = pool_state.0.read().await;
    role_service::update_role(&pool, &id, input, &user.id).await
}

/// Elimina un rol (solo roles custom, no del sistema)
#[tauri::command]
pub async fn delete_role(
    pool_state: State<'_, DbPool>,
    session: State<'_, SessionState>,
    id: String,
) -> Result<(), RoleError> {
    let _user =
        session.get_user().ok_or(RoleError::Unauthorized("Sesión requerida".to_string()))?;

    let pool = pool_state.0.read().await;
    role_service::delete_role(&pool, &id).await
}
