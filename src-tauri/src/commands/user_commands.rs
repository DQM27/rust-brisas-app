// src/commands/user_commands.rs

use crate::models::user::{
    UserResponse, UserListResponse,
    CreateUserInput, UpdateUserInput,
};
use crate::services::{user_service, sync_service};
use crate::services::sync_service::UserSyncData;
use crate::db::user_queries;
use crate::SupabaseState;
use sqlx::SqlitePool;
use tauri::State;
use std::sync::Arc;
use tokio::sync::RwLock;


#[tauri::command]
pub async fn create_user(
    pool: State<'_, SqlitePool>,
    supabase_state: State<'_, Arc<RwLock<SupabaseState>>>,
    input: CreateUserInput,
) -> Result<UserResponse, String> {
    // 1. Crear usuario localmente
    let user = user_service::create_user(&pool, input).await?;
    
    // 2. Obtener datos completos de SQLite para sync (incluyendo password_hash)
    let (user_with_pass, password_hash) = user_queries::find_by_email_with_password(&pool, &user.email).await?;
    
    // 3. Construir datos de sincronización
    let sync_data = UserSyncData {
        id: user_with_pass.id.clone(),
        email: user_with_pass.email.clone(),
        password_hash,
        nombre: user_with_pass.nombre.clone(),
        apellido: user_with_pass.apellido.clone(),
        role: user_with_pass.role.as_str().to_string(),
        is_active: user_with_pass.is_active,
        created_at: user_with_pass.created_at.clone(),
        updated_at: user_with_pass.updated_at.clone(),
    };
    
    // 4. Sincronizar a Supabase (no bloqueante)
    let supabase_clone = supabase_state.inner().clone();
    tokio::spawn(async move {
        let _ = sync_service::sync_user_to_supabase(supabase_clone, sync_data).await;
    });
    
    Ok(user)
}

#[tauri::command]
pub async fn update_user(
    pool: State<'_, SqlitePool>,
    supabase_state: State<'_, Arc<RwLock<SupabaseState>>>,
    id: String,
    input: UpdateUserInput,
) -> Result<UserResponse, String> {
    // 1. Actualizar localmente
    let user = user_service::update_user(&pool, id.clone(), input).await?;
    
    // 2. Obtener datos completos para sync
    let (user_with_pass, password_hash) = user_queries::find_by_email_with_password(&pool, &user.email).await?;
    
    // 3. Construir datos de sincronización
    let sync_data = UserSyncData {
        id: user_with_pass.id.clone(),
        email: user_with_pass.email.clone(),
        password_hash,
        nombre: user_with_pass.nombre.clone(),
        apellido: user_with_pass.apellido.clone(),
        role: user_with_pass.role.as_str().to_string(),
        is_active: user_with_pass.is_active,
        created_at: user_with_pass.created_at.clone(),
        updated_at: user_with_pass.updated_at.clone(),
    };
    
    // 4. Sincronizar a Supabase (no bloqueante)
    let supabase_clone = supabase_state.inner().clone();
    tokio::spawn(async move {
        let _ = sync_service::sync_user_to_supabase(supabase_clone, sync_data).await;
    });
    
    Ok(user)
}

#[tauri::command]
pub async fn delete_user(
    pool: State<'_, SqlitePool>,
    supabase_state: State<'_, Arc<RwLock<SupabaseState>>>,
    id: String,
) -> Result<(), String> {
    // 1. Eliminar localmente
    user_service::delete_user(&pool, id.clone()).await?;
    
    // 2. Eliminar de Supabase (no bloqueante)
    let supabase_clone = supabase_state.inner().clone();
    tokio::spawn(async move {
        let _ = sync_service::delete_user_from_supabase(supabase_clone, &id).await;
    });
    
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
pub async fn get_all_users(
    pool: State<'_, SqlitePool>,
) -> Result<UserListResponse, String> {
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