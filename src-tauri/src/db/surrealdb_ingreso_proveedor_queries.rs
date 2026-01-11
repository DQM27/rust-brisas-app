// ==========================================
// src/db/surrealdb_ingreso_proveedor_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::models::ingreso::{
    IngresoProveedor, IngresoProveedorCreateDTO, IngresoProveedorFetched, IngresoUpdateDTO,
};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use surrealdb::RecordId;

const TABLE: &str = "ingreso_proveedor";

pub async fn insert(
    dto: IngresoProveedorCreateDTO,
) -> Result<IngresoProveedorFetched, SurrealDbError> {
    let db = get_db().await?;

    let created: Option<IngresoProveedor> =
        db.query(format!("CREATE {TABLE} CONTENT $dto")).bind(("dto", dto)).await?.take(0)?;

    let ingreso = created.ok_or(SurrealDbError::TransactionError(
        "Error al insertar ingreso de proveedor".to_string(),
    ))?;

    // Fetch relations
    let mut result = db
        .query(
            "SELECT * FROM $id FETCH usuario_ingreso, usuario_salida, proveedor, proveedor.empresa",
        )
        .bind(("id", ingreso.id.clone()))
        .await?;

    let fetched: Option<IngresoProveedorFetched> = result.take(0)?;
    fetched.ok_or(SurrealDbError::TransactionError(
        "Ingreso creado pero no se pudo obtener con FETCH".to_string(),
    ))
}

pub async fn find_ingreso_abierto_by_proveedor(
    proveedor_id: &RecordId,
) -> Result<Option<IngresoProveedorFetched>, SurrealDbError> {
    let db = get_db().await?;

    let mut result = db
        .query(format!(
            "SELECT * FROM {TABLE} WHERE proveedor = $proveedor AND fecha_hora_salida IS NONE LIMIT 1 FETCH usuario_ingreso, usuario_salida, proveedor, proveedor.empresa"
        ))
        .bind(("proveedor", proveedor_id.clone()))
        .await?;

    Ok(result.take(0)?)
}

pub async fn update_salida(
    ingreso_id: &RecordId,
    usuario_salida_id: &RecordId,
    observaciones: Option<String>,
) -> Result<IngresoProveedorFetched, SurrealDbError> {
    let db = get_db().await?;

    let mut dto = IngresoUpdateDTO::default();
    dto.fecha_hora_salida = Some(surrealdb::Datetime::from(chrono::Utc::now()));
    dto.usuario_salida = Some(usuario_salida_id.clone());
    dto.observaciones = observaciones;

    let _: Option<IngresoProveedor> = db
        .query("UPDATE $id MERGE $dto")
        .bind(("id", ingreso_id.clone()))
        .bind(("dto", dto))
        .await?
        .take(0)?;

    let mut result = db
        .query(
            "SELECT * FROM $id FETCH usuario_ingreso, usuario_salida, proveedor, proveedor.empresa",
        )
        .bind(("id", ingreso_id.clone()))
        .await?;

    let fetched: Option<IngresoProveedorFetched> = result.take(0)?;
    fetched.ok_or(SurrealDbError::TransactionError(
        "Error al registrar salida de proveedor".to_string(),
    ))
}

pub async fn find_activos_fetched() -> Result<Vec<IngresoProveedorFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query(format!(
            "SELECT * FROM {TABLE} WHERE fecha_hora_salida IS NONE ORDER BY created_at DESC FETCH usuario_ingreso, usuario_salida, proveedor, proveedor.empresa"
        ))
        .await?;
    Ok(result.take(0)?)
}
