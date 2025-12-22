// src/services/ingreso_general_service.rs
// Servicio para operaciones generales de ingresos (consultas, estadísticas)

use crate::db::ingreso_general_queries as db;
use crate::models::ingreso::{IngresoListResponse, IngresoResponse};
use sqlx::SqlitePool;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IngresoGeneralError {
    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Error al procesar datos: {0}")]
    DataProcessing(String),
}

/// Obtiene todos los ingresos con estadísticas calculadas
pub async fn get_all_ingresos_with_stats(
    pool: &SqlitePool,
) -> Result<IngresoListResponse, IngresoGeneralError> {
    let results = db::find_all_with_details(pool).await?;

    let mut responses = Vec::new();
    for (ingreso, details) in results {
        if let Ok(mut response) = IngresoResponse::try_from(ingreso) {
            response.usuario_ingreso_nombre = details.usuario_ingreso_nombre.unwrap_or_default();
            response.usuario_salida_nombre = details.usuario_salida_nombre;
            response.vehiculo_placa = details.vehiculo_placa;
            responses.push(response);
        }
    }

    let total = responses.len();
    let adentro = responses.iter().filter(|i| i.fecha_hora_salida.is_none()).count();
    let salieron = total - adentro;

    Ok(IngresoListResponse { ingresos: responses, total, adentro, salieron })
}

/// Obtiene ingresos abiertos (personas adentro)
pub async fn get_ingresos_abiertos(
    pool: &SqlitePool,
) -> Result<Vec<IngresoResponse>, IngresoGeneralError> {
    let results = db::find_ingresos_abiertos_with_details(pool).await?;

    let mut responses = Vec::new();
    for (ingreso, details) in results {
        if let Ok(mut response) = IngresoResponse::try_from(ingreso) {
            response.usuario_ingreso_nombre = details.usuario_ingreso_nombre.unwrap_or_default();
            response.usuario_salida_nombre = details.usuario_salida_nombre;
            response.vehiculo_placa = details.vehiculo_placa;
            responses.push(response);
        }
    }
    Ok(responses)
}

/// Obtiene un ingreso por ID con detalles
pub async fn get_ingreso_by_id(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<IngresoResponse>, IngresoGeneralError> {
    let ingreso = match db::find_by_id(pool, id).await? {
        Some(i) => i,
        None => return Ok(None),
    };

    let details = db::find_details_by_id(pool, id).await?.unwrap_or(db::IngresoDetails {
        usuario_ingreso_nombre: None,
        usuario_salida_nombre: None,
        vehiculo_placa: None,
    });

    let mut response =
        IngresoResponse::try_from(ingreso).map_err(|e| IngresoGeneralError::DataProcessing(e))?;

    response.usuario_ingreso_nombre = details.usuario_ingreso_nombre.unwrap_or_default();
    response.usuario_salida_nombre = details.usuario_salida_nombre;
    response.vehiculo_placa = details.vehiculo_placa;

    Ok(Some(response))
}

/// Obtiene ingreso activo por gafete
pub async fn get_ingreso_by_gafete(
    pool: &SqlitePool,
    gafete_numero: &str,
) -> Result<Option<IngresoResponse>, IngresoGeneralError> {
    let ingreso = match db::find_ingreso_by_gafete(pool, gafete_numero).await? {
        Some(i) => i,
        None => return Ok(None),
    };

    let response =
        IngresoResponse::try_from(ingreso).map_err(|e| IngresoGeneralError::DataProcessing(e))?;

    Ok(Some(response))
}

/// Obtiene salidas en un rango de fechas
pub async fn get_salidas_en_rango(
    pool: &SqlitePool,
    fecha_inicio: &str,
    fecha_fin: &str,
) -> Result<Vec<IngresoResponse>, IngresoGeneralError> {
    let results = db::find_salidas_in_range_with_details(pool, fecha_inicio, fecha_fin).await?;

    let mut responses = Vec::new();
    for (ingreso, details) in results {
        if let Ok(mut response) = IngresoResponse::try_from(ingreso) {
            response.usuario_ingreso_nombre = details.usuario_ingreso_nombre.unwrap_or_default();
            response.usuario_salida_nombre = details.usuario_salida_nombre;
            response.vehiculo_placa = details.vehiculo_placa;
            responses.push(response);
        }
    }
    Ok(responses)
}
