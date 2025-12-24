// ==========================================
// src/commands/gafete_commands.rs
// ==========================================

use crate::db::DbPool;
use crate::domain::errors::GafeteError;
use crate::models::gafete::{
    CreateGafeteInput, CreateGafeteRangeInput, GafeteListResponse, GafeteResponse, TipoGafete,
    UpdateGafeteInput, UpdateGafeteStatusInput,
};
use crate::services::gafete_service;
use tauri::State;

#[tauri::command]
pub async fn create_gafete(
    pool_state: State<'_, DbPool>,
    input: CreateGafeteInput,
) -> Result<GafeteResponse, GafeteError> {
    let pool = pool_state.0.read().await;
    gafete_service::create_gafete(&pool, input).await
}

#[tauri::command]
pub async fn create_gafete_range(
    pool_state: State<'_, DbPool>,
    input: CreateGafeteRangeInput,
) -> Result<Vec<String>, GafeteError> {
    let pool = pool_state.0.read().await;
    gafete_service::create_gafete_range(&pool, input).await
}

#[tauri::command]
pub async fn get_gafete(
    pool_state: State<'_, DbPool>,
    numero: String,
    tipo: String,
) -> Result<GafeteResponse, GafeteError> {
    let pool = pool_state.0.read().await;
    gafete_service::get_gafete(&pool, &numero, &tipo).await
}

#[tauri::command]
pub async fn get_all_gafetes(
    pool_state: State<'_, DbPool>,
) -> Result<GafeteListResponse, GafeteError> {
    let pool = pool_state.0.read().await;
    gafete_service::get_all_gafetes(&pool).await
}

#[tauri::command]
pub async fn get_gafetes_disponibles(
    pool_state: State<'_, DbPool>,
    tipo: String,
) -> Result<Vec<GafeteResponse>, GafeteError> {
    let pool = pool_state.0.read().await;
    let tipo_enum: TipoGafete = tipo.parse().map_err(GafeteError::Validation)?;
    gafete_service::get_gafetes_disponibles(&pool, tipo_enum).await
}

#[tauri::command]
pub async fn is_gafete_disponible(
    pool_state: State<'_, DbPool>,
    numero: String,
    tipo: String,
) -> Result<bool, GafeteError> {
    let pool = pool_state.0.read().await;
    gafete_service::is_gafete_disponible(&pool, &numero, &tipo).await
}

#[tauri::command]
pub async fn update_gafete(
    pool_state: State<'_, DbPool>,
    numero: String,
    tipo: String,
    input: UpdateGafeteInput,
) -> Result<GafeteResponse, GafeteError> {
    let pool = pool_state.0.read().await;
    gafete_service::update_gafete(&pool, numero, tipo, input).await
}

#[tauri::command]
pub async fn update_gafete_status(
    pool_state: State<'_, DbPool>,
    numero: String,
    tipo: String,
    input: UpdateGafeteStatusInput,
    usuario_id: Option<String>,
    motivo: Option<String>,
) -> Result<GafeteResponse, GafeteError> {
    let pool = pool_state.0.read().await;
    gafete_service::update_gafete_status(&pool, numero, tipo, input.estado, usuario_id, motivo)
        .await
}

#[tauri::command]
pub async fn delete_gafete(
    pool_state: State<'_, DbPool>,
    numero: String,
    tipo: String,
) -> Result<(), GafeteError> {
    let pool = pool_state.0.read().await;
    gafete_service::delete_gafete(&pool, numero, tipo).await
}
