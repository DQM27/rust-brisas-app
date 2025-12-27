// ==========================================
// src/commands/ingreso_visita_commands.rs
// ==========================================
// Capa de API: Tauri command handlers

use crate::domain::errors::IngresoVisitaError;
use crate::domain::ingreso_visita::{
    CreateIngresoVisitaFullInput, IngresoVisita, IngresoVisitaPopulated,
};
use tauri::command;

#[command]
pub async fn crear_ingreso_visita_v2(
    _input: CreateIngresoVisitaFullInput,
) -> Result<IngresoVisita, IngresoVisitaError> {
    Err(IngresoVisitaError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

#[command]
pub async fn validar_ingreso_visita(
    _visitante_id: String,
) -> Result<crate::domain::ingreso_visita::ValidacionIngresoVisitaResponse, IngresoVisitaError> {
    Err(IngresoVisitaError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

#[command]
pub async fn get_ingresos_visitas_activos(
) -> Result<Vec<IngresoVisitaPopulated>, IngresoVisitaError> {
    Ok(vec![])
}

#[command]
pub async fn registrar_salida_visita(
    _id: String,
    _usuario_id: String,
    _devolvio_gafete: bool,
    _observaciones: Option<String>,
) -> Result<(), IngresoVisitaError> {
    Err(IngresoVisitaError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

#[command]
pub async fn get_ingresos_visitas_historial(
) -> Result<Vec<IngresoVisitaPopulated>, IngresoVisitaError> {
    Ok(vec![])
}
