// src/commands/ingreso_contratista_commands.rs

use crate::domain::errors::IngresoContratistaError;
use crate::models::ingreso::{
    CreateIngresoContratistaInput, IngresoResponse, RegistrarSalidaInput, ValidacionIngresoResponse,
};
use crate::services::ingreso_contratista_service as service;

// ==========================================
// 1. ENTRADA
// ==========================================

/// Valida si un contratista puede ingresar (pre-chequeo)
#[tauri::command]
pub async fn validate_ingreso_contratista(
    contratista_id: String,
) -> Result<ValidacionIngresoResponse, IngresoContratistaError> {
    service::validar_ingreso_contratista(contratista_id).await
}

/// Crea el ingreso de un contratista
#[tauri::command]
pub async fn create_ingreso_contratista(
    input: CreateIngresoContratistaInput,
    usuario_id: String,
) -> Result<IngresoResponse, IngresoContratistaError> {
    service::crear_ingreso_contratista(input, usuario_id).await
}

// ==========================================
// 2. SALIDA
// ==========================================

/// Valida si se puede registrar la salida (pre-chequeo)
#[tauri::command]
pub async fn validate_exit_contratista(
    ingreso_id: String,
    gafete_devuelto: Option<String>,
) -> Result<service::ResultadoValidacionSalida, IngresoContratistaError> {
    service::validar_puede_salir(&ingreso_id, gafete_devuelto.as_deref())
        .await
        .map_err(|e| IngresoContratistaError::Validation(e))
}

/// Registra la salida
#[tauri::command]
pub async fn register_exit_contratista(
    input: RegistrarSalidaInput,
    usuario_id: String,
) -> Result<IngresoResponse, IngresoContratistaError> {
    service::registrar_salida(input, usuario_id).await
}

// ==========================================
// 3. PERMANENCIA (MONITOREO)
// ==========================================

/// Obtiene todos los ingresos abiertos con su estado de tiempo calculado
#[tauri::command]
pub async fn get_permanencia_status(
) -> Result<Vec<service::IngresoConEstadoResponse>, IngresoContratistaError> {
    service::get_ingresos_abiertos_con_alertas().await
}

/// Verifica alertas de tiempo excedido (para notificaciones)
#[tauri::command]
pub async fn check_time_alerts(
) -> Result<Vec<service::AlertaTiempoExcedido>, IngresoContratistaError> {
    service::verificar_tiempos_excedidos().await
}

// ==========================================
// 4. CIERRE MANUAL
// ==========================================

/// Cierra un ingreso manualmente (cuando el guardia no registró salida)
#[tauri::command]
pub async fn cerrar_ingreso_manual(
    input: service::CerrarIngresoManualInput,
    usuario_id: String,
) -> Result<service::ResultadoCierreManualResponse, IngresoContratistaError> {
    service::cerrar_ingreso_manual(input, usuario_id).await
}

// ==========================================
// 5. INGRESO EXCEPCIONAL
// ==========================================

/// Registra un ingreso excepcional (contratista que normalmente no podría entrar)
#[tauri::command]
pub async fn registrar_ingreso_excepcional(
    input: service::IngresoExcepcionalInput,
    usuario_id: String,
) -> Result<service::IngresoExcepcionalResponse, IngresoContratistaError> {
    service::registrar_ingreso_excepcional(input, usuario_id).await
}
