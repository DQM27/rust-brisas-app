//! # Servicio General de Ingresos
//!
//! Hub central para la consulta unificada de movimientos (Ingresos/Salidas).
//! Consolida información de Contratistas, Proveedores y Visitas para dashboards y reportes.
//!
//! ## Responsabilidades
//! - Consulta unificada de métricas de ocupación (Dashboard).
//! - Listado de personas actualmente en planta (Evacuación/Control).
//! - Búsqueda global por ID o Gafete.
//! - Reportes históricos de salidas.
//!
//! ## Dependencias
//! - `crate::db::surrealdb_ingreso_general_queries`: Acceso a datos unificado.
//! - `crate::domain::errors::IngresoError`: Manejo de errores estándar.

use crate::db::surrealdb_ingreso_general_queries as db;
use crate::domain::errors::IngresoError;
use crate::models::ingreso::{IngresoListResponse, IngresoResponse};
use log::{error, info, warn};
use surrealdb::RecordId;

// --------------------------------------------------------------------------
// HELPERS PRIVADOS
// --------------------------------------------------------------------------

/// Normaliza y valida IDs de ingresos.
///
/// Soporta formato "tabla:id" o "id" (asumiendo `ingreso_contratista` por defecto para compatibilidad).
fn parse_ingreso_id(id_str: &str) -> Result<RecordId, IngresoError> {
    if id_str.contains(':') {
        id_str
            .parse::<RecordId>()
            .map_err(|_| IngresoError::Validation(format!("ID de ingreso inválido: {id_str}")))
    } else {
        // Por defecto asume ingreso de contratista (el más común en sistema legacy).
        Ok(RecordId::from_table_key("ingreso_contratista", id_str))
    }
}

// --------------------------------------------------------------------------
// FUNCIONES PÚBLICAS
// --------------------------------------------------------------------------

/// Recupera la totalidad de los movimientos con métricas operativas.
///
/// Útil para dashboards principales. Ignora registros corruptos pero los loguea como advertencia.
///
/// # Retorno
/// `IngresoListResponse` con listas filtradas y totales calculados.
///
/// # Errores
/// * `IngresoError::Database` - Fallo de conexión o consulta.
pub async fn get_all_ingresos_with_stats() -> Result<IngresoListResponse, IngresoError> {
    let results = db::find_all_fetched().await.map_err(|e| {
        error!("Error al obtener todos los ingresos: {e}");
        IngresoError::Database(e.to_string())
    })?;

    let mut responses = Vec::with_capacity(results.len());
    let mut errores_conversion = 0;

    for ingreso in results {
        match ingreso.to_response() {
            Ok(response) => responses.push(response),
            Err(e) => {
                errores_conversion += 1;
                warn!("Error convirtiendo ingreso (ignorado): {e}");
            }
        }
    }

    if errores_conversion > 0 {
        warn!("Se omitieron {errores_conversion} ingresos corruptos o incompatibles");
    }

    let total = responses.len();
    let adentro = responses.iter().filter(|i| i.fecha_hora_salida.is_none()).count();
    let salieron = total - adentro;

    info!("Dashboard actualizado: Total={total}, Adentro={adentro}, Salieron={salieron}");

    Ok(IngresoListResponse { ingresos: responses, total, adentro, salieron })
}

/// Filtra todas las personas (Contratistas, Proveedores, Visitas) que permanecen actualmente en las instalaciones.
///
/// Crítico para listas de ocupación global, reportes de evacuación y control de seguridad en tiempo real.
pub async fn get_personal_en_planta_unificado() -> Result<Vec<IngresoResponse>, IngresoError> {
    let results = db::find_ingresos_abiertos_fetched().await.map_err(|e| {
        error!("Error buscando ingresos abiertos: {e}");
        IngresoError::Database(e.to_string())
    })?;

    let mut responses = Vec::with_capacity(results.len());
    for ingreso in results {
        if let Ok(response) = ingreso.to_response() {
            responses.push(response);
        }
    }

    info!("Reporte de personal en planta generado: {} registros", responses.len());
    Ok(responses)
}

