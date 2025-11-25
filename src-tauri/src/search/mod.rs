// ==========================================
// src/search/mod.rs
// ==========================================

pub mod schema;
pub mod indexer;
pub mod searcher;
pub mod connection;

// Re-exports para facilitar imports
pub use schema::*;
pub use indexer::*;
pub use searcher::*;
pub use connection::*;