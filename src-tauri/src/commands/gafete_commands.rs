// ==========================================
// src/commands/gafete_commands.rs
// ==========================================

use crate::models::gafete::{
    CreateGafeteInput, UpdateGafeteInput, GafeteResponse, 
    GafeteListResponse, TipoGafete
};
use crate::services::gafete_service;
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn create_gafete(
    pool: State<'_, SqlitePool>,
    input: CreateGafeteInput,
) -> Result<GafeteResponse, String> {
    gafete_service::create_gafete(&pool, input).await
}

#[tauri::command]
pub async fn get_gafete(
    pool: State<'_, SqlitePool>,
    numero: String,
) -> Result<GafeteResponse, String> {
    gafete_service::get_gafete(&pool, &numero).await
}

#[tauri::command]
pub async fn get_all_gafetes(
    pool: State<'_, SqlitePool>,
) -> Result<GafeteListResponse, String> {
    gafete_service::get_all_gafetes(&pool).await
}

#[tauri::command]
pub async fn get_gafetes_disponibles(
    pool: State<'_, SqlitePool>,
    tipo: String,
) -> Result<Vec<GafeteResponse>, String> {
    let tipo_enum = TipoGafete::from_str(&tipo)?;
    gafete_service::get_gafetes_disponibles(&pool, tipo_enum).await
}

#[tauri::command]
pub async fn is_gafete_disponible(
    pool: State<'_, SqlitePool>,
    numero: String,
) -> Result<bool, String> {
    gafete_service::is_gafete_disponible(&pool, &numero).await
}

#[tauri::command]
pub async fn update_gafete(
    pool: State<'_, SqlitePool>,
    numero: String,
    input: UpdateGafeteInput,
) -> Result<GafeteResponse, String> {
    gafete_service::update_gafete(&pool, numero, input).await
}

#[tauri::command]
pub async fn delete_gafete(
    pool: State<'_, SqlitePool>,
    numero: String,
) -> Result<(), String> {
    gafete_service::delete_gafete(&pool, numero).await
}
