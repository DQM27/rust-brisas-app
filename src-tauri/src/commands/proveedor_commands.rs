// ==========================================
// src/commands/proveedor_commands.rs
// ==========================================
use crate::models::proveedor::{CreateProveedorInput, ProveedorResponse};
use crate::services::proveedor_service;
use crate::services::search_service::SearchService;
use sqlx::SqlitePool;
use std::sync::Arc;
use tauri::{command, State};

#[command]
pub async fn create_proveedor(
    pool: State<'_, SqlitePool>,
    search_service: State<'_, Arc<SearchService>>,
    input: CreateProveedorInput,
) -> Result<ProveedorResponse, String> {
    proveedor_service::create_proveedor(&pool, &search_service, input)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn search_proveedores_catalog(
    pool: State<'_, SqlitePool>,
    query: String,
) -> Result<Vec<ProveedorResponse>, String> {
    proveedor_service::search_proveedores(&pool, &query).await.map_err(|e| e.to_string())
}

#[command]
pub async fn get_proveedor_by_cedula(
    pool: State<'_, SqlitePool>,
    cedula: String,
) -> Result<Option<ProveedorResponse>, String> {
    proveedor_service::get_proveedor_by_cedula(&pool, &cedula).await.map_err(|e| e.to_string())
}

#[command]
pub async fn change_proveedor_status(
    pool: State<'_, SqlitePool>,
    search_service: State<'_, Arc<SearchService>>,
    id: String,
    new_status: String,
) -> Result<ProveedorResponse, String> {
    proveedor_service::change_status(&pool, &search_service, &id, &new_status)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn update_proveedor(
    pool: State<'_, SqlitePool>,
    search_service: State<'_, Arc<SearchService>>,
    id: String,
    input: crate::models::proveedor::UpdateProveedorInput,
) -> Result<ProveedorResponse, String> {
    proveedor_service::update_proveedor(&pool, &search_service, id, input)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn get_proveedor_by_id(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<ProveedorResponse, String> {
    proveedor_service::get_proveedor_by_id(&pool, &id).await.map_err(|e| e.to_string())
}
