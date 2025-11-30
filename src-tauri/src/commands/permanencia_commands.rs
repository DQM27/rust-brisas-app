// ==========================================
// src/commands/permanencia_commands.rs
// ==========================================
// Comandos Tauri para la fase de PERMANENCIA (monitoreo)

use crate::services::permanencia_service::{
    self, AlertaListaNegra, AlertaTiempoExcedido, IngresoConEstadoResponse, ResumenPermanencias,
};
use sqlx::SqlitePool;
use tauri::State;

// ==========================================
// CONSULTAS CON ESTADO
// ==========================================

/// Obtiene un ingreso por ID con información de estado de permanencia
#[tauri::command]
pub async fn get_ingreso_con_estado(
    pool: State<'_, SqlitePool>,
    ingreso_id: String,
) -> Result<IngresoConEstadoResponse, String> {
    permanencia_service::get_ingreso_con_estado(&pool, ingreso_id).await
}

/// Obtiene todos los ingresos abiertos con alertas de tiempo
#[tauri::command]
pub async fn get_ingresos_abiertos_con_alertas(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<IngresoConEstadoResponse>, String> {
    permanencia_service::get_ingresos_abiertos_con_alertas(&pool).await
}

// ==========================================
// VERIFICACIÓN DE ALERTAS
// ==========================================

/// Verifica si hay contratistas que excedieron el tiempo límite (>= 14h)
#[tauri::command]
pub async fn verificar_tiempos_excedidos(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<AlertaTiempoExcedido>, String> {
    permanencia_service::verificar_tiempos_excedidos(&pool).await
}

/// Verifica si hay contratistas próximos al límite (>= 13h 30min)
#[tauri::command]
pub async fn verificar_alertas_tempranas(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<AlertaTiempoExcedido>, String> {
    permanencia_service::verificar_alertas_tempranas(&pool).await
}

/// Verifica si un contratista específico fue bloqueado mientras estaba dentro
#[tauri::command]
pub async fn verificar_cambio_lista_negra(
    pool: State<'_, SqlitePool>,
    ingreso_id: String,
) -> Result<Option<AlertaListaNegra>, String> {
    permanencia_service::verificar_cambio_lista_negra(&pool, ingreso_id).await
}

/// Verifica cambios en lista negra para todos los ingresos abiertos
#[tauri::command]
pub async fn verificar_cambios_lista_negra_masivo(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<AlertaListaNegra>, String> {
    permanencia_service::verificar_cambios_lista_negra_masivo(&pool).await
}

// ==========================================
// DASHBOARD / ESTADÍSTICAS
// ==========================================

/// Obtiene resumen de estado de todos los ingresos abiertos
#[tauri::command]
pub async fn get_resumen_permanencias(
    pool: State<'_, SqlitePool>,
) -> Result<ResumenPermanencias, String> {
    permanencia_service::get_resumen_permanencias(&pool).await
}