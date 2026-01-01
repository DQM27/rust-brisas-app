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

// --------------------------------------------------------------------------
// CONSULTAS DE AGENDAMIENTO
// --------------------------------------------------------------------------

/// [Comando Tauri] Lista todas las citas programadas para el día actual.
///
/// # Argumentos
/// * `session` - Estado de la sesión actual para validación de permisos.
///
/// # Retorno
/// Lista de citas del día o error de permisos/base de datos.
#[command]
pub async fn get_citas_hoy(
    session: State<'_, SessionState>,
) -> Result<Vec<CitaResponse>, CitaError> {
    require_perm!(session, "citas:read")?;
    cita_service::get_citas_hoy().await
}

/// [Comando Tauri] Recupera las citas que aún no han sido procesadas.
///
/// # Argumentos
/// * `session` - Estado de la sesión actual.
///
/// # Retorno
/// Lista de citas en espera o error.
#[command]
pub async fn get_citas_pendientes(
    session: State<'_, SessionState>,
) -> Result<Vec<CitaResponse>, CitaError> {
    require_perm!(session, "citas:read")?;
    cita_service::get_citas_pendientes().await
}

/// [Comando Tauri] Obtiene el detalle de una cita por su identificador.
///
/// # Argumentos
/// * `session` - Estado de la sesión.
/// * `id` - ID de la cita solicitada.
///
/// # Retorno
/// Datos de la cita o error si no existe.
#[command]
pub async fn get_cita_by_id(
    session: State<'_, SessionState>,
    id: String,
) -> Result<CitaResponse, CitaError> {
    require_perm!(session, "citas:read")?;
    cita_service::get_cita_by_id(id).await
}

// --------------------------------------------------------------------------
// OPERACIONES DE GESTIÓN (MUTACIONES)
// --------------------------------------------------------------------------

/// [Comando Tauri] Reserva un espacio en la agenda para una visita futura.
///
/// Permite crear el perfil del visitante si no existe previamente.
///
/// # Argumentos
/// * `session` - Estado de la sesión.
/// * `cita` - Datos de la programación.
/// * `visitante` - Datos opcionales del nuevo visitante.
///
/// # Retorno
/// La cita creada o error de validación/permisos.
#[command]
pub async fn create_cita(
    session: State<'_, SessionState>,
    cita: CreateCitaInput,
    visitante: Option<CreateVisitanteInput>,
) -> Result<CitaResponse, CitaError> {
    let user = require_perm!(session, "citas:create", "Registrando nueva cita programada")?;
    cita_service::agendar_cita(cita_service::AgendarCitaParams {
        cita,
        visitante_extra: visitante,
        usuario_id: user.id,
    })
    .await
}

/// [Comando Tauri] Convierte una cita previa en un ingreso activo.
///
/// Se ejecuta cuando el visitante llega físicamente a la portería.
///
/// # Argumentos
/// * `session` - Estado de la sesión.
/// * `cita_id` - ID de la cita a completar.
/// * `gafete` - Número de identificación física asignada.
///
/// # Retorno
/// La cita actualizada o error de procesamiento.
///
/// # Errores
/// - `CitaError::Unauthorized`: Si el usuario no tiene permisos de actualización.
/// - `CitaError::NotFound`: Si la cita no existe.
/// - `CitaError::Database`: Fallo en la persistencia.
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

/// [Comando Tauri] Anula una cita antes de su ejecución.
///
/// # Argumentos
/// * `session` - Estado de la sesión.
/// * `id` - ID de la cita a cancelar.
///
/// # Retorno
/// Ok(()) si se canceló correctamente.
#[command]
pub async fn cancelar_cita(session: State<'_, SessionState>, id: String) -> Result<(), CitaError> {
    require_perm!(session, "citas:delete", format!("Cancelando cita #{}", id))?;
    cita_service::cancelar_cita(id).await
}
