/// Puertos de Entrada: Búsqueda Inteligente de Alto Rendimiento (Search Bridge).
///
/// Este módulo expone las capacidades del motor de búsqueda (Tantivy). Permite
/// localizar perfiles de manera instantánea entre miles de registros, superando
/// las limitaciones de velocidad de las consultas directas a base de datos.
use crate::domain::errors::SearchError;
use crate::search::searcher::SearchResultDto;
use crate::services::search_service::SearchService;
use crate::services::session::SessionState;
use std::sync::Arc;
use tauri::State;

/// Búsqueda Inteligente: Realiza consultas sobre el índice global (Usuarios, Contratistas, etc.).
#[tauri::command]
pub async fn search_omnibox(
    session: State<'_, SessionState>,
    search_service: State<'_, Arc<SearchService>>,
    query: String,
    limit: Option<usize>,
) -> Result<Vec<SearchResultDto>, SearchError> {
    // Permiso básico para buscar. Se asume que si puede ver contratistas o usuarios, puede buscar.
    // dealmente debería ser un permiso 'search:read', pero por compatibilidad usamos 'contratistas:read' OR 'users:read'.
    // Por ahora mantenemos 'contratistas:read' como base para el feature principal.
    require_perm!(session, "contratistas:read")?;
    search_service.search(&query, limit.unwrap_or(20)).map_err(SearchError::Engine)
}

/// Sincronización Manual: Forza la reconstrucción del índice de búsqueda global desde `SurrealDB`.
#[tauri::command]
pub async fn reindex_global_search(
    session: State<'_, SessionState>,
    search_service: State<'_, Arc<SearchService>>,
) -> Result<(), SearchError> {
    require_perm!(session, "config:update", "Sincronizando índices de búsqueda global")?;
    search_service.reindex_all().await.map_err(SearchError::Engine)
}

/// Búsqueda Unificada: Consulta global en múltiples entidades (Usuarios, Contratistas, etc.).
#[tauri::command]
pub async fn search_global(
    session: State<'_, SessionState>,
    search_service: State<'_, Arc<SearchService>>,
    query: String,
    limit: Option<usize>,
) -> Result<Vec<SearchResultDto>, SearchError> {
    require_perm!(session, "users:read", "Ejecutando búsqueda global en repositorios")?;
    search_service.search(&query, limit.unwrap_or(20)).map_err(SearchError::Engine)
}
