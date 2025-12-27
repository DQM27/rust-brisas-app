// ==========================================
// src/commands/search_commands.rs
// ==========================================
// Comandos Tauri para búsqueda

use crate::domain::errors::SearchError;
use crate::search::searcher::SearchResultDto;

#[tauri::command]
pub async fn search_contratistas(
    _query: String,
    _limit: Option<usize>,
) -> Result<Vec<SearchResultDto>, SearchError> {
    Ok(vec![])
}

#[tauri::command]
pub async fn reindex_all_contratistas() -> Result<(), SearchError> {
    Err(SearchError::Index("No implementado para SurrealDB aún".to_string()))
}

#[tauri::command]
pub async fn search_global(
    _query: String,
    _limit: Option<usize>,
) -> Result<Vec<SearchResultDto>, SearchError> {
    Ok(vec![])
}
