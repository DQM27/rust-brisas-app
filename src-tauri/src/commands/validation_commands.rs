/// Motor de Validación: Integridad de Bases de Datos y Unicidad.
///
/// Este módulo proporciona comandos genéricos para validar datos antes de su
/// persistencia, asegurando que campos críticos como cédulas o correos electrónicos
/// no estén duplicados en el sistema.
use crate::services::surrealdb_service::get_db;
use tauri::command;

/// Estructura interna para recibir el conteo de registros de `SurrealDB`.
#[derive(serde::Deserialize, Debug)]
struct CountResult {
    count: i64,
}

/// Verifica si un valor es único en una tabla y campo específicos.
///
/// Es altamente versátil: se usa para validar que una cédula no exista ya en
/// contratistas, proveedores o usuarios, soportando la exclusión de un ID
/// para permitir ediciones sin colisiones de unicidad consigo mismo.
#[command]
pub async fn check_unique(
    table: String,
    field: String,
    value: String,
    exclude_id: Option<String>,
) -> Result<bool, String> {
    let db = get_db().await.map_err(|e| e.to_string())?;

    // Sanitización básica de nombres de tabla y campos (Prevención de Inyección)
    if !table.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err("Nombre de tabla inválido".to_string());
    }
    if !field.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err("Nombre de campo inválido".to_string());
    }

    // Construcción segura de la consulta SurrealQL
    let mut query_string =
        format!("SELECT count() FROM type::table($table) WHERE {field} = $value");

    if exclude_id.is_some() {
        query_string.push_str(" AND id != $exclude_id");
    }

    let mut query = db.query(&query_string).bind(("table", table)).bind(("value", value));

    if let Some(eid) = exclude_id {
        query = query.bind(("exclude_id", eid));
    }

    let mut response = query.await.map_err(|e| e.to_string())?;

    // Procesamiento del resultado: [{ count: N }]
    let result: Vec<CountResult> =
        response.take(0).map_err(|e| format!("Error de deserialización: {e}"))?;

    let count = result.first().map_or(0, |r| r.count);

    // Retorna true si no hay duplicados (count == 0)
    Ok(count == 0)
}
