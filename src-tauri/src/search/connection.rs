// ==========================================
// src/search/connection.rs
// ==========================================

use crate::config::AppConfig;
use crate::search::errors::SearchError;
use crate::services::search_service::SearchService;
use std::sync::Arc;

/// Inicializa el servicio de búsqueda usando la misma ubicación que la DB
pub fn init_search_service(config: &AppConfig) -> Result<Arc<SearchService>, SearchError> {
    let db_path = crate::config::manager::get_database_path(config);
    let data_dir = db_path
        .parent()
        .ok_or(SearchError::InitializationError(
            "No se pudo obtener directorio de datos".to_string(),
        ))?
        .to_str()
        .ok_or(SearchError::InitializationError("Ruta inválida".to_string()))?;

    let search_service = SearchService::new(data_dir)?;

    Ok(Arc::new(search_service))
}