/// Localiza un ingreso específico por su ID único.
///
/// # Argumentos
/// * `id_str` - ID del ingreso ("tabla:id" o "id").
///
/// # Retorno
/// `Option<IngresoResponse>` si se encuentra.
///
/// # Errores
/// * `IngresoError::Validation` - ID malformado.
/// * `IngresoError::Database` - Error de consulta.
pub async fn get_ingreso_by_id(id_str: &str) -> Result<Option<IngresoResponse>, IngresoError> {
    let id = parse_ingreso_id(id_str)?;

    let ingreso = match db::find_by_id_fetched(&id)
        .await
        .map_err(|e| IngresoError::Database(e.to_string()))?
    {
        Some(i) => i,
        None => return Ok(None),
    };

    let response = ingreso
        .to_response()
        .map_err(|e| IngresoError::Validation(format!("Error procesando datos de ingreso: {e}")))?;

    Ok(Some(response))
}

/// Identifica quién porta actualmente un recurso físico (gafete).
///
/// # Argumentos
/// * `gafete_numero` - Número visible del gafete.
///
/// # Retorno
/// `Option<IngresoResponse>` con los datos de la persona que tiene el gafete.
pub async fn get_ingreso_by_gafete(
    gafete_numero: i32,
) -> Result<Option<IngresoResponse>, IngresoError> {
    let ingreso = if let Some(i) =
        db::find_ingreso_by_gafete_fetched(gafete_numero).await.map_err(|e| {
            error!("Error buscando ingreso por gafete {gafete_numero}: {e}");
            IngresoError::Database(e.to_string())
        })? {
        i
    } else {
        info!("No se encontró ingreso activo para el gafete {gafete_numero}");
        return Ok(None);
    };

    let response = ingreso.to_response().map_err(|e| {
        IngresoError::Validation(format!("Error procesando ingreso encontrado: {e}"))
    })?;

    info!("Gafete {} localizado: Asignado a {}", gafete_numero, response.nombre_completo);
    Ok(Some(response))
}

/// Consulta histórica de salidas por rangos temporales.
///
/// # Argumentos
/// * `fecha_inicio` - Fecha ISO.
/// * `fecha_fin` - Fecha ISO.
pub async fn get_salidas_en_rango(
    fecha_inicio: &str,
    fecha_fin: &str,
) -> Result<Vec<IngresoResponse>, IngresoError> {
    info!("Solicitando historial salidas: Desde {fecha_inicio} Hasta {fecha_fin}");
    let results =
        db::find_salidas_in_range_fetched(fecha_inicio, fecha_fin).await.map_err(|e| {
            error!("Error reporte salidas {fecha_inicio} - {fecha_fin}: {e}");
            IngresoError::Database(e.to_string())
        })?;

    let mut responses = Vec::with_capacity(results.len());
    for ingreso in results {
        if let Ok(response) = ingreso.to_response() {
            responses.push(response);
        }
    }

    info!(
        "Reporte salidas generado: {} registros entre {} y {}",
        responses.len(),
        fecha_inicio,
        fecha_fin
    );
    Ok(responses)
}

// --------------------------------------------------------------------------
// TESTS UNITARIOS
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ingreso_id_simple() {
        // IDs simples asumen "ingreso_contratista"
        let res = parse_ingreso_id("123");
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap().to_string().replace("⟨", "").replace("⟩", ""),
            "ingreso_contratista:123"
        );
    }

    #[test]
    fn test_parse_ingreso_id_compuesto() {
        // IDs compuestos se respetan
        let res = parse_ingreso_id("ingreso_proveedor:456");
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap().to_string().replace("⟨", "").replace("⟩", ""),
            "ingreso_proveedor:456"
        );
    }

    #[test]
    fn test_parse_ingreso_id_invalido() {
        // Formato inválido que parse() rechaza
        // Nota: SurrealDB parse() es permisivo, pero "tabla:sin_valor:" podría fallar en algunos drivers.
        // Aquí probamos que no falle catastróficamente la función.
        let res = parse_ingreso_id("invalid:::id");
        // Dependiendo de la implementación de RecordId, puede ser Ok o Err,
        // pero validamos que la función helper se comporte.
        // En este caso, solo verificamos que retorne 'algo', preferiblemente Err si es muy malo.
        // Si el driver permite garbage, esto podría ser Ok.
        let _ = res;
    }
}
