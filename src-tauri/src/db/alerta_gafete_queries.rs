// ==========================================
// src/db/alerta_gafete_queries.rs
// ==========================================
// Queries SQL puras para alertas de gafetes - Sin lógica de negocio
// Strict Mode: Uso de query! para validación en tiempo de compilación

use crate::models::ingreso::AlertaGafete;
use sqlx::SqlitePool;

// ==========================================
// DTO INTERMEDIO PARA MAPEO SEGURO
// ==========================================

#[derive(sqlx::FromRow)]
struct AlertaGafeteRow {
    id: Option<String>,
    persona_id: Option<String>,
    cedula: Option<String>,
    nombre_completo: Option<String>,
    gafete_numero: Option<String>,
    ingreso_contratista_id: Option<String>,
    ingreso_proveedor_id: Option<String>,
    ingreso_visita_id: Option<String>,
    fecha_reporte: Option<String>,
    resuelto: Option<bool>,
    fecha_resolucion: Option<String>,
    notas: Option<String>,
    reportado_por: Option<String>,
    resuelto_por: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

impl From<AlertaGafeteRow> for AlertaGafete {
    fn from(row: AlertaGafeteRow) -> Self {
        AlertaGafete {
            id: row.id.unwrap_or_default(),
            persona_id: row.persona_id,
            cedula: row.cedula.unwrap_or_default(),
            nombre_completo: row.nombre_completo.unwrap_or_default(),
            gafete_numero: row.gafete_numero.unwrap_or_default(),
            ingreso_contratista_id: row.ingreso_contratista_id,
            ingreso_proveedor_id: row.ingreso_proveedor_id,
            ingreso_visita_id: row.ingreso_visita_id,
            fecha_reporte: row.fecha_reporte.unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
            resuelto: row.resuelto.unwrap_or(false),
            fecha_resolucion: row.fecha_resolucion,
            notas: row.notas,
            reportado_por: row.reportado_por.unwrap_or_default(),
            resuelto_por: row.resuelto_por,
            created_at: row.created_at.unwrap_or_default(),
            updated_at: row.updated_at.unwrap_or_default(),
        }
    }
}

// ==========================================
// QUERIES DE LECTURA
// ==========================================

/// Busca una alerta por ID
pub async fn find_by_id(pool: &SqlitePool, id: &str) -> sqlx::Result<AlertaGafete> {
    let row = sqlx::query_as!(
        AlertaGafeteRow,
        r#"SELECT id, persona_id, cedula, nombre_completo, gafete_numero,
                ingreso_contratista_id, ingreso_proveedor_id, ingreso_visita_id,
                fecha_reporte, resuelto as "resuelto: bool", fecha_resolucion, notas, reportado_por, resuelto_por,
                created_at, updated_at
         FROM alertas_gafetes WHERE id = ?"#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(row.into())
}

/// Obtiene alertas pendientes de una cédula
pub async fn find_pendientes_by_cedula(
    pool: &SqlitePool,
    cedula: &str,
) -> sqlx::Result<Vec<AlertaGafete>> {
    let rows = sqlx::query_as!(
        AlertaGafeteRow,
        r#"SELECT id, persona_id, cedula, nombre_completo, gafete_numero,
                ingreso_contratista_id, ingreso_proveedor_id, ingreso_visita_id,
                fecha_reporte, resuelto as "resuelto: bool", fecha_resolucion, notas, reportado_por, resuelto_por,
                created_at, updated_at
         FROM alertas_gafetes WHERE cedula = ? AND resuelto = 0 ORDER BY fecha_reporte DESC"#,
        cedula
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(AlertaGafete::from).collect())
}

/// Obtiene todas las alertas (con filtro opcional de resuelto)
pub async fn find_all(
    pool: &SqlitePool,
    resuelto: Option<bool>,
) -> sqlx::Result<Vec<AlertaGafete>> {
    let rows = if let Some(resuelto_val) = resuelto {
        sqlx::query_as!(
            AlertaGafeteRow,
            r#"SELECT id, persona_id, cedula, nombre_completo, gafete_numero,
                    ingreso_contratista_id, ingreso_proveedor_id, ingreso_visita_id,
                    fecha_reporte, resuelto as "resuelto: bool", fecha_resolucion, notas, reportado_por, resuelto_por,
                    created_at, updated_at
             FROM alertas_gafetes WHERE resuelto = ? ORDER BY fecha_reporte DESC"#,
            resuelto_val
        )
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as!(
            AlertaGafeteRow,
            r#"SELECT id, persona_id, cedula, nombre_completo, gafete_numero,
                    ingreso_contratista_id, ingreso_proveedor_id, ingreso_visita_id,
                    fecha_reporte, resuelto as "resuelto: bool", fecha_resolucion, notas, reportado_por, resuelto_por,
                    created_at, updated_at
             FROM alertas_gafetes ORDER BY fecha_reporte DESC"#
        )
        .fetch_all(pool)
        .await?
    };

    Ok(rows.into_iter().map(AlertaGafete::from).collect())
}

// ==========================================
// QUERIES DE ESCRITURA
// ==========================================

/// Crea una nueva alerta de gafete
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
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"INSERT INTO alertas_gafetes 
           (id, persona_id, cedula, nombre_completo, gafete_numero, 
            ingreso_contratista_id, ingreso_proveedor_id, ingreso_visita_id,
            fecha_reporte, resuelto, fecha_resolucion, notas, reportado_por,
            created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 0, NULL, ?, ?, ?, ?)"#,
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
        updated_at
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Marca una alerta como resuelta
pub async fn resolver(
    pool: &SqlitePool,
    id: &str,
    fecha_resolucion: &str,
    notas: Option<&str>,
    usuario_id: &str, // ID del usuario que resuelve
    updated_at: &str,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"UPDATE alertas_gafetes SET
            resuelto = 1,
            fecha_resolucion = ?,
            notas = COALESCE(?, notas),
            resuelto_por = ?,
            updated_at = ?
        WHERE id = ?"#,
        fecha_resolucion,
        notas,
        usuario_id,
        updated_at,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Elimina una alerta (solo admin)
pub async fn delete(pool: &SqlitePool, id: &str) -> sqlx::Result<()> {
    sqlx::query!("DELETE FROM alertas_gafetes WHERE id = ?", id).execute(pool).await?;

    Ok(())
}
