// ==========================================
// src/db/lista_negra_queries.rs
// ==========================================
// Capa de data access: queries SQL puras
// Sin lógica de negocio, solo interacción con la base de datos

use crate::models::lista_negra::ListaNegra;
use sqlx::{SqlitePool, Row};

// ==========================================
// QUERIES DE LECTURA
// ==========================================

/// Busca un bloqueo por ID
pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<ListaNegra, String> {
    let row = sqlx::query(
        r#"SELECT 
            id, contratista_id, cedula, nombre, apellido,
            motivo_bloqueo, fecha_inicio_bloqueo, fecha_fin_bloqueo,
            bloqueado_por, observaciones, is_active, 
            created_at, updated_at
           FROM lista_negra
           WHERE id = ?"#
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|_| "Registro no encontrado".to_string())?;
    
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
        is_active: row.get::<i32, _>("is_active") != 0,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

/// Busca un bloqueo activo por cédula
pub async fn find_active_by_cedula(pool: &SqlitePool, cedula: &str) -> Result<Option<ListaNegra>, String> {
    let row = sqlx::query(
        r#"SELECT 
            id, contratista_id, cedula, nombre, apellido,
            motivo_bloqueo, fecha_inicio_bloqueo, fecha_fin_bloqueo,
            bloqueado_por, observaciones, is_active, 
            created_at, updated_at
           FROM lista_negra
           WHERE cedula = ? AND is_active = 1
           LIMIT 1"#
    )
    .bind(cedula)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Error al buscar bloqueo: {}", e))?;
    
    if let Some(row) = row {
        Ok(Some(ListaNegra {
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
            is_active: row.get::<i32, _>("is_active") != 0,
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }))
    } else {
        Ok(None)
    }
}

/// Obtiene todos los bloqueos
pub async fn find_all(pool: &SqlitePool) -> Result<Vec<ListaNegra>, String> {
    let rows = sqlx::query(
        r#"SELECT 
            id, contratista_id, cedula, nombre, apellido,
            motivo_bloqueo, fecha_inicio_bloqueo, fecha_fin_bloqueo,
            bloqueado_por, observaciones, is_active, 
            created_at, updated_at
           FROM lista_negra
           ORDER BY created_at DESC"#
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error al obtener lista negra: {}", e))?;
    
    let bloqueados: Vec<ListaNegra> = rows
        .into_iter()
        .map(|row| ListaNegra {
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
            is_active: row.get::<i32, _>("is_active") != 0,
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect();
    
    Ok(bloqueados)
}

/// Obtiene todos los bloqueos activos
pub async fn find_activos(pool: &SqlitePool) -> Result<Vec<ListaNegra>, String> {
    let rows = sqlx::query(
        r#"SELECT 
            id, contratista_id, cedula, nombre, apellido,
            motivo_bloqueo, fecha_inicio_bloqueo, fecha_fin_bloqueo,
            bloqueado_por, observaciones, is_active, 
            created_at, updated_at
           FROM lista_negra
           WHERE is_active = 1
           ORDER BY fecha_inicio_bloqueo DESC"#
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error al obtener bloqueados activos: {}", e))?;
    
    let bloqueados: Vec<ListaNegra> = rows
        .into_iter()
        .map(|row| ListaNegra {
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
            is_active: row.get::<i32, _>("is_active") != 0,
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect();
    
    Ok(bloqueados)
}

/// Cuenta cuántos bloqueos activos existen para una cédula (para verificar unicidad)
pub async fn count_active_by_cedula(pool: &SqlitePool, cedula: &str) -> Result<i32, String> {
    let row = sqlx::query(
        "SELECT COUNT(*) as count FROM lista_negra WHERE cedula = ? AND is_active = 1"
    )
    .bind(cedula)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error al verificar bloqueo existente: {}", e))?;
    
    Ok(row.get("count"))
}

/// Verifica si un contratista existe
pub async fn contratista_exists(pool: &SqlitePool, contratista_id: &str) -> Result<bool, String> {
    let row = sqlx::query("SELECT COUNT(*) as count FROM contratistas WHERE id = ?")
        .bind(contratista_id)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Error al verificar contratista: {}", e))?;
    
    let count: i32 = row.get("count");
    Ok(count > 0)
}

/// Obtiene datos de un contratista por ID
pub async fn get_contratista_data(
    pool: &SqlitePool,
    contratista_id: &str,
) -> Result<(String, String, String), String> {
    let row = sqlx::query(
        "SELECT cedula, nombre, apellido FROM contratistas WHERE id = ?"
    )
    .bind(contratista_id)
    .fetch_one(pool)
    .await
    .map_err(|_| "El contratista especificado no existe".to_string())?;
    
    Ok((
        row.get("cedula"),
        row.get("nombre"),
        row.get("apellido"),
    ))
}

// ==========================================
// QUERIES DE ESCRITURA
// ==========================================

/// Inserta un nuevo bloqueo en la base de datos
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
           (id, contratista_id, cedula, nombre, apellido, motivo_bloqueo, 
            fecha_inicio_bloqueo, fecha_fin_bloqueo, bloqueado_por, observaciones, 
            is_active, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 1, ?, ?)"#
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
    .map_err(|e| format!("Error al agregar a lista negra: {}", e))?;
    
    Ok(())
}

/// Actualiza un bloqueo existente (campos opcionales)
pub async fn update(
    pool: &SqlitePool,
    id: &str,
    motivo_bloqueo: Option<&str>,
    fecha_fin_bloqueo: Option<&str>,
    observaciones: Option<&str>,
    updated_at: &str,
) -> Result<(), String> {
    sqlx::query(
        r#"UPDATE lista_negra SET
            motivo_bloqueo = COALESCE(?, motivo_bloqueo),
            fecha_fin_bloqueo = COALESCE(?, fecha_fin_bloqueo),
            observaciones = COALESCE(?, observaciones),
            updated_at = ?
        WHERE id = ?"#
    )
    .bind(motivo_bloqueo)
    .bind(fecha_fin_bloqueo)
    .bind(observaciones)
    .bind(updated_at)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| format!("Error al actualizar bloqueo: {}", e))?;
    
    Ok(())
}

/// Desactiva un bloqueo (soft delete)
pub async fn deactivate(pool: &SqlitePool, id: &str, updated_at: &str) -> Result<(), String> {
    sqlx::query(
        "UPDATE lista_negra SET is_active = 0, updated_at = ? WHERE id = ?"
    )
    .bind(updated_at)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| format!("Error al desactivar bloqueo: {}", e))?;
    
    Ok(())
}

/// Elimina un bloqueo por ID (hard delete)
pub async fn delete(pool: &SqlitePool, id: &str) -> Result<(), String> {
    sqlx::query("DELETE FROM lista_negra WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| format!("Error al eliminar registro: {}", e))?;
    
    Ok(())
}