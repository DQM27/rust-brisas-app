// ==========================================
// src/commands/lista_negra_commands.rs
// ==========================================
// Capa de API: Tauri command handlers (thin wrappers)
// Solo delega a la capa de servicio

use crate::db::DbPool;
use crate::domain::errors::ListaNegraError;
use crate::models::lista_negra::{
    AddToListaNegraInput, BlockCheckResponse, ListaNegraListResponse, ListaNegraResponse,
    UpdateListaNegraInput,
};
use crate::services::lista_negra_service;
use crate::services::search_service::SearchState;
use tauri::State;

/// Agrega una persona a la lista negra
#[tauri::command]
pub async fn add_to_lista_negra(
    pool_state: State<'_, DbPool>,
    search_state: State<'_, SearchState>,
    input: AddToListaNegraInput,
) -> Result<ListaNegraResponse, ListaNegraError> {
    let pool = pool_state.0.read().await;
    let search_service = search_state.0.read().await;
    lista_negra_service::add_to_lista_negra(&pool, &search_service, input).await
}

/// Obtiene un registro de lista negra por ID
#[tauri::command]
pub async fn get_lista_negra_by_id(
    pool_state: State<'_, DbPool>,
    id: String,
) -> Result<ListaNegraResponse, ListaNegraError> {
    let pool = pool_state.0.read().await;
    lista_negra_service::get_lista_negra_by_id(&pool, &id).await
}

/// Obtiene todos los registros de lista negra
#[tauri::command]
pub async fn get_all_lista_negra(
    pool_state: State<'_, DbPool>,
) -> Result<ListaNegraListResponse, ListaNegraError> {
    let pool = pool_state.0.read().await;
    lista_negra_service::get_all_lista_negra(&pool).await
}

/// Obtiene solo los registros activos de lista negra
#[tauri::command]
pub async fn get_lista_negra_activos(
    pool_state: State<'_, DbPool>,
) -> Result<Vec<ListaNegraResponse>, ListaNegraError> {
    let pool = pool_state.0.read().await;
    lista_negra_service::get_lista_negra_activos(&pool).await
}

/// Verifica si una cédula está bloqueada (CRÍTICO para validaciones)
#[tauri::command]
pub async fn check_is_blocked(
    pool_state: State<'_, DbPool>,
    cedula: String,
) -> Result<BlockCheckResponse, ListaNegraError> {
    let pool = pool_state.0.read().await;
    lista_negra_service::check_is_blocked(&pool, cedula).await
}

/// Obtiene información de bloqueo por cédula
#[tauri::command]
pub async fn get_blocked_by_cedula(
    pool_state: State<'_, DbPool>,
    cedula: String,
) -> Result<Option<ListaNegraResponse>, ListaNegraError> {
    let pool = pool_state.0.read().await;
    lista_negra_service::get_blocked_by_cedula(&pool, cedula).await
}

/// Desactiva un bloqueo (quita de lista negra)
#[tauri::command]
pub async fn remove_from_lista_negra(
    pool_state: State<'_, DbPool>,
    search_state: State<'_, SearchState>,
    id: String,
) -> Result<ListaNegraResponse, ListaNegraError> {
    let pool = pool_state.0.read().await;
    let search_service = search_state.0.read().await;
    lista_negra_service::remove_from_lista_negra(&pool, &search_service, id).await
}

/// Reactiva un bloqueo (re-bloquear persona previamente desbloqueada)
#[tauri::command]
pub async fn reactivate_lista_negra(
    pool_state: State<'_, DbPool>,
    search_state: State<'_, SearchState>,
    id: String,
    nivel_severidad: String,
    motivo_bloqueo: String,
    bloqueado_por: String,
) -> Result<ListaNegraResponse, ListaNegraError> {
    let pool = pool_state.0.read().await;
    let search_service = search_state.0.read().await;
    lista_negra_service::reactivate_lista_negra(
        &pool,
        &search_service,
        id,
        nivel_severidad,
        motivo_bloqueo,
        bloqueado_por,
    )
    .await
}

/// Actualiza información de un bloqueo
#[tauri::command]
pub async fn update_lista_negra(
    pool_state: State<'_, DbPool>,
    search_state: State<'_, SearchState>,
    id: String,
    input: UpdateListaNegraInput,
) -> Result<ListaNegraResponse, ListaNegraError> {
    let pool = pool_state.0.read().await;
    let search_service = search_state.0.read().await;
    lista_negra_service::update_lista_negra(&pool, &search_service, id, input).await
}

/// Elimina permanentemente un registro de lista negra
#[tauri::command]
pub async fn delete_lista_negra(
    pool_state: State<'_, DbPool>,
    search_state: State<'_, SearchState>,
    id: String,
) -> Result<(), ListaNegraError> {
    let pool = pool_state.0.read().await;
    let search_service = search_state.0.read().await;
    lista_negra_service::delete_lista_negra(&pool, &search_service, id).await
}

/// Busca personas (contratistas, proveedores, visitas) para formulario de bloqueo
#[tauri::command]
pub async fn search_personas_for_block(
    pool_state: State<'_, DbPool>,
    query: String,
) -> Result<Vec<crate::models::lista_negra::PersonaSearchResult>, ListaNegraError> {
    let pool = pool_state.0.read().await;
    lista_negra_service::search_personas_for_block(&pool, query).await
}
