// src/commands/ingreso_proveedor_commands.rs

use crate::domain::errors::IngresoProveedorError;
use crate::domain::ingreso_proveedor::{
    CreateIngresoProveedorInput,
    IngresoProveedor, // ProveedorSnapshot, // Usaremos serde_json::Value por ahora si hace falta
    ValidacionIngresoProveedorResponse,
};
use crate::services::ingreso_proveedor_service as service;
use tauri::command;

#[command]
pub async fn crear_ingreso_proveedor_v2(
    input: CreateIngresoProveedorInput,
) -> Result<IngresoProveedor, IngresoProveedorError> {
    service::registrar_ingreso(input).await
}

#[command]
pub async fn get_ingresos_proveedores_activos(
) -> Result<Vec<IngresoProveedor>, IngresoProveedorError> {
    service::get_activos().await
}

#[command]
pub async fn get_ingresos_proveedores_historial(
) -> Result<Vec<IngresoProveedor>, IngresoProveedorError> {
    service::get_historial().await
}

#[command]
pub async fn registrar_salida_proveedor(
    id: String,
    usuario_id: String,
    observaciones: Option<String>,
    devolvio_gafete: bool,
) -> Result<(), IngresoProveedorError> {
    service::registrar_salida(id, usuario_id, observaciones, devolvio_gafete).await
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
