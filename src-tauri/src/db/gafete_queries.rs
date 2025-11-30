// ==========================================
// src/db/gafete_queries.rs
// ==========================================
// Queries SQL puras - Sin lógica de negocio

use crate::models::gafete::{Gafete, TipoGafete};
use sqlx::{SqlitePool, Row};

// ==========================================
// QUERIES DE LECTURA
// ==========================================

/// Busca un gafete por número
pub async fn find_by_numero(pool: &SqlitePool, numero: &str) -> Result<Gafete, String> {
    let row = sqlx::query(
        "SELECT numero, tipo, created_at, updated_at FROM gafetes WHERE numero = ?"
    )
    .bind(numero)
    .fetch_one(pool)
    .await
    .map_err(|_| format!("Gafete {} no encontrado", numero))?;
    
    Ok(Gafete {
        numero: row.get("numero"),
        tipo: TipoGafete::from_str(row.get("tipo"))?,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

/// Obtiene todos los gafetes
pub async fn find_all(pool: &SqlitePool) -> Result<Vec<Gafete>, String> {
    let rows = sqlx::query(
        "SELECT numero, tipo, created_at, updated_at FROM gafetes ORDER BY numero"
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
        "SELECT numero, tipo, created_at, updated_at FROM gafetes WHERE tipo = ? ORDER BY numero"
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

/// Verifica si un gafete está en uso (tiene ingreso activo)
pub async fn is_en_uso(pool: &SqlitePool, numero: &str) -> Result<bool, String> {
    let row = sqlx::query(
        "SELECT COUNT(*) as count FROM ingresos 
         WHERE gafete_numero = ? AND fecha_hora_salida IS NULL"
    )
    .bind(numero)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error al verificar disponibilidad: {}", e))?;
    
    let count: i32 = row.get("count");
    Ok(count > 0)
}

/// Obtiene números de gafetes disponibles de un tipo
pub async fn find_disponibles_by_tipo(pool: &SqlitePool, tipo: &str) -> Result<Vec<String>, String> {
    let rows = sqlx::query(
        "SELECT g.numero FROM gafetes g
         LEFT JOIN ingresos i ON g.numero = i.gafete_numero AND i.fecha_salida IS NULL
         WHERE g.tipo = ? AND i.id IS NULL AND g.numero != 'S/G'
         ORDER BY g.numero"
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
    sqlx::query(
        "INSERT INTO gafetes (numero, tipo, created_at, updated_at) VALUES (?, ?, ?, ?)"
    )
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
    sqlx::query(
        "UPDATE gafetes SET tipo = COALESCE(?, tipo), updated_at = ? WHERE numero = ?"
    )
    .bind(tipo)
    .bind(updated_at)
    .bind(numero)
    .execute(pool)
    .await
    .map_err(|e| format!("Error al actualizar gafete: {}", e))?;
    
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