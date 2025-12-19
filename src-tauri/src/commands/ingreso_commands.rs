// ==========================================
// src/commands/ingreso_commands.rs
// ==========================================
// Comandos generales de consulta de ingresos
// (Los comandos de entrada/salida están en sus propios módulos)

use crate::db::alerta_gafete_queries as alerta_db;
use crate::db::ingreso_general_queries as db;
use crate::models::ingreso::{
    AlertaGafeteResponse, IngresoListResponse, IngresoResponse, ResolverAlertaInput,
};
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
    let ingreso = db::find_by_id(&pool, &id).await?;
    let details = db::find_details_by_id(&pool, &id).await?;

    let mut response = IngresoResponse::from(ingreso);
    response.usuario_ingreso_nombre = details.usuario_ingreso_nombre.unwrap_or_default();
    response.usuario_salida_nombre = details.usuario_salida_nombre;
    response.vehiculo_placa = details.vehiculo_placa;

    Ok(response)
}

/// Obtiene todos los ingresos (limitado a 500)
#[tauri::command]
pub async fn get_all_ingresos(pool: State<'_, SqlitePool>) -> Result<IngresoListResponse, String> {
    let results = db::find_all_with_details(&pool).await?;

    let mut responses = Vec::new();
    for (ingreso, details) in results {
        let mut response = IngresoResponse::from(ingreso);
        response.usuario_ingreso_nombre = details.usuario_ingreso_nombre.unwrap_or_default();
        response.usuario_salida_nombre = details.usuario_salida_nombre;
        response.vehiculo_placa = details.vehiculo_placa;
        responses.push(response);
    }

    let total = responses.len();
    let adentro = responses
        .iter()
        .filter(|i| i.fecha_hora_salida.is_none())
        .count();
    let salieron = total - adentro;

    Ok(IngresoListResponse {
        ingresos: responses,
        total,
        adentro,
        salieron,
    })
}

/// Obtiene solo ingresos abiertos (personas adentro)
#[tauri::command]
pub async fn get_ingresos_abiertos(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<IngresoResponse>, String> {
    let results = db::find_ingresos_abiertos_with_details(&pool).await?;

    let mut responses = Vec::new();
    for (ingreso, details) in results {
        let mut response = IngresoResponse::from(ingreso);
        response.usuario_ingreso_nombre = details.usuario_ingreso_nombre.unwrap_or_default();
        response.usuario_salida_nombre = details.usuario_salida_nombre;
        response.vehiculo_placa = details.vehiculo_placa;
        responses.push(response);
    }

    Ok(responses)
}

/// Busca ingreso abierto por número de gafete
#[tauri::command]
pub async fn get_ingreso_by_gafete(
    pool: State<'_, SqlitePool>,
    gafete_numero: String,
) -> Result<IngresoResponse, String> {
    let ingreso = db::find_ingreso_by_gafete(&pool, &gafete_numero).await?;
    let response = IngresoResponse::from(ingreso);
    Ok(response)
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
    let alertas = alerta_db::find_pendientes_by_cedula(&pool, &cedula).await?;
    let responses: Vec<_> = alertas
        .into_iter()
        .map(AlertaGafeteResponse::from)
        .collect();
    Ok(responses)
}

/// Obtiene todas las alertas de gafetes
#[tauri::command]
pub async fn get_all_alertas_gafetes(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<AlertaGafeteResponse>, String> {
    let alertas = alerta_db::find_all(&pool, None).await?;
    let responses: Vec<_> = alertas
        .into_iter()
        .map(AlertaGafeteResponse::from)
        .collect();
    Ok(responses)
}

/// Marca una alerta de gafete como resuelta
#[tauri::command]
pub async fn resolver_alerta_gafete(
    pool: State<'_, SqlitePool>,
    input: ResolverAlertaInput,
) -> Result<AlertaGafeteResponse, String> {
    let now = Utc::now().to_rfc3339();
    let resolver_id = input.usuario_id.unwrap_or_else(|| "sistema".to_string());
    alerta_db::resolver(
        &pool,
        &input.alerta_id,
        &now,
        input.notas.as_deref(),
        &resolver_id,
        &now,
    )
    .await?;

    let alerta = alerta_db::find_by_id(&pool, &input.alerta_id).await?;
    Ok(AlertaGafeteResponse::from(alerta))
}
