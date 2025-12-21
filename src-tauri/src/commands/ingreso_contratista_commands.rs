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
    service::validar_ingreso_contratista(&pool, contratista_id).await
}

/// Crea el ingreso de un contratista
#[tauri::command]
pub async fn create_ingreso_contratista(
    pool: State<'_, SqlitePool>,
    input: CreateIngresoContratistaInput,
    usuario_id: String,
) -> Result<IngresoResponse, String> {
    service::crear_ingreso_contratista(&pool, input, usuario_id).await
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
    service::registrar_salida(&pool, input, usuario_id).await
}

// ==========================================
// 3. PERMANENCIA (MONITOREO)
// ==========================================

/// Obtiene todos los ingresos abiertos con su estado de tiempo calculado
#[tauri::command]
pub async fn get_permanencia_status(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<service::IngresoConEstadoResponse>, String> {
    service::get_ingresos_abiertos_con_alertas(&pool).await
}

/// Verifica alertas de tiempo excedido (para notificaciones)
#[tauri::command]
pub async fn check_time_alerts(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<service::AlertaTiempoExcedido>, String> {
    service::verificar_tiempos_excedidos(&pool).await
}
