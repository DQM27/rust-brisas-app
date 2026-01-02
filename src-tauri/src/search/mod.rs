//! # Módulo Search: Motor de Búsqueda Basado en Tantivy
//!
//! Este módulo centraliza toda la lógica de búsqueda full-text de la aplicación.
//! Utiliza Tantivy para indexar y buscar sobre múltiples entidades de forma rápida.
//!
//! ## Submódulos
//! - `indexer`: Lógica de escritura e indexación.
//! - `searcher`: Lógica de consulta y recuperación.
//! - `schema`: Definición de la estructura de datos del índice.
//! - `errors`: Manejo de errores específicos.
//! - `connection`: Gestión del estado global y persistencia del índice.

pub mod connection;
pub mod errors;
pub mod indexer;
pub mod schema;
pub mod searcher;

pub use connection::init_search_service;
pub use errors::SearchError;

// Re-exports para facilitar imports
pub use indexer::*;
pub use schema::*;
pub use searcher::*;
