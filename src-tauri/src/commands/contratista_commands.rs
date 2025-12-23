// ==========================================
// src/commands/contratista_commands.rs
// ==========================================
// Capa de comandos Tauri: delega al servicio

use crate::domain::errors::ContratistaError;
use crate::models::contratista::{
    CambiarEstadoInput, ContratistaListResponse, ContratistaResponse, CreateContratistaInput,
    UpdateContratistaInput,
};
use crate::services::contratista_service;
use crate::services::search_service::SearchService;
use sqlx::SqlitePool;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub async fn create_contratista(
    pool: State<'_, SqlitePool>,
    search_service: State<'_, Arc<SearchService>>,
    input: CreateContratistaInput,
) -> Result<ContratistaResponse, ContratistaError> {
    contratista_service::create_contratista(&pool, &search_service, input).await
}

#[tauri::command]
pub async fn get_contratista_by_id(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<ContratistaResponse, ContratistaError> {
    contratista_service::get_contratista_by_id(&pool, &id).await
}

#[tauri::command]
pub async fn get_contratista_by_cedula(
    pool: State<'_, SqlitePool>,
    cedula: String,
) -> Result<ContratistaResponse, ContratistaError> {
    contratista_service::get_contratista_by_cedula(&pool, &cedula).await
}

#[tauri::command]
pub async fn get_all_contratistas(
    pool: State<'_, SqlitePool>,
) -> Result<ContratistaListResponse, ContratistaError> {
    contratista_service::get_all_contratistas(&pool).await
}

#[tauri::command]
pub async fn get_contratistas_activos(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<ContratistaResponse>, ContratistaError> {
    contratista_service::get_contratistas_activos(&pool).await
}

#[tauri::command]
pub async fn update_contratista(
    pool: State<'_, SqlitePool>,
    search_service: State<'_, Arc<SearchService>>,
    id: String,
    input: UpdateContratistaInput,
) -> Result<ContratistaResponse, ContratistaError> {
    contratista_service::update_contratista(&pool, &search_service, id, input).await
}

#[tauri::command]
pub async fn cambiar_estado_contratista(
    pool: State<'_, SqlitePool>,
    search_service: State<'_, Arc<SearchService>>,
    id: String,
    input: CambiarEstadoInput,
) -> Result<ContratistaResponse, ContratistaError> {
    contratista_service::cambiar_estado_contratista(&pool, &search_service, id, input).await
}

#[tauri::command]
pub async fn delete_contratista(
    pool: State<'_, SqlitePool>,
    search_service: State<'_, Arc<SearchService>>,
    id: String,
) -> Result<(), ContratistaError> {
    contratista_service::delete_contratista(&pool, &search_service, id).await
}

// ==========================================
// COMANDOS CON AUDITORÍA
// ==========================================

/// Actualiza la fecha PRAIND de un contratista con registro en historial
#[tauri::command]
pub async fn actualizar_praind_con_historial(
    pool: State<'_, SqlitePool>,
    search_service: State<'_, Arc<SearchService>>,
    input: contratista_service::ActualizarPraindInput,
    usuario_id: String,
) -> Result<ContratistaResponse, ContratistaError> {
    contratista_service::actualizar_praind_con_historial(&pool, &search_service, input, usuario_id)
        .await
}

/// Cambia el estado de un contratista con registro de motivo en historial
#[tauri::command]
pub async fn cambiar_estado_con_historial(
    pool: State<'_, SqlitePool>,
    search_service: State<'_, Arc<SearchService>>,
    input: contratista_service::CambiarEstadoConHistorialInput,
    usuario_id: String,
) -> Result<ContratistaResponse, ContratistaError> {
    contratista_service::cambiar_estado_con_historial(&pool, &search_service, input, usuario_id)
        .await
}
