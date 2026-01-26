// ==========================================
// src/commands/pre_registro_visita_commands.rs
// ==========================================

use crate::db::surrealdb_pre_registro_visita_queries as db;
use crate::domain::errors::PreRegistroError;
use crate::models::ingreso::{CreatePreRegistroInput, PreRegistroEstado, PreRegistroVisitaFetched};
use crate::services::session::SessionState;
use surrealdb::RecordId;
use tauri::State;

/// Parsea un ID de usuario (acepta "user:id" o "id").
fn parse_user_id(id: &str) -> RecordId {
    let clean_id = id
        .trim_start_matches("⟨")
        .trim_end_matches("⟩")
        .trim_start_matches('<')
        .trim_end_matches('>');

    if clean_id.contains(':') {
        let parts: Vec<&str> = clean_id.split(':').collect();
        let key = parts[1]
            .trim_start_matches("⟨")
            .trim_end_matches("⟩")
            .trim_start_matches('<')
            .trim_end_matches('>');
        RecordId::from_table_key(parts[0], key)
    } else {
        RecordId::from_table_key("user", clean_id)
    }
}

use crate::services::search_service::SearchService;
use std::sync::Arc;

#[tauri::command]
pub async fn create_pre_registro_visita(
    session: State<'_, SessionState>,
    search_service: State<'_, Arc<SearchService>>,
    input: CreatePreRegistroInput,
) -> Result<PreRegistroVisitaFetched, PreRegistroError> {
    let user = session
        .get_user()
        .ok_or_else(|| PreRegistroError::Validation("Usuario no autenticado".to_string()))?;

    let user_id = parse_user_id(&user.id);

    crate::services::pre_registro_visita_service::create_pre_registro(
        search_service.inner(),
        input,
        user_id,
    )
    .await
}

#[tauri::command]
pub async fn get_pre_registros_pendientes(
) -> Result<Vec<PreRegistroVisitaFetched>, PreRegistroError> {
    db::find_pending().await.map_err(|e| PreRegistroError::Database(e.to_string()))
}

#[tauri::command]
pub async fn check_pre_registro_by_cedula(
    cedula: String,
) -> Result<Option<PreRegistroVisitaFetched>, PreRegistroError> {
    db::find_by_cedula_pending(&cedula).await.map_err(|e| PreRegistroError::Database(e.to_string()))
}

#[tauri::command]
pub async fn cancel_pre_registro_visita(
    id: RecordId,
) -> Result<PreRegistroVisitaFetched, PreRegistroError> {
    db::update_status(&id, PreRegistroEstado::Cancelado)
        .await
        .map_err(|e| PreRegistroError::Database(e.to_string()))
}
