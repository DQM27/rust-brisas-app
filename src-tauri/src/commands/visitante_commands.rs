// ==========================================
// src/commands/visitante_commands.rs
// ==========================================

use crate::domain::errors::VisitanteError;
use crate::models::visitante::{CreateVisitanteInput, VisitanteResponse};
use crate::services::session::SessionState;
use crate::services::visitante_service;
use tauri::{command, State};

#[command]
pub async fn create_visitante(
    session: State<'_, SessionState>,
    input: CreateVisitanteInput,
) -> Result<VisitanteResponse, VisitanteError> {
    require_perm!(session, "visitantes:create", "Creando visitante")?;
    visitante_service::create_visitante(input).await
}

#[command]
pub async fn search_visitantes_catalog(
    session: State<'_, SessionState>,
    query: String,
) -> Result<Vec<VisitanteResponse>, VisitanteError> {
    require_perm!(session, "visitantes:read")?;
    visitante_service::search_visitantes(&query).await
}

#[command]
pub async fn get_visitante_by_cedula(cedula: String) -> Result<Option<VisitanteResponse>, String> {
    visitante_service::get_visitante_by_cedula(&cedula).await.map_err(|e| e.to_string())
}

#[command]
pub async fn update_visitante(
    session: State<'_, SessionState>,
    id: String,
    input: CreateVisitanteInput,
) -> Result<VisitanteResponse, VisitanteError> {
    require_perm!(session, "visitantes:update", format!("Actualizando visitante {}", id))?;
    visitante_service::update_visitante(&id, input).await
}

#[command]
pub async fn delete_visitante(
    session: State<'_, SessionState>,
    id: String,
) -> Result<(), VisitanteError> {
    require_perm!(session, "visitantes:delete", format!("Eliminando visitante {}", id))?;
    visitante_service::delete_visitante(&id).await
}

#[command]
pub async fn restore_visitante(id: String) -> Result<VisitanteResponse, String> {
    visitante_service::restore_visitante(&id).await.map_err(|e| e.to_string())
}

#[command]
pub async fn get_archived_visitantes() -> Result<Vec<VisitanteResponse>, String> {
    visitante_service::get_archived_visitantes().await.map_err(|e| e.to_string())
}

#[command]
pub async fn list_visitantes(
    session: State<'_, SessionState>,
) -> Result<Vec<VisitanteResponse>, VisitanteError> {
    require_perm!(session, "visitantes:read")?;
    visitante_service::get_all_visitantes().await
}
