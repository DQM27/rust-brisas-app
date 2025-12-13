// ==========================================
// src/db/gafete_queries.rs
// ==========================================
// Queries SQL puras - Sin lógica de negocio

use crate::models::gafete::{Gafete, GafeteEstado, TipoGafete};
use sqlx::{Row, SqlitePool};

// ==========================================
// QUERIES DE LECTURA
// ==========================================

/// Busca un gafete por número
pub async fn find_by_numero(pool: &SqlitePool, numero: &str) -> Result<Gafete, String> {
    let row = sqlx::query(
        "SELECT numero, tipo, estado, created_at, updated_at FROM gafetes WHERE numero = ?",
    )
    .bind(numero)
    .fetch_one(pool)
    .await
    .map_err(|_| format!("Gafete {} no encontrado", numero))?;

    Ok(Gafete {
        numero: row.get("numero"),
        tipo: TipoGafete::from_str(row.get("tipo"))?,
        estado: GafeteEstado::from_str(row.get("estado"))?,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

/// Obtiene todos los gafetes
pub async fn find_all(pool: &SqlitePool) -> Result<Vec<Gafete>, String> {
    let rows = sqlx::query(
        "SELECT numero, tipo, estado, created_at, updated_at FROM gafetes ORDER BY numero",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error al obtener gafetes: {}", e))?;

    let gafetes: Vec<Gafete> = rows
        .into_iter()
        .filter_map(|row| {
            Some(Gafete {
                numero: row.get("numero"),
                tipo: TipoGafete::from_str(row.get("tipo")).ok()?,
                estado: GafeteEstado::from_str(row.get("estado")).ok()?,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
        })
        .collect();

    Ok(gafetes)
}

/// Busca gafetes de un tipo específico
pub async fn find_by_tipo(pool: &SqlitePool, tipo: &str) -> Result<Vec<Gafete>, String> {
    let rows = sqlx::query(
        "SELECT numero, tipo, estado, created_at, updated_at FROM gafetes WHERE tipo = ? ORDER BY numero",
    )
    .bind(tipo)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error al obtener gafetes: {}", e))?;

    let gafetes: Vec<Gafete> = rows
        .into_iter()
        .filter_map(|row| {
            Some(Gafete {
                numero: row.get("numero"),
                tipo: TipoGafete::from_str(row.get("tipo")).ok()?,
                estado: GafeteEstado::from_str(row.get("estado")).ok()?,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
        })
        .collect();

    Ok(gafetes)
}

/// Cuenta gafetes por número (para verificar unicidad)
pub async fn count_by_numero(pool: &SqlitePool, numero: &str) -> Result<i32, String> {
    let row = sqlx::query("SELECT COUNT(*) as count FROM gafetes WHERE numero = ?")
        .bind(numero)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Error al verificar número: {}", e))?;

    Ok(row.get("count"))
}

/// Verifica si ya existe un gafete con ese número + tipo (nueva regla de unicidad)
pub async fn exists_by_numero_and_tipo(
    pool: &SqlitePool,
    numero: &str,
    tipo: &str,
) -> Result<bool, String> {
    let row = sqlx::query("SELECT 1 FROM gafetes WHERE numero = ? AND tipo = ? LIMIT 1")
        .bind(numero)
        .bind(tipo)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Error al verificar número y tipo: {}", e))?;

    Ok(row.is_some())
}

/// Verifica si un gafete está en uso (tiene ingreso activo)
pub async fn is_en_uso(pool: &SqlitePool, numero: &str) -> Result<bool, String> {
    let row = sqlx::query(
        "SELECT COUNT(*) as count FROM ingresos 
         WHERE gafete_numero = ? AND fecha_hora_salida IS NULL",
    )
    .bind(numero)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error al verificar uso: {}", e))?;

    let count: i32 = row.get("count");
    Ok(count > 0)
}

/// Verifica si un gafete tiene una alerta pendiente (no resuelta)
pub async fn has_unresolved_alert(pool: &SqlitePool, numero: &str) -> Result<bool, String> {
    let row = sqlx::query(
        "SELECT COUNT(*) as count FROM alertas_gafetes 
         WHERE gafete_numero = ? AND resuelto = 0",
    )
    .bind(numero)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error al verificar alertas: {}", e))?;

    let count: i32 = row.get("count");
    Ok(count > 0)
}

/// Obtiene números de gafetes disponibles de un tipo
/// Solo devuelve gafetes 'activos' (no dañados ni extraviados)
pub async fn find_disponibles_by_tipo(
    pool: &SqlitePool,
    tipo: &str,
) -> Result<Vec<String>, String> {
    let rows = sqlx::query(
        "SELECT g.numero FROM gafetes g
         LEFT JOIN ingresos i ON g.numero = i.gafete_numero AND i.fecha_hora_salida IS NULL
         LEFT JOIN alertas_gafetes a ON g.numero = a.gafete_numero AND a.resuelto = 0
         WHERE g.tipo = ? AND g.estado = 'activo'
         AND i.id IS NULL AND a.id IS NULL AND g.numero != 'S/G'
         ORDER BY g.numero",
    )
    .bind(tipo)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error al obtener disponibles: {}", e))?;

    Ok(rows.into_iter().map(|row| row.get("numero")).collect())
}

// ==========================================
// QUERIES DE ESCRITURA
// ==========================================

/// Inserta un nuevo gafete
pub async fn insert(
    pool: &SqlitePool,
    numero: &str,
    tipo: &str,
    created_at: &str,
    updated_at: &str,
) -> Result<(), String> {
    sqlx::query("INSERT INTO gafetes (numero, tipo, estado, created_at, updated_at) VALUES (?, ?, 'activo', ?, ?)")
        .bind(numero)
        .bind(tipo)
        .bind(created_at)
        .bind(updated_at)
        .execute(pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                format!("Ya existe un gafete con el número {}", numero)
            } else {
                format!("Error al crear gafete: {}", e)
            }
        })?;

    Ok(())
}

/// Actualiza el tipo de un gafete
pub async fn update(
    pool: &SqlitePool,
    numero: &str,
    tipo: Option<&str>,
    updated_at: &str,
) -> Result<(), String> {
    sqlx::query("UPDATE gafetes SET tipo = COALESCE(?, tipo), updated_at = ? WHERE numero = ?")
        .bind(tipo)
        .bind(updated_at)
        .bind(numero)
        .execute(pool)
        .await
        .map_err(|e| format!("Error al actualizar gafete: {}", e))?;

    Ok(())
}

/// Actualiza el estado de un gafete
pub async fn update_status(
    pool: &SqlitePool,
    numero: &str,
    estado: &str,
    updated_at: &str,
) -> Result<(), String> {
    sqlx::query("UPDATE gafetes SET estado = ?, updated_at = ? WHERE numero = ?")
        .bind(estado)
        .bind(updated_at)
        .bind(numero)
        .execute(pool)
        .await
        .map_err(|e| format!("Error al actualizar estado del gafete: {}", e))?;

    Ok(())
}

/// Elimina un gafete
pub async fn delete(pool: &SqlitePool, numero: &str) -> Result<(), String> {
    sqlx::query("DELETE FROM gafetes WHERE numero = ?")
        .bind(numero)
        .execute(pool)
        .await
        .map_err(|e| format!("Error al eliminar gafete: {}", e))?;

    Ok(())
}

/// Obtiene la alerta más reciente de un gafete (si existe)
pub async fn get_recent_alert_for_gafete(
    pool: &SqlitePool,
    numero: &str,
) -> Result<Option<(String, String, String, bool)>, String> {
    let row = sqlx::query(
        "SELECT id, fecha_reporte, nombre_completo, resuelto 
         FROM alertas_gafetes 
         WHERE gafete_numero = ? 
         ORDER BY fecha_reporte DESC 
         LIMIT 1",
    )
    .bind(numero)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Error al obtener alerta: {}", e))?;

    Ok(row.map(|r| {
        let resuelto_int: i32 = r.get("resuelto");
        (
            r.get("id"),
            r.get("fecha_reporte"),
            r.get("nombre_completo"),
            resuelto_int != 0,
        )
    }))
}
