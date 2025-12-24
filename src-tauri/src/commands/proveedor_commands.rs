// ==========================================
// src/commands/proveedor_commands.rs
// ==========================================
use crate::db::DbPool;
use crate::models::proveedor::{CreateProveedorInput, ProveedorResponse};
use crate::services::proveedor_service;
use crate::services::search_service::SearchState;
use tauri::{command, State};

#[command]
pub async fn create_proveedor(
    pool_state: State<'_, DbPool>,
    search_state: State<'_, SearchState>,
    input: CreateProveedorInput,
) -> Result<ProveedorResponse, String> {
    let pool = pool_state.0.read().await;
    let search_service = search_state.0.read().await;
    proveedor_service::create_proveedor(&pool, &search_service, input)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn search_proveedores_catalog(
    pool_state: State<'_, DbPool>,
    query: String,
) -> Result<Vec<ProveedorResponse>, String> {
    let pool = pool_state.0.read().await;
    proveedor_service::search_proveedores(&pool, &query).await.map_err(|e| e.to_string())
}

#[command]
pub async fn get_proveedor_by_cedula(
    pool_state: State<'_, DbPool>,
    cedula: String,
) -> Result<Option<ProveedorResponse>, String> {
    let pool = pool_state.0.read().await;
    proveedor_service::get_proveedor_by_cedula(&pool, &cedula).await.map_err(|e| e.to_string())
}

#[command]
pub async fn change_proveedor_status(
    pool_state: State<'_, DbPool>,
    search_state: State<'_, SearchState>,
    id: String,
    new_status: String,
) -> Result<ProveedorResponse, String> {
    let pool = pool_state.0.read().await;
    let search_service = search_state.0.read().await;
    proveedor_service::change_status(&pool, &search_service, &id, &new_status)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn update_proveedor(
    pool_state: State<'_, DbPool>,
    search_state: State<'_, SearchState>,
    id: String,
    input: crate::models::proveedor::UpdateProveedorInput,
) -> Result<ProveedorResponse, String> {
    let pool = pool_state.0.read().await;
    let search_service = search_state.0.read().await;
    proveedor_service::update_proveedor(&pool, &search_service, id, input)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn get_proveedor_by_id(
    pool_state: State<'_, DbPool>,
    id: String,
) -> Result<ProveedorResponse, String> {
    let pool = pool_state.0.read().await;
    proveedor_service::get_proveedor_by_id(&pool, &id).await.map_err(|e| e.to_string())
}
