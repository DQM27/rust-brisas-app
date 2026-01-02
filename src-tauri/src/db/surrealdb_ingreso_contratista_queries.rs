//! # Queries SurrealDB: Ingreso Contratista
//!
//! Operaciones de base de datos para control de acceso de contratistas.
//!
//! ## Responsabilidades
//! - Registro de entradas y salidas
//! - Consulta de ingresos abiertos
//! - FETCH con relaciones (contratista, empresa, usuarios)
//!
//! ## Tabla: `ingreso_contratista`

use crate::models::ingreso::{
    IngresoContratista, IngresoContratistaCreateDTO, IngresoContratistaFetched,
    IngresoContratistaUpdateDTO,
};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use log::{debug, info, warn};
use surrealdb::RecordId;

/// Tabla en SurrealDB
const TABLE: &str = "ingreso_contratista";

/// Registra un nuevo ingreso de contratista.
///
/// ## Query
/// ```sql
/// CREATE ingreso_contratista CONTENT $dto
/// ```
pub async fn insert(
    dto: IngresoContratistaCreateDTO,
) -> Result<IngresoContratistaFetched, SurrealDbError> {
    debug!("➕ Registrando nuevo ingreso de contratista");
    let db = get_db().await?;

    // CREATE doesn't support FETCH, so we need two queries
    let created: Option<IngresoContratista> =
        db.query(format!("CREATE {} CONTENT $dto", TABLE)).bind(("dto", dto)).await?.take(0)?;

    let ingreso = created.ok_or(SurrealDbError::TransactionError(
        "Error al insertar ingreso_contratista".to_string(),
    ))?;

    // Fetch with all relations
    let mut result = db
        .query("SELECT * FROM $id FETCH usuario_ingreso, usuario_salida, contratista, contratista.empresa")
        .bind(("id", ingreso.id.clone()))
        .await?;

    let fetched: Option<IngresoContratistaFetched> = result.take(0)?;

    match fetched {
        Some(f) => {
            info!("✅ Ingreso registrado: id={}", ingreso.id);
            Ok(f)
        }
        None => {
            warn!("⚠️ Ingreso creado pero FETCH falló: id={}", ingreso.id);
            Err(SurrealDbError::TransactionError(
                "Ingreso creado pero no se pudo obtener con FETCH".to_string(),
            ))
        }
    }
}

/// Busca ingreso abierto (sin salida) de un contratista.
///
/// ## Query
/// ```sql
/// SELECT * FROM ingreso_contratista
/// WHERE contratista = $contratista AND fecha_hora_salida IS NONE
/// LIMIT 1 FETCH ...
/// ```
pub async fn find_ingreso_abierto_by_contratista(
    contratista_id: &RecordId,
) -> Result<Option<IngresoContratistaFetched>, SurrealDbError> {
    debug!("🔍 Buscando ingreso abierto para contratista: {}", contratista_id);
    let db = get_db().await?;

    let mut result = db
        .query(format!(
            "SELECT * FROM {} WHERE contratista = $contratista AND fecha_hora_salida IS NONE LIMIT 1 FETCH usuario_ingreso, usuario_salida, contratista, contratista.empresa",
            TABLE
        ))
        .bind(("contratista", contratista_id.clone()))
        .await?;

    Ok(result.take(0)?)
}

/// Registra la salida de un contratista.
///
/// ## Query
/// ```sql
/// UPDATE $id MERGE { fecha_hora_salida, usuario_salida, observaciones }
/// ```
pub async fn update_salida(
    ingreso_id: &RecordId,
    usuario_salida_id: &RecordId,
    observaciones: Option<String>,
) -> Result<IngresoContratistaFetched, SurrealDbError> {
    debug!("🚪 Registrando salida: ingreso={}", ingreso_id);
    let db = get_db().await?;

    let mut dto = IngresoContratistaUpdateDTO::default();
    dto.fecha_hora_salida = Some(surrealdb::Datetime::from(chrono::Utc::now()));
    dto.usuario_salida = Some(usuario_salida_id.clone());
    dto.observaciones = observaciones;

    // UPDATE doesn't support FETCH, so we need two queries
    let _: Option<IngresoContratista> = db
        .query("UPDATE $id MERGE $dto")
        .bind(("id", ingreso_id.clone()))
        .bind(("dto", dto))
        .await?
        .take(0)?;

    // Fetch with all relations
    let mut result = db
        .query("SELECT * FROM $id FETCH usuario_ingreso, usuario_salida, contratista, contratista.empresa")
        .bind(("id", ingreso_id.clone()))
        .await?;

    let fetched: Option<IngresoContratistaFetched> = result.take(0)?;

    match fetched {
        Some(f) => {
            info!("✅ Salida registrada: ingreso={}", ingreso_id);
            Ok(f)
        }
        None => {
            warn!("⚠️ Error al registrar salida, FETCH falló: ingreso={}", ingreso_id);
            Err(SurrealDbError::TransactionError("Error al registrar salida".to_string()))
        }
    }
}

/// Busca un ingreso por su ID.
pub async fn find_by_id(id: &RecordId) -> Result<Option<IngresoContratista>, SurrealDbError> {
    debug!("🔍 Buscando ingreso por ID: {}", id);
    let db = get_db().await?;
    let result: Option<IngresoContratista> = db.select(id.clone()).await?;
    Ok(result)
}

/// Busca un ingreso por ID con relaciones pobladas.
pub async fn find_by_id_fetched(
    id: &RecordId,
) -> Result<Option<IngresoContratistaFetched>, SurrealDbError> {
    debug!("🔍 Buscando ingreso (fetched) por ID: {}", id);
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM $id FETCH usuario_ingreso, usuario_salida, contratista, contratista.empresa")
        .bind(("id", id.clone()))
        .await?;
    Ok(result.take(0)?)
}
