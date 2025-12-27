// ==========================================
// src/commands/role_commands.rs
// ==========================================
// Comandos Tauri para gestión de roles

use crate::domain::errors::RoleError;
use crate::models::role::{
    CreateRoleInput, Module, Permission, RoleListResponse, RoleResponse, UpdateRoleInput,
    VisibleModule,
};
use crate::services::session::SessionState;
use tauri::State;

// ==========================================
// IMPORTS CONDICIONALES
// ==========================================

#[cfg(not(feature = "surrealdb-backend"))]
use crate::db::DbPool;
#[cfg(not(feature = "surrealdb-backend"))]
use crate::services::role_service;

#[cfg(feature = "surrealdb-backend")]
use crate::services::surrealdb_authorization;

// ==========================================
// CONSULTAS - SQLite
// ==========================================

#[cfg(not(feature = "surrealdb-backend"))]
#[tauri::command]
pub async fn get_all_roles(pool_state: State<'_, DbPool>) -> Result<RoleListResponse, RoleError> {
    let pool = pool_state.0.read().await;
    role_service::get_all_roles(&pool).await
}

#[cfg(not(feature = "surrealdb-backend"))]
#[tauri::command]
pub async fn get_role_by_id(
    pool_state: State<'_, DbPool>,
    id: String,
) -> Result<RoleResponse, RoleError> {
    let pool = pool_state.0.read().await;
    role_service::get_role_by_id(&pool, &id).await
}

#[cfg(not(feature = "surrealdb-backend"))]
#[tauri::command]
pub async fn get_all_permissions(
    pool_state: State<'_, DbPool>,
) -> Result<Vec<Permission>, RoleError> {
    let pool = pool_state.0.read().await;
    role_service::get_all_permissions(&pool).await
}

#[cfg(not(feature = "surrealdb-backend"))]
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
// CONSULTAS - SurrealDB
// ==========================================

#[cfg(feature = "surrealdb-backend")]
#[tauri::command]
pub async fn get_all_roles() -> Result<RoleListResponse, RoleError> {
    // TODO: Implementar para SurrealDB
    Err(RoleError::NotFound)
}

#[cfg(feature = "surrealdb-backend")]
#[tauri::command]
pub async fn get_role_by_id(id: String) -> Result<RoleResponse, RoleError> {
    // TODO: Implementar para SurrealDB
    let _ = id;
    Err(RoleError::NotFound)
}

#[cfg(feature = "surrealdb-backend")]
#[tauri::command]
pub async fn get_all_permissions() -> Result<Vec<Permission>, RoleError> {
    // TODO: Implementar para SurrealDB - por ahora retorna lista vacía
    Ok(vec![])
}

#[cfg(feature = "surrealdb-backend")]
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
// MUTACIONES - SQLite
// ==========================================

#[cfg(not(feature = "surrealdb-backend"))]
#[tauri::command]
pub async fn create_role(
    pool_state: State<'_, DbPool>,
    session: State<'_, SessionState>,
    input: CreateRoleInput,
) -> Result<RoleResponse, RoleError> {
    log::info!("🚀 create_role invoked. Input name: {}", input.name);

    let maybe_user = session.get_user();
    match &maybe_user {
        Some(u) => log::info!("✅ Session found for user: {} ({})", u.nombre, u.id),
        None => log::error!("❌ No active session found in SessionState!"),
    }

    let _user = maybe_user
        .ok_or(RoleError::Unauthorized("Sesión requerida (Backend check failed)".to_string()))?;

    let pool = pool_state.0.read().await;
    role_service::create_role(&pool, input).await
}

#[cfg(not(feature = "surrealdb-backend"))]
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

#[cfg(not(feature = "surrealdb-backend"))]
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

// ==========================================
// MUTACIONES - SurrealDB (stubs)
// ==========================================

#[cfg(feature = "surrealdb-backend")]
#[tauri::command]
pub async fn create_role(
    session: State<'_, SessionState>,
    input: CreateRoleInput,
) -> Result<RoleResponse, RoleError> {
    let _user =
        session.get_user().ok_or(RoleError::Unauthorized("Sesión requerida".to_string()))?;
    let _ = input;
    // TODO: Implementar para SurrealDB
    Err(RoleError::Validation("create_role no implementado para SurrealDB aún".to_string()))
}

#[cfg(feature = "surrealdb-backend")]
#[tauri::command]
pub async fn update_role(
    session: State<'_, SessionState>,
    id: String,
    input: UpdateRoleInput,
) -> Result<RoleResponse, RoleError> {
    let _user =
        session.get_user().ok_or(RoleError::Unauthorized("Sesión requerida".to_string()))?;
    let _ = (id, input);
    // TODO: Implementar para SurrealDB
    Err(RoleError::Validation("update_role no implementado para SurrealDB aún".to_string()))
}

#[cfg(feature = "surrealdb-backend")]
#[tauri::command]
pub async fn delete_role(session: State<'_, SessionState>, id: String) -> Result<(), RoleError> {
    let _user =
        session.get_user().ok_or(RoleError::Unauthorized("Sesión requerida".to_string()))?;
    let _ = id;
    // TODO: Implementar para SurrealDB
    Err(RoleError::Validation("delete_role no implementado para SurrealDB aún".to_string()))
}
