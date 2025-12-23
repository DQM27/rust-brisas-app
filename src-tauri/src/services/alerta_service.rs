// ==========================================
// src/services/alerta_service.rs
// ==========================================

use crate::db::alerta_gafete_queries as db;
use crate::domain::errors::AlertaError;
use crate::models::ingreso::AlertaGafete;
use log::{error, info};
use sqlx::SqlitePool;

pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<AlertaGafete, AlertaError> {
    db::find_by_id(pool, id).await.map_err(AlertaError::Database)
}

pub async fn find_pendientes_by_cedula(
    pool: &SqlitePool,
    cedula: &str,
) -> Result<Vec<AlertaGafete>, AlertaError> {
    db::find_pendientes_by_cedula(pool, cedula).await.map_err(AlertaError::Database)
}

pub async fn find_all(
    pool: &SqlitePool,
    resuelto: Option<bool>,
) -> Result<Vec<AlertaGafete>, AlertaError> {
    db::find_all(pool, resuelto).await.map_err(AlertaError::Database)
}

#[allow(clippy::too_many_arguments)]
pub async fn insert(
    pool: &SqlitePool,
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
    created_at: &str,
    updated_at: &str,
) -> Result<(), AlertaError> {
    db::insert(
        pool,
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
        created_at,
        updated_at,
    )
    .await
    .map_err(|e| {
        error!("Error de base de datos al insertar alerta para {}: {}", cedula, e);
        AlertaError::Database(e)
    })?;

    info!("Alerta registrada para {} (Gafete: {})", cedula, gafete_numero);
    Ok(())
}

pub async fn resolver(
    pool: &SqlitePool,
    id: &str,
    fecha_resolucion: &str,
    notas: Option<&str>,
    usuario_id: &str,
    updated_at: &str,
) -> Result<(), AlertaError> {
    info!("Resolviendo alerta {}", id);
    db::resolver(pool, id, fecha_resolucion, notas, usuario_id, updated_at).await.map_err(|e| {
        error!("Error al resolver alerta {}: {}", id, e);
        AlertaError::Database(e)
    })?;

    info!("Alerta {} resuelta exitosamente", id);
    Ok(())
}

pub async fn delete(pool: &SqlitePool, id: &str) -> Result<(), AlertaError> {
    db::delete(pool, id).await.map_err(AlertaError::Database)
}
