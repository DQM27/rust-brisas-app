/// Puertos de Entrada: Gestión de Citas y Pre-registros (Booking Bridge).
///
/// Este módulo permite la planificación anticipada de visitas, facilitando que
/// los anfitriones registren a sus invitados antes de que lleguen físicamente a planta,
/// agilizando así el proceso de recepción.
use crate::domain::errors::CitaError;
use crate::models::cita::{CitaResponse, CreateCitaInput};
use crate::models::visitante::CreateVisitanteInput;
use crate::services::cita_service;
use crate::services::session::SessionState;
use tauri::{command, State};

// ==========================================
// CONSULTAS DE AGENDAMIENTO
// ==========================================

/// Lista todas las citas programadas para el día actual.
#[command]
pub async fn get_citas_hoy(
    session: State<'_, SessionState>,
) -> Result<Vec<CitaResponse>, CitaError> {
    require_perm!(session, "citas:read")?;
    cita_service::get_citas_hoy().await
}

/// Recupera las citas que aún no han sido procesadas o están pendientes de llegada.
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
// OPERACIONES DE GESTIÓN (MUTACIONES)
// ==========================================

/// Reserva un espacio en la agenda para una visita futura.
/// Puede crear el perfil del visitante si es su primera vez.
#[command]
pub async fn create_cita(
    session: State<'_, SessionState>,
    cita: CreateCitaInput,
    visitante: Option<CreateVisitanteInput>,
) -> Result<CitaResponse, CitaError> {
    let user = require_perm!(session, "citas:create", "Registrando nueva cita programada")?;
    cita_service::agendar_cita(
        cita.visitante_id,
        visitante,
        cita.fecha_cita,
        cita.anfitrion,
        cita.area_visitada,
        cita.motivo,
        user.id,
    )
    .await
}

/// Convierte una cita previa en un ingreso activo cuando el visitante llega a portería.
#[command]
pub async fn procesar_ingreso_cita(
    session: State<'_, SessionState>,
    cita_id: String,
    gafete: Option<String>,
) -> Result<CitaResponse, CitaError> {
    let user = require_perm!(
        session,
        "citas:update",
        format!("Validando llegada física para cita #{}", cita_id)
    )?;
    cita_service::procesar_ingreso_cita(cita_id, gafete, user.id).await
}

/// Anula una cita antes de su ejecución.
#[command]
pub async fn cancelar_cita(session: State<'_, SessionState>, id: String) -> Result<(), CitaError> {
    require_perm!(session, "citas:delete", format!("Cancelando cita #{}", id))?;
    cita_service::cancelar_cita(id).await
}
