// ==========================================
// src/commands/lista_negra_commands.rs
// ==========================================
// Capa de API: Tauri command handlers (thin wrappers)
// Solo delega a la capa de servicio

use crate::models::lista_negra::{
    AddToListaNegraInput, BlockCheckResponse, ListaNegraListResponse, ListaNegraResponse,
    UpdateListaNegraInput,
};
use crate::services::lista_negra_service;
use crate::services::search_service::SearchService;
use sqlx::SqlitePool;
use std::sync::Arc;
use tauri::State;

/// Agrega una persona a la lista negra
#[tauri::command]
pub async fn add_to_lista_negra(
    pool: State<'_, SqlitePool>,
    search_service: State<'_, Arc<SearchService>>,
    input: AddToListaNegraInput,
) -> Result<ListaNegraResponse, String> {
    lista_negra_service::add_to_lista_negra(&pool, &search_service, input).await
}

/// Obtiene un registro de lista negra por ID
#[tauri::command]
pub async fn get_lista_negra_by_id(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<ListaNegraResponse, String> {
    lista_negra_service::get_lista_negra_by_id(&pool, &id).await
}

/// Obtiene todos los registros de lista negra
#[tauri::command]
pub async fn get_all_lista_negra(
    pool: State<'_, SqlitePool>,
) -> Result<ListaNegraListResponse, String> {
    lista_negra_service::get_all_lista_negra(&pool).await
}

/// Obtiene solo los registros activos de lista negra
#[tauri::command]
pub async fn get_lista_negra_activos(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<ListaNegraResponse>, String> {
    lista_negra_service::get_lista_negra_activos(&pool).await
}

/// Verifica si una cédula está bloqueada (CRÍTICO para validaciones)
#[tauri::command]
pub async fn check_is_blocked(
    pool: State<'_, SqlitePool>,
    cedula: String,
) -> Result<BlockCheckResponse, String> {
    lista_negra_service::check_is_blocked(&pool, cedula).await
}

/// Obtiene información de bloqueo por cédula
#[tauri::command]
pub async fn get_blocked_by_cedula(
    pool: State<'_, SqlitePool>,
    cedula: String,
) -> Result<Option<ListaNegraResponse>, String> {
    lista_negra_service::get_blocked_by_cedula(&pool, cedula).await
}

/// Desactiva un bloqueo (quita de lista negra)
#[tauri::command]
pub async fn remove_from_lista_negra(
    pool: State<'_, SqlitePool>,
    search_service: State<'_, Arc<SearchService>>,
    id: String,
    motivo: String,
    observacion: Option<String>,
) -> Result<ListaNegraResponse, String> {
    lista_negra_service::remove_from_lista_negra(&pool, &search_service, id, motivo, observacion)
        .await
}

/// Reactiva un bloqueo (re-bloquear persona previamente desbloqueada)
#[tauri::command]
pub async fn reactivate_lista_negra(
    pool: State<'_, SqlitePool>,
    search_service: State<'_, Arc<SearchService>>,
    id: String,
    motivo_bloqueo: String,
    observaciones: Option<String>,
    bloqueado_por: String,
) -> Result<ListaNegraResponse, String> {
    lista_negra_service::reactivate_lista_negra(
        &pool,
        &search_service,
        id,
        motivo_bloqueo,
        observaciones,
        bloqueado_por,
    )
    .await
}

/// Actualiza información de un bloqueo
#[tauri::command]
pub async fn update_lista_negra(
    pool: State<'_, SqlitePool>,
    search_service: State<'_, Arc<SearchService>>,
    id: String,
    input: UpdateListaNegraInput,
) -> Result<ListaNegraResponse, String> {
    lista_negra_service::update_lista_negra(&pool, &search_service, id, input).await
}

/// Elimina permanentemente un registro de lista negra
#[tauri::command]
pub async fn delete_lista_negra(
    pool: State<'_, SqlitePool>,
    search_service: State<'_, Arc<SearchService>>,
    id: String,
) -> Result<(), String> {
    lista_negra_service::delete_lista_negra(&pool, &search_service, id).await
}