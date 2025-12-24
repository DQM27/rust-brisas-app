// src/commands/user_commands.rs

use crate::domain::errors::UserError;
use crate::models::user::{
    ChangePasswordInput, CreateUserInput, UpdateUserInput, UserListResponse, UserResponse,
};
use crate::services::search_service::SearchService;
use crate::services::user_service;

use sqlx::SqlitePool;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub async fn create_user(
    pool: State<'_, SqlitePool>,
    search_service: State<'_, Arc<SearchService>>,
    input: CreateUserInput,
) -> Result<UserResponse, UserError> {
    user_service::create_user(&pool, &search_service, input).await
}

#[tauri::command]
pub async fn update_user(
    pool: State<'_, SqlitePool>,
    search_service: State<'_, Arc<SearchService>>,
    id: String,
    input: UpdateUserInput,
) -> Result<UserResponse, UserError> {
    user_service::update_user(&pool, &search_service, id.clone(), input).await
}

#[tauri::command]
pub async fn delete_user(
    pool: State<'_, SqlitePool>,
    search_service: State<'_, Arc<SearchService>>,
    id: String,
) -> Result<(), UserError> {
    user_service::delete_user(&pool, &search_service, id.clone()).await
}

#[tauri::command]
pub async fn get_user_by_id(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<UserResponse, UserError> {
    user_service::get_user_by_id(&pool, &id).await
}

#[tauri::command]
pub async fn get_all_users(pool: State<'_, SqlitePool>) -> Result<UserListResponse, UserError> {
    user_service::get_all_users(&pool).await
}

#[tauri::command]
pub async fn login(
    pool: State<'_, SqlitePool>,
    email: String,
    password: String,
) -> Result<UserResponse, UserError> {
    user_service::login(&pool, email, password).await
}

#[tauri::command]
pub async fn change_password(
    pool: State<'_, SqlitePool>,
    id: String,
    input: ChangePasswordInput,
) -> Result<(), UserError> {
    user_service::change_password(&pool, id, input).await
}

/// Ejecuta el seed de demostración y logea con un usuario demo
/// email debe ser uno de: marie.curie@demo.com, albert.einstein@demo.com, richard.feynman@demo.com
#[tauri::command]
pub async fn demo_login(
    pool: State<'_, SqlitePool>,
    email: String,
) -> Result<UserResponse, UserError> {
    // 1. Ejecutar seed_demo (es idempotente, no duplica datos)
    crate::config::seed_demo::run_demo_seed(&pool)
        .await
        .map_err(|e| UserError::Database(sqlx::Error::Protocol(e.to_string())))?;

    // 2. Logear con el usuario demo (password siempre es demo123)
    user_service::login(&pool, email, "demo123".to_string()).await
}
