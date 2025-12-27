// ==========================================
// src/commands/contratista_commands.rs
// ==========================================

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
    search_state: State<'_, SearchState>,
    input: CreateContratistaInput,
) -> Result<ContratistaResponse, ContratistaError> {
    let search_service = search_state.0.read().await;
    contratista_service::create_contratista(&search_service, input).await
}

#[tauri::command]
pub async fn get_contratista_by_id(id: String) -> Result<ContratistaResponse, ContratistaError> {
    contratista_service::get_contratista_by_id(&id).await
}

#[tauri::command]
pub async fn get_contratista_by_cedula(
    cedula: String,
) -> Result<ContratistaResponse, ContratistaError> {
    contratista_service::get_contratista_by_cedula(&cedula).await
}

#[tauri::command]
pub async fn get_all_contratistas() -> Result<ContratistaListResponse, ContratistaError> {
    contratista_service::get_all_contratistas().await
}

#[tauri::command]
pub async fn get_contratistas_activos() -> Result<Vec<ContratistaResponse>, ContratistaError> {
    contratista_service::get_contratistas_activos().await
}

#[tauri::command]
pub async fn update_contratista(
    search_state: State<'_, SearchState>,
    id: String,
    input: UpdateContratistaInput,
) -> Result<ContratistaResponse, ContratistaError> {
    let search_service = search_state.0.read().await;
    contratista_service::update_contratista(&search_service, id, input).await
}

#[tauri::command]
pub async fn cambiar_estado_contratista(
    search_state: State<'_, SearchState>,
    id: String,
    input: CambiarEstadoInput,
) -> Result<ContratistaResponse, ContratistaError> {
    let search_service = search_state.0.read().await;
    contratista_service::cambiar_estado_contratista(&search_service, id, input).await
}

#[tauri::command]
pub async fn delete_contratista(
    search_state: State<'_, SearchState>,
    id: String,
) -> Result<(), ContratistaError> {
    let search_service = search_state.0.read().await;
    contratista_service::delete_contratista(&search_service, id).await
}

// ==========================================
// COMANDOS CON AUDITORÍA
// ==========================================

#[tauri::command]
pub async fn actualizar_praind_con_historial(
    search_state: State<'_, SearchState>,
    input: contratista_service::ActualizarPraindInput,
    usuario_id: String,
) -> Result<ContratistaResponse, ContratistaError> {
    let search_service = search_state.0.read().await;
    contratista_service::actualizar_praind_con_historial(&search_service, input, usuario_id).await
}

#[tauri::command]
pub async fn cambiar_estado_con_historial(
    search_state: State<'_, SearchState>,
    input: contratista_service::CambiarEstadoConHistorialInput,
    usuario_id: String,
) -> Result<ContratistaResponse, ContratistaError> {
    let search_service = search_state.0.read().await;
    contratista_service::cambiar_estado_con_historial(&search_service, input, usuario_id).await
}
