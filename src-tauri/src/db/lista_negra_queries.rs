// ==========================================
// src/db/lista_negra_queries.rs
// ==========================================

use crate::models::lista_negra::ListaNegra;
use sqlx::SqlitePool;

#[derive(Debug)]
pub struct BlockStatus {
    pub blocked: bool,
    pub motivo: Option<String>,
}

// ==========================================
// DTO & CONVERSION
// ==========================================

#[derive(sqlx::FromRow)]
struct ListaNegraRow {
    id: String,
    contratista_id: Option<String>,
    cedula: Option<String>,
    nombre: Option<String>,
    segundo_nombre: Option<String>,
    apellido: Option<String>,
    segundo_apellido: Option<String>,
    motivo_bloqueo: Option<String>,
    fecha_inicio_bloqueo: Option<String>,
    fecha_fin_bloqueo: Option<String>,
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
            contratista_id: row.contratista_id,
            cedula: row.cedula.unwrap_or_default(),
            nombre: row.nombre.unwrap_or_default(),
            segundo_nombre: row.segundo_nombre,
            apellido: row.apellido.unwrap_or_default(),
            segundo_apellido: row.segundo_apellido,
            motivo_bloqueo: row.motivo_bloqueo.unwrap_or_default(),
            fecha_inicio_bloqueo: row.fecha_inicio_bloqueo.unwrap_or_default(),
            fecha_fin_bloqueo: row.fecha_fin_bloqueo,
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

/// Verifica si un contratista está bloqueado
pub async fn check_if_blocked(
    pool: &SqlitePool,
    contratista_id: &str,
) -> sqlx::Result<BlockStatus> {
    let row = sqlx::query!(
        "SELECT COUNT(*) as count, motivo_bloqueo 
         FROM lista_negra 
         WHERE contratista_id = ? AND is_active = 1",
        contratista_id
    )
    .fetch_one(pool)
    .await?;

    Ok(BlockStatus { blocked: row.count > 0, motivo: row.motivo_bloqueo })
}

/// Verifica si una cédula está bloqueada
pub async fn check_if_blocked_by_cedula(
    pool: &SqlitePool,
    cedula: &str,
) -> sqlx::Result<BlockStatus> {
    let row = sqlx::query!(
        "SELECT COUNT(*) as count, motivo_bloqueo 
         FROM lista_negra 
         WHERE cedula = ? AND is_active = 1",
        cedula
    )
    .fetch_one(pool)
    .await?;

    Ok(BlockStatus { blocked: row.count > 0, motivo: row.motivo_bloqueo })
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

/// Busca todos los registros
pub async fn find_all(pool: &SqlitePool) -> sqlx::Result<Vec<ListaNegra>> {
    let rows = sqlx::query_as!(ListaNegraRow, "SELECT * FROM lista_negra ORDER BY created_at DESC")
        .fetch_all(pool)
        .await?;

    Ok(rows.into_iter().map(ListaNegra::from).collect())
}

/// Busca registros activos
pub async fn find_activos(pool: &SqlitePool) -> sqlx::Result<Vec<ListaNegra>> {
    let rows = sqlx::query_as!(
        ListaNegraRow,
        "SELECT * FROM lista_negra WHERE is_active = 1 ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(ListaNegra::from).collect())
}

/// Busca activo por cédula
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

// ==========================================
// QUERIES DE ESCRITURA
// ==========================================

/// Inserta un nuevo registro en lista negra
#[allow(clippy::too_many_arguments)]
pub async fn insert(
    pool: &SqlitePool,
    id: &str,
    contratista_id: Option<&str>,
    cedula: &str,
    nombre: &str,
    segundo_nombre: Option<&str>,
    apellido: &str,
    segundo_apellido: Option<&str>,
    motivo_bloqueo: &str,
    fecha_inicio_bloqueo: &str,
    fecha_fin_bloqueo: Option<&str>,
    bloqueado_por: &str,
    observaciones: Option<&str>,
    created_at: &str,
    updated_at: &str,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"INSERT INTO lista_negra 
           (id, contratista_id, cedula, nombre, segundo_nombre, apellido, segundo_apellido, motivo_bloqueo, fecha_inicio_bloqueo, 
            fecha_fin_bloqueo, bloqueado_por, observaciones, is_active, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 1, ?, ?)"#,
        id,
        contratista_id,
        cedula,
        nombre,
        segundo_nombre,
        apellido,
        segundo_apellido,
        motivo_bloqueo,
        fecha_inicio_bloqueo,
        fecha_fin_bloqueo,
        bloqueado_por,
        observaciones,
        created_at,
        updated_at
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Desactiva un bloqueo
pub async fn deactivate(
    pool: &SqlitePool,
    id: &str,
    motivo: &str,
    observaciones: Option<&str>,
    updated_at: &str,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"UPDATE lista_negra SET 
           is_active = 0, 
           motivo_bloqueo = ?, 
           observaciones = COALESCE(?, observaciones),
           updated_at = ? 
           WHERE id = ?"#,
        motivo,
        observaciones,
        updated_at,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Reactiva un bloqueo
pub async fn reactivate(
    pool: &SqlitePool,
    id: &str,
    motivo: &str,
    observaciones: Option<&str>,
    bloqueado_por: &str,
    updated_at: &str,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"UPDATE lista_negra SET 
           is_active = 1, 
           motivo_bloqueo = ?, 
           observaciones = COALESCE(?, observaciones),
           bloqueado_por = ?,
           updated_at = ? 
           WHERE id = ?"#,
        motivo,
        observaciones,
        bloqueado_por,
        updated_at,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Actualiza un registro
pub async fn update(
    pool: &SqlitePool,
    id: &str,
    motivo: Option<&str>,
    fecha_fin: Option<&str>,
    observaciones: Option<&str>,
    updated_at: &str,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"UPDATE lista_negra SET 
           motivo_bloqueo = COALESCE(?, motivo_bloqueo),
           fecha_fin_bloqueo = COALESCE(?, fecha_fin_bloqueo),
           observaciones = COALESCE(?, observaciones),
           updated_at = ?
           WHERE id = ?"#,
        motivo,
        fecha_fin,
        observaciones,
        updated_at,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Elimina un registro
pub async fn delete(pool: &SqlitePool, id: &str) -> sqlx::Result<()> {
    sqlx::query!("DELETE FROM lista_negra WHERE id = ?", id).execute(pool).await?;

    Ok(())
}
