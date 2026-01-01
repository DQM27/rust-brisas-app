/// Puertos de Entrada: Gestión Integral de Personal Externo (Contractor Bridge).
///
/// Este módulo es central para la administración de la fuerza laboral externa.
/// Orquesta el registro, validación de certificaciones (PRAIND) y auditoría
/// de contratistas, asegurando que el frontend tenga acceso a perfiles actualizados.
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

/// Registra un nuevo contratista en el padrón de la empresa.
/// Dispara la indexación automática en el motor de búsqueda.
#[command]
pub async fn create_contratista(
    session: State<'_, SessionState>,
    search_service: State<'_, Arc<SearchService>>,
    input: CreateContratistaInput,
) -> Result<ContratistaResponse, ContratistaError> {
    require_perm!(session, "contratistas:create", "Registrando nuevo perfil de contratista")?;
    contratista_service::create_contratista(&search_service, input).await
}

#[command]
pub async fn get_contratista_by_id(
    session: State<'_, SessionState>,
    id: String,
) -> Result<ContratistaResponse, ContratistaError> {
    require_perm!(session, "contratistas:read")?;
    contratista_service::get_contratista_by_id(&id).await
}

/// Localiza un perfil de manera unívoca mediante su documento de identidad.
#[command]
pub async fn get_contratista_by_cedula(
    cedula: String,
) -> Result<ContratistaResponse, ContratistaError> {
    contratista_service::get_contratista_by_cedula(&cedula).await
}

/// Recupera el censo completo de contratistas para vistas administrativas.
#[command]
pub async fn get_all_contratistas(
    session: State<'_, SessionState>,
) -> Result<ContratistaListResponse, ContratistaError> {
    require_perm!(session, "contratistas:read")?;
    contratista_service::get_all_contratistas().await
}

/// Filtra exclusivamente al personal que se encuentra actualmente habilitado para laborar.
#[command]
pub async fn get_contratistas_activos() -> Result<Vec<ContratistaResponse>, ContratistaError> {
    contratista_service::get_contratistas_activos().await
}

/// Actualiza los datos generales (Contacto, Cargo, etc.) de un contratista existente.
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

/// Permite el cambio de estado operativo (Ej: Activo a Inactivo) con validación de permisos.
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

/// Baja administrativa: Envía el perfil al archivo histórico (Soft Delete).
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

// ==========================================
// OPERACIONES CON TRAZABILIDAD (AUDITORÍA)
// ==========================================

/// Control de Certificación: Actualiza la vigencia del PRAIND manteniendo un historial de cambios.
#[command]
pub async fn actualizar_praind_con_historial(
    search_service: State<'_, Arc<SearchService>>,
    input: contratista_service::ActualizarPraindInput,
    usuario_id: String,
) -> Result<ContratistaResponse, ContratistaError> {
    contratista_service::actualizar_praind_con_historial(&search_service, input, usuario_id).await
}

#[command]
pub async fn cambiar_estado_con_historial(
    search_service: State<'_, Arc<SearchService>>,
    input: contratista_service::CambiarEstadoConHistorialInput,
    usuario_id: String,
) -> Result<ContratistaResponse, ContratistaError> {
    contratista_service::cambiar_estado_con_historial(&search_service, input, usuario_id).await
}

/// Proceso de Recuperación: Restablece un perfil que fue previamente archivado.
#[command]
pub async fn restore_contratista(
    search_service: State<'_, Arc<SearchService>>,
    id: String,
) -> Result<(), ContratistaError> {
    contratista_service::restore_contratista(&search_service, id).await
}

/// Consulta el almacén histórico de contratistas que no están activos ni en planta.
#[command]
pub async fn get_archived_contratistas() -> Result<Vec<ContratistaResponse>, ContratistaError> {
    contratista_service::get_archived_contratistas().await
}
