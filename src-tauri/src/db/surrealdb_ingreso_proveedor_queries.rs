// ==========================================
// src/db/surrealdb_ingreso_proveedor_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::models::ingreso::{Ingreso, IngresoCreateDTO, IngresoFetched, IngresoUpdateDTO};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use surrealdb::RecordId;

pub async fn insert(dto: IngresoCreateDTO) -> Result<IngresoFetched, SurrealDbError> {
    let db = get_db().await?;

    // CREATE doesn't support FETCH, so we need two queries
    let created: Option<Ingreso> =
        db.query("CREATE ingreso CONTENT $dto").bind(("dto", dto)).await?.take(0)?;

    let ingreso = created.ok_or(SurrealDbError::TransactionError(
        "Error al insertar ingreso de proveedor".to_string(),
    ))?;

    // Fetch with all relations
    let mut result = db
        .query("SELECT * FROM $id FETCH usuario_ingreso, usuario_salida, vehiculo, contratista, contratista.empresa")
        .bind(("id", ingreso.id.clone()))
        .await?;

    let fetched: Option<IngresoFetched> = result.take(0)?;
    fetched.ok_or(SurrealDbError::TransactionError(
        "Ingreso creado pero no se pudo obtener con FETCH".to_string(),
    ))
}

pub async fn find_ingreso_abierto_by_cedula(
    cedula: &str,
) -> Result<Option<IngresoFetched>, SurrealDbError> {
    let db = get_db().await?;

    let mut result = db
        .query(
            r#"
            SELECT * FROM ingreso 
            WHERE cedula = $cedula 
            AND tipo_ingreso = 'proveedor'
            AND fecha_hora_salida IS NONE
            FETCH usuario_ingreso, usuario_salida, vehiculo, contratista, contratista.empresa
            LIMIT 1
        "#,
        )
        .bind(("cedula", cedula.to_string()))
        .await?;

    Ok(result.take(0)?)
}

pub async fn update_salida(
    ingreso_id: &RecordId,
    usuario_salida_id: &RecordId,
    observaciones: Option<String>,
) -> Result<IngresoFetched, SurrealDbError> {
    let db = get_db().await?;

    let mut dto = IngresoUpdateDTO::default();
    dto.fecha_hora_salida = Some(surrealdb::Datetime::from(chrono::Utc::now()));
    dto.usuario_salida = Some(usuario_salida_id.clone());
    dto.observaciones_salida = observaciones;
    dto.updated_at = Some(surrealdb::Datetime::from(chrono::Utc::now()));

    // UPDATE doesn't support FETCH, so we need two queries
    let _: Option<Ingreso> = db
        .query("UPDATE $id MERGE $dto")
        .bind(("id", ingreso_id.clone()))
        .bind(("dto", dto))
        .await?
        .take(0)?;

    // Fetch with all relations
    let mut result = db
        .query("SELECT * FROM $id FETCH usuario_ingreso, usuario_salida, vehiculo, contratista, contratista.empresa")
        .bind(("id", ingreso_id.clone()))
        .await?;

    let fetched: Option<IngresoFetched> = result.take(0)?;
    fetched.ok_or(SurrealDbError::TransactionError(
        "Error al registrar salida de proveedor".to_string(),
    ))
}

pub async fn find_activos() -> Result<Vec<IngresoFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM ingreso WHERE tipo_ingreso = 'proveedor' AND fecha_hora_salida IS NONE ORDER BY fecha_hora_ingreso DESC FETCH usuario_ingreso, usuario_salida, vehiculo, contratista, contratista.empresa")
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_historial() -> Result<Vec<IngresoFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM ingreso WHERE tipo_ingreso = 'proveedor' AND fecha_hora_salida IS NOT NONE ORDER BY fecha_hora_ingreso DESC LIMIT 100 FETCH usuario_ingreso, usuario_salida, vehiculo, contratista, contratista.empresa")
        .await?;
    Ok(result.take(0)?)
}
