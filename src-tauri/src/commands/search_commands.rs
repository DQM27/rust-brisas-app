// ==========================================
// src/commands/search_commands.rs
// ==========================================
// Comandos Tauri para búsqueda

use crate::domain::errors::SearchError;
use crate::search::searcher::SearchResultDto;
use crate::services::search_service::SearchService;
use crate::services::session::SessionState;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub async fn search_contratistas(
    session: State<'_, SessionState>,
    search_service: State<'_, Arc<SearchService>>,
    query: String,
    limit: Option<usize>,
) -> Result<Vec<SearchResultDto>, SearchError> {
    require_perm!(session, "contratistas:read")?;
    search_service.search(&query, limit.unwrap_or(20)).map_err(SearchError::Engine)
}

#[tauri::command]
pub async fn reindex_all_contratistas(
    session: State<'_, SessionState>,
    search_service: State<'_, Arc<SearchService>>,
) -> Result<(), SearchError> {
    require_perm!(session, "config:update", "Reindexando contratistas")?;
    search_service.reindex_all().await.map_err(SearchError::Engine)
}

#[tauri::command]
pub async fn search_global(
    session: State<'_, SessionState>,
    search_service: State<'_, Arc<SearchService>>,
    query: String,
    limit: Option<usize>,
) -> Result<Vec<SearchResultDto>, SearchError> {
    require_perm!(session, "users:read", "Búsqueda global")?; // Permiso básico
    search_service.search(&query, limit.unwrap_or(20)).map_err(SearchError::Engine)
}
