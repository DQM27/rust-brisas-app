// ==========================================
// src/commands/cita_commands.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::domain::errors::CitaError;
use crate::models::cita::{CitaResponse, CreateCitaInput};
use crate::models::visitante::CreateVisitanteInput;
use crate::services::cita_service;
use crate::services::session::SessionState;
use log::debug;
use tauri::{command, State};

// ==========================================
// QUERIES
// ==========================================

#[command]
pub async fn get_citas_hoy(
    session: State<'_, SessionState>,
) -> Result<Vec<CitaResponse>, CitaError> {
    require_perm!(session, "citas:read")?;
    cita_service::get_citas_hoy().await
}

#[command]
pub async fn get_citas_pendientes(
    session: State<'_, SessionState>,
) -> Result<Vec<CitaResponse>, CitaError> {
    require_perm!(session, "citas:read")?;
    cita_service::get_citas_pendientes().await
}

#[command]
pub async fn get_cita_by_id(
    session: State<'_, SessionState>,
    id: String,
) -> Result<CitaResponse, CitaError> {
    require_perm!(session, "citas:read")?;
    cita_service::get_cita_by_id(id).await
}

// ==========================================
// MUTATIONS
// ==========================================

#[command]
pub async fn create_cita(
    session: State<'_, SessionState>,
    cita: CreateCitaInput,
    visitante: Option<CreateVisitanteInput>,
) -> Result<CitaResponse, CitaError> {
    // Asegurarse de que el usuario tiene permisos
    let user = require_perm!(session, "citas:create", "Registrando cita")?;
    debug!("Creating Cita: {:?}", cita);

    cita_service::agendar_cita(
        cita.visitante_id,
        visitante,
        cita.fecha_cita,
        cita.anfitrion,
        cita.area_visitada,
        cita.motivo,
        user.id, // Usar ID de usuario de sesión
    )
    .await
}

#[command]
pub async fn procesar_ingreso_cita(
    session: State<'_, SessionState>,
    cita_id: String,
    gafete: Option<String>,
) -> Result<CitaResponse, CitaError> {
    let user =
        require_perm!(session, "citas:update", format!("Procesando ingreso de cita {}", cita_id))?;
    cita_service::procesar_ingreso_cita(cita_id, gafete, user.id).await
}

#[command]
pub async fn cancelar_cita(session: State<'_, SessionState>, id: String) -> Result<(), CitaError> {
    require_perm!(session, "citas:delete", format!("Cancelando cita {}", id))?;
    cita_service::cancelar_cita(id).await
}

#[command]
pub async fn completar_cita(
    session: State<'_, SessionState>,
    id: String,
) -> Result<CitaResponse, CitaError> {
    require_perm!(session, "citas:update", format!("Completando cita {}", id))?;
    cita_service::completar_cita(id).await
}
