// src/commands/user_commands.rs

use crate::domain::errors::UserError;
use crate::models::user::{
    ChangePasswordInput, CreateUserInput, UpdateUserInput, UserListResponse, UserResponse,
};
use crate::services::search_service::SearchService;
use crate::services::session::{SessionState, SessionUser};
use crate::services::user_service;
use std::sync::Arc;
use tauri::State;

// ==========================================
// COMMANDS
// ==========================================

#[tauri::command]
pub async fn create_user(
    search: State<'_, Arc<SearchService>>,
    input: CreateUserInput,
) -> Result<UserResponse, UserError> {
    user_service::create_user(&search, input).await
}

#[tauri::command]
pub async fn update_user(
    search: State<'_, Arc<SearchService>>,
    id: String,
    input: UpdateUserInput,
) -> Result<UserResponse, UserError> {
    user_service::update_user(&search, id, input).await
}

#[tauri::command]
pub async fn delete_user(
    search: State<'_, Arc<SearchService>>,
    id: String,
) -> Result<(), UserError> {
    user_service::delete_user(&search, id).await
}

#[tauri::command]
pub async fn get_user_by_id(id: String) -> Result<UserResponse, UserError> {
    user_service::get_user_by_id(&id).await
}

#[tauri::command]
pub async fn get_all_users() -> Result<UserListResponse, UserError> {
    user_service::get_all_users().await
}

#[tauri::command]
pub async fn login(
    session: State<'_, SessionState>,
    email: String,
    password: String,
) -> Result<UserResponse, UserError> {
    log::info!("🔐 Login request using SurrealDB Native");
    let user_response = user_service::login(email, password).await?;

    // Update SessionState
    let session_user = SessionUser {
        id: user_response.id.clone(),
        email: user_response.email.clone(),
        nombre: user_response.nombre.clone(),
        apellido: user_response.apellido.clone(),
        role_id: user_response.role_id.clone(),
        role_name: user_response.role_name.clone(),
    };

    session.set_user(session_user);
    log::info!("✅ Sesión establecida para: {}", user_response.email);

    Ok(user_response)
}

#[tauri::command]
pub async fn change_password(id: String, input: ChangePasswordInput) -> Result<(), UserError> {
    user_service::change_password(id, input).await
}

/// Ejecuta el seed de demostración y logea con un usuario demo
#[tauri::command]
pub async fn demo_login(
    session: State<'_, SessionState>,
    email: String,
) -> Result<UserResponse, UserError> {
    log::warn!("⚠️ demo_login (SurrealDB Native)");
    let user_response = user_service::login(email, "demo".to_string()).await?;

    // Update SessionState
    let session_user = SessionUser {
        id: user_response.id.clone(),
        email: user_response.email.clone(),
        nombre: user_response.nombre.clone(),
        apellido: user_response.apellido.clone(),
        role_id: user_response.role_id.clone(),
        role_name: user_response.role_name.clone(),
    };

    session.set_user(session_user);
    Ok(user_response)
}

// ==========================================
// AVATAR COMMANDS (Encrypted Storage)
// ==========================================

#[tauri::command]
pub async fn upload_user_avatar(user_id: String, file_path: String) -> Result<String, UserError> {
    log::info!("📸 Comando: upload_user_avatar para {}", user_id);
    crate::services::avatar_service::upload_avatar(&user_id, &file_path)
        .await
        .map_err(|e| UserError::Validation(e))
}

#[tauri::command]
pub async fn get_user_avatar(user_id: String) -> Result<String, UserError> {
    crate::services::avatar_service::get_avatar(&user_id)
        .await
        .map_err(|e| UserError::Validation(e))
}
