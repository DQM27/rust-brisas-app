// ==========================================
// src/commands/ingreso_commands.rs
// ==========================================
// Comandos generales de consulta de ingresos
// Capa delgada que delega al servicio

use crate::domain::errors::AlertaError;
use crate::models::ingreso::{
    AlertaGafeteResponse, IngresoListResponse, IngresoResponse, ResolverAlertaInput,
};
use crate::services::alerta_service;
use crate::services::ingreso_general_service;
use crate::services::session::SessionState;
use tauri::State;

// ==========================================
// CONSULTAS GENERALES DE INGRESOS
// ==========================================

/// Obtiene un ingreso por ID
#[tauri::command]
pub async fn get_ingreso_by_id(id: String) -> Result<IngresoResponse, String> {
    ingreso_general_service::get_ingreso_by_id(&id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Ingreso no encontrado".to_string())
}

/// Obtiene todos los ingresos (limitado a 500)
#[tauri::command]
pub async fn get_all_ingresos() -> Result<IngresoListResponse, String> {
    ingreso_general_service::get_all_ingresos_with_stats().await.map_err(|e| e.to_string())
}

/// Obtiene solo ingresos abiertos (personas adentro)
#[tauri::command]
pub async fn get_ingresos_abiertos() -> Result<Vec<IngresoResponse>, String> {
    ingreso_general_service::get_ingresos_abiertos().await.map_err(|e| e.to_string())
}

/// Busca ingreso abierto por número de gafete
#[tauri::command]
pub async fn get_ingreso_by_gafete(gafete_numero: String) -> Result<IngresoResponse, String> {
    ingreso_general_service::get_ingreso_by_gafete(&gafete_numero)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "No se encontró ingreso activo para este gafete".to_string())
}

/// Obtiene salidas en rango de fechas
#[tauri::command]
pub async fn get_salidas_en_rango(
    fecha_inicio: String,
    fecha_fin: String,
) -> Result<Vec<IngresoResponse>, String> {
    ingreso_general_service::get_salidas_en_rango(&fecha_inicio, &fecha_fin)
        .await
        .map_err(|e| e.to_string())
}

/// Obtiene salidas de un día (YYYY-MM-DD)
#[tauri::command]
pub async fn get_salidas_del_dia(fecha: String) -> Result<Vec<IngresoResponse>, String> {
    // Reutilizamos rango: inicio del dia a fin del dia
    // Asumimos que fecha es YYYY-MM-DD
    // O implementamos lógica en servicio.
    // Como las fechas en DB son ISO, YYYY-MM-DD cubre YYYY-MM-DDT00:00:00 a YYYY-MM-DDT23:59:59 si comparamos string prefix o rango.
    // El query `get_salidas_en_rango` hace >= start y <= end.
    // Si paso "2023-01-01T00:00:00" y "2023-01-01T23:59:59".
    let start = format!("{}T00:00:00Z", fecha); // Asumiendo UTC o local ISO sin offset
    let end = format!("{}T23:59:59Z", fecha);
    ingreso_general_service::get_salidas_en_rango(&start, &end).await.map_err(|e| e.to_string())
}

// ==========================================
// GESTIÓN DE ALERTAS DE GAFETES
// ==========================================

/// Obtiene alertas pendientes de gafetes por cédula
#[tauri::command]
pub async fn get_alertas_pendientes_by_cedula(
    cedula: String,
) -> Result<Vec<AlertaGafeteResponse>, AlertaError> {
    let alertas = alerta_service::find_pendientes_by_cedula(&cedula).await?;
    let response = alertas.into_iter().map(AlertaGafeteResponse::from).collect();
    Ok(response)
}

/// Obtiene todas las alertas de gafetes
#[tauri::command]
pub async fn get_all_alertas_gafetes() -> Result<Vec<AlertaGafeteResponse>, AlertaError> {
    let alertas = alerta_service::find_all(None).await?;
    let response = alertas.into_iter().map(AlertaGafeteResponse::from).collect();
    Ok(response)
}

/// Marca una alerta de gafete como resuelta
#[tauri::command]
pub async fn resolver_alerta_gafete(
    session: State<'_, SessionState>,
    input: ResolverAlertaInput,
) -> Result<(), AlertaError> {
    let user =
        session.get_user().ok_or(AlertaError::Validation("No hay sesión activa".to_string()))?;

    alerta_service::resolver(&input.alerta_id, input.notas.as_deref(), &user.id).await?;

    Ok(())
}
