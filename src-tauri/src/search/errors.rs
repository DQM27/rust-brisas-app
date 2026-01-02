//! # Errors: Errores del Módulo Search
//!
//! Define los tipos de errores específicos que pueden ocurrir durante la inicialización,
//! indexación o búsqueda en el motor de Tantivy.

use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize, Clone)]
pub enum SearchError {
    #[error("Error de inicialización: {0}")]
    InitializationError(String),

    #[error("Error de I/O: {0}")]
    IoError(String),

    #[error("Error de Tantivy: {0}")]
    TantivyError(String),

    #[error("Campo no encontrado en schema: {0}")]
    FieldNotFound(String),

    #[error("Error al adquirir lock de escritura")]
    LockError,

    #[error("Error de query: {0}")]
    QueryError(String),

    #[error("Error de base de datos: {0}")]
    DatabaseError(String),
}

impl From<std::io::Error> for SearchError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err.to_string())
    }
}

impl From<tantivy::TantivyError> for SearchError {
    fn from(err: tantivy::TantivyError) -> Self {
        Self::TantivyError(err.to_string())
    }
}

pub type SearchResult<T> = Result<T, SearchError>;
