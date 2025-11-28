// src/commands/user_commands.rs

use crate::models::user::{CreateUserInput, UpdateUserInput, UserListResponse, UserResponse};
use crate::services::user_service;

use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn create_user(
    pool: State<'_, SqlitePool>,
    input: CreateUserInput,
) -> Result<UserResponse, String> {
    // 1. Crear usuario localmente
    let user = user_service::create_user(&pool, input).await?;

    Ok(user)
}

#[tauri::command]
pub async fn update_user(
    pool: State<'_, SqlitePool>,
    id: String,
    input: UpdateUserInput,
) -> Result<UserResponse, String> {
    // 1. Actualizar localmente
    let user = user_service::update_user(&pool, id.clone(), input).await?;

    Ok(user)
}

#[tauri::command]
pub async fn delete_user(pool: State<'_, SqlitePool>, id: String) -> Result<(), String> {
    // 1. Eliminar localmente
    user_service::delete_user(&pool, id.clone()).await?;

    Ok(())
}

#[tauri::command]
pub async fn get_user_by_id(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<UserResponse, String> {
    user_service::get_user_by_id(&pool, &id).await
}

#[tauri::command]
pub async fn get_all_users(pool: State<'_, SqlitePool>) -> Result<UserListResponse, String> {
    user_service::get_all_users(&pool).await
}

#[tauri::command]
pub async fn login(
    pool: State<'_, SqlitePool>,
    email: String,
    password: String,
) -> Result<UserResponse, String> {
    user_service::login(&pool, email, password).await
}
