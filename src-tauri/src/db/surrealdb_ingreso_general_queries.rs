// ==========================================
// src/db/surrealdb_ingreso_general_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::models::ingreso::UniversalIngresoFetched;
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use surrealdb::RecordId;

// NOTE: Now unifies [ingreso_contratista, ingreso_proveedor, ingreso_visita]

const TABLES: &str = "[ingreso_contratista, ingreso_proveedor, ingreso_visita]";
const FETCH_ALL: &str = "FETCH usuario_ingreso, usuario_salida, contratista, contratista.empresa, proveedor, proveedor.empresa";

pub async fn find_all() -> Result<Vec<serde_json::Value>, SurrealDbError> {
    let db = get_db().await?;
    let mut result =
        db.query(format!("SELECT * FROM {TABLES} ORDER BY created_at DESC LIMIT 500")).await?;
    Ok(result.take(0)?)
}

pub async fn find_all_fetched() -> Result<Vec<UniversalIngresoFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query(format!("SELECT *, type::table(id) AS tipo_ingreso FROM {TABLES} ORDER BY created_at DESC LIMIT 500 {FETCH_ALL}"))
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_ingresos_abiertos_fetched() -> Result<Vec<UniversalIngresoFetched>, SurrealDbError>
{
    let db = get_db().await?;
    let mut result = db
        .query(format!(
            "SELECT *, type::table(id) AS tipo_ingreso FROM {TABLES} WHERE fecha_hora_salida IS NONE ORDER BY created_at DESC {FETCH_ALL}"
        ))
        .await?;

    // LOGGING DEBUG
    // We cannot peek into result without consuming it easily, so we rely on the specific error context
    // or we fetch as Value separately for debugging (expensive but safe).
    // Better approach: Let's catch the error if deserialization fails?
    // Actually, SurrealDB driver errors on .take() if schema mismatch.

    // For specific debugging, let's verify data structure by adding a raw query.
    #[cfg(debug_assertions)]
    {
        let raw: Vec<serde_json::Value> = db.query(format!("SELECT *, type::table(id) AS tipo_ingreso FROM {TABLES} WHERE fecha_hora_salida IS NONE ORDER BY created_at DESC {FETCH_ALL}"))
            .await?
            .take(0)?;
        println!(
            "[DEBUG] Raw Ingresos Fetch: {}",
            serde_json::to_string_pretty(&raw).unwrap_or_default()
        );
    }

    Ok(result.take(0)?)
}

pub async fn find_by_id_fetched(
    id: &RecordId,
) -> Result<Option<UniversalIngresoFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query(format!("SELECT *, type::table(id) AS tipo_ingreso FROM $id {FETCH_ALL}"))
        .bind(("id", id.clone()))
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_ingreso_by_gafete_fetched(
    gafete: &str,
) -> Result<Option<UniversalIngresoFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query(format!(
            "SELECT *, type::table(id) AS tipo_ingreso FROM {TABLES} WHERE gafete_numero = $gafete AND fecha_hora_salida IS NONE LIMIT 1 {FETCH_ALL}"
        ))
        .bind(("gafete", gafete.to_string()))
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_salidas_in_range_fetched(
    start: &str,
    end: &str,
) -> Result<Vec<UniversalIngresoFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query(format!(
            "SELECT *, type::table(id) AS tipo_ingreso FROM {TABLES} WHERE fecha_hora_salida >= type::datetime($start) AND fecha_hora_salida <= type::datetime($end) ORDER BY fecha_hora_salida DESC {FETCH_ALL}"
        ))
        .bind(("start", start.to_string()))
        .bind(("end", end.to_string()))
        .await?;

    Ok(result.take(0)?)
}
