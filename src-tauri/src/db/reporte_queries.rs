// ==========================================
// src/db/reporte_queries.rs
// ==========================================
// Capa de acceso a datos para reportes

use crate::models::reporte::{EstadoReporte, Reporte, TipoReporte};
use sqlx::{Row, SqlitePool};

// ==========================================
// INSERTAR REPORTE
// ==========================================

#[allow(clippy::too_many_arguments)]
pub async fn insert(
    pool: &SqlitePool,
    id: &str,
    tipo: &str,
    asunto: &str,
    mensaje: &str,
    contacto: Option<&str>,
    tiene_adjunto: bool,
    nombre_adjunto: Option<&str>,
    estado: &str,
    created_at: &str,
    updated_at: &str,
) -> Result<(), String> {
    sqlx::query(
        r#"INSERT INTO reportes
           (id, tipo, asunto, mensaje, contacto, tiene_adjunto, nombre_adjunto, estado, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(id)
    .bind(tipo)
    .bind(asunto)
    .bind(mensaje)
    .bind(contacto)
    .bind(tiene_adjunto as i32)
    .bind(nombre_adjunto)
    .bind(estado)
    .bind(created_at)
    .bind(updated_at)
    .execute(pool)
    .await
    .map_err(|e| format!("Error al insertar reporte: {}", e))?;

    Ok(())
}

// ==========================================
// BUSCAR POR ID
// ==========================================

pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<Reporte, String> {
    let row = sqlx::query("SELECT * FROM reportes WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Error buscando reporte: {}", e))?;

    if let Some(row) = row {
        row_to_reporte(&row)
    } else {
        Err("Reporte no encontrado".to_string())
    }
}

// ==========================================
// BUSCAR TODOS
// ==========================================

pub async fn find_all(pool: &SqlitePool) -> Result<Vec<Reporte>, String> {
    let rows = sqlx::query("SELECT * FROM reportes ORDER BY created_at DESC")
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Error buscando reportes: {}", e))?;

    let mut reportes = Vec::new();
    for row in rows {
        reportes.push(row_to_reporte(&row)?);
    }
    Ok(reportes)
}

// ==========================================
// BUSCAR POR TIPO
// ==========================================

pub async fn find_by_tipo(pool: &SqlitePool, tipo: &str) -> Result<Vec<Reporte>, String> {
    let rows = sqlx::query("SELECT * FROM reportes WHERE tipo = ? ORDER BY created_at DESC")
        .bind(tipo)
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Error buscando reportes por tipo: {}", e))?;

    let mut reportes = Vec::new();
    for row in rows {
        reportes.push(row_to_reporte(&row)?);
    }
    Ok(reportes)
}

// ==========================================
// BUSCAR POR ESTADO
// ==========================================

pub async fn find_by_estado(pool: &SqlitePool, estado: &str) -> Result<Vec<Reporte>, String> {
    let rows = sqlx::query("SELECT * FROM reportes WHERE estado = ? ORDER BY created_at DESC")
        .bind(estado)
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Error buscando reportes por estado: {}", e))?;

    let mut reportes = Vec::new();
    for row in rows {
        reportes.push(row_to_reporte(&row)?);
    }
    Ok(reportes)
}

// ==========================================
// ACTUALIZAR ESTADO
// ==========================================

pub async fn update_estado(
    pool: &SqlitePool,
    id: &str,
    estado: &str,
    error_envio: Option<&str>,
    enviado_at: Option<&str>,
    updated_at: &str,
) -> Result<(), String> {
    sqlx::query(
        "UPDATE reportes SET estado = ?, error_envio = ?, enviado_at = ?, updated_at = ? WHERE id = ?",
    )
    .bind(estado)
    .bind(error_envio)
    .bind(enviado_at)
    .bind(updated_at)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| format!("Error al actualizar estado del reporte: {}", e))?;

    Ok(())
}

// ==========================================
// CONTAR POR ESTADO
// ==========================================

pub async fn count_by_estado(pool: &SqlitePool, estado: &str) -> Result<i64, String> {
    let row = sqlx::query("SELECT COUNT(*) as count FROM reportes WHERE estado = ?")
        .bind(estado)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Error contando reportes: {}", e))?;

    Ok(row.get("count"))
}

// ==========================================
// HELPER: ROW TO REPORTE
// ==========================================

fn row_to_reporte(row: &sqlx::sqlite::SqliteRow) -> Result<Reporte, String> {
    let tipo_str: String = row.get("tipo");
    let estado_str: String = row.get("estado");
    let tiene_adjunto_int: i32 = row.get("tiene_adjunto");

    let tipo = TipoReporte::from_str(&tipo_str).unwrap_or(TipoReporte::Sugerencia);
    let estado = EstadoReporte::from_str(&estado_str).unwrap_or(EstadoReporte::Pendiente);

    Ok(Reporte {
        id: row.get("id"),
        tipo,
        asunto: row.get("asunto"),
        mensaje: row.get("mensaje"),
        contacto: row.get("contacto"),
        tiene_adjunto: tiene_adjunto_int == 1,
        nombre_adjunto: row.get("nombre_adjunto"),
        estado,
        error_envio: row.get("error_envio"),
        enviado_at: row.get("enviado_at"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}
