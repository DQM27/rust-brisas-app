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
    _contratista_id: String,
) -> Result<ValidacionIngresoResponse, IngresoContratistaError> {
    Err(IngresoContratistaError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

/// Crea el ingreso de un contratista
#[tauri::command]
pub async fn create_ingreso_contratista(
    _input: CreateIngresoContratistaInput,
    _usuario_id: String,
) -> Result<IngresoResponse, IngresoContratistaError> {
    Err(IngresoContratistaError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

// ==========================================
// 2. SALIDA
// ==========================================

/// Valida si se puede registrar la salida (pre-chequeo)
#[tauri::command]
pub async fn validate_exit_contratista(
    _ingreso_id: String,
    _gafete_devuelto: Option<String>,
) -> Result<service::ResultadoValidacionSalida, IngresoContratistaError> {
    Err(IngresoContratistaError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

/// Registra la salida
#[tauri::command]
pub async fn register_exit_contratista(
    _input: RegistrarSalidaInput,
    _usuario_id: String,
) -> Result<IngresoResponse, IngresoContratistaError> {
    Err(IngresoContratistaError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

// ==========================================
// 3. PERMANENCIA (MONITOREO)
// ==========================================

/// Obtiene todos los ingresos abiertos con su estado de tiempo calculado
#[tauri::command]
pub async fn get_permanencia_status(
) -> Result<Vec<service::IngresoConEstadoResponse>, IngresoContratistaError> {
    Ok(vec![])
}

/// Verifica alertas de tiempo excedido (para notificaciones)
#[tauri::command]
pub async fn check_time_alerts(
) -> Result<Vec<service::AlertaTiempoExcedido>, IngresoContratistaError> {
    Ok(vec![])
}

// ==========================================
// 4. CIERRE MANUAL
// ==========================================

/// Cierra un ingreso manualmente (cuando el guardia no registró salida)
#[tauri::command]
pub async fn cerrar_ingreso_manual(
    _input: service::CerrarIngresoManualInput,
    _usuario_id: String,
) -> Result<service::ResultadoCierreManualResponse, IngresoContratistaError> {
    Err(IngresoContratistaError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

// ==========================================
// 5. INGRESO EXCEPCIONAL
// ==========================================

/// Registra un ingreso excepcional (contratista que normalmente no podría entrar)
#[tauri::command]
pub async fn registrar_ingreso_excepcional(
    _input: service::IngresoExcepcionalInput,
    _usuario_id: String,
) -> Result<service::IngresoExcepcionalResponse, IngresoContratistaError> {
    Err(IngresoContratistaError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}
