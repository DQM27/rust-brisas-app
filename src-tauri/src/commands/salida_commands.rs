// ==========================================
// src/commands/salida_commands.rs
// ==========================================
// Comandos Tauri para la fase de SALIDA

use crate::models::ingreso::{IngresoResponse, RegistrarSalidaInput};
use crate::services::salida_service::{self, EstadisticasSalidas, ResultadoValidacionSalida};
use sqlx::SqlitePool;
use tauri::State;

// ==========================================
// VALIDACIÓN PRE-SALIDA
// ==========================================

/// Valida que se puede registrar la salida de un ingreso
#[tauri::command]
pub async fn validar_puede_salir(
    pool: State<'_, SqlitePool>,
    ingreso_id: String,
    gafete_devuelto: Option<String>,
) -> Result<ResultadoValidacionSalida, String> {
    salida_service::validar_puede_salir(&pool, &ingreso_id, gafete_devuelto.as_deref()).await
}

// ==========================================
// REGISTRAR SALIDA
// ==========================================

/// Registra la salida de un contratista
#[tauri::command]
pub async fn registrar_salida(
    pool: State<'_, SqlitePool>,
    input: RegistrarSalidaInput,
) -> Result<IngresoResponse, String> {
    let usuario_id = input.usuario_salida_id.clone();
    salida_service::registrar_salida(&pool, input, usuario_id).await
}

/// Registra salida con verificación estricta de gafete
#[tauri::command]
pub async fn registrar_salida_con_verificacion_gafete(
    pool: State<'_, SqlitePool>,
    ingreso_id: String,
    devolvio_gafete: bool,
    gafete_devuelto: Option<String>,
    usuario_id: String,
    observaciones_salida: Option<String>,
) -> Result<IngresoResponse, String> {
    salida_service::registrar_salida_con_verificacion_gafete(
        &pool,
        ingreso_id,
        devolvio_gafete,
        gafete_devuelto,
        usuario_id,
        observaciones_salida,
    )
    .await
}

// ==========================================
// CONSULTAS Y REPORTES
// ==========================================

/// Obtiene todas las salidas de un día específico
#[tauri::command]
pub async fn get_salidas_del_dia(
    pool: State<'_, SqlitePool>,
    fecha: String, // formato: "YYYY-MM-DD"
) -> Result<Vec<IngresoResponse>, String> {
    salida_service::get_salidas_del_dia(&pool, &fecha).await
}

/// Obtiene todas las salidas en un rango de fechas
#[tauri::command]
pub async fn get_salidas_en_rango(
    pool: State<'_, SqlitePool>,
    fecha_inicio: String, // formato: "YYYY-MM-DD"
    fecha_fin: String,    // formato: "YYYY-MM-DD"
) -> Result<Vec<IngresoResponse>, String> {
    salida_service::get_salidas_en_rango(&pool, &fecha_inicio, &fecha_fin).await
}

/// Obtiene estadísticas de salidas en un rango de fechas
#[tauri::command]
pub async fn get_estadisticas_salidas(
    pool: State<'_, SqlitePool>,
    fecha_desde: Option<String>,
    fecha_hasta: Option<String>,
) -> Result<EstadisticasSalidas, String> {
    salida_service::get_estadisticas_salidas(
        &pool,
        fecha_desde.as_deref(),
        fecha_hasta.as_deref(),
    )
    .await
}