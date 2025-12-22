// ==========================================
// src/commands/search_commands.rs
// ==========================================
// Comandos Tauri para búsqueda

use crate::search::searcher::SearchResultDto;
use crate::services::search_service::SearchService;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub async fn search_contratistas(
    search_service: State<'_, Arc<SearchService>>,
    query: String,
    limit: Option<usize>,
) -> Result<Vec<SearchResultDto>, String> {
    let limit = limit.unwrap_or(10); // Default: 10 resultados
    search_service
        .search(&query, limit)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn reindex_all_contratistas(
    search_service: State<'_, Arc<SearchService>>,
    pool: State<'_, sqlx::SqlitePool>,
) -> Result<(), String> {
    search_service
        .reindex_all_contratistas(&pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_global(
    search_service: State<'_, Arc<SearchService>>,
    query: String,
    limit: Option<usize>,
) -> Result<Vec<SearchResultDto>, String> {
    let limit = limit.unwrap_or(20);
    search_service
        .search(&query, limit)
        .map_err(|e| e.to_string())
}
