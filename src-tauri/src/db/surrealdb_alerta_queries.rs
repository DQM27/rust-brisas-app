// ==========================================
// src/db/surrealdb_alerta_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::models::ingreso::AlertaGafete;
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use std::str::FromStr;

pub async fn insert(
    input: crate::models::ingreso::CreateAlertaInput,
) -> Result<AlertaGafete, SurrealDbError> {
    let db = get_db().await?;

    // Owned conversions are handled by the DTO structure itself mostly, just ensuring binding types
    let persona_rid = input
        .persona_id
        .as_ref()
        .map(|s| surrealdb::RecordId::from_str(s))
        .transpose()
        .map_err(|e| SurrealDbError::Query(format!("Error parsing persona_id: {e}")))?;

    let ingreso_contratista_rid = input
        .ingreso_contratista_id
        .as_ref()
        .map(|s| surrealdb::RecordId::from_str(s))
        .transpose()
        .map_err(|e| SurrealDbError::Query(format!("Error parsing ingreso_contratista_id: {e}")))?;

    let ingreso_proveedor_rid = input
        .ingreso_proveedor_id
        .as_ref()
        .map(|s| surrealdb::RecordId::from_str(s))
        .transpose()
        .map_err(|e| SurrealDbError::Query(format!("Error parsing ingreso_proveedor_id: {e}")))?;

    let ingreso_visita_rid = input
        .ingreso_visita_id
        .as_ref()
        .map(|s| surrealdb::RecordId::from_str(s))
        .transpose()
        .map_err(|e| SurrealDbError::Query(format!("Error parsing ingreso_visita_id: {e}")))?;

    let reportado_por_rid = surrealdb::RecordId::from_str(&input.reportado_por)
        .map_err(|e| SurrealDbError::Query(format!("Error parsing reportado_por: {e}")))?;

    let mut result = db
        .query(
            r"
            CREATE type::thing('alerta_gafete', $id) CONTENT {
                id: $id,
                persona: $persona_id,
                cedula: $cedula,
                nombreCompleto: $nombre_completo,
                gafeteNumero: $gafete_numero,
                ingresoContratista: $ingreso_contratista_id,
                ingresoProveedor: $ingreso_proveedor_id,
                ingresoVisita: $ingreso_visita_id,
                fechaReporte: $fecha_reporte,
                resuelto: false,
                notas: $notas,
                reportadoPor: $reportado_por,
                createdAt: time::now(),
                updatedAt: time::now()
            }
        ",
        )
        .bind(("id", input.id))
        .bind(("persona_id", persona_rid))
        .bind(("cedula", input.cedula))
        .bind(("nombre_completo", input.nombre_completo))
        .bind(("gafete_numero", input.gafete_numero))
        .bind(("ingreso_contratista_id", ingreso_contratista_rid))
        .bind(("ingreso_proveedor_id", ingreso_proveedor_rid))
        .bind(("ingreso_visita_id", ingreso_visita_rid))
        .bind(("fecha_reporte", input.fecha_reporte))
        .bind(("notas", input.notas))
        .bind(("reportado_por", reportado_por_rid))
        .await?;

    let created: Option<AlertaGafete> = result.take(0)?;
    created.ok_or(SurrealDbError::Query("No se pudo crear la alerta".to_string()))
}

pub async fn find_by_id(id: &str) -> Result<Option<AlertaGafete>, SurrealDbError> {
    let db = get_db().await?;
    // Strip table prefix and Unicode brackets that SurrealDB uses internally
    let id_only = id.strip_prefix("alerta_gafete:").unwrap_or(id).replace(['⟨', '⟩'], "");

    let mut result =
        db.query("SELECT * FROM type::thing('alerta_gafete', $id)").bind(("id", id_only)).await?;
    Ok(result.take(0)?)
}

pub async fn find_pendientes_by_cedula(cedula: &str) -> Result<Vec<AlertaGafete>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM alerta_gafete WHERE cedula = $cedula AND resuelto = false ORDER BY createdAt DESC")
        .bind(("cedula", cedula.to_string()))
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_all(resuelto: Option<bool>) -> Result<Vec<AlertaGafete>, SurrealDbError> {
    let db = get_db().await?;

    let sql = match resuelto {
        Some(_) => "SELECT * FROM alerta_gafete WHERE resuelto = $resuelto ORDER BY createdAt DESC",
        None => "SELECT * FROM alerta_gafete ORDER BY createdAt DESC",
    };

    let mut query = db.query(sql);
    if let Some(r) = resuelto {
        query = query.bind(("resuelto", r));
    }

    let mut result = query.await?;
    Ok(result.take(0)?)
}

pub async fn resolver(
    input: crate::models::ingreso::ResolverAlertaInput,
) -> Result<Option<AlertaGafete>, SurrealDbError> {
    let db = get_db().await?;
    // Strip table prefix and Unicode brackets
    let id_only = input
        .alerta_id
        .strip_prefix("alerta_gafete:")
        .unwrap_or(&input.alerta_id)
        .replace(['⟨', '⟩'], "");

    let usuario_rid = input
        .usuario_id
        .as_ref()
        .map(|s| surrealdb::RecordId::from_str(s))
        .transpose()
        .map_err(|e| SurrealDbError::Query(format!("Error parsing usuario_id: {e}")))?;

    let mut result = db
        .query(
            r"
            UPDATE type::thing('alerta_gafete', $id) MERGE {
                resuelto: true,
                fechaResolucion: time::now(),
                notas: $notas,
                resueltoPor: $usuario_id,
                updatedAt: time::now()
            }
        ",
        )
        .bind(("id", id_only))
        .bind(("notas", input.notas))
        .bind(("usuario_id", usuario_rid))
        .await?;

    Ok(result.take(0)?)
}

pub async fn delete(id: &str) -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    let id_only = id.strip_prefix("alerta_gafete:").unwrap_or(id).to_string();
    db.query("DELETE type::thing('alerta_gafete', $id)").bind(("id", id_only)).await?;
    Ok(())
}
