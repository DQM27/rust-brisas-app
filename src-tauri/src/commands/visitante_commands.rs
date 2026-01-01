/// Puertos de Entrada: Gestión de Visitantes Ocasionales y Técnicos (Visitor Bridge).
///
/// Este módulo gestiona los perfiles de personas que no tienen una relación
/// contractual permanente con la empresa, pero requieren acceso para gestiones
/// puntuales, visitas personales o servicios técnicos.
use crate::domain::errors::VisitanteError;
use crate::models::visitante::{CreateVisitanteInput, VisitanteResponse};
use crate::services::session::SessionState;
use crate::services::visitante_service;
use tauri::{command, State};

/// Registra los datos básicos de un nuevo visitante en la base de datos de seguridad.
#[command]
pub async fn create_visitante(
    session: State<'_, SessionState>,
    input: CreateVisitanteInput,
) -> Result<VisitanteResponse, VisitanteError> {
    require_perm!(session, "visitantes:create", "Registrando nuevo perfil de visitante")?;
    visitante_service::create_visitante(input).await
}

/// Motor de Búsqueda: Localiza visitantes recurrentes para agilizar su re-ingreso.
#[command]
pub async fn search_visitantes_catalog(
    session: State<'_, SessionState>,
    query: String,
) -> Result<Vec<VisitanteResponse>, VisitanteError> {
    require_perm!(session, "visitantes:read")?;
    visitante_service::search_visitantes(&query).await
}

/// Identificación Unívoca: Recupera la ficha técnica del visitante mediante su documento.
#[command]
pub async fn get_visitante_by_cedula(cedula: String) -> Result<Option<VisitanteResponse>, String> {
    visitante_service::get_visitante_by_cedula(&cedula).await.map_err(|e| e.to_string())
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
    visitante_service::update_visitante(&id, input).await
}

/// Baja Administrativa: Archiva el perfil del visitante del catálogo operativo.
#[command]
pub async fn delete_visitante(
    session: State<'_, SessionState>,
    id: String,
) -> Result<(), VisitanteError> {
    require_perm!(session, "visitantes:delete", format!("Archivando perfil de visitante {}", id))?;
    visitante_service::delete_visitante(&id).await
}

/// Restablecimiento: Recupera un perfil de visitante anteriormente archivado.
#[command]
pub async fn restore_visitante(id: String) -> Result<VisitanteResponse, String> {
    visitante_service::restore_visitante(&id).await.map_err(|e| e.to_string())
}

/// Consulta histórica de visitantes dados de baja administrativa.
#[command]
pub async fn get_archived_visitantes() -> Result<Vec<VisitanteResponse>, String> {
    visitante_service::get_archived_visitantes().await.map_err(|e| e.to_string())
}

/// Reporte Operativo: Lista la totalidad de visitantes registrados en el sistema.
#[command]
pub async fn list_visitantes(
    session: State<'_, SessionState>,
) -> Result<Vec<VisitanteResponse>, VisitanteError> {
    require_perm!(session, "visitantes:read")?;
    visitante_service::get_all_visitantes().await
}
