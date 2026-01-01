/// Servicio: Gestión de Alertas.
///
/// Orquestador de la lógica de negocio para la gestión de incidencias de seguridad (AlertaGafete).
///
/// Responsabilidades:
/// - Registrar nuevas alertas detectadas por el sistema o usuarios.
/// - Consultar historial de alertas (pendientes vs resueltas).
/// - Gestionar la resolución de alertas por parte de seguridad.
use crate::db::surrealdb_alerta_queries as db;
use crate::domain::errors::AlertaError;
use crate::models::ingreso::AlertaGafete;
use log::{error, info};

/// Localiza una alerta específica para su auditoría o resolución.
pub async fn find_by_id(id: &str) -> Result<AlertaGafete, AlertaError> {
    db::find_by_id(id)
        .await
        .map_err(|e| AlertaError::Database(e.to_string()))?
        .ok_or(AlertaError::NotFound)
}

/// Recupera todas las alertas pendientes asociadas a una persona (Cédula).
/// Fundamental para detectar reincidencias durante el proceso de ingreso.
pub async fn find_pendientes_by_cedula(cedula: &str) -> Result<Vec<AlertaGafete>, AlertaError> {
    db::find_pendientes_by_cedula(cedula).await.map_err(|e| AlertaError::Database(e.to_string()))
}

/// Lista alertas filtradas por su estado de resolución.
pub async fn find_all(resuelto: Option<bool>) -> Result<Vec<AlertaGafete>, AlertaError> {
    db::find_all(resuelto).await.map_err(|e| AlertaError::Database(e.to_string()))
}

/// Registra una nueva alerta en el sistema.
///
/// Se invoca automáticamente por el sistema de monitoreo o manualmente por el guardia.
///
/// # Arguments
///
/// * `input` - DTO con los datos completos de la alerta.
///
/// # Errors
///
/// * `AlertaError::Database`: Fallo de conexión o inserción.
pub async fn insert(input: crate::models::ingreso::CreateAlertaInput) -> Result<(), AlertaError> {
    let cedula = input.cedula.clone();
    let gafete = input.gafete_numero;

    db::insert(input).await.map_err(|e| {
        error!("Error de base de datos al insertar alerta para {}: {}", cedula, e);
        AlertaError::Database(e.to_string())
    })?;

    info!("Alerta crítica registrada para {} (Gafete: {})", cedula, gafete);
    Ok(())
}

/// Marca una alerta como gestionada/resuelta.
///
/// Registra qué usuario resolvió la incidencia y las notas correspondientes.
///
/// # Arguments
///
/// * `input` - DTO con ID de alerta, usuario resolutor y notas.
///
/// # Errors
///
/// * `AlertaError::Database`: Fallo al actualizar el registro.
pub async fn resolver(
    input: crate::models::ingreso::ResolverAlertaInput,
) -> Result<(), AlertaError> {
    let id = input.alerta_id.clone();

    info!("Resolviendo alerta {}", id);
    db::resolver(input).await.map_err(|e| {
        error!("Error al resolver alerta {}: {}", id, e);
        AlertaError::Database(e.to_string())
    })?;

    info!("Alerta {} resuelta exitosamente", id);
    Ok(())
}

/// Elimina físicamente una alerta (Solo para correcciones administrativas).
pub async fn delete(id: &str) -> Result<(), AlertaError> {
    db::delete(id).await.map_err(|e| AlertaError::Database(e.to_string()))
}
