// src/commands/ingreso_contratista_commands.rs

use crate::models::ingreso::{
    CreateIngresoContratistaInput, IngresoResponse, RegistrarSalidaInput, ValidacionIngresoResponse,
};
use crate::services::ingreso_contratista_service as service;
use sqlx::SqlitePool;
use tauri::State;

// ==========================================
// 1. ENTRADA
// ==========================================

/// Valida si un contratista puede ingresar (pre-chequeo)
#[tauri::command]
pub async fn validate_ingreso_contratista(
    pool: State<'_, SqlitePool>,
    contratista_id: String,
) -> Result<ValidacionIngresoResponse, String> {
    service::validar_ingreso_contratista(&pool, contratista_id).await.map_err(|e| e.to_string())
}

/// Crea el ingreso de un contratista
#[tauri::command]
pub async fn create_ingreso_contratista(
    pool: State<'_, SqlitePool>,
    input: CreateIngresoContratistaInput,
    usuario_id: String,
) -> Result<IngresoResponse, String> {
    service::crear_ingreso_contratista(&pool, input, usuario_id).await.map_err(|e| e.to_string())
}

// ==========================================
// 2. SALIDA
// ==========================================

/// Valida si se puede registrar la salida (pre-chequeo)
#[tauri::command]
pub async fn validate_exit_contratista(
    pool: State<'_, SqlitePool>,
    ingreso_id: String,
    gafete_devuelto: Option<String>,
) -> Result<service::ResultadoValidacionSalida, String> {
    service::validar_puede_salir(&pool, &ingreso_id, gafete_devuelto.as_deref()).await
}

/// Registra la salida
#[tauri::command]
pub async fn register_exit_contratista(
    pool: State<'_, SqlitePool>,
    input: RegistrarSalidaInput,
    usuario_id: String,
) -> Result<IngresoResponse, String> {
    service::registrar_salida(&pool, input, usuario_id).await.map_err(|e| e.to_string())
}

// ==========================================
// 3. PERMANENCIA (MONITOREO)
// ==========================================

/// Obtiene todos los ingresos abiertos con su estado de tiempo calculado
#[tauri::command]
pub async fn get_permanencia_status(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<service::IngresoConEstadoResponse>, String> {
    service::get_ingresos_abiertos_con_alertas(&pool).await.map_err(|e| e.to_string())
}

/// Verifica alertas de tiempo excedido (para notificaciones)
#[tauri::command]
pub async fn check_time_alerts(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<service::AlertaTiempoExcedido>, String> {
    service::verificar_tiempos_excedidos(&pool).await.map_err(|e| e.to_string())
}

// ==========================================
// 4. CIERRE MANUAL
// ==========================================

/// Cierra un ingreso manualmente (cuando el guardia no registró salida)
#[tauri::command]
pub async fn cerrar_ingreso_manual(
    pool: State<'_, SqlitePool>,
    input: service::CerrarIngresoManualInput,
    usuario_id: String,
) -> Result<service::ResultadoCierreManualResponse, String> {
    service::cerrar_ingreso_manual(&pool, input, usuario_id).await.map_err(|e| e.to_string())
}

// ==========================================
// 5. INGRESO EXCEPCIONAL
// ==========================================

/// Registra un ingreso excepcional (contratista que normalmente no podría entrar)
#[tauri::command]
pub async fn registrar_ingreso_excepcional(
    pool: State<'_, SqlitePool>,
    input: service::IngresoExcepcionalInput,
    usuario_id: String,
) -> Result<service::IngresoExcepcionalResponse, String> {
    service::registrar_ingreso_excepcional(&pool, input, usuario_id)
        .await
        .map_err(|e| e.to_string())
}
