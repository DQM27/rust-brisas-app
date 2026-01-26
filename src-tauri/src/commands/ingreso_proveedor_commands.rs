/// Puertos de Entrada: Gestión de Admisión de Proveedores (UI Bridge).
///
/// Expone las operaciones comerciales de entrada y salida para personal
/// de suministros. Asegura que el frontend reciba respuestas estructuradas
/// sobre la trazabilidad de las entregas y servicios externos.
use crate::domain::errors::IngresoProveedorError;
use crate::domain::ingreso_proveedor::ValidacionIngresoProveedorResponse;
use crate::models::ingreso::{CreateIngresoProveedorInput, IngresoResponse};
use crate::services::ingreso_proveedor_service as service;
use tauri::{command, AppHandle, Emitter};

/// Registra físicamente la llegada de un proveedor.
#[command]
pub async fn crear_ingreso_proveedor_v2(
    app: AppHandle,
    input: CreateIngresoProveedorInput,
    usuario_id: String,
) -> Result<IngresoResponse, IngresoProveedorError> {
    let res = service::registrar_ingreso(input, usuario_id).await?;

    // Emitir evento para refrescar lista de gafetes en tiempo real
    let _ = app.emit("gafetes:refresh", ());

    Ok(res)
}

/// Lista los proveedores que están actualmente dentro de las instalaciones.
#[command]
pub async fn get_ingresos_proveedores_activos(
) -> Result<Vec<IngresoResponse>, IngresoProveedorError> {
    service::get_activos().await
}

/// Lista el historial de accesos de proveedores finalizados en un rango de fechas.
#[command]
pub async fn get_ingresos_proveedores_historial(
    fecha_inicio: String,
    fecha_fin: String,
) -> Result<Vec<IngresoResponse>, IngresoProveedorError> {
    service::get_historial(fecha_inicio, fecha_fin).await
}

/// Cierra el ciclo de admisión registrando la salida física.
#[command]
pub async fn registrar_salida_proveedor(
    app: AppHandle,
    id: String,
    usuario_id: String,
    observaciones: Option<String>,
    devolvio_gafete: bool,
) -> Result<IngresoResponse, IngresoProveedorError> {
    let res = service::registrar_salida(id, usuario_id, observaciones, devolvio_gafete).await?;

    // Emitir evento para refrescar lista de gafetes en tiempo real
    let _ = app.emit("gafetes:refresh", ());

    Ok(res)
}

/// Motor de Búsqueda: Localiza un perfil de proveedor para agilizar su ingreso.
#[command]
pub async fn search_proveedores_by_cedula(
    query: String,
) -> Result<Vec<crate::models::proveedor::ProveedorResponse>, IngresoProveedorError> {
    crate::services::proveedor_service::search_proveedores(&query)
        .await
        .map_err(|e| IngresoProveedorError::Validation(e.to_string()))
}

/// Pre-chequeo de Seguridad: Valida si el proveedor es elegible para ingresar (Lista Negra).
#[command]
pub async fn validar_ingreso_proveedor(
    proveedor_id: String,
) -> Result<ValidacionIngresoProveedorResponse, IngresoProveedorError> {
    service::validar_ingreso(proveedor_id).await
}
