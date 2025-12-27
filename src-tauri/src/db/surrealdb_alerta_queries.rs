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
    created_at: &str,
    updated_at: &str,
) -> Result<AlertaGafete, SurrealDbError> {
    let client = get_db().await?;

    let sql = r#"
        CREATE type::thing('alertas', $id) CONTENT {
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
            created_at: $created_at,
            updated_at: $updated_at
        }
    "#;

    // Binds con .to_string() para evitar lifetimes
    let mut result = client
        .query(sql)
        .bind(("id", id.to_string()))
        .bind(("persona_id", persona_id.map(String::from)))
        .bind(("cedula", cedula.to_string()))
        .bind(("nombre_completo", nombre_completo.to_string()))
        .bind(("gafete_numero", gafete_numero.to_string()))
        .bind(("ingreso_contratista_id", ingreso_contratista_id.map(String::from)))
        .bind(("ingreso_proveedor_id", ingreso_proveedor_id.map(String::from)))
        .bind(("ingreso_visita_id", ingreso_visita_id.map(String::from)))
        .bind(("fecha_reporte", fecha_reporte.to_string()))
        .bind(("notas", notas.map(String::from)))
        .bind(("reportado_por", reportado_por.to_string()))
        .bind(("created_at", created_at.to_string()))
        .bind(("updated_at", updated_at.to_string()))
        .await?;

    let created: Option<AlertaGafete> = result.take(0)?;
    created.ok_or(SurrealDbError::Query("No se pudo crear la alerta".to_string()))
}

pub async fn find_by_id(id: &str) -> Result<Option<AlertaGafete>, SurrealDbError> {
    let client = get_db().await?;
    let sql = "SELECT * FROM type::thing('alertas', $id)";
    let mut result = client.query(sql).bind(("id", id.to_string())).await?;
    Ok(result.take(0)?)
}

pub async fn find_pendientes_by_cedula(cedula: &str) -> Result<Vec<AlertaGafete>, SurrealDbError> {
    let client = get_db().await?;
    let sql = "SELECT * FROM alertas WHERE cedula = $cedula AND resuelto = false ORDER BY created_at DESC";
    let mut result = client.query(sql).bind(("cedula", cedula.to_string())).await?;
    Ok(result.take(0)?)
}

pub async fn find_all(resuelto: Option<bool>) -> Result<Vec<AlertaGafete>, SurrealDbError> {
    let client = get_db().await?;
    let mut sql = "SELECT * FROM alertas".to_string();

    if resuelto.is_some() {
        sql.push_str(" WHERE resuelto = $resuelto");
    }

    sql.push_str(" ORDER BY created_at DESC");

    let mut query = client.query(&sql);

    if let Some(r) = resuelto {
        query = query.bind(("resuelto", r));
    }

    let mut result = query.await?;
    Ok(result.take(0)?)
}

pub async fn resolver(
    id: &str,
    fecha_resolucion: &str,
    notas: Option<&str>,
    usuario_id: &str,
    updated_at: &str,
) -> Result<Option<AlertaGafete>, SurrealDbError> {
    let client = get_db().await?;

    let sql = r#"
        UPDATE type::thing('alertas', $id) MERGE {
            resuelto: true,
            fecha_resolucion: $fecha_resolucion,
            notas: $notas, // Sobreescribe o añade? Usualmente añade si no existe, o reemplaza.
            resuelto_por: $usuario_id,
            updated_at: $updated_at
        }
    "#;

    let mut result = client
        .query(sql)
        .bind(("id", id.to_string()))
        .bind(("fecha_resolucion", fecha_resolucion.to_string()))
        .bind(("notas", notas.map(String::from)))
        .bind(("usuario_id", usuario_id.to_string()))
        .bind(("updated_at", updated_at.to_string()))
        .await?;

    Ok(result.take(0)?)
}

pub async fn delete(id: &str) -> Result<(), SurrealDbError> {
    let client = get_db().await?;
    let sql = "DELETE type::thing('alertas', $id)";
    let mut _result = client.query(sql).bind(("id", id.to_string())).await?;
    Ok(())
}
