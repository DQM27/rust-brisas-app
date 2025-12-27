// ==========================================
// src/commands/ingreso_proveedor_commands.rs
// ==========================================
// Capa de API: Tauri command handlers

use crate::domain::errors::IngresoProveedorError;
use crate::domain::ingreso_proveedor::{
    CreateIngresoProveedorInput, IngresoProveedor, ProveedorSnapshot,
    ValidacionIngresoProveedorResponse,
};
use tauri::command;

#[command]
pub async fn crear_ingreso_proveedor_v2(
    _input: CreateIngresoProveedorInput,
) -> Result<IngresoProveedor, IngresoProveedorError> {
    Err(IngresoProveedorError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

#[command]
pub async fn get_ingresos_proveedores_activos(
) -> Result<Vec<IngresoProveedor>, IngresoProveedorError> {
    Ok(vec![])
}

#[command]
pub async fn get_ingresos_proveedores_historial(
) -> Result<Vec<IngresoProveedor>, IngresoProveedorError> {
    Ok(vec![])
}

#[command]
pub async fn registrar_salida_proveedor(
    _id: String,
    _usuario_id: String,
    _observaciones: Option<String>,
    _devolvio_gafete: bool,
) -> Result<(), IngresoProveedorError> {
    Err(IngresoProveedorError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

#[command]
pub async fn search_proveedores(
    _query: String,
) -> Result<Vec<ProveedorSnapshot>, IngresoProveedorError> {
    Ok(vec![])
}

#[command]
pub async fn validar_ingreso_proveedor(
    _proveedor_id: String,
) -> Result<ValidacionIngresoProveedorResponse, IngresoProveedorError> {
    Err(IngresoProveedorError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}
