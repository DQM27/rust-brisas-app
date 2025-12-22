// ==========================================
// src/search/mod.rs
// ==========================================

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
