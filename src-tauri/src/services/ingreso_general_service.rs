/// Hub de Inteligencia: Monitoreo Unificado de Ingresos.
///
/// Este servicio consolida la información de todos los puntos de acceso.
/// Es el motor detrás de los tableros de control (dashboards) y permite al
/// personal de seguridad tener una visión 360° de quién está dentro de la
/// planta en cualquier momento, sin importar su categoría (contratista, proveedor, etc).
use crate::db::surrealdb_ingreso_general_queries as db;
use crate::models::ingreso::{IngresoListResponse, IngresoResponse};
use crate::services::surrealdb_service::SurrealDbError;
use surrealdb::RecordId;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IngresoGeneralError {
    #[error("Fallo de infraestructura en base de datos: {0}")]
    Database(String),

    #[error("Error al procesar la lógica de negocio de ingresos: {0}")]
    DataProcessing(String),
}

/// Normalización y tipado de IDs para consultas transversales.
fn parse_ingreso_id(id_str: &str) -> RecordId {
    if id_str.contains(':') {
        let parts: Vec<&str> = id_str.split(':').collect();
        RecordId::from_table_key(parts[0], parts[1])
    } else {
        // Por defecto asume ingreso de contratista (el más común).
        RecordId::from_table_key("ingreso_contratista", id_str)
    }
}

impl From<SurrealDbError> for IngresoGeneralError {
    fn from(e: SurrealDbError) -> Self {
        IngresoGeneralError::Database(e.to_string())
    }
}

/// Recupera la totalidad de los movimientos con métricas operativas calculadas.
///
/// Útil para la generación de reportes maestros de seguridad y estadísticas de flujo.
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

/// Filtra exclusivamente a las personas que permanecen actualmente en las instalaciones.
///
/// Es la fuente de verdad para evacuación de emergencia y control de personal activo.
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

/// Localiza un ingreso específico mediante su ID único.
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

/// Identifica quién porta actualmente un recurso físico (gafete).
///
/// Permite rastrear la identidad de una persona en planta a partir de su número de identificación visual.
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

/// Consulta histórica por rangos temporales.
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
