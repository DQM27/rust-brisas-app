// ==========================================
// src/db/lista_negra_queries.rs
// ==========================================
// Queries de base de datos para Lista Negra

use crate::models::lista_negra::ListaNegra;
use sqlx::SqlitePool;

// ==========================================
// TIPOS AUXILIARES
// ==========================================

#[derive(Debug)]
pub struct BlockStatus {
    pub blocked: bool,
    pub nivel_severidad: Option<String>,
    pub bloqueado_desde: Option<String>,
}

// ==========================================
// DTO & CONVERSION
// ==========================================

#[derive(sqlx::FromRow)]
struct ListaNegraRow {
    id: String,
    cedula: Option<String>,
    nombre: Option<String>,
    segundo_nombre: Option<String>,
    apellido: Option<String>,
    segundo_apellido: Option<String>,
    empresa_id: Option<String>,
    empresa_nombre: Option<String>,
    nivel_severidad: Option<String>,
    motivo_bloqueo: Option<String>,
    bloqueado_por: Option<String>,
    observaciones: Option<String>,
    is_active: i64,
    created_at: Option<String>,
    updated_at: Option<String>,
}

impl From<ListaNegraRow> for ListaNegra {
    fn from(row: ListaNegraRow) -> Self {
        ListaNegra {
            id: row.id,
            cedula: row.cedula.unwrap_or_default(),
            nombre: row.nombre.unwrap_or_default(),
            segundo_nombre: row.segundo_nombre,
            apellido: row.apellido.unwrap_or_default(),
            segundo_apellido: row.segundo_apellido,
            empresa_id: row.empresa_id,
            empresa_nombre: row.empresa_nombre,
            nivel_severidad: row.nivel_severidad.unwrap_or_else(|| "BAJO".to_string()),
            motivo_bloqueo: row.motivo_bloqueo.unwrap_or_default(),
            bloqueado_por: row.bloqueado_por.unwrap_or_default(),
            observaciones: row.observaciones,
            is_active: row.is_active != 0,
            created_at: row.created_at.unwrap_or_default(),
            updated_at: row.updated_at.unwrap_or_default(),
        }
    }
}

// ==========================================
// QUERIES DE LECTURA
// ==========================================

/// Verifica si una cédula está bloqueada (para guardias - info mínima)
pub async fn check_if_blocked_by_cedula(
    pool: &SqlitePool,
    cedula: &str,
) -> sqlx::Result<BlockStatus> {
    let row = sqlx::query!(
        r#"SELECT nivel_severidad, created_at 
         FROM lista_negra 
         WHERE cedula = ? AND is_active = 1
         LIMIT 1"#,
        cedula
    )
    .fetch_optional(pool)
    .await?;

    match row {
        Some(r) => Ok(BlockStatus {
            blocked: true,
            nivel_severidad: Some(r.nivel_severidad),
            bloqueado_desde: Some(r.created_at),
        }),
        None => Ok(BlockStatus { blocked: false, nivel_severidad: None, bloqueado_desde: None }),
    }
}

/// Cuenta bloqueos activos por cédula
pub async fn count_active_by_cedula(pool: &SqlitePool, cedula: &str) -> sqlx::Result<i64> {
    let row = sqlx::query!(
        "SELECT COUNT(*) as count FROM lista_negra WHERE cedula = ? AND is_active = 1",
        cedula
    )
    .fetch_one(pool)
    .await?;

    Ok(row.count as i64)
}

/// Busca por ID
pub async fn find_by_id(pool: &SqlitePool, id: &str) -> sqlx::Result<ListaNegra> {
    let row = sqlx::query_as!(ListaNegraRow, "SELECT * FROM lista_negra WHERE id = ?", id)
        .fetch_one(pool)
        .await?;

    Ok(row.into())
}

/// Busca todos los registros (historial completo)
pub async fn find_all(pool: &SqlitePool) -> sqlx::Result<Vec<ListaNegra>> {
    let rows = sqlx::query_as!(ListaNegraRow, "SELECT * FROM lista_negra ORDER BY created_at DESC")
        .fetch_all(pool)
        .await?;

    Ok(rows.into_iter().map(ListaNegra::from).collect())
}

