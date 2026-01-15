// src/db/surrealdb_utils.rs
//
// Utility functions for SurrealDB operations
// Reduces boilerplate across query modules

/// Strips table prefix from ID if present
/// Example: "contratista:abc123" -> "abc123"
pub fn strip_table_prefix<'a>(id: &'a str, table: &str) -> &'a str {
    let prefix = format!("{table}:");
    id.strip_prefix(&prefix).unwrap_or(id)
}

/// Creates a proper `SurrealDB` record ID string
/// Example: ("contratista", "abc123") -> "contratista:abc123"
pub fn make_record_id(table: &str, id: &str) -> String {
    let id_only = strip_table_prefix(id, table);
    format!("{table}:{id_only}")
}

/// Normalizes an ID, ensuring it doesn't have duplicate prefixes
pub fn normalize_id(id: &str, table: &str) -> String {
    strip_table_prefix(id, table).to_string()
}

