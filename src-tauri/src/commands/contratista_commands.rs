// ==========================================
// src/commands/contratista_commands.rs
// ==========================================
// Capa de comandos Tauri: delega al servicio

use crate::db::DbPool;
use crate::domain::errors::ContratistaError;
use crate::models::contratista::{
    CambiarEstadoInput, ContratistaListResponse, ContratistaResponse, CreateContratistaInput,
    UpdateContratistaInput,
};
use crate::services::contratista_service;
use crate::services::search_service::SearchState;
use tauri::State;

#[tauri::command]
pub async fn create_contratista(
    pool_state: State<'_, DbPool>,
    search_state: State<'_, SearchState>,
    input: CreateContratistaInput,
) -> Result<ContratistaResponse, ContratistaError> {
    let pool = pool_state.0.read().await;
    let search_service = search_state.0.read().await;
    contratista_service::create_contratista(&pool, &search_service, input).await
}

#[tauri::command]
pub async fn get_contratista_by_id(
    pool_state: State<'_, DbPool>,
    id: String,
) -> Result<ContratistaResponse, ContratistaError> {
    let pool = pool_state.0.read().await;
    contratista_service::get_contratista_by_id(&pool, &id).await
}

#[tauri::command]
pub async fn get_contratista_by_cedula(
    pool_state: State<'_, DbPool>,
    cedula: String,
) -> Result<ContratistaResponse, ContratistaError> {
    let pool = pool_state.0.read().await;
    contratista_service::get_contratista_by_cedula(&pool, &cedula).await
}

#[tauri::command]
pub async fn get_all_contratistas(
    pool_state: State<'_, DbPool>,
) -> Result<ContratistaListResponse, ContratistaError> {
    let pool = pool_state.0.read().await;
    contratista_service::get_all_contratistas(&pool).await
}

#[tauri::command]
pub async fn get_contratistas_activos(
    pool_state: State<'_, DbPool>,
) -> Result<Vec<ContratistaResponse>, ContratistaError> {
    let pool = pool_state.0.read().await;
    contratista_service::get_contratistas_activos(&pool).await
}

#[tauri::command]
pub async fn update_contratista(
    pool_state: State<'_, DbPool>,
    search_state: State<'_, SearchState>,
    id: String,
    input: UpdateContratistaInput,
) -> Result<ContratistaResponse, ContratistaError> {
    let pool = pool_state.0.read().await;
    let search_service = search_state.0.read().await;
    contratista_service::update_contratista(&pool, &search_service, id, input).await
}

#[tauri::command]
pub async fn cambiar_estado_contratista(
    pool_state: State<'_, DbPool>,
    search_state: State<'_, SearchState>,
    id: String,
    input: CambiarEstadoInput,
) -> Result<ContratistaResponse, ContratistaError> {
    let pool = pool_state.0.read().await;
    let search_service = search_state.0.read().await;
    contratista_service::cambiar_estado_contratista(&pool, &search_service, id, input).await
}

#[tauri::command]
pub async fn delete_contratista(
    pool_state: State<'_, DbPool>,
    search_state: State<'_, SearchState>,
    id: String,
) -> Result<(), ContratistaError> {
    let pool = pool_state.0.read().await;
    let search_service = search_state.0.read().await;
    contratista_service::delete_contratista(&pool, &search_service, id).await
}

// ==========================================
// COMANDOS CON AUDITORÍA
// ==========================================

/// Actualiza la fecha PRAIND de un contratista con registro en historial
#[tauri::command]
pub async fn actualizar_praind_con_historial(
    pool_state: State<'_, DbPool>,
    search_state: State<'_, SearchState>,
    input: contratista_service::ActualizarPraindInput,
    usuario_id: String,
) -> Result<ContratistaResponse, ContratistaError> {
    let pool = pool_state.0.read().await;
    let search_service = search_state.0.read().await;
    contratista_service::actualizar_praind_con_historial(&pool, &search_service, input, usuario_id)
        .await
}

/// Cambia el estado de un contratista con registro de motivo en historial
#[tauri::command]
pub async fn cambiar_estado_con_historial(
    pool_state: State<'_, DbPool>,
    search_state: State<'_, SearchState>,
    input: contratista_service::CambiarEstadoConHistorialInput,
    usuario_id: String,
) -> Result<ContratistaResponse, ContratistaError> {
    let pool = pool_state.0.read().await;
    let search_service = search_state.0.read().await;
    contratista_service::cambiar_estado_con_historial(&pool, &search_service, input, usuario_id)
        .await
}
