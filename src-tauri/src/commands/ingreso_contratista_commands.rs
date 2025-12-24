// src/commands/ingreso_contratista_commands.rs

use crate::db::DbPool;
use crate::domain::errors::IngresoContratistaError;
use crate::models::ingreso::{
    CreateIngresoContratistaInput, IngresoResponse, RegistrarSalidaInput, ValidacionIngresoResponse,
};
use crate::services::ingreso_contratista_service as service;
use tauri::State;

// ==========================================
// 1. ENTRADA
// ==========================================

/// Valida si un contratista puede ingresar (pre-chequeo)
#[tauri::command]
pub async fn validate_ingreso_contratista(
    pool_state: State<'_, DbPool>,
    contratista_id: String,
) -> Result<ValidacionIngresoResponse, IngresoContratistaError> {
    let pool = pool_state.0.read().await;
    service::validar_ingreso_contratista(&pool, contratista_id).await
}

/// Crea el ingreso de un contratista
#[tauri::command]
pub async fn create_ingreso_contratista(
    pool_state: State<'_, DbPool>,
    input: CreateIngresoContratistaInput,
    usuario_id: String,
) -> Result<IngresoResponse, IngresoContratistaError> {
    let pool = pool_state.0.read().await;
    service::crear_ingreso_contratista(&pool, input, usuario_id).await
}

// ==========================================
// 2. SALIDA
// ==========================================

/// Valida si se puede registrar la salida (pre-chequeo)
#[tauri::command]
pub async fn validate_exit_contratista(
    pool_state: State<'_, DbPool>,
    ingreso_id: String,
    gafete_devuelto: Option<String>,
) -> Result<service::ResultadoValidacionSalida, IngresoContratistaError> {
    let pool = pool_state.0.read().await;
    service::validar_puede_salir(&pool, &ingreso_id, gafete_devuelto.as_deref())
        .await
        .map_err(|e| IngresoContratistaError::Internal(e))
}

/// Registra la salida
#[tauri::command]
pub async fn register_exit_contratista(
    pool_state: State<'_, DbPool>,
    input: RegistrarSalidaInput,
    usuario_id: String,
) -> Result<IngresoResponse, IngresoContratistaError> {
    let pool = pool_state.0.read().await;
    service::registrar_salida(&pool, input, usuario_id).await
}

// ==========================================
// 3. PERMANENCIA (MONITOREO)
// ==========================================

/// Obtiene todos los ingresos abiertos con su estado de tiempo calculado
#[tauri::command]
pub async fn get_permanencia_status(
    pool_state: State<'_, DbPool>,
) -> Result<Vec<service::IngresoConEstadoResponse>, IngresoContratistaError> {
    let pool = pool_state.0.read().await;
    service::get_ingresos_abiertos_con_alertas(&pool).await
}

/// Verifica alertas de tiempo excedido (para notificaciones)
#[tauri::command]
pub async fn check_time_alerts(
    pool_state: State<'_, DbPool>,
) -> Result<Vec<service::AlertaTiempoExcedido>, IngresoContratistaError> {
    let pool = pool_state.0.read().await;
    service::verificar_tiempos_excedidos(&pool).await
}

// ==========================================
// 4. CIERRE MANUAL
// ==========================================

/// Cierra un ingreso manualmente (cuando el guardia no registró salida)
#[tauri::command]
pub async fn cerrar_ingreso_manual(
    pool_state: State<'_, DbPool>,
    input: service::CerrarIngresoManualInput,
    usuario_id: String,
) -> Result<service::ResultadoCierreManualResponse, IngresoContratistaError> {
    let pool = pool_state.0.read().await;
    service::cerrar_ingreso_manual(&pool, input, usuario_id).await
}

// ==========================================
// 5. INGRESO EXCEPCIONAL
// ==========================================

/// Registra un ingreso excepcional (contratista que normalmente no podría entrar)
#[tauri::command]
pub async fn registrar_ingreso_excepcional(
    pool_state: State<'_, DbPool>,
    input: service::IngresoExcepcionalInput,
    usuario_id: String,
) -> Result<service::IngresoExcepcionalResponse, IngresoContratistaError> {
    let pool = pool_state.0.read().await;
    service::registrar_ingreso_excepcional(&pool, input, usuario_id).await
}
