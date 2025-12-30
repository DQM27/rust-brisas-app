// ==========================================
// src/search/connection.rs
// ==========================================

use crate::config::AppConfig;
use crate::search::errors::SearchError;
use crate::services::search_service::SearchService;
use std::sync::Arc;

/// Inicializa el servicio de búsqueda
pub fn init_search_service(_config: &AppConfig) -> Result<Arc<SearchService>, SearchError> {
    let index_path_buf = crate::config::manager::get_search_index_path();

    let index_path = index_path_buf.to_str().ok_or(SearchError::InitializationError(
        "Ruta de índice inválida (no UTF-8)".to_string(),
    ))?;

    let search_service = SearchService::new(index_path)?;

    Ok(Arc::new(search_service))
}
