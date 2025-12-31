use crate::services::surrealdb_service::get_db;
use tauri::command;

#[command]
pub async fn check_unique(
    table: String,
    field: String,
    value: String,
    exclude_id: Option<String>,
) -> Result<bool, String> {
    let db = get_db().await.map_err(|e| e.to_string())?;

    // Sanitize table and field to prevent basic injection (allow only alphanumeric + underscore)
    if !table.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err("Invalid table name".to_string());
    }
    if !field.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err("Invalid field name".to_string());
    }

    // Construct query safely
    // SELECT count() FROM type::table($table) WHERE field = $value AND id != $exclude_id
    let mut query_string =
        format!("SELECT count() FROM type::table($table) WHERE {} = $value", field);

    if exclude_id.is_some() {
        query_string.push_str(" AND id != $exclude_id");
    }

    let mut query = db.query(&query_string).bind(("table", table)).bind(("value", value));

    if let Some(eid) = exclude_id {
        // Ensure exclude_id is proper format or let database handle comparison
        query = query.bind(("exclude_id", eid));
    }

    let mut response = query.await.map_err(|e| e.to_string())?;

    // Parse result: [{ count: 0 }] or similar
    // SurrealDB 'count()' aggregation returns number
    let count: Option<i64> = response.take(0).map_err(|e| e.to_string())?;

    match count {
        Some(c) => Ok(c == 0),
        None => Ok(true), // Should not happen with count(), but safe default
    }
}
