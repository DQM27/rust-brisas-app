// ==========================================
// src/services/alerta_service.rs
// ==========================================

use crate::db::surrealdb_alerta_queries as db;
use crate::domain::errors::AlertaError;
use crate::models::ingreso::AlertaGafete;
use log::{error, info};

pub async fn find_by_id(id: &str) -> Result<AlertaGafete, AlertaError> {
    db::find_by_id(id)
        .await
        .map_err(|e| AlertaError::Database(e.to_string()))?
        .ok_or(AlertaError::NotFound)
}

pub async fn find_pendientes_by_cedula(cedula: &str) -> Result<Vec<AlertaGafete>, AlertaError> {
    db::find_pendientes_by_cedula(cedula).await.map_err(|e| AlertaError::Database(e.to_string()))
}

pub async fn find_all(resuelto: Option<bool>) -> Result<Vec<AlertaGafete>, AlertaError> {
    db::find_all(resuelto).await.map_err(|e| AlertaError::Database(e.to_string()))
}

#[allow(clippy::too_many_arguments)]
pub async fn insert(
    id: &str,
    persona_id: Option<&str>,
    cedula: &str,
    nombre_completo: &str,
    gafete_numero: &str,
    ingreso_contratista_id: Option<&str>,
    ingreso_proveedor_id: Option<&str>,
    ingreso_visita_id: Option<&str>,
    fecha_reporte: &str,
    notas: Option<&str>,
    reportado_por: &str,
) -> Result<(), AlertaError> {
    db::insert(
        id,
        persona_id,
        cedula,
        nombre_completo,
        gafete_numero,
        ingreso_contratista_id,
        ingreso_proveedor_id,
        ingreso_visita_id,
        fecha_reporte,
        notas,
        reportado_por,
    )
    .await
    .map_err(|e| {
        error!("Error de base de datos al insertar alerta para {}: {}", cedula, e);
        AlertaError::Database(e.to_string())
    })?;

    info!("Alerta registrada para {} (Gafete: {})", cedula, gafete_numero);
    Ok(())
}

pub async fn resolver(id: &str, notas: Option<&str>, usuario_id: &str) -> Result<(), AlertaError> {
    info!("Resolviendo alerta {}", id);
    db::resolver(id, notas, usuario_id).await.map_err(|e| {
        error!("Error al resolver alerta {}: {}", id, e);
        AlertaError::Database(e.to_string())
    })?;

    info!("Alerta {} resuelta exitosamente", id);
    Ok(())
}

pub async fn delete(id: &str) -> Result<(), AlertaError> {
    db::delete(id).await.map_err(|e| AlertaError::Database(e.to_string()))
}
