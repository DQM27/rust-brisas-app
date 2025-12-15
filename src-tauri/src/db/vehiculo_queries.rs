// ==========================================
// src/db/vehiculo_queries.rs
// ==========================================
// Capa de data access: queries SQL puras
// Sin lógica de negocio, solo interacción con la base de datos

use crate::models::vehiculo::{TipoVehiculo, Vehiculo};
use sqlx::{Row, SqlitePool};

// ==========================================
// QUERIES DE LECTURA
// ==========================================

/// Busca un vehículo por ID con datos del contratista
pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<Vehiculo, String> {
    let row = sqlx::query(
        r#"SELECT 
            v.id, v.contratista_id, v.proveedor_id, v.tipo_vehiculo, v.placa, v.marca, v.modelo, v.color,
            v.is_active, v.created_at, v.updated_at
           FROM vehiculos v
           WHERE v.id = ?"#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|_| "Vehículo no encontrado".to_string())?;

    Ok(Vehiculo {
        id: row.get("id"),
        contratista_id: row.get("contratista_id"),
        proveedor_id: row.get("proveedor_id"),
        tipo_vehiculo: TipoVehiculo::from_str(row.get("tipo_vehiculo"))?,
        placa: row.get("placa"),
        marca: row.get("marca"),
        modelo: row.get("modelo"),
        color: row.get("color"),
        is_active: row.get::<i32, _>("is_active") != 0,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

/// Busca un vehículo por placa
pub async fn find_by_placa(pool: &SqlitePool, placa: &str) -> Result<Vehiculo, String> {
    let row = sqlx::query(
        r#"SELECT 
            v.id, v.contratista_id, v.proveedor_id, v.tipo_vehiculo, v.placa, v.marca, v.modelo, v.color,
            v.is_active, v.created_at, v.updated_at
           FROM vehiculos v
           WHERE v.placa = ?"#,
    )
    .bind(placa)
    .fetch_one(pool)
    .await
    .map_err(|_| format!("Vehículo con placa {} no encontrado", placa))?;

    Ok(Vehiculo {
        id: row.get("id"),
        contratista_id: row.get("contratista_id"),
        proveedor_id: row.get("proveedor_id"),
        tipo_vehiculo: TipoVehiculo::from_str(row.get("tipo_vehiculo"))?,
        placa: row.get("placa"),
        marca: row.get("marca"),
        modelo: row.get("modelo"),
        color: row.get("color"),
        is_active: row.get::<i32, _>("is_active") != 0,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

/// Obtiene todos los vehículos con datos del contratista
pub async fn find_all(pool: &SqlitePool) -> Result<Vec<Vehiculo>, String> {
    let rows = sqlx::query(
        r#"SELECT 
            v.id, v.contratista_id, v.proveedor_id, v.tipo_vehiculo, v.placa, v.marca, v.modelo, v.color,
            v.is_active, v.created_at, v.updated_at
           FROM vehiculos v
           ORDER BY v.created_at DESC"#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error al obtener vehículos: {}", e))?;

    let vehiculos: Vec<Vehiculo> = rows
        .into_iter()
        .filter_map(|row| {
            Some(Vehiculo {
                id: row.get("id"),
                contratista_id: row.get("contratista_id"),
                proveedor_id: row.get("proveedor_id"),
                tipo_vehiculo: TipoVehiculo::from_str(row.get("tipo_vehiculo")).ok()?,
                placa: row.get("placa"),
                marca: row.get("marca"),
                modelo: row.get("modelo"),
                color: row.get("color"),
                is_active: row.get::<i32, _>("is_active") != 0,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
        })
        .collect();

    Ok(vehiculos)
}

/// Obtiene todos los vehículos activos
pub async fn find_activos(pool: &SqlitePool) -> Result<Vec<Vehiculo>, String> {
    let rows = sqlx::query(
        r#"SELECT 
            v.id, v.contratista_id, v.proveedor_id, v.tipo_vehiculo, v.placa, v.marca, v.modelo, v.color,
            v.is_active, v.created_at, v.updated_at
           FROM vehiculos v
           WHERE v.is_active = 1
           ORDER BY v.placa"#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error al obtener vehículos activos: {}", e))?;

    let vehiculos: Vec<Vehiculo> = rows
        .into_iter()
        .filter_map(|row| {
            Some(Vehiculo {
                id: row.get("id"),
                contratista_id: row.get("contratista_id"),
                proveedor_id: row.get("proveedor_id"),
                tipo_vehiculo: TipoVehiculo::from_str(row.get("tipo_vehiculo")).ok()?,
                placa: row.get("placa"),
                marca: row.get("marca"),
                modelo: row.get("modelo"),
                color: row.get("color"),
                is_active: row.get::<i32, _>("is_active") != 0,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
        })
        .collect();

    Ok(vehiculos)
}

/// Obtiene todos los vehículos de un contratista específico
pub async fn find_by_contratista(
    pool: &SqlitePool,
    contratista_id: &str,
) -> Result<Vec<Vehiculo>, String> {
    let rows = sqlx::query(
        r#"SELECT 
            v.id, v.contratista_id, v.proveedor_id, v.tipo_vehiculo, v.placa, v.marca, v.modelo, v.color,
            v.is_active, v.created_at, v.updated_at
           FROM vehiculos v
           WHERE v.contratista_id = ?
           ORDER BY v.is_active DESC, v.placa"#,
    )
    .bind(contratista_id)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error al obtener vehículos del contratista: {}", e))?;

    let vehiculos: Vec<Vehiculo> = rows
        .into_iter()
        .filter_map(|row| {
            Some(Vehiculo {
                id: row.get("id"),
                contratista_id: row.get("contratista_id"),
                proveedor_id: row.get("proveedor_id"),
                tipo_vehiculo: TipoVehiculo::from_str(row.get("tipo_vehiculo")).ok()?,
                placa: row.get("placa"),
                marca: row.get("marca"),
                modelo: row.get("modelo"),
                color: row.get("color"),
                is_active: row.get::<i32, _>("is_active") != 0,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
        })
        .collect();

    Ok(vehiculos)
}

/// Cuenta cuántos vehículos tienen una placa específica (para verificar unicidad)
pub async fn count_by_placa(pool: &SqlitePool, placa: &str) -> Result<i32, String> {
    let row = sqlx::query("SELECT COUNT(*) as count FROM vehiculos WHERE placa = ?")
        .bind(placa)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Error al verificar placa: {}", e))?;

    Ok(row.get("count"))
}

/// Cuenta cuántos vehículos tienen una placa específica excluyendo un ID
/// (útil para updates)
pub async fn count_by_placa_excluding_id(
    pool: &SqlitePool,
    placa: &str,
    exclude_id: &str,
) -> Result<i32, String> {
    let row = sqlx::query("SELECT COUNT(*) as count FROM vehiculos WHERE placa = ? AND id != ?")
        .bind(placa)
        .bind(exclude_id)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Error al verificar placa: {}", e))?;

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

/// Obtiene todos los vehículos de un proveedor específico
pub async fn find_by_proveedor(
    pool: &SqlitePool,
    proveedor_id: &str,
) -> Result<Vec<Vehiculo>, String> {
    let rows = sqlx::query(
        r#"SELECT 
            v.id, v.contratista_id, v.proveedor_id, v.tipo_vehiculo, v.placa, v.marca, v.modelo, v.color,
            v.is_active, v.created_at, v.updated_at
           FROM vehiculos v
           WHERE v.proveedor_id = ?
           ORDER BY v.is_active DESC, v.placa"#
    )
    .bind(proveedor_id)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error al obtener vehículos del proveedor: {}", e))?;

    let vehiculos: Vec<Vehiculo> = rows
        .into_iter()
        .filter_map(|row| {
            Some(Vehiculo {
                id: row.get("id"),
                contratista_id: row.get("contratista_id"),
                proveedor_id: row.get("proveedor_id"),
                tipo_vehiculo: TipoVehiculo::from_str(row.get("tipo_vehiculo")).ok()?,
                placa: row.get("placa"),
                marca: row.get("marca"),
                modelo: row.get("modelo"),
                color: row.get("color"),
                is_active: row.get::<i32, _>("is_active") != 0,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
        })
        .collect();

    Ok(vehiculos)
}

// ==========================================
// QUERIES DE ESCRITURA
// ==========================================

/// Inserta un nuevo vehículo en la base de datos
pub async fn insert(
    pool: &SqlitePool,
    id: &str,
    contratista_id: Option<&str>,
    proveedor_id: Option<&str>,
    tipo_vehiculo: &str,
    placa: &str,
    marca: Option<&str>,
    modelo: Option<&str>,
    color: Option<&str>,
    created_at: &str,
    updated_at: &str,
) -> Result<(), String> {
    sqlx::query(
        r#"INSERT INTO vehiculos 
           (id, contratista_id, proveedor_id, tipo_vehiculo, placa, marca, modelo, color, is_active, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, 1, ?, ?)"#
    )
    .bind(id)
    .bind(contratista_id)
    .bind(proveedor_id)
    .bind(tipo_vehiculo)
    .bind(placa)
    .bind(marca)
    .bind(modelo)
    .bind(color)
    .bind(created_at)
    .bind(updated_at)
    .execute(pool)
    .await
    .map_err(|e| format!("Error al crear vehículo: {}", e))?;

    Ok(())
}

/// Actualiza un vehículo existente (campos opcionales)
pub async fn update(
    pool: &SqlitePool,
    id: &str,
    tipo_vehiculo: Option<&str>,
    marca: Option<&str>,
    modelo: Option<&str>,
    color: Option<&str>,
    is_active: Option<i32>,
    updated_at: &str,
) -> Result<(), String> {
    sqlx::query(
        r#"UPDATE vehiculos SET
            tipo_vehiculo = COALESCE(?, tipo_vehiculo),
            marca = COALESCE(?, marca),
            modelo = COALESCE(?, modelo),
            color = COALESCE(?, color),
            is_active = COALESCE(?, is_active),
            updated_at = ?
        WHERE id = ?"#,
    )
    .bind(tipo_vehiculo)
    .bind(marca)
    .bind(modelo)
    .bind(color)
    .bind(is_active)
    .bind(updated_at)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| format!("Error al actualizar vehículo: {}", e))?;

    Ok(())
}

/// Elimina un vehículo por ID
pub async fn delete(pool: &SqlitePool, id: &str) -> Result<(), String> {
    sqlx::query("DELETE FROM vehiculos WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| format!("Error al eliminar vehículo: {}", e))?;

    Ok(())
}
