// ==========================================
// src/db/alerta_gafete_queries.rs
// ==========================================
// Queries SQL puras para alertas de gafetes - Sin lógica de negocio

use crate::models::ingreso::AlertaGafete;
use sqlx::{Row, SqlitePool};

// ==========================================
// QUERIES DE LECTURA
// ==========================================

/// Busca una alerta por ID
pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<AlertaGafete, String> {
    let row = sqlx::query(
        "SELECT id, persona_id, cedula, nombre_completo, gafete_numero,
                ingreso_contratista_id, ingreso_proveedor_id,
                fecha_reporte, resuelto, fecha_resolucion, notas, reportado_por,
                created_at, updated_at
         FROM alertas_gafetes WHERE id = ?",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|_| "Alerta no encontrada".to_string())?;

    Ok(row_to_alerta(row))
}

/// Obtiene alertas pendientes de una cédula
pub async fn find_pendientes_by_cedula(
    pool: &SqlitePool,
    cedula: &str,
) -> Result<Vec<AlertaGafete>, String> {
    let rows = sqlx::query(
        "SELECT id, persona_id, cedula, nombre_completo, gafete_numero,
                ingreso_contratista_id, ingreso_proveedor_id,
                fecha_reporte, resuelto, fecha_resolucion, notas, reportado_por,
                created_at, updated_at
         FROM alertas_gafetes WHERE cedula = ? AND resuelto = 0 ORDER BY fecha_reporte DESC",
    )
    .bind(cedula)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error al obtener alertas: {}", e))?;

    Ok(rows.into_iter().map(row_to_alerta).collect())
}

/// Obtiene todas las alertas (con filtro opcional de resuelto)
pub async fn find_all(
    pool: &SqlitePool,
    resuelto: Option<bool>,
) -> Result<Vec<AlertaGafete>, String> {
    let rows = if let Some(resuelto_val) = resuelto {
        let resuelto_int = if resuelto_val { 1 } else { 0 };
        sqlx::query(
            "SELECT id, persona_id, cedula, nombre_completo, gafete_numero,
                    ingreso_contratista_id, ingreso_proveedor_id,
                    fecha_reporte, resuelto, fecha_resolucion, notas, reportado_por,
                    created_at, updated_at
             FROM alertas_gafetes WHERE resuelto = ? ORDER BY fecha_reporte DESC",
        )
        .bind(resuelto_int)
        .fetch_all(pool)
        .await
    } else {
        sqlx::query(
            "SELECT id, persona_id, cedula, nombre_completo, gafete_numero,
                    ingreso_contratista_id, ingreso_proveedor_id,
                    fecha_reporte, resuelto, fecha_resolucion, notas, reportado_por,
                    created_at, updated_at
             FROM alertas_gafetes ORDER BY fecha_reporte DESC",
        )
        .fetch_all(pool)
        .await
    };

    let rows = rows.map_err(|e| format!("Error al obtener alertas: {}", e))?;
    Ok(rows.into_iter().map(row_to_alerta).collect())
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
    fecha_reporte: &str,
    notas: Option<&str>,
    reportado_por: &str,
    created_at: &str,
    updated_at: &str,
) -> Result<(), String> {
    sqlx::query(
        r#"INSERT INTO alertas_gafetes 
           (id, persona_id, cedula, nombre_completo, gafete_numero, 
            ingreso_contratista_id, ingreso_proveedor_id,
            fecha_reporte, resuelto, fecha_resolucion, notas, reportado_por,
            created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, 0, NULL, ?, ?, ?, ?)"#,
    )
    .bind(id)
    .bind(persona_id)
    .bind(cedula)
    .bind(nombre_completo)
    .bind(gafete_numero)
    .bind(ingreso_contratista_id)
    .bind(ingreso_proveedor_id)
    .bind(fecha_reporte)
    .bind(notas)
    .bind(reportado_por)
    .bind(created_at)
    .bind(updated_at)
    .execute(pool)
    .await
    .map_err(|e| format!("Error al crear alerta: {}", e))?;

    Ok(())
}

/// Marca una alerta como resuelta
pub async fn resolver(
    pool: &SqlitePool,
    id: &str,
    fecha_resolucion: &str,
    notas: Option<&str>,
    updated_at: &str,
) -> Result<(), String> {
    sqlx::query(
        r#"UPDATE alertas_gafetes SET
            resuelto = 1,
            fecha_resolucion = ?,
            notas = COALESCE(?, notas),
            updated_at = ?
        WHERE id = ?"#,
    )
    .bind(fecha_resolucion)
    .bind(notas)
    .bind(updated_at)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| format!("Error al resolver alerta: {}", e))?;

    Ok(())
}

/// Elimina una alerta (solo admin)
pub async fn delete(pool: &SqlitePool, id: &str) -> Result<(), String> {
    sqlx::query("DELETE FROM alertas_gafetes WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| format!("Error al eliminar alerta: {}", e))?;

    Ok(())
}

// ==========================================
// HELPERS INTERNOS
// ==========================================

fn row_to_alerta(row: sqlx::sqlite::SqliteRow) -> AlertaGafete {
    let resuelto_int: i32 = row.get("resuelto");

    AlertaGafete {
        id: row.get("id"),
        persona_id: row.get("persona_id"),
        cedula: row.get("cedula"),
        nombre_completo: row.get("nombre_completo"),
        gafete_numero: row.get("gafete_numero"),
        ingreso_contratista_id: row.get("ingreso_contratista_id"),
        ingreso_proveedor_id: row.get("ingreso_proveedor_id"),
        fecha_reporte: row.get("fecha_reporte"),
        resuelto: resuelto_int != 0,
        fecha_resolucion: row.get("fecha_resolucion"),
        notas: row.get("notas"),
        reportado_por: row.get("reportado_por"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}
