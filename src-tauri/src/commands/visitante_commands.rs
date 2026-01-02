//! # Comandos: Gestión de Visitantes (Tauri Bridge)
//!
//! Este módulo expone las capacidades de control de visitantes al frontend,
//! regulando el acceso mediante permisos (RBAC) y garantizando la auditoría
//! de registros de seguridad.

use crate::domain::errors::VisitanteError;
use crate::models::visitante::{CreateVisitanteInput, VisitanteResponse};
use crate::require_perm;
use crate::services::session::SessionState;
use crate::services::visitante_service as service;
use tauri::{command, State};

/// Registra los datos básicos de un nuevo visitante en la base de datos de seguridad.
#[command]
pub async fn create_visitante(
    session: State<'_, SessionState>,
    input: CreateVisitanteInput,
) -> Result<VisitanteResponse, VisitanteError> {
    require_perm!(session, "visitantes:create", "Registrando nuevo perfil de visitante")?;
    service::create_visitante(input).await
}

/// Motor de Búsqueda: Localiza visitantes recurrentes para agilizar su re-ingreso.
#[command]
pub async fn search_visitantes_catalog(
    session: State<'_, SessionState>,
    query: String,
) -> Result<Vec<VisitanteResponse>, VisitanteError> {
    require_perm!(session, "visitantes:read")?;
    service::search_visitantes(&query).await
}

/// Identificación Unívoca: Recupera la ficha técnica del visitante mediante su documento.
#[command]
pub async fn get_visitante_by_cedula(
    session: State<'_, SessionState>,
    cedula: String,
) -> Result<Option<VisitanteResponse>, VisitanteError> {
    require_perm!(session, "visitantes:read")?;
    service::get_visitante_by_cedula(&cedula).await
}

/// Actualiza la información personal o motivos recurrentes de un visitante.
#[command]
pub async fn update_visitante(
    session: State<'_, SessionState>,
    id: String,
    input: CreateVisitanteInput,
) -> Result<VisitanteResponse, VisitanteError> {
    require_perm!(
        session,
        "visitantes:update",
        format!("Actualizando información de visitante ID: {}", id)
    )?;
    service::update_visitante(&id, input).await
}

/// Baja Administrativa: Archiva el perfil del visitante del catálogo operativo.
#[command]
pub async fn delete_visitante(
    session: State<'_, SessionState>,
    id: String,
) -> Result<(), VisitanteError> {
    require_perm!(session, "visitantes:delete", format!("Archivando perfil de visitante {}", id))?;
    service::delete_visitante(&id).await
}

/// Restablecimiento: Recupera un perfil de visitante anteriormente archivado.
#[command]
pub async fn restore_visitante(
    session: State<'_, SessionState>,
    id: String,
) -> Result<VisitanteResponse, VisitanteError> {
    require_perm!(session, "visitantes:delete", format!("Restaurando perfil de visitante {}", id))?;
    service::restore_visitante(&id).await
}

/// Consulta histórica de visitantes dados de baja administrativa.
#[command]
pub async fn get_archived_visitantes(
    session: State<'_, SessionState>,
) -> Result<Vec<VisitanteResponse>, VisitanteError> {
    require_perm!(session, "visitantes:read")?;
    service::get_archived_visitantes().await
}

/// Reporte Operativo: Lista la totalidad de visitantes registrados en el sistema.
#[command]
pub async fn list_visitantes(
    session: State<'_, SessionState>,
) -> Result<Vec<VisitanteResponse>, VisitanteError> {
    require_perm!(session, "visitantes:read")?;
    service::get_all_visitantes().await
}
