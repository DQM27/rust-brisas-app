// ==========================================
// src/commands/search_commands.rs
// ==========================================
// Comandos Tauri para búsqueda

use crate::db::DbPool;
use crate::domain::errors::SearchError;
use crate::search::searcher::SearchResultDto;
use crate::services::search_service::SearchState;
use tauri::State;

#[tauri::command]
pub async fn search_contratistas(
    search_state: State<'_, SearchState>,
    query: String,
    limit: Option<usize>,
) -> Result<Vec<SearchResultDto>, SearchError> {
    let limit = limit.unwrap_or(10); // Default: 10 resultados
    let search_service = search_state.0.read().await;
    Ok(search_service.search(&query, limit)?)
}

#[tauri::command]
pub async fn reindex_all_contratistas(
    search_state: State<'_, SearchState>,
    pool_state: State<'_, DbPool>,
) -> Result<(), SearchError> {
    let search_service = search_state.0.read().await;
    let pool = pool_state.0.read().await;
    Ok(search_service.reindex_all_contratistas(&pool).await?)
}

#[tauri::command]
pub async fn search_global(
    search_state: State<'_, SearchState>,
    query: String,
    limit: Option<usize>,
) -> Result<Vec<SearchResultDto>, SearchError> {
    let limit = limit.unwrap_or(20);
    let search_service = search_state.0.read().await;
    Ok(search_service.search(&query, limit)?)
}
