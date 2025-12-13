// ==========================================
// src/commands/proveedor_commands.rs
// ==========================================
use crate::models::proveedor::{CreateProveedorInput, ProveedorResponse, UpdateProveedorInput};
use crate::services::proveedor_service::ProveedorService;
use sqlx::SqlitePool;
use tauri::{command, State};

#[command]
pub async fn create_proveedor(
    pool: State<'_, SqlitePool>,
    input: CreateProveedorInput,
) -> Result<ProveedorResponse, String> {
    let service = ProveedorService::new(pool.inner().clone());
    service.create_proveedor(input).await
}

#[command]
pub async fn search_proveedores_catalog(
    pool: State<'_, SqlitePool>,
    query: String,
) -> Result<Vec<ProveedorResponse>, String> {
    let service = ProveedorService::new(pool.inner().clone());
    service.search_proveedores(&query).await
}

#[command]
pub async fn get_proveedor_by_cedula(
    pool: State<'_, SqlitePool>,
    cedula: String,
) -> Result<Option<ProveedorResponse>, String> {
    let service = ProveedorService::new(pool.inner().clone());
    service.get_proveedor_by_cedula(&cedula).await
}
