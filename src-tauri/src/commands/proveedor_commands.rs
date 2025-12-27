// ==========================================
// src/commands/proveedor_commands.rs
// ==========================================

use crate::models::proveedor::{CreateProveedorInput, ProveedorResponse, UpdateProveedorInput};
use crate::services::proveedor_service;
use crate::services::search_service::SearchService;
use std::sync::Arc;
use tauri::{command, State};

#[command]
pub async fn create_proveedor(
    search_service: State<'_, Arc<SearchService>>,
    input: CreateProveedorInput,
) -> Result<ProveedorResponse, String> {
    proveedor_service::create_proveedor(&search_service, input).await.map_err(|e| e.to_string())
}

#[command]
pub async fn search_proveedores_catalog(query: String) -> Result<Vec<ProveedorResponse>, String> {
    proveedor_service::search_proveedores(&query).await.map_err(|e| e.to_string())
}

#[command]
pub async fn get_proveedor_by_cedula(cedula: String) -> Result<Option<ProveedorResponse>, String> {
    proveedor_service::get_proveedor_by_cedula(&cedula).await.map_err(|e| e.to_string())
}

#[command]
pub async fn change_proveedor_status(
    search_service: State<'_, Arc<SearchService>>,
    id: String,
    new_status: String,
) -> Result<ProveedorResponse, String> {
    proveedor_service::change_status(&search_service, &id, &new_status)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn update_proveedor(
    search_service: State<'_, Arc<SearchService>>,
    id: String,
    input: UpdateProveedorInput,
) -> Result<ProveedorResponse, String> {
    proveedor_service::update_proveedor(&search_service, id, input).await.map_err(|e| e.to_string())
}

#[command]
pub async fn get_proveedor_by_id(id: String) -> Result<ProveedorResponse, String> {
    proveedor_service::get_proveedor_by_id(&id).await.map_err(|e| e.to_string())
}

#[command]
pub async fn delete_proveedor(
    search_service: State<'_, Arc<SearchService>>,
    id: String,
) -> Result<(), String> {
    proveedor_service::delete_proveedor(&search_service, &id).await.map_err(|e| e.to_string())
}

#[command]
pub async fn restore_proveedor(
    search_service: State<'_, Arc<SearchService>>,
    id: String,
) -> Result<ProveedorResponse, String> {
    proveedor_service::restore_proveedor(&search_service, &id).await.map_err(|e| e.to_string())
}

#[command]
pub async fn get_archived_proveedores() -> Result<Vec<ProveedorResponse>, String> {
    proveedor_service::get_archived_proveedores().await.map_err(|e| e.to_string())
}
