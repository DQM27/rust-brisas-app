// ==========================================
// src/commands/ingreso_commands.rs
// ==========================================
// Comandos generales de consulta de ingresos
// Capa delgada que delega al servicio

use crate::models::ingreso::{
    AlertaGafeteResponse, IngresoListResponse, IngresoResponse, ResolverAlertaInput,
};
use crate::services::alerta_service;
use crate::services::ingreso_general_service as service;
use chrono::Utc;
use sqlx::SqlitePool;
use tauri::State;

// ==========================================
// CONSULTAS GENERALES DE INGRESOS
// ==========================================

/// Obtiene un ingreso por ID
#[tauri::command]
pub async fn get_ingreso_by_id(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<IngresoResponse, String> {
    service::get_ingreso_by_id(&pool, &id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Ingreso no encontrado".to_string())
}

/// Obtiene todos los ingresos (limitado a 500)
#[tauri::command]
pub async fn get_all_ingresos(pool: State<'_, SqlitePool>) -> Result<IngresoListResponse, String> {
    service::get_all_ingresos_with_stats(&pool).await.map_err(|e| e.to_string())
}

/// Obtiene solo ingresos abiertos (personas adentro)
#[tauri::command]
pub async fn get_ingresos_abiertos(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<IngresoResponse>, String> {
    service::get_ingresos_abiertos(&pool).await.map_err(|e| e.to_string())
}

/// Busca ingreso abierto por número de gafete
#[tauri::command]
pub async fn get_ingreso_by_gafete(
    pool: State<'_, SqlitePool>,
    gafete_numero: String,
) -> Result<IngresoResponse, String> {
    service::get_ingreso_by_gafete(&pool, &gafete_numero)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("No se encontró ingreso activo con gafete {}", gafete_numero))
}

/// Obtiene salidas en rango de fechas (YYYY-MM-DD)
#[tauri::command]
pub async fn get_salidas_en_rango(
    pool: State<'_, SqlitePool>,
    fecha_inicio: String,
    fecha_fin: String,
) -> Result<Vec<IngresoResponse>, String> {
    service::get_salidas_en_rango(&pool, &fecha_inicio, &fecha_fin).await.map_err(|e| e.to_string())
}

/// Obtiene salidas de un día (YYYY-MM-DD)
#[tauri::command]
pub async fn get_salidas_del_dia(
    pool: State<'_, SqlitePool>,
    fecha: String,
) -> Result<Vec<IngresoResponse>, String> {
    service::get_salidas_en_rango(&pool, &fecha, &fecha).await.map_err(|e| e.to_string())
}

// ==========================================
// GESTIÓN DE ALERTAS DE GAFETES
// ==========================================

/// Obtiene alertas pendientes de gafetes por cédula
#[tauri::command]
pub async fn get_alertas_pendientes_by_cedula(
    pool: State<'_, SqlitePool>,
    cedula: String,
) -> Result<Vec<AlertaGafeteResponse>, String> {
    alerta_service::find_pendientes_by_cedula(&pool, &cedula)
        .await
        .map_err(|e| e.to_string())
        .map(|alertas| alertas.into_iter().map(AlertaGafeteResponse::from).collect())
}

/// Obtiene todas las alertas de gafetes
#[tauri::command]
pub async fn get_all_alertas_gafetes(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<AlertaGafeteResponse>, String> {
    alerta_service::find_all(&pool, None)
        .await
        .map_err(|e| e.to_string())
        .map(|alertas| alertas.into_iter().map(AlertaGafeteResponse::from).collect())
}

/// Marca una alerta de gafete como resuelta
#[tauri::command]
pub async fn resolver_alerta_gafete(
    pool: State<'_, SqlitePool>,
    input: ResolverAlertaInput,
) -> Result<AlertaGafeteResponse, String> {
    let now = Utc::now().to_rfc3339();
    let resolver_id = input.usuario_id.unwrap_or_else(|| "sistema".to_string());

    alerta_service::resolver(
        &pool,
        &input.alerta_id,
        &now,
        input.notas.as_deref(),
        &resolver_id,
        &now,
    )
    .await
    .map_err(|e| e.to_string())?;

    let alerta =
        alerta_service::find_by_id(&pool, &input.alerta_id).await.map_err(|e| e.to_string())?;

    Ok(AlertaGafeteResponse::from(alerta))
}
