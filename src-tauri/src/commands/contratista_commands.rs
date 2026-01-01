/// Puertos de Entrada: Gestión Integral de Personal Externo (Contractor Bridge).
///
/// Este módulo es central para la administración de la fuerza laboral externa.
/// Orquesta el registro, validación de certificaciones (PRAIND) y auditoría
/// de contratistas.
use crate::domain::errors::ContratistaError;
use crate::models::contratista::{
    CambiarEstadoInput, ContratistaListResponse, ContratistaResponse, CreateContratistaInput,
    UpdateContratistaInput,
};
use crate::services::contratista_service;
use crate::services::search_service::SearchService;
use crate::services::session::SessionState;
use std::sync::Arc;
use tauri::{command, State};

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
    contratista_service::get_contratista_by_id(&id).await
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
    contratista_service::get_contratista_by_cedula(&cedula).await
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
    contratista_service::get_all_contratistas().await
}

/// [Comando Tauri] Filtra contratistas con estado Activo.
///
/// # Retorno
/// Vector de contratistas habilitados para laborar.
#[command]
pub async fn get_contratistas_activos() -> Result<Vec<ContratistaResponse>, ContratistaError> {
    contratista_service::get_contratistas_activos().await
}

/// [Comando Tauri] Consulta contratistas archivados (soft-deleted).
///
/// # Retorno
/// Lista de contratistas en archivo histórico.
#[command]
pub async fn get_archived_contratistas() -> Result<Vec<ContratistaResponse>, ContratistaError> {
    contratista_service::get_archived_contratistas().await
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
    contratista_service::create_contratista(&search_service, input).await
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
    contratista_service::update_contratista(&search_service, id, input).await
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
    contratista_service::cambiar_estado_contratista(&search_service, id, input).await
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
    contratista_service::delete_contratista(&search_service, id).await
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
    search_service: State<'_, Arc<SearchService>>,
    id: String,
) -> Result<(), ContratistaError> {
    contratista_service::restore_contratista(&search_service, id).await
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
    search_service: State<'_, Arc<SearchService>>,
    input: contratista_service::ActualizarPraindInput,
    usuario_id: String,
) -> Result<ContratistaResponse, ContratistaError> {
    contratista_service::actualizar_praind_con_historial(&search_service, input, usuario_id).await
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
    search_service: State<'_, Arc<SearchService>>,
    input: contratista_service::CambiarEstadoConHistorialInput,
    usuario_id: String,
) -> Result<ContratistaResponse, ContratistaError> {
    contratista_service::cambiar_estado_con_historial(&search_service, input, usuario_id).await
}
