// ==========================================
// src/commands/role_commands.rs
// ==========================================
// Comandos Tauri para gestión de roles

use crate::domain::errors::RoleError;
use crate::models::role::{
    CreateRoleInput, Permission, RoleListResponse, RoleResponse, UpdateRoleInput, VisibleModule,
};
use crate::services::{role_service, session::SessionState};
use sqlx::SqlitePool;
use tauri::State;

// ==========================================
// CONSULTAS
// ==========================================

/// Obtiene todos los roles
#[tauri::command]
pub async fn get_all_roles(pool: State<'_, SqlitePool>) -> Result<RoleListResponse, RoleError> {
    role_service::get_all_roles(&pool).await
}

/// Obtiene un rol por ID
#[tauri::command]
pub async fn get_role_by_id(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<RoleResponse, RoleError> {
    role_service::get_role_by_id(&pool, &id).await
}

/// Obtiene todos los permisos disponibles
#[tauri::command]
pub async fn get_all_permissions(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<Permission>, RoleError> {
    role_service::get_all_permissions(&pool).await
}

/// Obtiene los módulos visibles para el usuario actual
#[tauri::command]
pub async fn get_visible_modules(
    pool: State<'_, SqlitePool>,
    session: State<'_, SessionState>,
) -> Result<Vec<VisibleModule>, RoleError> {
    let user = session.get_user().ok_or(RoleError::Unauthorized("Sesión requerida".to_string()))?;

    role_service::get_user_visible_modules(&pool, &user.id, &user.role_id).await
}

// ==========================================
// MUTACIONES
// ==========================================

/// Crea un nuevo rol (solo admin)
#[tauri::command]
pub async fn create_role(
    pool: State<'_, SqlitePool>,
    session: State<'_, SessionState>,
    input: CreateRoleInput,
) -> Result<RoleResponse, RoleError> {
    // Verificar sesión
    let _user =
        session.get_user().ok_or(RoleError::Unauthorized("Sesión requerida".to_string()))?;

    // Por ahora permitimos a cualquier admin crear roles
    // TODO: Verificar permiso roles:create

    role_service::create_role(&pool, input).await
}

/// Actualiza un rol existente
#[tauri::command]
pub async fn update_role(
    pool: State<'_, SqlitePool>,
    session: State<'_, SessionState>,
    id: String,
    input: UpdateRoleInput,
) -> Result<RoleResponse, RoleError> {
    let user = session.get_user().ok_or(RoleError::Unauthorized("Sesión requerida".to_string()))?;

    role_service::update_role(&pool, &id, input, &user.id).await
}

/// Elimina un rol (solo roles custom, no del sistema)
#[tauri::command]
pub async fn delete_role(
    pool: State<'_, SqlitePool>,
    session: State<'_, SessionState>,
    id: String,
) -> Result<(), RoleError> {
    let _user =
        session.get_user().ok_or(RoleError::Unauthorized("Sesión requerida".to_string()))?;

    role_service::delete_role(&pool, &id).await
}
