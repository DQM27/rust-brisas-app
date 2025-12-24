// ==========================================
// src/search/connection.rs
// ==========================================

use crate::config::AppConfig;
use crate::search::errors::SearchError;
use crate::services::search_service::SearchService;
use std::sync::Arc;

/// Inicializa el servicio de búsqueda usando la misma ubicación que la DB
/// Inicializa el servicio de búsqueda usando la ubicación correcta (Prod o Demo)
pub fn init_search_service(config: &AppConfig) -> Result<Arc<SearchService>, SearchError> {
    let index_path_buf = if config.setup.show_demo_mode {
        crate::config::manager::get_demo_search_path()
    } else {
        crate::config::manager::get_search_index_path()
    };

    let index_path = index_path_buf.to_str().ok_or(SearchError::InitializationError(
        "Ruta de índice inválida (no UTF-8)".to_string(),
    ))?;

    let search_service = SearchService::new(index_path)?;

    Ok(Arc::new(search_service))
}
