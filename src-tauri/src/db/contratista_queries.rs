// src/db/contratista_queries.rs

use crate::models::contratista::Contratista;
use serde::Serialize;
use sqlx::{Row, SqlitePool};

#[derive(Debug, Serialize)]
pub struct ContratistaInfo {
    pub id: String,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub empresa_nombre: String,
    pub estado: String,
    pub fecha_vencimiento_praind: String,
}

/// Busca información básica de un contratista por ID
pub async fn find_basic_info_by_id(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<ContratistaInfo>, String> {
    let row = sqlx::query(
        "SELECT c.id, c.cedula, c.nombre, c.apellido, e.nombre as empresa_nombre, c.estado, c.fecha_vencimiento_praind 
         FROM contratistas c
         LEFT JOIN empresas e ON c.empresa_id = e.id
         WHERE c.id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Error buscando contratista: {}", e))?;

    match row {
        Some(r) => {
            let estado: String = r.get("estado");
            Ok(Some(ContratistaInfo {
                id: r.get("id"),
                cedula: r.get("cedula"),
                nombre: r.get("nombre"),
                apellido: r.get("apellido"),
                empresa_nombre: r.get("empresa_nombre"),
                estado,
                fecha_vencimiento_praind: r.get("fecha_vencimiento_praind"),
            }))
        }
        None => Ok(None),
    }
}

/// Obtiene datos básicos de un contratista (cédula, nombre, apellido)
/// Usado por lista_negra para crear registros
pub async fn get_basic_data(
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

/// Cuenta contratistas por cédula
pub async fn count_by_cedula(pool: &SqlitePool, cedula: &str) -> Result<i64, String> {
    let row = sqlx::query("SELECT COUNT(*) as count FROM contratistas WHERE cedula = ?")
        .bind(cedula)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Error contando contratistas: {}", e))?;

    Ok(row.get("count"))
}

/// Inserta un nuevo contratista
#[allow(clippy::too_many_arguments)]
pub async fn insert(
    pool: &SqlitePool,
    id: &str,
    cedula: &str,
    nombre: &str,
    segundo_nombre: Option<&str>,
    apellido: &str,
    segundo_apellido: Option<&str>,
    empresa_id: &str,
    fecha_vencimiento_praind: &str,
    estado: &str,
    created_at: &str,
    updated_at: &str,
) -> Result<(), String> {
    sqlx::query(
        r#"INSERT INTO contratistas 
           (id, cedula, nombre, segundo_nombre, apellido, segundo_apellido, empresa_id, fecha_vencimiento_praind, estado, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
    )
    .bind(id)
    .bind(cedula)
    .bind(nombre)
    .bind(segundo_nombre)
    .bind(apellido)
    .bind(segundo_apellido)
    .bind(empresa_id)
    .bind(fecha_vencimiento_praind)
    .bind(estado)
    .bind(created_at)
    .bind(updated_at)
    .execute(pool)
    .await
    .map_err(|e| format!("Error al insertar contratista: {}", e))?;

    Ok(())
}

/// Busca contratista por ID con nombre de empresa
pub async fn find_by_id_with_empresa(
    pool: &SqlitePool,
    id: &str,
) -> Result<(Contratista, String), String> {
    let row = sqlx::query(
        r#"SELECT c.*, e.nombre as empresa_nombre
           FROM contratistas c
           LEFT JOIN empresas e ON c.empresa_id = e.id
           WHERE c.id = ?"#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Error buscando contratista: {}", e))?;

    if let Some(row) = row {
        let contratista = row_to_contratista(&row)?;
        let empresa_nombre: String = row.get("empresa_nombre");
        Ok((contratista, empresa_nombre))
    } else {
        Err("Contratista no encontrado".to_string())
    }
}

/// Busca contratista por cédula con nombre de empresa
pub async fn find_by_cedula_with_empresa(
    pool: &SqlitePool,
    cedula: &str,
) -> Result<(Contratista, String), String> {
    let row = sqlx::query(
        r#"SELECT c.*, e.nombre as empresa_nombre
           FROM contratistas c
           LEFT JOIN empresas e ON c.empresa_id = e.id
           WHERE c.cedula = ?"#,
    )
    .bind(cedula)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Error buscando contratista: {}", e))?;

    if let Some(row) = row {
        let contratista = row_to_contratista(&row)?;
        let empresa_nombre: String = row.get("empresa_nombre");
        Ok((contratista, empresa_nombre))
    } else {
        Err("Contratista no encontrado".to_string())
    }
}

/// Busca todos los contratistas con nombre de empresa
pub async fn find_all_with_empresa(
    pool: &SqlitePool,
) -> Result<Vec<(Contratista, String)>, String> {
    let rows = sqlx::query(
        r#"SELECT c.*, e.nombre as empresa_nombre
           FROM contratistas c
           LEFT JOIN empresas e ON c.empresa_id = e.id
           ORDER BY c.updated_at DESC"#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error buscando contratistas: {}", e))?;

    let mut result = Vec::new();
    for row in rows {
        let contratista = row_to_contratista(&row)?;
        let empresa_nombre: String = row.get("empresa_nombre");
        result.push((contratista, empresa_nombre));
    }
    Ok(result)
}

/// Busca contratistas activos con nombre de empresa
pub async fn find_activos_with_empresa(
    pool: &SqlitePool,
) -> Result<Vec<(Contratista, String)>, String> {
    let rows = sqlx::query(
        r#"SELECT c.*, e.nombre as empresa_nombre
           FROM contratistas c
           LEFT JOIN empresas e ON c.empresa_id = e.id
           WHERE c.estado = 'activo'
           ORDER BY c.nombre ASC"#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error buscando contratistas activos: {}", e))?;

    let mut result = Vec::new();
    for row in rows {
        let contratista = row_to_contratista(&row)?;
        let empresa_nombre: String = row.get("empresa_nombre");
        result.push((contratista, empresa_nombre));
    }
    Ok(result)
}

/// Actualiza un contratista
#[allow(clippy::too_many_arguments)]
pub async fn update(
    pool: &SqlitePool,
    id: &str,
    nombre: Option<&str>,
    segundo_nombre: Option<&str>,
    apellido: Option<&str>,
    segundo_apellido: Option<&str>,
    empresa_id: Option<&str>,
    fecha_vencimiento_praind: Option<&str>,
    updated_at: &str,
) -> Result<(), String> {
    sqlx::query(
        r#"UPDATE contratistas SET
            nombre = COALESCE(?, nombre),
            segundo_nombre = COALESCE(?, segundo_nombre),
            apellido = COALESCE(?, apellido),
            segundo_apellido = COALESCE(?, segundo_apellido),
            empresa_id = COALESCE(?, empresa_id),
            fecha_vencimiento_praind = COALESCE(?, fecha_vencimiento_praind),
            updated_at = ?
        WHERE id = ?"#,
    )
    .bind(nombre)
    .bind(segundo_nombre)
    .bind(apellido)
    .bind(segundo_apellido)
    .bind(empresa_id)
    .bind(fecha_vencimiento_praind)
    .bind(updated_at)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| format!("Error al actualizar contratista: {}", e))?;

    Ok(())
}