/// Busca solo registros activos (bloqueados actualmente)
pub async fn find_activos(pool: &SqlitePool) -> sqlx::Result<Vec<ListaNegra>> {
    let rows = sqlx::query_as!(
        ListaNegraRow,
        "SELECT * FROM lista_negra WHERE is_active = 1 ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(ListaNegra::from).collect())
}

/// Busca bloqueo activo por cédula (para validaciones)
pub async fn find_active_by_cedula(
    pool: &SqlitePool,
    cedula: &str,
) -> sqlx::Result<Option<ListaNegra>> {
    let row = sqlx::query_as!(
        ListaNegraRow,
        "SELECT * FROM lista_negra WHERE cedula = ? AND is_active = 1",
        cedula
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(ListaNegra::from))
}

/// Busca por nivel de severidad
pub async fn find_by_nivel(pool: &SqlitePool, nivel: &str) -> sqlx::Result<Vec<ListaNegra>> {
    let rows = sqlx::query_as!(
        ListaNegraRow,
        "SELECT * FROM lista_negra WHERE nivel_severidad = ? AND is_active = 1 ORDER BY created_at DESC",
        nivel
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(ListaNegra::from).collect())
}

// ==========================================
// QUERIES DE ESCRITURA
// ==========================================

/// Inserta un nuevo bloqueo
#[allow(clippy::too_many_arguments)]
pub async fn insert(
    pool: &SqlitePool,
    id: &str,
    cedula: &str,
    nombre: &str,
    segundo_nombre: Option<&str>,
    apellido: &str,
    segundo_apellido: Option<&str>,
    empresa_id: Option<&str>,
    empresa_nombre: Option<&str>,
    nivel_severidad: &str,
    motivo_bloqueo: &str,
    bloqueado_por: &str,
    observaciones: Option<&str>,
    created_at: &str,
    updated_at: &str,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"INSERT INTO lista_negra 
           (id, cedula, nombre, segundo_nombre, apellido, segundo_apellido, 
            empresa_id, empresa_nombre, nivel_severidad, motivo_bloqueo, 
            bloqueado_por, observaciones, is_active, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 1, ?, ?)"#,
        id,
        cedula,
        nombre,
        segundo_nombre,
        apellido,
        segundo_apellido,
        empresa_id,
        empresa_nombre,
        nivel_severidad,
        motivo_bloqueo,
        bloqueado_por,
        observaciones,
        created_at,
        updated_at
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Desactiva un bloqueo (no borra, para mantener historial)
pub async fn deactivate(pool: &SqlitePool, id: &str, updated_at: &str) -> sqlx::Result<()> {
    sqlx::query!(
        r#"UPDATE lista_negra SET 
           is_active = 0, 
           updated_at = ? 
           WHERE id = ?"#,
        updated_at,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Reactiva un bloqueo existente
pub async fn reactivate(
    pool: &SqlitePool,
    id: &str,
    nivel_severidad: &str,
    motivo: &str,
    bloqueado_por: &str,
    updated_at: &str,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"UPDATE lista_negra SET 
           is_active = 1, 
           nivel_severidad = ?,
           motivo_bloqueo = ?, 
           bloqueado_por = ?,
           updated_at = ? 
           WHERE id = ?"#,
        nivel_severidad,
        motivo,
        bloqueado_por,
        updated_at,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Actualiza un bloqueo
pub async fn update(
    pool: &SqlitePool,
    id: &str,
    nivel_severidad: Option<&str>,
    motivo: Option<&str>,
    observaciones: Option<&str>,
    updated_at: &str,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"UPDATE lista_negra SET 
           nivel_severidad = COALESCE(?, nivel_severidad),
           motivo_bloqueo = COALESCE(?, motivo_bloqueo),
           observaciones = COALESCE(?, observaciones),
           updated_at = ?
           WHERE id = ?"#,
        nivel_severidad,
        motivo,
        observaciones,
        updated_at,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Elimina un registro permanentemente (usar con precaución)
pub async fn delete(pool: &SqlitePool, id: &str) -> sqlx::Result<()> {
    sqlx::query!("DELETE FROM lista_negra WHERE id = ?", id).execute(pool).await?;

    Ok(())
}
