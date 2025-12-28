// src/services/ingreso_general_service.rs
// Servicio para operaciones generales de ingresos (consultas, estadísticas)

use crate::db::surrealdb_ingreso_general_queries as db;
use crate::models::ingreso::{IngresoListResponse, IngresoResponse};
use crate::services::surrealdb_service::SurrealDbError;
use surrealdb::RecordId;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IngresoGeneralError {
    #[error("Error de base de datos: {0}")]
    Database(String),

    #[error("Error al procesar datos: {0}")]
    DataProcessing(String),
}

/// Helper para parsear ID de ingreso
fn parse_ingreso_id(id_str: &str) -> RecordId {
    if id_str.contains(':') {
        let parts: Vec<&str> = id_str.split(':').collect();
        RecordId::from_table_key(parts[0], parts[1])
    } else {
        RecordId::from_table_key("ingreso_contratista", id_str)
    }
}

impl From<SurrealDbError> for IngresoGeneralError {
    fn from(e: SurrealDbError) -> Self {
        IngresoGeneralError::Database(e.to_string())
    }
}

/// Obtiene todos los ingresos con estadísticas calculadas
pub async fn get_all_ingresos_with_stats() -> Result<IngresoListResponse, IngresoGeneralError> {
    let results = db::find_all_fetched().await?;

    let mut responses = Vec::with_capacity(results.len());
    for ingreso in results {
        if let Ok(response) = IngresoResponse::from_contratista_fetched(ingreso) {
            responses.push(response);
        }
    }

    let total = responses.len();
    let adentro = responses.iter().filter(|i| i.fecha_hora_salida.is_none()).count();
    let salieron = total - adentro;

    Ok(IngresoListResponse { ingresos: responses, total, adentro, salieron })
}

/// Obtiene ingresos abiertos (personas adentro)
pub async fn get_ingresos_abiertos() -> Result<Vec<IngresoResponse>, IngresoGeneralError> {
    let results = db::find_ingresos_abiertos_fetched().await?;

    let mut responses = Vec::with_capacity(results.len());
    for ingreso in results {
        if let Ok(response) = IngresoResponse::from_contratista_fetched(ingreso) {
            responses.push(response);
        }
    }
    Ok(responses)
}

/// Obtiene un ingreso por ID con detalles
pub async fn get_ingreso_by_id(
    id_str: &str,
) -> Result<Option<IngresoResponse>, IngresoGeneralError> {
    let id = parse_ingreso_id(id_str);
    let ingreso = match db::find_by_id_fetched(&id).await? {
        Some(i) => i,
        None => return Ok(None),
    };

    let response = IngresoResponse::from_contratista_fetched(ingreso)
        .map_err(|e| IngresoGeneralError::DataProcessing(e))?;

    Ok(Some(response))
}

/// Obtiene ingreso activo por gafete
pub async fn get_ingreso_by_gafete(
    gafete_numero: &str,
) -> Result<Option<IngresoResponse>, IngresoGeneralError> {
    let ingreso = match db::find_ingreso_by_gafete_fetched(gafete_numero).await? {
        Some(i) => i,
        None => return Ok(None),
    };

    let response = IngresoResponse::from_contratista_fetched(ingreso)
        .map_err(|e| IngresoGeneralError::DataProcessing(e))?;

    Ok(Some(response))
}

/// Obtiene salidas en un rango de fechas
pub async fn get_salidas_en_rango(
    fecha_inicio: &str,
    fecha_fin: &str,
) -> Result<Vec<IngresoResponse>, IngresoGeneralError> {
    let results = db::find_salidas_in_range_fetched(fecha_inicio, fecha_fin).await?;

    let mut responses = Vec::with_capacity(results.len());
    for ingreso in results {
        if let Ok(response) = IngresoResponse::from_contratista_fetched(ingreso) {
            responses.push(response);
        }
    }
    Ok(responses)
}
