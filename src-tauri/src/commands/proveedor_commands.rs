// ==========================================
// src/commands/proveedor_commands.rs
// ==========================================
use crate::models::proveedor::{CreateProveedorInput, ProveedorResponse};
use tauri::command;

#[command]
pub async fn create_proveedor(_input: CreateProveedorInput) -> Result<ProveedorResponse, String> {
    Err("No implementado para SurrealDB aún".to_string())
}

#[command]
pub async fn search_proveedores_catalog(_query: String) -> Result<Vec<ProveedorResponse>, String> {
    Ok(vec![])
}

#[command]
pub async fn get_proveedor_by_cedula(_cedula: String) -> Result<Option<ProveedorResponse>, String> {
    Ok(None)
}

#[command]
pub async fn change_proveedor_status(
    _id: String,
    _new_status: String,
) -> Result<ProveedorResponse, String> {
    Err("No implementado para SurrealDB aún".to_string())
}

#[command]
pub async fn update_proveedor(
    _id: String,
    _input: crate::models::proveedor::UpdateProveedorInput,
) -> Result<ProveedorResponse, String> {
    Err("No implementado para SurrealDB aún".to_string())
}

#[command]
pub async fn get_proveedor_by_id(_id: String) -> Result<ProveedorResponse, String> {
    Err("No implementado para SurrealDB aún".to_string())
}
