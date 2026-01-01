// ==========================================
// src/db/surrealdb_alerta_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::models::ingreso::AlertaGafete;
use crate::services::surrealdb_service::{get_db, SurrealDbError};

pub async fn insert(
    input: crate::models::ingreso::CreateAlertaInput,
) -> Result<AlertaGafete, SurrealDbError> {
    let db = get_db().await?;

    // Owned conversions are handled by the DTO structure itself mostly, just ensuring binding types
    let mut result = db
        .query(
            r#"
            CREATE type::thing('alerta_gafete', $id) CONTENT {
                id: $id,
                persona_id: $persona_id,
                cedula: $cedula,
                nombre_completo: $nombre_completo,
                gafete_numero: $gafete_numero,
                ingreso_contratista_id: $ingreso_contratista_id,
                ingreso_proveedor_id: $ingreso_proveedor_id,
                ingreso_visita_id: $ingreso_visita_id,
                fecha_reporte: $fecha_reporte,
                resuelto: false,
                notas: $notas,
                reportado_por: $reportado_por,
                created_at: time::now(),
                updated_at: time::now()
            }
        "#,
        )
        .bind(("id", input.id))
        .bind(("persona_id", input.persona_id))
        .bind(("cedula", input.cedula))
        .bind(("nombre_completo", input.nombre_completo))
        .bind(("gafete_numero", input.gafete_numero))
        .bind(("ingreso_contratista_id", input.ingreso_contratista_id))
        .bind(("ingreso_proveedor_id", input.ingreso_proveedor_id))
        .bind(("ingreso_visita_id", input.ingreso_visita_id))
        .bind(("fecha_reporte", input.fecha_reporte))
        .bind(("notas", input.notas))
        .bind(("reportado_por", input.reportado_por))
        .await?;

    let created: Option<AlertaGafete> = result.take(0)?;
    created.ok_or(SurrealDbError::Query("No se pudo crear la alerta".to_string()))
}

pub async fn find_by_id(id: &str) -> Result<Option<AlertaGafete>, SurrealDbError> {
    let db = get_db().await?;
    let id_only = id.strip_prefix("alerta_gafete:").unwrap_or(id).to_string();

    let mut result =
        db.query("SELECT * FROM type::thing('alerta_gafete', $id)").bind(("id", id_only)).await?;
    Ok(result.take(0)?)
}

pub async fn find_pendientes_by_cedula(cedula: &str) -> Result<Vec<AlertaGafete>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM alerta_gafete WHERE cedula = $cedula AND resuelto = false ORDER BY created_at DESC")
        .bind(("cedula", cedula.to_string()))
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_all(resuelto: Option<bool>) -> Result<Vec<AlertaGafete>, SurrealDbError> {
    let db = get_db().await?;

    let sql = match resuelto {
        Some(_) => {
            "SELECT * FROM alerta_gafete WHERE resuelto = $resuelto ORDER BY created_at DESC"
        }
        None => "SELECT * FROM alerta_gafete ORDER BY created_at DESC",
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
    let id_only =
        input.alerta_id.strip_prefix("alerta_gafete:").unwrap_or(&input.alerta_id).to_string();

    let mut result = db
        .query(
            r#"
            UPDATE type::thing('alerta_gafete', $id) MERGE {
                resuelto: true,
                fecha_resolucion: time::now(),
                notas: $notas,
                resuelto_por: $usuario_id,
                updated_at: time::now()
            }
        "#,
        )
        .bind(("id", id_only))
        .bind(("notas", input.notas))
        .bind(("usuario_id", input.usuario_id))
        .await?;

    Ok(result.take(0)?)
}

pub async fn delete(id: &str) -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    let id_only = id.strip_prefix("alerta_gafete:").unwrap_or(id).to_string();
    db.query("DELETE type::thing('alerta_gafete', $id)").bind(("id", id_only)).await?;
    Ok(())
}
