/// Puertos de Entrada: Gestión Integral de Personal Externo (Contractor Bridge).
///
/// Este módulo es central para la administración de la fuerza laboral externa.
/// Orquesta el registro, validación de certificaciones (PRAIND) y auditoría
/// de contratistas.
use crate::domain::errors::ContratistaError;
use crate::models::contratista::{
    ActualizarPraindInput, CambiarEstadoConHistorialInput, CambiarEstadoInput,
    ContratistaListResponse, ContratistaResponse, CreateContratistaInput, UpdateContratistaInput,
};
use crate::repositories::contratista::{
    SurrealAuditRepository, SurrealContratistaRepository, SurrealEmpresaRepository,
    SurrealSecurityRepository, SurrealVehiculoRepository,
};
use crate::services::contratista_service::ContratistaService;
use crate::services::search_service::SearchService;
use crate::services::session::SessionState;
use std::sync::Arc;
use tauri::{command, State};

// --------------------------------------------------------------------------
// HELPERS: Construcción del Servicio
// --------------------------------------------------------------------------

/// Crea una instancia del servicio con implementaciones concretas de SurrealDB.
fn create_service(
    search_service: Option<Arc<SearchService>>,
) -> ContratistaService<
    SurrealContratistaRepository,
    SurrealSecurityRepository,
    SurrealEmpresaRepository,
    SurrealVehiculoRepository,
    SurrealAuditRepository,
> {
    ContratistaService::new(
        SurrealContratistaRepository,
        SurrealSecurityRepository,
        SurrealEmpresaRepository,
        SurrealVehiculoRepository,
        SurrealAuditRepository,
        search_service,
    )
}

// --------------------------------------------------------------------------
// CONSULTAS DE CONTRATISTAS
// --------------------------------------------------------------------------

/// [Comando Tauri] Obtiene el perfil detallado de un contratista por ID.
///
/// # Argumentos
/// * `session` - Estado de la sesión para validación de permisos.
/// * `id` - Identificador del contratista.
///
/// # Retorno
/// Perfil completo del contratista o error si no existe.
#[command]
pub async fn get_contratista_by_id(
    session: State<'_, SessionState>,
    id: String,
) -> Result<ContratistaResponse, ContratistaError> {
    require_perm!(session, "contratistas:read")?;
    create_service(None).get_contratista_by_id(&id).await
}

/// [Comando Tauri] Localiza un contratista por su documento de identidad.
///
/// # Argumentos
/// * `cedula` - Número de cédula.
///
/// # Retorno
/// Perfil del contratista si existe.
#[command]
pub async fn get_contratista_by_cedula(
    cedula: String,
) -> Result<ContratistaResponse, ContratistaError> {
    create_service(None).get_contratista_by_cedula(&cedula).await
}

/// [Comando Tauri] Recupera el censo completo de contratistas.
///
/// # Argumentos
/// * `session` - Estado de la sesión.
///
/// # Retorno
/// Lista con estadísticas de activos, PRAIND vencido, etc.
#[command]
pub async fn get_all_contratistas(
    session: State<'_, SessionState>,
) -> Result<ContratistaListResponse, ContratistaError> {
    require_perm!(session, "contratistas:read")?;
    create_service(None).get_all_contratistas().await
}

/// [Comando Tauri] Filtra contratistas con estado Activo.
///
/// # Retorno
/// Vector de contratistas habilitados para laborar.
#[command]
pub async fn get_contratistas_activos() -> Result<Vec<ContratistaResponse>, ContratistaError> {
    create_service(None).get_contratistas_activos().await
}

/// [Comando Tauri] Consulta contratistas archivados (soft-deleted).
///
/// # Retorno
/// Lista de contratistas en archivo histórico.
#[command]
pub async fn get_archived_contratistas() -> Result<Vec<ContratistaResponse>, ContratistaError> {
    create_service(None).get_archived_contratistas().await
}

// --------------------------------------------------------------------------
// OPERACIONES DE GESTIÓN (MUTACIONES)
// --------------------------------------------------------------------------

/// [Comando Tauri] Registra un nuevo contratista en el padrón.
///
/// Dispara validación contra Lista Negra e indexación automática.
///
/// # Argumentos
/// * `session` - Estado de la sesión.
/// * `search_service` - Servicio de indexación.
/// * `input` - Datos del contratista.
///
/// # Retorno
/// El perfil del contratista recién creado.
#[command]
pub async fn create_contratista(
    session: State<'_, SessionState>,
    search_service: State<'_, Arc<SearchService>>,
    input: CreateContratistaInput,
) -> Result<ContratistaResponse, ContratistaError> {
    require_perm!(session, "contratistas:create", "Registrando nuevo perfil de contratista")?;
    create_service(Some(search_service.inner().clone())).create_contratista(input).await
}

