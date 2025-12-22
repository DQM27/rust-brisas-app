// ==========================================
// src/commands/contratista_commands.rs
// ==========================================
// Capa de comandos Tauri: delega al servicio

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
) -> Result<ContratistaResponse, String> {
    contratista_service::create_contratista(&pool, &search_service, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_contratista_by_id(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<ContratistaResponse, String> {
    contratista_service::get_contratista_by_id(&pool, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_contratista_by_cedula(
    pool: State<'_, SqlitePool>,
    cedula: String,
) -> Result<ContratistaResponse, String> {
    contratista_service::get_contratista_by_cedula(&pool, &cedula)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_all_contratistas(
    pool: State<'_, SqlitePool>,
) -> Result<ContratistaListResponse, String> {
    contratista_service::get_all_contratistas(&pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_contratistas_activos(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<ContratistaResponse>, String> {
    contratista_service::get_contratistas_activos(&pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_contratista(
    pool: State<'_, SqlitePool>,
    search_service: State<'_, Arc<SearchService>>,
    id: String,
    input: UpdateContratistaInput,
) -> Result<ContratistaResponse, String> {
    contratista_service::update_contratista(&pool, &search_service, id, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cambiar_estado_contratista(
    pool: State<'_, SqlitePool>,
    search_service: State<'_, Arc<SearchService>>,
    id: String,
    input: CambiarEstadoInput,
) -> Result<ContratistaResponse, String> {
    contratista_service::cambiar_estado_contratista(&pool, &search_service, id, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_contratista(
    pool: State<'_, SqlitePool>,
    search_service: State<'_, Arc<SearchService>>,
    id: String,
) -> Result<(), String> {
    contratista_service::delete_contratista(&pool, &search_service, id)
        .await
        .map_err(|e| e.to_string())
}
