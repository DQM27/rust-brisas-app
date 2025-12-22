// ==========================================
// src/commands/gafete_commands.rs
// ==========================================

use crate::models::gafete::{
    CreateGafeteInput, CreateGafeteRangeInput, GafeteListResponse, GafeteResponse, TipoGafete,
    UpdateGafeteInput, UpdateGafeteStatusInput,
};
use crate::services::gafete_service;
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn create_gafete(
    pool: State<'_, SqlitePool>,
    input: CreateGafeteInput,
) -> Result<GafeteResponse, String> {
    gafete_service::create_gafete(&pool, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_gafete_range(
    pool: State<'_, SqlitePool>,
    input: CreateGafeteRangeInput,
) -> Result<Vec<String>, String> {
    gafete_service::create_gafete_range(&pool, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_gafete(
    pool: State<'_, SqlitePool>,
    numero: String,
    tipo: String,
) -> Result<GafeteResponse, String> {
    gafete_service::get_gafete(&pool, &numero, &tipo)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_all_gafetes(pool: State<'_, SqlitePool>) -> Result<GafeteListResponse, String> {
    gafete_service::get_all_gafetes(&pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_gafetes_disponibles(
    pool: State<'_, SqlitePool>,
    tipo: String,
) -> Result<Vec<GafeteResponse>, String> {
    let tipo_enum: TipoGafete = tipo.parse()?;
    gafete_service::get_gafetes_disponibles(&pool, tipo_enum)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn is_gafete_disponible(
    pool: State<'_, SqlitePool>,
    numero: String,
    tipo: String,
) -> Result<bool, String> {
    gafete_service::is_gafete_disponible(&pool, &numero, &tipo)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_gafete(
    pool: State<'_, SqlitePool>,
    numero: String,
    tipo: String,
    input: UpdateGafeteInput,
) -> Result<GafeteResponse, String> {
    gafete_service::update_gafete(&pool, numero, tipo, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_gafete_status(
    pool: State<'_, SqlitePool>,
    numero: String,
    tipo: String,
    input: UpdateGafeteStatusInput,
    usuario_id: Option<String>,
) -> Result<GafeteResponse, String> {
    gafete_service::update_gafete_status(&pool, numero, tipo, input.estado, usuario_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_gafete(
    pool: State<'_, SqlitePool>,
    numero: String,
    tipo: String,
) -> Result<(), String> {
    gafete_service::delete_gafete(&pool, numero, tipo)
        .await
        .map_err(|e| e.to_string())
}
