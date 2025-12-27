// ==========================================
// src/commands/search_commands.rs
// ==========================================
// Comandos Tauri para búsqueda

use crate::domain::errors::SearchError;
use crate::search::searcher::SearchResultDto;
use crate::services::search_service::SearchService;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub async fn search_contratistas(
    search_service: State<'_, Arc<SearchService>>,
    query: String,
    limit: Option<usize>,
) -> Result<Vec<SearchResultDto>, SearchError> {
    search_service.search(&query, limit.unwrap_or(20))
}

#[tauri::command]
pub async fn reindex_all_contratistas(
    search_service: State<'_, Arc<SearchService>>,
) -> Result<(), SearchError> {
    search_service.reindex_all().await
}

#[tauri::command]
pub async fn search_global(
    search_service: State<'_, Arc<SearchService>>,
    query: String,
    limit: Option<usize>,
) -> Result<Vec<SearchResultDto>, SearchError> {
    search_service.search(&query, limit.unwrap_or(20))
}
