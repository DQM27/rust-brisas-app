// src/db/lista_negra_queries.rs

use crate::models::lista_negra::ListaNegra;
use sqlx::{Row, SqlitePool};

pub struct BlockStatus {
    pub blocked: bool,
    pub motivo: Option<String>,
}

/// Verifica si un contratista está bloqueado
pub async fn check_if_blocked(
    pool: &SqlitePool,
    contratista_id: &str,
) -> Result<BlockStatus, String> {
    let row = sqlx::query(
        "SELECT COUNT(*) as count, motivo_bloqueo 
         FROM lista_negra 
         WHERE contratista_id = ? AND activo = 1",
    )
    .bind(contratista_id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error verificando lista negra: {}", e))?;

    let count: i32 = row.get("count");
    let motivo: Option<String> = row.get("motivo_bloqueo");

    Ok(BlockStatus {
        blocked: count > 0,
        motivo,
    })
}

/// Verifica si una cédula está bloqueada
pub async fn check_if_blocked_by_cedula(
    pool: &SqlitePool,
    cedula: &str,
) -> Result<BlockStatus, String> {
    let row = sqlx::query(
        "SELECT COUNT(*) as count, motivo_bloqueo 
         FROM lista_negra 
         WHERE cedula = ? AND activo = 1",
    )
    .bind(cedula)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error verificando lista negra: {}", e))?;

    let count: i32 = row.get("count");
    let motivo: Option<String> = row.get("motivo_bloqueo");

    Ok(BlockStatus {
        blocked: count > 0,
        motivo,
    })
}

/// Obtiene datos básicos de un contratista para lista negra
pub async fn get_contratista_data(
    pool: &SqlitePool,
    contratista_id: &str,
) -> Result<(String, String, String), String> {
    let row = sqlx::query("SELECT cedula, nombre, apellido FROM contratistas WHERE id = ?")
        .bind(contratista_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Error buscando contratista: {}", e))?;

    if let Some(row) = row {
        Ok((row.get("cedula"), row.get("nombre"), row.get("apellido")))
    } else {
        Err("Contratista no encontrado".to_string())
    }
}

/// Cuenta bloqueos activos por cédula
pub async fn count_active_by_cedula(pool: &SqlitePool, cedula: &str) -> Result<i64, String> {
    let row =
        sqlx::query("SELECT COUNT(*) as count FROM lista_negra WHERE cedula = ? AND activo = 1")
            .bind(cedula)
            .fetch_one(pool)
            .await
            .map_err(|e| format!("Error contando bloqueos: {}", e))?;

    Ok(row.get("count"))
}

/// Inserta un nuevo registro en lista negra
#[allow(clippy::too_many_arguments)]
pub async fn insert(
    pool: &SqlitePool,
    id: &str,
    contratista_id: Option<&str>,
    cedula: &str,
    nombre: &str,
    apellido: &str,
    motivo_bloqueo: &str,
    fecha_inicio_bloqueo: &str,
    fecha_fin_bloqueo: Option<&str>,
    bloqueado_por: &str,
    observaciones: Option<&str>,
    created_at: &str,
    updated_at: &str,
) -> Result<(), String> {
    sqlx::query(
        r#"INSERT INTO lista_negra 
           (id, contratista_id, cedula, nombre, apellido, motivo_bloqueo, fecha_inicio_bloqueo, 
            fecha_fin_bloqueo, bloqueado_por, observaciones, activo, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 1, ?, ?)"#,
    )
    .bind(id)
    .bind(contratista_id)
    .bind(cedula)
    .bind(nombre)
    .bind(apellido)
    .bind(motivo_bloqueo)
    .bind(fecha_inicio_bloqueo)
    .bind(fecha_fin_bloqueo)
    .bind(bloqueado_por)
    .bind(observaciones)
    .bind(created_at)
    .bind(updated_at)
    .execute(pool)
    .await
    .map_err(|e| format!("Error al insertar en lista negra: {}", e))?;

    Ok(())
}

/// Busca por ID
pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<ListaNegra, String> {
    let row = sqlx::query("SELECT * FROM lista_negra WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|_| "Registro no encontrado".to_string())?;

    row_to_lista_negra(&row)
}

/// Busca todos los registros
pub async fn find_all(pool: &SqlitePool) -> Result<Vec<ListaNegra>, String> {
    let rows = sqlx::query("SELECT * FROM lista_negra ORDER BY created_at DESC")
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Error buscando lista negra: {}", e))?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row_to_lista_negra(&row)?);
    }
    Ok(result)
}

/// Busca registros activos
pub async fn find_activos(pool: &SqlitePool) -> Result<Vec<ListaNegra>, String> {
    let rows = sqlx::query("SELECT * FROM lista_negra WHERE activo = 1 ORDER BY created_at DESC")
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Error buscando activos: {}", e))?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row_to_lista_negra(&row)?);
    }
    Ok(result)
}

/// Busca activo por cédula
pub async fn find_active_by_cedula(
    pool: &SqlitePool,
    cedula: &str,
) -> Result<Option<ListaNegra>, String> {
    let row = sqlx::query("SELECT * FROM lista_negra WHERE cedula = ? AND activo = 1")
        .bind(cedula)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Error buscando por cédula: {}", e))?;

    if let Some(row) = row {
        Ok(Some(row_to_lista_negra(&row)?))
    } else {
        Ok(None)
    }
}

/// Desactiva un bloqueo
pub async fn deactivate(
    pool: &SqlitePool,
    id: &str,
    motivo: &str,
    observaciones: Option<&str>,
    updated_at: &str,
) -> Result<(), String> {
    sqlx::query(
        r#"UPDATE lista_negra SET 
           activo = 0, 
           motivo_bloqueo = ?, 
           observaciones = COALESCE(?, observaciones),
           updated_at = ? 
           WHERE id = ?"#,
    )
    .bind(motivo)
    .bind(observaciones)
    .bind(updated_at)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| format!("Error al desactivar: {}", e))?;

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
) -> Result<(), String> {
    sqlx::query(
        r#"UPDATE lista_negra SET 
           activo = 1, 
           motivo_bloqueo = ?, 
           observaciones = COALESCE(?, observaciones),
           bloqueado_por = ?,
           updated_at = ? 
           WHERE id = ?"#,
    )
    .bind(motivo)
    .bind(observaciones)
    .bind(bloqueado_por)
    .bind(updated_at)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| format!("Error al reactivar: {}", e))?;

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
) -> Result<(), String> {
    sqlx::query(
        r#"UPDATE lista_negra SET 
           motivo_bloqueo = COALESCE(?, motivo_bloqueo),
           fecha_fin_bloqueo = COALESCE(?, fecha_fin_bloqueo),
           observaciones = COALESCE(?, observaciones),
           updated_at = ?
           WHERE id = ?"#,
    )
    .bind(motivo)
    .bind(fecha_fin)
    .bind(observaciones)
    .bind(updated_at)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| format!("Error al actualizar: {}", e))?;

    Ok(())
}

/// Elimina un registro
pub async fn delete(pool: &SqlitePool, id: &str) -> Result<(), String> {
    sqlx::query("DELETE FROM lista_negra WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| format!("Error al eliminar: {}", e))?;

    Ok(())
}

fn row_to_lista_negra(row: &sqlx::sqlite::SqliteRow) -> Result<ListaNegra, String> {
    let activo_int: i32 = row.get("activo");

    Ok(ListaNegra {
        id: row.get("id"),
        contratista_id: row.get("contratista_id"),
        cedula: row.get("cedula"),
        nombre: row.get("nombre"),
        apellido: row.get("apellido"),
        motivo_bloqueo: row.get("motivo_bloqueo"),
        fecha_inicio_bloqueo: row.get("fecha_inicio_bloqueo"),
        fecha_fin_bloqueo: row.get("fecha_fin_bloqueo"),
        bloqueado_por: row.get("bloqueado_por"),
        observaciones: row.get("observaciones"),
        is_active: activo_int == 1,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}
