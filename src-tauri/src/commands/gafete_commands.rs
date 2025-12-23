// ==========================================
// src/commands/gafete_commands.rs
// ==========================================

use crate::domain::errors::GafeteError;
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
) -> Result<GafeteResponse, GafeteError> {
    gafete_service::create_gafete(&pool, input).await
}

#[tauri::command]
pub async fn create_gafete_range(
    pool: State<'_, SqlitePool>,
    input: CreateGafeteRangeInput,
) -> Result<Vec<String>, GafeteError> {
    gafete_service::create_gafete_range(&pool, input).await
}

#[tauri::command]
pub async fn get_gafete(
    pool: State<'_, SqlitePool>,
    numero: String,
    tipo: String,
) -> Result<GafeteResponse, GafeteError> {
    gafete_service::get_gafete(&pool, &numero, &tipo).await
}

#[tauri::command]
pub async fn get_all_gafetes(
    pool: State<'_, SqlitePool>,
) -> Result<GafeteListResponse, GafeteError> {
    gafete_service::get_all_gafetes(&pool).await
}

#[tauri::command]
pub async fn get_gafetes_disponibles(
    pool: State<'_, SqlitePool>,
    tipo: String,
) -> Result<Vec<GafeteResponse>, GafeteError> {
    let tipo_enum: TipoGafete = tipo.parse().map_err(GafeteError::Validation)?;
    gafete_service::get_gafetes_disponibles(&pool, tipo_enum).await
}

#[tauri::command]
pub async fn is_gafete_disponible(
    pool: State<'_, SqlitePool>,
    numero: String,
    tipo: String,
) -> Result<bool, GafeteError> {
    gafete_service::is_gafete_disponible(&pool, &numero, &tipo).await
}

#[tauri::command]
pub async fn update_gafete(
    pool: State<'_, SqlitePool>,
    numero: String,
    tipo: String,
    input: UpdateGafeteInput,
) -> Result<GafeteResponse, GafeteError> {
    gafete_service::update_gafete(&pool, numero, tipo, input).await
}

#[tauri::command]
pub async fn update_gafete_status(
    pool: State<'_, SqlitePool>,
    numero: String,
    tipo: String,
    input: UpdateGafeteStatusInput,
    usuario_id: Option<String>,
    motivo: Option<String>,
) -> Result<GafeteResponse, GafeteError> {
    gafete_service::update_gafete_status(&pool, numero, tipo, input.estado, usuario_id, motivo)
        .await
}

#[tauri::command]
pub async fn delete_gafete(
    pool: State<'_, SqlitePool>,
    numero: String,
    tipo: String,
) -> Result<(), GafeteError> {
    gafete_service::delete_gafete(&pool, numero, tipo).await
}
