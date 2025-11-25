// ==========================================
// src/commands/search_commands.rs
// ==========================================
// Comandos Tauri para búsqueda

use crate::services::search_service::SearchService;
use crate::search::SearchResult;
use tauri::State;
use std::sync::Arc;

#[tauri::command]
pub async fn search_contratistas(
    search_service: State<'_, Arc<SearchService>>,
    query: String,
    limit: Option<usize>,
) -> Result<Vec<SearchResult>, String> {
    let limit = limit.unwrap_or(10); // Default: 10 resultados
    search_service.search(&query, limit)
}

#[tauri::command]
pub async fn reindex_all_contratistas(
    search_service: State<'_, Arc<SearchService>>,
    pool: State<'_, sqlx::SqlitePool>,
) -> Result<(), String> {
    search_service.reindex_all_contratistas(&pool).await
}