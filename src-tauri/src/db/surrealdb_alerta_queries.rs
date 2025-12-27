// ==========================================
// src/db/surrealdb_alerta_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::models::ingreso::AlertaGafete;
use crate::services::surrealdb_service::{get_db, SurrealDbError};

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
) -> Result<AlertaGafete, SurrealDbError> {
    let db = get_db().await?;

    // Convert all to owned types
    let id_owned = id.to_string();
    let persona_id_owned = persona_id.map(String::from);
    let cedula_owned = cedula.to_string();
    let nombre_completo_owned = nombre_completo.to_string();
    let gafete_numero_owned = gafete_numero.to_string();
    let ingreso_contratista_owned = ingreso_contratista_id.map(String::from);
    let ingreso_proveedor_owned = ingreso_proveedor_id.map(String::from);
    let ingreso_visita_owned = ingreso_visita_id.map(String::from);
    let fecha_reporte_owned = fecha_reporte.to_string();
    let notas_owned = notas.map(String::from);
    let reportado_por_owned = reportado_por.to_string();

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
        .bind(("id", id_owned))
        .bind(("persona_id", persona_id_owned))
        .bind(("cedula", cedula_owned))
        .bind(("nombre_completo", nombre_completo_owned))
        .bind(("gafete_numero", gafete_numero_owned))
        .bind(("ingreso_contratista_id", ingreso_contratista_owned))
        .bind(("ingreso_proveedor_id", ingreso_proveedor_owned))
        .bind(("ingreso_visita_id", ingreso_visita_owned))
        .bind(("fecha_reporte", fecha_reporte_owned))
        .bind(("notas", notas_owned))
        .bind(("reportado_por", reportado_por_owned))
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
    id: &str,
    notas: Option<&str>,
    usuario_id: &str,
) -> Result<Option<AlertaGafete>, SurrealDbError> {
    let db = get_db().await?;
    let id_only = id.strip_prefix("alerta_gafete:").unwrap_or(id).to_string();
    let notas_owned = notas.map(String::from);
    let usuario_id_owned = usuario_id.to_string();

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
        .bind(("notas", notas_owned))
        .bind(("usuario_id", usuario_id_owned))
        .await?;

    Ok(result.take(0)?)
}

pub async fn delete(id: &str) -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    let id_only = id.strip_prefix("alerta_gafete:").unwrap_or(id).to_string();
    db.query("DELETE type::thing('alerta_gafete', $id)").bind(("id", id_only)).await?;
    Ok(())
}
