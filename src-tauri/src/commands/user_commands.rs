// src/commands/user_commands.rs

use crate::db::DbPool;
use crate::domain::errors::UserError;
use crate::models::user::{
    ChangePasswordInput, CreateUserInput, UpdateUserInput, UserListResponse, UserResponse,
};
use crate::services::search_service::SearchState;
use crate::services::user_service;

use tauri::State;

#[tauri::command]
pub async fn create_user(
    pool_state: State<'_, DbPool>,
    search_state: State<'_, SearchState>,
    input: CreateUserInput,
) -> Result<UserResponse, UserError> {
    let pool = pool_state.0.read().await;
    let search_service = search_state.0.read().await;
    user_service::create_user(&pool, &search_service, input).await
}

#[tauri::command]
pub async fn update_user(
    pool_state: State<'_, DbPool>,
    search_state: State<'_, SearchState>,
    id: String,
    input: UpdateUserInput,
) -> Result<UserResponse, UserError> {
    let pool = pool_state.0.read().await;
    let search_service = search_state.0.read().await;
    user_service::update_user(&pool, &search_service, id.clone(), input).await
}

#[tauri::command]
pub async fn delete_user(
    pool_state: State<'_, DbPool>,
    search_state: State<'_, SearchState>,
    id: String,
) -> Result<(), UserError> {
    let pool = pool_state.0.read().await;
    let search_service = search_state.0.read().await;
    user_service::delete_user(&pool, &search_service, id.clone()).await
}

#[tauri::command]
pub async fn get_user_by_id(
    pool_state: State<'_, DbPool>,
    id: String,
) -> Result<UserResponse, UserError> {
    let pool = pool_state.0.read().await;
    user_service::get_user_by_id(&pool, &id).await
}

#[tauri::command]
pub async fn get_all_users(pool_state: State<'_, DbPool>) -> Result<UserListResponse, UserError> {
    let pool = pool_state.0.read().await;
    user_service::get_all_users(&pool).await
}

#[tauri::command]
pub async fn login(
    pool_state: State<'_, DbPool>,
    email: String,
    password: String,
) -> Result<UserResponse, UserError> {
    let pool = pool_state.0.read().await;

    // DEBUG: Verificar a qué DB estamos conectados
    if let Ok(path) =
        sqlx::query_scalar::<_, String>("SELECT file FROM pragma_database_list WHERE name='main'")
            .fetch_one(&*pool)
            .await
    {
        log::info!("🔐 Login request using DB file: {}", path);
    }

    user_service::login(&pool, email, password).await
}

#[tauri::command]
pub async fn change_password(
    pool_state: State<'_, DbPool>,
    id: String,
    input: ChangePasswordInput,
) -> Result<(), UserError> {
    let pool = pool_state.0.read().await;
    user_service::change_password(&pool, id, input).await
}

/// Ejecuta el seed de demostración y logea con un usuario demo
/// email debe ser uno de: marie.curie@demo.com, albert.einstein@demo.com, richard.feynman@demo.com
#[tauri::command]
pub async fn demo_login(
    pool_state: State<'_, DbPool>,
    email: String,
) -> Result<UserResponse, UserError> {
    let pool = pool_state.0.read().await;

    // 1. Ejecutar seed_demo (es idempotente, no duplica datos)
    crate::config::seed_demo::run_demo_seed(&pool)
        .await
        .map_err(|e| UserError::Database(sqlx::Error::Protocol(e.to_string())))?;

    // 2. Logear con el usuario demo (password siempre es demo123)
    user_service::login(&pool, email, "demo123".to_string()).await
}
