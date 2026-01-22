// ==========================================
// src/db/surrealdb_pre_registro_visita_queries.rs
// ==========================================

use crate::models::ingreso::{
    PreRegistroEstado, PreRegistroVisita, PreRegistroVisitaCreateDTO, PreRegistroVisitaFetched,
};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use surrealdb::RecordId;

const TABLE: &str = "pre_registro_visita";

pub async fn create(
    dto: PreRegistroVisitaCreateDTO,
) -> Result<PreRegistroVisitaFetched, SurrealDbError> {
    let db = get_db().await?;

    let created: Option<PreRegistroVisita> =
        db.query(format!("CREATE {TABLE} CONTENT $dto")).bind(("dto", dto)).await?.take(0)?;

    let registro = created.ok_or(SurrealDbError::TransactionError(
        "Error al crear pre-registro de visita".to_string(),
    ))?;

    // Fetch relations
    let mut result = db
        .query("SELECT * FROM $id FETCH registrado_por")
        .bind(("id", registro.id.clone()))
        .await?;

    let fetched: Option<PreRegistroVisitaFetched> = result.take(0)?;
    fetched.ok_or(SurrealDbError::TransactionError(
        "Pre-registro creado pero no se pudo obtener con FETCH".to_string(),
    ))
}

pub async fn find_pending() -> Result<Vec<PreRegistroVisitaFetched>, SurrealDbError> {
    let db = get_db().await?;
    // Filtramos solo los que están en estado PENDIENTE
    // Podríamos añadir filtro de fecha >= hoy si queremos ocultar los muy viejos (aunque hay job de expiración)
    let mut result = db
        .query(format!(
            "SELECT * FROM {TABLE} WHERE estado = $estado ORDER BY fecha_esperada ASC, created_at ASC FETCH registrado_por"
        ))
        .bind(("estado", PreRegistroEstado::Pendiente.to_string()))
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_by_cedula_pending(
    cedula: &str,
) -> Result<Option<PreRegistroVisitaFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query(format!(
            "SELECT * FROM {TABLE} WHERE cedula = $cedula AND estado = $estado LIMIT 1 FETCH registrado_por"
        ))
        .bind(("cedula", cedula.to_string()))
        .bind(("estado", PreRegistroEstado::Pendiente.to_string()))
        .await?;
    Ok(result.take(0)?)
}

pub async fn update_status(
    id: &RecordId,
    new_status: PreRegistroEstado,
) -> Result<PreRegistroVisitaFetched, SurrealDbError> {
    let db = get_db().await?;

    // Solo actualizamos el estado
    let _: Option<PreRegistroVisita> = db
        .query("UPDATE $id SET estado = $status")
        .bind(("id", id.clone()))
        .bind(("status", new_status.to_string()))
        .await?
        .take(0)?;

    let mut result =
        db.query("SELECT * FROM $id FETCH registrado_por").bind(("id", id.clone())).await?;

    let fetched: Option<PreRegistroVisitaFetched> = result.take(0)?;
    fetched.ok_or(SurrealDbError::TransactionError(
        "Error al actualizar estado del pre-registro".to_string(),
    ))
}

// Check para saber si una cédula existe en pre-registro y obtener sus datos para llenado rápido
pub async fn check_pending_by_cedula(
    cedula: &str,
) -> Result<Option<PreRegistroVisita>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query(format!("SELECT * FROM {TABLE} WHERE cedula = $cedula AND estado = $estado LIMIT 1"))
        .bind(("cedula", cedula.to_string()))
        .bind(("estado", PreRegistroEstado::Pendiente.to_string()))
        .await?;
    Ok(result.take(0)?)
}