/// [Comando Tauri] Actualiza datos de un contratista existente.
///
/// # Argumentos
/// * `session` - Estado de la sesión.
/// * `search_service` - Servicio de indexación.
/// * `id` - ID del contratista.
/// * `input` - Campos a actualizar.
///
/// # Retorno
/// Perfil actualizado.
#[command]
pub async fn update_contratista(
    session: State<'_, SessionState>,
    search_service: State<'_, Arc<SearchService>>,
    id: String,
    input: UpdateContratistaInput,
) -> Result<ContratistaResponse, ContratistaError> {
    require_perm!(
        session,
        "contratistas:update",
        format!("Actualizando información de contratista ID: {}", id)
    )?;
    create_service(Some(search_service.inner().clone())).update_contratista(id, input).await
}

/// [Comando Tauri] Cambia el estado operativo de un contratista.
///
/// # Argumentos
/// * `session` - Estado de la sesión.
/// * `search_service` - Servicio de indexación.
/// * `id` - ID del contratista.
/// * `input` - Nuevo estado.
///
/// # Retorno
/// Perfil con estado actualizado.
#[command]
pub async fn cambiar_estado_contratista(
    session: State<'_, SessionState>,
    search_service: State<'_, Arc<SearchService>>,
    id: String,
    input: CambiarEstadoInput,
) -> Result<ContratistaResponse, ContratistaError> {
    require_perm!(
        session,
        "contratistas:update",
        format!("Cambiando estatus administrativo para el contratista {}", id)
    )?;
    create_service(Some(search_service.inner().clone())).cambiar_estado_contratista(id, input).await
}

/// [Comando Tauri] Archiva un contratista (Soft Delete).
///
/// # Argumentos
/// * `session` - Estado de la sesión.
/// * `search_service` - Servicio de indexación.
/// * `id` - ID del contratista.
///
/// # Retorno
/// Ok(()) si el archivado fue exitoso.
#[command]
pub async fn delete_contratista(
    session: State<'_, SessionState>,
    search_service: State<'_, Arc<SearchService>>,
    id: String,
) -> Result<(), ContratistaError> {
    require_perm!(
        session,
        "contratistas:delete",
        format!("Archivando perfil de contratista {}", id)
    )?;
    create_service(Some(search_service.inner().clone())).delete_contratista(id).await
}

/// [Comando Tauri] Restaura un contratista previamente archivado.
///
/// # Argumentos
/// * `search_service` - Servicio de indexación.
/// * `id` - ID del contratista.
///
/// # Retorno
/// Ok(()) si la restauración fue exitosa.
#[command]
pub async fn restore_contratista(
    session: State<'_, SessionState>,
    search_service: State<'_, Arc<SearchService>>,
    id: String,
) -> Result<(), ContratistaError> {
    require_perm!(
        session,
        "contratistas:delete",
        format!("Restaurando perfil de contratista {}", id)
    )?;
    create_service(Some(search_service.inner().clone())).restore_contratista(id).await
}

// --------------------------------------------------------------------------
// OPERACIONES CON TRAZABILIDAD (AUDITORÍA)
// --------------------------------------------------------------------------

/// [Comando Tauri] Actualiza PRAIND con registro en historial de auditoría.
///
/// # Argumentos
/// * `search_service` - Servicio de indexación.
/// * `input` - Datos de la actualización.
/// * `usuario_id` - ID del usuario que realiza el cambio.
///
/// # Retorno
/// Perfil con fecha PRAIND actualizada.
#[command]
pub async fn actualizar_praind_con_historial(
    session: State<'_, SessionState>,
    search_service: State<'_, Arc<SearchService>>,
    input: ActualizarPraindInput,
) -> Result<ContratistaResponse, ContratistaError> {
    let user =
        session.get_user().ok_or(ContratistaError::Unauthorized("Sesión no válida".to_string()))?;
    create_service(Some(search_service.inner().clone()))
        .actualizar_praind_con_historial(input, user.id.clone())
        .await
}

/// [Comando Tauri] Cambia estado con registro de auditoría.
///
/// # Argumentos
/// * `search_service` - Servicio de indexación.
/// * `input` - Datos del cambio de estado.
/// * `usuario_id` - ID del usuario que realiza el cambio.
///
/// # Retorno
/// Perfil con estado actualizado.
#[command]
pub async fn cambiar_estado_con_historial(
    session: State<'_, SessionState>,
    search_service: State<'_, Arc<SearchService>>,
    input: CambiarEstadoConHistorialInput,
) -> Result<ContratistaResponse, ContratistaError> {
    let user =
        session.get_user().ok_or(ContratistaError::Unauthorized("Sesión no válida".to_string()))?;
    create_service(Some(search_service.inner().clone()))
        .cambiar_estado_con_historial(input, user.id.clone())
        .await
}
