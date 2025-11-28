// ==========================================
// src/db/empresa_queries.rs
// ==========================================

use crate::models::empresa::Empresa;
use sqlx::{Row, SqlitePool};

// ==========================================
// QUERIES DE LECTURA
// ==========================================

pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<Empresa, String> {
    let row = sqlx::query(
        "SELECT id, nombre, is_active, created_at, updated_at 
         FROM empresas WHERE id = ?"
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|_| "Empresa no encontrada".to_string())?;

    Ok(row_to_empresa(row))
}

pub async fn exists(pool: &SqlitePool, id: &str) -> Result<bool, String> {
    let row = sqlx::query("SELECT COUNT(*) as count FROM empresas WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Error verificando empresa: {}", e))?;

    let count: i64 = row.get("count");
    Ok(count > 0)
}

pub async fn find_all(pool: &SqlitePool) -> Result<Vec<Empresa>, String> {
    let rows = sqlx::query(
        "SELECT id, nombre, is_active, created_at, updated_at 
         FROM empresas ORDER BY nombre ASC"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error al obtener empresas: {}", e))?;

    Ok(rows.into_iter().map(row_to_empresa).collect())
}

pub async fn find_activas(pool: &SqlitePool) -> Result<Vec<Empresa>, String> {
    let rows = sqlx::query(
        "SELECT id, nombre, is_active, created_at, updated_at 
         FROM empresas WHERE is_active = 1 ORDER BY nombre ASC"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error al obtener empresas activas: {}", e))?;

    Ok(rows.into_iter().map(row_to_empresa).collect())
}

pub async fn count_by_nombre(pool: &SqlitePool, nombre: &str) -> Result<i32, String> {
    let row = sqlx::query("SELECT COUNT(*) as count FROM empresas WHERE nombre = ?")
        .bind(nombre)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Error al contar empresas: {}", e))?;

    Ok(row.get("count"))
}

pub async fn count_by_nombre_excluding_id(
    pool: &SqlitePool,
    nombre: &str,
    id: &str,
) -> Result<i32, String> {
    let row = sqlx::query("SELECT COUNT(*) as count FROM empresas WHERE nombre = ? AND id != ?")
        .bind(nombre)
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Error al verificar nombre: {}", e))?;

    Ok(row.get("count"))
}

pub async fn count_contratistas(pool: &SqlitePool, empresa_id: &str) -> Result<i32, String> {
    let row = sqlx::query("SELECT COUNT(*) as count FROM contratistas WHERE empresa_id = ?")
        .bind(empresa_id)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Error al contar contratistas: {}", e))?;

    Ok(row.get("count"))
}

// ==========================================
// QUERIES DE ESCRITURA
// ==========================================

pub async fn insert(
    pool: &SqlitePool,
    id: &str,
    nombre: &str,
    created_at: &str,
    updated_at: &str,
) -> Result<(), String> {
    sqlx::query(
        "INSERT INTO empresas (id, nombre, is_active, created_at, updated_at) 
         VALUES (?, ?, 1, ?, ?)"
    )
    .bind(id)
    .bind(nombre)
    .bind(created_at)
    .bind(updated_at)
    .execute(pool)
    .await
    .map_err(|e| format!("Error al crear empresa: {}", e))?;

    Ok(())
}

pub async fn update(
    pool: &SqlitePool,
    id: &str,
    nombre: Option<&str>,
    is_active: Option<bool>,
    updated_at: &str,
) -> Result<(), String> {
    let is_active_int = is_active.map(|b| if b { 1 } else { 0 });

    sqlx::query(
        "UPDATE empresas SET 
            nombre = COALESCE(?, nombre),
            is_active = COALESCE(?, is_active),
            updated_at = ?
         WHERE id = ?"
    )
    .bind(nombre)
    .bind(is_active_int)
    .bind(updated_at)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| format!("Error al actualizar empresa: {}", e))?;

    Ok(())
}

pub async fn delete(pool: &SqlitePool, id: &str) -> Result<(), String> {
    sqlx::query("DELETE FROM empresas WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| format!("Error al eliminar empresa: {}", e))?;

    Ok(())
}

// ==========================================
// HELPERS
// ==========================================

fn row_to_empresa(row: sqlx::sqlite::SqliteRow) -> Empresa {
    Empresa {
        id: row.get("id"),
        nombre: row.get("nombre"),
        is_active: row.get::<i32, _>("is_active") != 0,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}