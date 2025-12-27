// ==========================================
// src/commands/ingreso_visita_commands.rs
// ==========================================
// Capa de API: Tauri command handlers

use crate::domain::errors::IngresoVisitaError;
use crate::domain::ingreso_visita::{CreateIngresoVisitaFullInput, IngresoVisitaPopulated};
use crate::models::ingreso::IngresoResponse;
use crate::services::ingreso_visita_service as service;
use tauri::command;

#[command]
pub async fn crear_ingreso_visita_v2(
    input: CreateIngresoVisitaFullInput,
    usuario_id: String,
) -> Result<IngresoResponse, IngresoVisitaError> {
    service::registrar_ingreso_full(input, usuario_id).await
}

#[command]
pub async fn validar_ingreso_visita(
    visitante_id: String,
) -> Result<crate::domain::ingreso_visita::ValidacionIngresoVisitaResponse, IngresoVisitaError> {
    service::validar_ingreso(&visitante_id).await
}

#[command]
pub async fn get_ingresos_visitas_activos(
) -> Result<Vec<IngresoVisitaPopulated>, IngresoVisitaError> {
    service::get_activos().await
}

#[command]
pub async fn registrar_salida_visita(
    id: String,
    usuario_id: String,
    devolvio_gafete: bool,
    observaciones: Option<String>,
) -> Result<IngresoResponse, IngresoVisitaError> {
    service::registrar_salida(id, usuario_id, devolvio_gafete, observaciones).await
}

#[command]
pub async fn get_ingresos_visitas_historial(
) -> Result<Vec<IngresoVisitaPopulated>, IngresoVisitaError> {
    service::get_historial().await
}
