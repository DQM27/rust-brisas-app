// ==========================================
// src/commands/user_commands.rs (REFACTORIZADO)
// ==========================================
// Comandos Tauri: solo orquestación, delegan todo al servicio

use crate::models::user::{
    UserResponse, UserListResponse,
    CreateUserInput, UpdateUserInput,
};
use crate::services::user_service;
use sqlx::SqlitePool;
use tauri::State;

// ==========================================
// COMANDOS CRUD
// ==========================================

#[tauri::command]
pub async fn create_user(
    pool: State<'_, SqlitePool>,
    input: CreateUserInput,
) -> Result<UserResponse, String> {
    user_service::create_user(&pool, input).await
}

#[tauri::command]
pub async fn get_user_by_id(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<UserResponse, String> {
    user_service::get_user_by_id(&pool, &id).await
}

#[tauri::command]
pub async fn get_all_users(
    pool: State<'_, SqlitePool>,
) -> Result<UserListResponse, String> {
    user_service::get_all_users(&pool).await
}

#[tauri::command]
pub async fn update_user(
    pool: State<'_, SqlitePool>,
    id: String,
    input: UpdateUserInput,
) -> Result<UserResponse, String> {
    user_service::update_user(&pool, id, input).await
}

#[tauri::command]
pub async fn delete_user(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    user_service::delete_user(&pool, id).await
}

// ==========================================
// AUTENTICACIÓN
// ==========================================

#[tauri::command]
pub async fn login(
    pool: State<'_, SqlitePool>,
    email: String,
    password: String,
) -> Result<UserResponse, String> {
    user_service::login(&pool, email, password).await
}