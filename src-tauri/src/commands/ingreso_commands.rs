// ==========================================
// src/commands/ingreso_commands.rs
// ==========================================
// Comandos generales de consulta de ingresos
// Capa delgada que delega al servicio

use crate::models::ingreso::{
    AlertaGafeteResponse, IngresoListResponse, IngresoResponse, ResolverAlertaInput,
};

// ==========================================
// CONSULTAS GENERALES DE INGRESOS
// ==========================================

/// Obtiene un ingreso por ID
#[tauri::command]
pub async fn get_ingreso_by_id(_id: String) -> Result<IngresoResponse, String> {
    Err("No implementado para SurrealDB aún".to_string())
}

/// Obtiene todos los ingresos (limitado a 500)
#[tauri::command]
pub async fn get_all_ingresos() -> Result<IngresoListResponse, String> {
    Err("No implementado para SurrealDB aún".to_string())
}

/// Obtiene solo ingresos abiertos (personas adentro)
#[tauri::command]
pub async fn get_ingresos_abiertos() -> Result<Vec<IngresoResponse>, String> {
    Ok(vec![])
}

/// Busca ingreso abierto por número de gafete
#[tauri::command]
pub async fn get_ingreso_by_gafete(_gafete_numero: String) -> Result<IngresoResponse, String> {
    Err("No implementado para SurrealDB aún".to_string())
}

/// Obtiene salidas en rango de fechas (YYYY-MM-DD)
#[tauri::command]
pub async fn get_salidas_en_rango(
    _fecha_inicio: String,
    _fecha_fin: String,
) -> Result<Vec<IngresoResponse>, String> {
    Ok(vec![])
}

/// Obtiene salidas de un día (YYYY-MM-DD)
#[tauri::command]
pub async fn get_salidas_del_dia(_fecha: String) -> Result<Vec<IngresoResponse>, String> {
    Ok(vec![])
}

// ==========================================
// GESTIÓN DE ALERTAS DE GAFETES
// ==========================================

/// Obtiene alertas pendientes de gafetes por cédula
#[tauri::command]
pub async fn get_alertas_pendientes_by_cedula(
    _cedula: String,
) -> Result<Vec<AlertaGafeteResponse>, String> {
    Ok(vec![])
}

/// Obtiene todas las alertas de gafetes
#[tauri::command]
pub async fn get_all_alertas_gafetes() -> Result<Vec<AlertaGafeteResponse>, String> {
    Ok(vec![])
}

/// Marca una alerta de gafete como resuelta
#[tauri::command]
pub async fn resolver_alerta_gafete(
    _input: ResolverAlertaInput,
) -> Result<AlertaGafeteResponse, String> {
    Err("No implementado para SurrealDB aún".to_string())
}
