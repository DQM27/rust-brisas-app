// ==========================================
// src/db/surrealdb_ingreso_visita_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::models::ingreso::{
    IngresoUpdateDTO, IngresoVisita, IngresoVisitaCreateDTO, IngresoVisitaFetched,
};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use surrealdb::RecordId;

const TABLE: &str = "ingreso_visita";

pub async fn insert(dto: IngresoVisitaCreateDTO) -> Result<IngresoVisitaFetched, SurrealDbError> {
    let db = get_db().await?;

    let created: Option<IngresoVisita> =
        db.query(format!("CREATE {TABLE} CONTENT $dto")).bind(("dto", dto)).await?.take(0)?;

    let ingreso = created.ok_or(SurrealDbError::TransactionError(
        "Error al insertar ingreso de visita".to_string(),
    ))?;

    // Fetch relations
    let mut result = db
        .query("SELECT * FROM $id FETCH usuario_ingreso, usuario_salida")
        .bind(("id", ingreso.id.clone()))
        .await?;

    let fetched: Option<IngresoVisitaFetched> = result.take(0)?;
    fetched.ok_or(SurrealDbError::TransactionError(
        "Ingreso creado pero no se pudo obtener con FETCH".to_string(),
    ))
}

pub async fn find_ingreso_abierto_by_cedula(
    cedula: &str,
) -> Result<Option<IngresoVisitaFetched>, SurrealDbError> {
    let db = get_db().await?;

    let mut result = db
        .query(format!(
            "SELECT * FROM {TABLE} WHERE cedula = $cedula AND fecha_hora_salida IS NONE LIMIT 1 FETCH usuario_ingreso, usuario_salida"
        ))
        .bind(("cedula", cedula.to_string()))
        .await?;

    Ok(result.take(0)?)
}

pub async fn update_salida(
    ingreso_id: &RecordId,
    usuario_salida_id: &RecordId,
    observaciones: Option<String>,
) -> Result<IngresoVisitaFetched, SurrealDbError> {
    let db = get_db().await?;

    let mut dto = IngresoUpdateDTO::default();
    dto.fecha_hora_salida = Some(surrealdb::Datetime::from(chrono::Utc::now()));
    dto.usuario_salida = Some(usuario_salida_id.clone());
    dto.observaciones = observaciones;

    let _: Option<IngresoVisita> = db
        .query("UPDATE $id MERGE $dto")
        .bind(("id", ingreso_id.clone()))
        .bind(("dto", dto))
        .await?
        .take(0)?;

    let mut result = db
        .query("SELECT * FROM $id FETCH usuario_ingreso, usuario_salida")
        .bind(("id", ingreso_id.clone()))
        .await?;

    let fetched: Option<IngresoVisitaFetched> = result.take(0)?;
    fetched
        .ok_or(SurrealDbError::TransactionError("Error al registrar salida de visita".to_string()))
}
