// ==========================================
// src/db/empresa_queries.rs
// ==========================================

use crate::models::empresa::Empresa;
use sqlx::SqlitePool;

// ==========================================
// QUERIES DE LECTURA
// ==========================================

/// Busca empresa por ID
pub async fn find_by_id(pool: &SqlitePool, id: &str) -> sqlx::Result<Option<Empresa>> {
    sqlx::query_as!(
        Empresa,
        r#"
        SELECT 
            id, 
            nombre, 
            is_active as "is_active: bool", 
            created_at, 
            updated_at
        FROM empresas 
        WHERE id = ?
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

/// Verifica si existe una empresa por ID
pub async fn exists(pool: &SqlitePool, id: &str) -> sqlx::Result<bool> {
    let row = sqlx::query!("SELECT COUNT(*) as count FROM empresas WHERE id = ?", id)
        .fetch_one(pool)
        .await?;

    Ok(row.count > 0)
}

/// Obtiene todas las empresas ordenadas por nombre
pub async fn find_all(pool: &SqlitePool) -> sqlx::Result<Vec<Empresa>> {
    sqlx::query_as!(
        Empresa,
        r#"
        SELECT 
            id, 
            nombre, 
            is_active as "is_active: bool", 
            created_at, 
            updated_at 
        FROM empresas 
        ORDER BY nombre ASC
        "#
    )
    .fetch_all(pool)
    .await
}

/// Obtiene empresas activas ordenadas por nombre
pub async fn find_activas(pool: &SqlitePool) -> sqlx::Result<Vec<Empresa>> {
    sqlx::query_as!(
        Empresa,
        r#"
        SELECT 
            id, 
            nombre, 
            is_active as "is_active: bool", 
            created_at, 
            updated_at 
        FROM empresas 
        WHERE is_active = 1 
        ORDER BY nombre ASC
        "#
    )
    .fetch_all(pool)
    .await
}

/// Cuenta empresas con un nombre específico (para validación de duplicados)
pub async fn count_by_nombre(pool: &SqlitePool, nombre: &str) -> sqlx::Result<i64> {
    let row = sqlx::query!("SELECT COUNT(*) as count FROM empresas WHERE nombre = ?", nombre)
        .fetch_one(pool)
        .await?;

    Ok(row.count as i64)
}

/// Cuenta empresas con un nombre específico excluyendo un ID (para update)
pub async fn count_by_nombre_excluding_id(
    pool: &SqlitePool,
    nombre: &str,
    id: &str,
) -> sqlx::Result<i64> {
    let row = sqlx::query!(
        "SELECT COUNT(*) as count FROM empresas WHERE nombre = ? AND id != ?",
        nombre,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(row.count as i64)
}

/// Cuenta contratistas asociados a una empresa
pub async fn count_contratistas(pool: &SqlitePool, empresa_id: &str) -> sqlx::Result<i64> {
    let row =
        sqlx::query!("SELECT COUNT(*) as count FROM contratistas WHERE empresa_id = ?", empresa_id)
            .fetch_one(pool)
            .await?;

    Ok(row.count as i64)
}

// ==========================================
// QUERIES DE ESCRITURA
// ==========================================

/// Inserta una nueva empresa
pub async fn insert(
    pool: &SqlitePool,
    id: &str,
    nombre: &str,
    created_at: &str,
    updated_at: &str,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO empresas (id, nombre, is_active, created_at, updated_at) 
        VALUES (?, ?, 1, ?, ?)
        "#,
        id,
        nombre,
        created_at,
        updated_at
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Actualiza una empresa
pub async fn update(
    pool: &SqlitePool,
    id: &str,
    nombre: Option<&str>,
    is_active: Option<bool>,
    updated_at: &str,
) -> sqlx::Result<()> {
    // SQLx maneja la conversión de Option<bool> a INTEGER en SQLite si se usa query! correctamente?
    // SQLite no tiene tipo bool nativo (usa 0/1). SQLx macro suele manejar bool -> integer automáticamente.
    // Si falla, se puede castear manualmente, pero probemos directo primero como en user_queries.

    // Nota: Aunque user_queries pasa Option<bool>, es porque la columna destino es "INTEGER" o similar.
    sqlx::query!(
        r#"
        UPDATE empresas SET 
            nombre = COALESCE(?, nombre),
            is_active = COALESCE(?, is_active),
            updated_at = ?
        WHERE id = ?
        "#,
        nombre,
        is_active,
        updated_at,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Elimina una empresa
pub async fn delete(pool: &SqlitePool, id: &str) -> sqlx::Result<()> {
    sqlx::query!("DELETE FROM empresas WHERE id = ?", id).execute(pool).await?;

    Ok(())
}
