// src/commands/user_commands.rs

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
) -> Result<UserResponse, String> {
    // 1. Crear usuario localmente
    let user = user_service::create_user(&pool, &search_service, input)
        .await
        .map_err(|e| e.to_string())?;

    Ok(user)
}

#[tauri::command]
pub async fn update_user(
    pool: State<'_, SqlitePool>,
    search_service: State<'_, Arc<SearchService>>,
    id: String,
    input: UpdateUserInput,
) -> Result<UserResponse, String> {
    // 1. Actualizar localmente
    let user = user_service::update_user(&pool, &search_service, id.clone(), input)
        .await
        .map_err(|e| e.to_string())?;

    Ok(user)
}

#[tauri::command]
pub async fn delete_user(
    pool: State<'_, SqlitePool>,
    search_service: State<'_, Arc<SearchService>>,
    id: String,
) -> Result<(), String> {
    // 1. Eliminar localmente
    user_service::delete_user(&pool, &search_service, id.clone())
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn get_user_by_id(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<UserResponse, String> {
    user_service::get_user_by_id(&pool, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_all_users(pool: State<'_, SqlitePool>) -> Result<UserListResponse, String> {
    user_service::get_all_users(&pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn login(
    pool: State<'_, SqlitePool>,
    email: String,
    password: String,
) -> Result<UserResponse, String> {
    user_service::login(&pool, email, password)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn change_password(
    pool: State<'_, SqlitePool>,
    id: String,
    input: ChangePasswordInput,
) -> Result<(), String> {
    user_service::change_password(&pool, id, input)
        .await
        .map_err(|e| e.to_string())
}
