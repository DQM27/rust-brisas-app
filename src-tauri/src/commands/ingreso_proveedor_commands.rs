// src/commands/ingreso_proveedor_commands.rs

use crate::domain::errors::IngresoProveedorError;
use crate::domain::ingreso_proveedor::ValidacionIngresoProveedorResponse;
use crate::models::ingreso::{CreateIngresoProveedorInput, IngresoResponse};
use crate::services::ingreso_proveedor_service as service;
use tauri::command;

#[command]
pub async fn crear_ingreso_proveedor_v2(
    input: CreateIngresoProveedorInput,
    usuario_id: String,
) -> Result<IngresoResponse, String> {
    service::registrar_ingreso(input, usuario_id).await.map_err(|e| e.to_string())
}

#[command]
pub async fn get_ingresos_proveedores_activos() -> Result<Vec<IngresoResponse>, String> {
    service::get_activos().await.map_err(|e| e.to_string())
}

#[command]
pub async fn get_ingresos_proveedores_historial() -> Result<Vec<IngresoResponse>, String> {
    service::get_historial().await.map_err(|e| e.to_string())
}

#[command]
pub async fn registrar_salida_proveedor(
    id: String,
    usuario_id: String,
    observaciones: Option<String>,
    devolvio_gafete: bool,
) -> Result<IngresoResponse, String> {
    service::registrar_salida(id, usuario_id, observaciones, devolvio_gafete)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn search_proveedores(
    query: String,
) -> Result<Vec<serde_json::Value>, IngresoProveedorError> {
    service::search_proveedores(&query).await
}

#[command]
pub async fn validar_ingreso_proveedor(
    proveedor_id: String,
) -> Result<ValidacionIngresoProveedorResponse, IngresoProveedorError> {
    let res = service::validar_ingreso(proveedor_id).await?;
    // Convertir de Value a struct si es posible, o cambiar firma.
    // Domain struct es ValidacionIngresoProveedorResponse { puedeIngresar: bool... }
    serde_json::from_value(res)
        .map_err(|_| IngresoProveedorError::Validation("Error parsing validation".to_string()))
}
