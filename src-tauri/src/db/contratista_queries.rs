// ==========================================
// src/db/contratista_queries.rs
// ==========================================
// Capa de data access: queries SQL puras
// Sin lógica de negocio, solo interacción con la base de datos

use crate::models::contratista::{Contratista, EstadoContratista};
use sqlx::{SqlitePool, Row};

// ==========================================
// QUERIES DE LECTURA
// ==========================================

/// Busca un contratista por ID con datos de empresa
pub async fn find_by_id_with_empresa(
    pool: &SqlitePool,
    id: &str,
) -> Result<(Contratista, String), String> {
    let row = sqlx::query(
        r#"SELECT 
            c.id, c.cedula, c.nombre, c.segundo_nombre, c.apellido, c.segundo_apellido,
            c.empresa_id, c.fecha_vencimiento_praind, c.estado, c.created_at, c.updated_at,
            e.nombre as empresa_nombre
           FROM contratistas c
           INNER JOIN empresas e ON c.empresa_id = e.id
           WHERE c.id = ?"#
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Contratista no encontrado: {}", e))?;
    
    let contratista = Contratista {
        id: row.get("id"),
        cedula: row.get("cedula"),
        nombre: row.get("nombre"),
        segundo_nombre: row.get("segundo_nombre"),
        apellido: row.get("apellido"),
        segundo_apellido: row.get("segundo_apellido"),
        empresa_id: row.get("empresa_id"),
        fecha_vencimiento_praind: row.get("fecha_vencimiento_praind"),
        estado: EstadoContratista::from_str(row.get("estado"))?,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    };
    
    let empresa_nombre: String = row.get("empresa_nombre");
    
    Ok((contratista, empresa_nombre))
}

/// Busca un contratista por cédula con datos de empresa
pub async fn find_by_cedula_with_empresa(
    pool: &SqlitePool,
    cedula: &str,
) -> Result<(Contratista, String), String> {
    let row = sqlx::query(
        r#"SELECT 
            c.id, c.cedula, c.nombre, c.segundo_nombre, c.apellido, c.segundo_apellido,
            c.empresa_id, c.fecha_vencimiento_praind, c.estado, c.created_at, c.updated_at,
            e.nombre as empresa_nombre
           FROM contratistas c
           INNER JOIN empresas e ON c.empresa_id = e.id
           WHERE c.cedula = ?"#
    )
    .bind(cedula)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Contratista no encontrado: {}", e))?;
    
    let contratista = Contratista {
        id: row.get("id"),
        cedula: row.get("cedula"),
        nombre: row.get("nombre"),
        segundo_nombre: row.get("segundo_nombre"),
        apellido: row.get("apellido"),
        segundo_apellido: row.get("segundo_apellido"),
        empresa_id: row.get("empresa_id"),
        fecha_vencimiento_praind: row.get("fecha_vencimiento_praind"),
        estado: EstadoContratista::from_str(row.get("estado"))?,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    };
    
    let empresa_nombre: String = row.get("empresa_nombre");
    
    Ok((contratista, empresa_nombre))
}

/// Obtiene todos los contratistas con datos de empresa
pub async fn find_all_with_empresa(pool: &SqlitePool) -> Result<Vec<(Contratista, String)>, String> {
    let rows = sqlx::query(
        r#"SELECT 
            c.id, c.cedula, c.nombre, c.segundo_nombre, c.apellido, c.segundo_apellido,
            c.empresa_id, c.fecha_vencimiento_praind, c.estado, c.created_at, c.updated_at,
            e.nombre as empresa_nombre
           FROM contratistas c
           INNER JOIN empresas e ON c.empresa_id = e.id
           ORDER BY c.created_at DESC"#
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error al obtener contratistas: {}", e))?;
    
    let contratistas: Vec<(Contratista, String)> = rows
        .into_iter()
        .filter_map(|row| {
            let contratista = Contratista {
                id: row.get("id"),
                cedula: row.get("cedula"),
                nombre: row.get("nombre"),
                segundo_nombre: row.get("segundo_nombre"),
                apellido: row.get("apellido"),
                segundo_apellido: row.get("segundo_apellido"),
                empresa_id: row.get("empresa_id"),
                fecha_vencimiento_praind: row.get("fecha_vencimiento_praind"),
                estado: EstadoContratista::from_str(row.get("estado")).ok()?,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            
            let empresa_nombre: String = row.get("empresa_nombre");
            Some((contratista, empresa_nombre))
        })
        .collect();
    
    Ok(contratistas)
}

/// Obtiene todos los contratistas activos con datos de empresa
pub async fn find_activos_with_empresa(pool: &SqlitePool) -> Result<Vec<(Contratista, String)>, String> {
    let rows = sqlx::query(
        r#"SELECT 
            c.id, c.cedula, c.nombre, c.segundo_nombre, c.apellido, c.segundo_apellido,
            c.empresa_id, c.fecha_vencimiento_praind, c.estado, c.created_at, c.updated_at,
            e.nombre as empresa_nombre
           FROM contratistas c
           INNER JOIN empresas e ON c.empresa_id = e.id
           WHERE c.estado = ? 
           ORDER BY c.apellido, c.nombre"#
    )
    .bind(EstadoContratista::Activo.as_str())
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error al obtener contratistas activos: {}", e))?;
    
    let contratistas: Vec<(Contratista, String)> = rows
        .into_iter()
        .filter_map(|row| {
            let contratista = Contratista {
                id: row.get("id"),
                cedula: row.get("cedula"),
                nombre: row.get("nombre"),
                segundo_nombre: row.get("segundo_nombre"),
                apellido: row.get("apellido"),
                segundo_apellido: row.get("segundo_apellido"),
                empresa_id: row.get("empresa_id"),
                fecha_vencimiento_praind: row.get("fecha_vencimiento_praind"),
                estado: EstadoContratista::from_str(row.get("estado")).ok()?,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            
            let empresa_nombre: String = row.get("empresa_nombre");
            Some((contratista, empresa_nombre))
        })
        .collect();
    
    Ok(contratistas)
}

/// Cuenta cuántos contratistas tienen una cédula específica
pub async fn count_by_cedula(pool: &SqlitePool, cedula: &str) -> Result<i32, String> {
    let row = sqlx::query("SELECT COUNT(*) as count FROM contratistas WHERE cedula = ?")
        .bind(cedula)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Error al verificar cédula: {}", e))?;
    
    Ok(row.get("count"))
}

/// Verifica si una cédula está en lista negra activa
pub async fn count_cedula_in_blacklist(pool: &SqlitePool, cedula: &str) -> Result<i32, String> {
    let row = sqlx::query(
        "SELECT COUNT(*) as count FROM lista_negra WHERE cedula = ? AND is_active = 1"
    )
    .bind(cedula)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error al verificar lista negra: {}", e))?;
    
    Ok(row.get("count"))
}

/// Obtiene detalles del bloqueo en lista negra
pub async fn get_blacklist_details(
    pool: &SqlitePool,
    cedula: &str,
) -> Result<(String, String), String> {
    let row = sqlx::query(
        "SELECT motivo_bloqueo, bloqueado_por FROM lista_negra WHERE cedula = ? AND is_active = 1 LIMIT 1"
    )
    .bind(cedula)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error al obtener detalles de bloqueo: {}", e))?;
    
    let motivo: String = row.get("motivo_bloqueo");
    let bloqueado_por: String = row.get("bloqueado_por");
    
    Ok((motivo, bloqueado_por))
}

/// Verifica si una empresa existe
pub async fn empresa_exists(pool: &SqlitePool, empresa_id: &str) -> Result<bool, String> {
    let row = sqlx::query("SELECT COUNT(*) as count FROM empresas WHERE id = ?")
        .bind(empresa_id)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Error al verificar empresa: {}", e))?;
    
    let count: i32 = row.get("count");
    Ok(count > 0)
}

// ==========================================
// QUERIES DE ESCRITURA
// ==========================================

/// Inserta un nuevo contratista
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
    .map_err(|e| format!("Error al crear contratista: {}", e))?;
    
    Ok(())
}

/// Actualiza un contratista existente
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
        WHERE id = ?"#
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

/// Cambia el estado de un contratista
pub async fn update_estado(
    pool: &SqlitePool,
    id: &str,
    estado: &str,
    updated_at: &str,
) -> Result<(), String> {
    sqlx::query(
        "UPDATE contratistas SET estado = ?, updated_at = ? WHERE id = ?"
    )
    .bind(estado)
    .bind(updated_at)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| format!("Error al cambiar estado: {}", e))?;
    
    Ok(())
}

/// Elimina un contratista por ID
pub async fn delete(pool: &SqlitePool, id: &str) -> Result<(), String> {
    sqlx::query("DELETE FROM contratistas WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| format!("Error al eliminar contratista: {}", e))?;
    
    Ok(())
}