/// Actualiza el estado de un contratista
pub async fn update_estado(
    pool: &SqlitePool,
    id: &str,
    estado: &str,
    updated_at: &str,
) -> Result<(), String> {
    sqlx::query("UPDATE contratistas SET estado = ?, updated_at = ? WHERE id = ?")
        .bind(estado)
        .bind(updated_at)
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| format!("Error al actualizar estado: {}", e))?;

    Ok(())
}

/// Elimina un contratista
pub async fn delete(pool: &SqlitePool, id: &str) -> Result<(), String> {
    sqlx::query("DELETE FROM contratistas WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| format!("Error al eliminar contratista: {}", e))?;

    Ok(())
}

fn row_to_contratista(row: &sqlx::sqlite::SqliteRow) -> Result<Contratista, String> {
    let estado_str: String = row.get("estado");
    let estado = crate::models::contratista::EstadoContratista::from_str(&estado_str)
        .unwrap_or(crate::models::contratista::EstadoContratista::Inactivo);

    Ok(Contratista {
        id: row.get("id"),
        cedula: row.get("cedula"),
        nombre: row.get("nombre"),
        segundo_nombre: row.get("segundo_nombre"),
        apellido: row.get("apellido"),
        segundo_apellido: row.get("segundo_apellido"),
        empresa_id: row.get("empresa_id"),
        fecha_vencimiento_praind: row.get("fecha_vencimiento_praind"),
        estado,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}