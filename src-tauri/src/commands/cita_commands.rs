// ==========================================
// src/commands/cita_commands.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::domain::errors::CitaError;
use crate::models::cita::{CitaResponse, CreateCitaInput};
use crate::models::visitante::{CreateVisitanteInput, VisitanteResponse};
use crate::services::cita_service;
use log::debug;
use tauri::command;

// ==========================================
// QUERIES
// ==========================================

#[command]
pub async fn get_citas_hoy() -> Result<Vec<CitaResponse>, CitaError> {
    cita_service::get_citas_hoy().await
}

#[command]
pub async fn get_citas_pendientes() -> Result<Vec<CitaResponse>, CitaError> {
    cita_service::get_citas_pendientes().await
}

#[command]
pub async fn get_cita_by_id(id: String) -> Result<CitaResponse, CitaError> {
    cita_service::get_cita_by_id(id).await
}

#[command]
pub async fn get_visitante_by_cedula(
    cedula: String,
) -> Result<Option<VisitanteResponse>, CitaError> {
    crate::services::visitante_service::get_visitante_by_cedula(&cedula)
        .await
        .map_err(|e| CitaError::Database(e.to_string()))
}

// ==========================================
// MUTATIONS
// ==========================================

#[command]
pub async fn create_cita(
    cita: CreateCitaInput,
    visitante: Option<CreateVisitanteInput>,
    usuario_id: String,
) -> Result<CitaResponse, CitaError> {
    debug!("Creating Cita: {:?}", cita);

    cita_service::agendar_cita(
        cita.visitante_id,
        visitante,
        cita.fecha_cita,
        cita.anfitrion,
        cita.area_visitada,
        cita.motivo,
        usuario_id,
    )
    .await
}

#[command]
pub async fn procesar_ingreso_cita(
    cita_id: String,
    gafete: Option<String>,
    usuario_id: String,
) -> Result<CitaResponse, CitaError> {
    cita_service::procesar_ingreso_cita(cita_id, gafete, usuario_id).await
}

#[command]
pub async fn cancelar_cita(id: String) -> Result<(), CitaError> {
    cita_service::cancelar_cita(id).await
}

#[command]
pub async fn completar_cita(id: String) -> Result<CitaResponse, CitaError> {
    cita_service::completar_cita(id).await
}
