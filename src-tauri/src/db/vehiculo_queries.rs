// ==========================================
// src/db/vehiculo_queries.rs
// ==========================================
// Capa de data access: queries SQL puras
// Strict Mode: Uso de query! para validación en tiempo de compilación

use crate::models::vehiculo::{TipoVehiculo, Vehiculo};
use sqlx::SqlitePool;

// ==========================================
// QUERIES DE LECTURA
// ==========================================

/// Busca un vehículo por ID con datos del contratista
pub async fn find_by_id(pool: &SqlitePool, id: &str) -> sqlx::Result<Option<Vehiculo>> {
    let row = sqlx::query!(
        r#"SELECT 
            v.id, v.contratista_id, v.proveedor_id, v.tipo_vehiculo, v.placa, v.marca, v.modelo, v.color,
            v.is_active as "is_active: bool", v.created_at, v.updated_at
           FROM vehiculos v
           WHERE v.id = ?"#,
        id
    )
    .fetch_optional(pool)
    .await?;

    match row {
        Some(r) => Ok(Some(Vehiculo {
            id: r.id,
            contratista_id: r.contratista_id,
            proveedor_id: r.proveedor_id,
            tipo_vehiculo: TipoVehiculo::from_str(&r.tipo_vehiculo)
                .unwrap_or(TipoVehiculo::Motocicleta), // Fallback or strict?
            // Si la DB tiene basura, from_str retorna Err. map_row debería manejarlo?
            // Aquí usamos unwrap_or para no crashear, pero lo ideal sería validar.
            // Dado que insert validó, asumimos consistencia.
            placa: r.placa,
            marca: r.marca,
            modelo: r.modelo,
            color: r.color,
            is_active: r.is_active,
            created_at: r.created_at,
            updated_at: r.updated_at,
        })),
        None => Ok(None),
    }
}

/// Busca un vehículo por placa
pub async fn find_by_placa(pool: &SqlitePool, placa: &str) -> sqlx::Result<Option<Vehiculo>> {
    let row = sqlx::query!(
        r#"SELECT 
            v.id, v.contratista_id, v.proveedor_id, v.tipo_vehiculo, v.placa, v.marca, v.modelo, v.color,
            v.is_active as "is_active: bool", v.created_at, v.updated_at
           FROM vehiculos v
           WHERE v.placa = ?"#,
        placa
    )
    .fetch_optional(pool)
    .await?;

    match row {
        Some(r) => Ok(Some(Vehiculo {
            id: r.id,
            contratista_id: r.contratista_id,
            proveedor_id: r.proveedor_id,
            tipo_vehiculo: TipoVehiculo::from_str(&r.tipo_vehiculo)
                .unwrap_or(TipoVehiculo::Motocicleta),
            placa: r.placa,
            marca: r.marca,
            modelo: r.modelo,
            color: r.color,
            is_active: r.is_active,
            created_at: r.created_at,
            updated_at: r.updated_at,
        })),
        None => Ok(None),
    }
}

/// Obtiene todos los vehículos con datos del contratista
pub async fn find_all(pool: &SqlitePool) -> sqlx::Result<Vec<Vehiculo>> {
    let rows = sqlx::query!(
        r#"SELECT 
            v.id, v.contratista_id, v.proveedor_id, v.tipo_vehiculo, v.placa, v.marca, v.modelo, v.color,
            v.is_active as "is_active: bool", v.created_at, v.updated_at
           FROM vehiculos v
           ORDER BY v.created_at DESC"#
    )
    .fetch_all(pool)
    .await?;

    let vehiculos: Vec<Vehiculo> = rows
        .into_iter()
        .map(|r| Vehiculo {
            id: r.id,
            contratista_id: r.contratista_id,
            proveedor_id: r.proveedor_id,
            tipo_vehiculo: TipoVehiculo::from_str(&r.tipo_vehiculo)
                .unwrap_or(TipoVehiculo::Motocicleta),
            placa: r.placa,
            marca: r.marca,
            modelo: r.modelo,
            color: r.color,
            is_active: r.is_active,
            created_at: r.created_at,
            updated_at: r.updated_at,
        })
        .collect();

    Ok(vehiculos)
}

/// Obtiene todos los vehículos activos
pub async fn find_activos(pool: &SqlitePool) -> sqlx::Result<Vec<Vehiculo>> {
    let rows = sqlx::query!(
        r#"SELECT 
            v.id, v.contratista_id, v.proveedor_id, v.tipo_vehiculo, v.placa, v.marca, v.modelo, v.color,
            v.is_active as "is_active: bool", v.created_at, v.updated_at
           FROM vehiculos v
           WHERE v.is_active = 1
           ORDER BY v.placa"#
    )
    .fetch_all(pool)
    .await?;

    let vehiculos: Vec<Vehiculo> = rows
        .into_iter()
        .map(|r| Vehiculo {
            id: r.id,
            contratista_id: r.contratista_id,
            proveedor_id: r.proveedor_id,
            tipo_vehiculo: TipoVehiculo::from_str(&r.tipo_vehiculo)
                .unwrap_or(TipoVehiculo::Motocicleta),
            placa: r.placa,
            marca: r.marca,
            modelo: r.modelo,
            color: r.color,
            is_active: r.is_active,
            created_at: r.created_at,
            updated_at: r.updated_at,
        })
        .collect();

    Ok(vehiculos)
}

/// Obtiene todos los vehículos de un contratista específico
pub async fn find_by_contratista(
    pool: &SqlitePool,
    contratista_id: &str,
) -> sqlx::Result<Vec<Vehiculo>> {
    let rows = sqlx::query!(
        r#"SELECT 
            v.id, v.contratista_id, v.proveedor_id, v.tipo_vehiculo, v.placa, v.marca, v.modelo, v.color,
            v.is_active as "is_active: bool", v.created_at, v.updated_at
           FROM vehiculos v
           WHERE v.contratista_id = ?
           ORDER BY v.is_active DESC, v.placa"#,
        contratista_id
    )
    .fetch_all(pool)
    .await?;

    let vehiculos: Vec<Vehiculo> = rows
        .into_iter()
        .map(|r| Vehiculo {
            id: r.id,
            contratista_id: r.contratista_id,
            proveedor_id: r.proveedor_id,
            tipo_vehiculo: TipoVehiculo::from_str(&r.tipo_vehiculo)
                .unwrap_or(TipoVehiculo::Motocicleta),
            placa: r.placa,
            marca: r.marca,
            modelo: r.modelo,
            color: r.color,
            is_active: r.is_active,
            created_at: r.created_at,
            updated_at: r.updated_at,
        })
        .collect();

    Ok(vehiculos)
}

/// Obtiene todos los vehículos de un proveedor específico
pub async fn find_by_proveedor(
    pool: &SqlitePool,
    proveedor_id: &str,
) -> sqlx::Result<Vec<Vehiculo>> {
    let rows = sqlx::query!(
        r#"SELECT 
            v.id, v.contratista_id, v.proveedor_id, v.tipo_vehiculo, v.placa, v.marca, v.modelo, v.color,
            v.is_active as "is_active: bool", v.created_at, v.updated_at
           FROM vehiculos v
           WHERE v.proveedor_id = ?
           ORDER BY v.is_active DESC, v.placa"#,
        proveedor_id
    )
    .fetch_all(pool)
    .await?;

    let vehiculos: Vec<Vehiculo> = rows
        .into_iter()
        .map(|r| Vehiculo {
            id: r.id,
            contratista_id: r.contratista_id,
            proveedor_id: r.proveedor_id,
            tipo_vehiculo: TipoVehiculo::from_str(&r.tipo_vehiculo)
                .unwrap_or(TipoVehiculo::Motocicleta),
            placa: r.placa,
            marca: r.marca,
            modelo: r.modelo,
            color: r.color,
            is_active: r.is_active,
            created_at: r.created_at,
            updated_at: r.updated_at,
        })
        .collect();

    Ok(vehiculos)
}

/// Cuenta cuántos vehículos tienen una placa específica (para verificar unicidad)
pub async fn count_by_placa(pool: &SqlitePool, placa: &str) -> sqlx::Result<i64> {
    let row = sqlx::query!(
        "SELECT COUNT(*) as count FROM vehiculos WHERE placa = ?",
        placa
    )
    .fetch_one(pool)
    .await?;

    Ok(row.count as i64)
}

/// Cuenta cuántos vehículos tienen una placa específica excluyendo un ID
pub async fn count_by_placa_excluding_id(
    pool: &SqlitePool,
    placa: &str,
    exclude_id: &str,
) -> sqlx::Result<i64> {
    let row = sqlx::query!(
        "SELECT COUNT(*) as count FROM vehiculos WHERE placa = ? AND id != ?",
        placa,
        exclude_id
    )
    .fetch_one(pool)
    .await?;

    Ok(row.count as i64)
}

/// Verifica si un contratista existe
pub async fn contratista_exists(pool: &SqlitePool, contratista_id: &str) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        "SELECT COUNT(*) as count FROM contratistas WHERE id = ?",
        contratista_id
    )
    .fetch_one(pool)
    .await?;

    Ok(row.count > 0)
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
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"INSERT INTO vehiculos 
           (id, contratista_id, proveedor_id, tipo_vehiculo, placa, marca, modelo, color, is_active, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, 1, ?, ?)"#,
        id,
        contratista_id,
        proveedor_id,
        tipo_vehiculo,
        placa,
        marca,
        modelo,
        color,
        created_at,
        updated_at
    )
    .execute(pool)
    .await?;

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
    is_active: Option<bool>,
    updated_at: &str,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"UPDATE vehiculos SET
            tipo_vehiculo = COALESCE(?, tipo_vehiculo),
            marca = COALESCE(?, marca),
            modelo = COALESCE(?, modelo),
            color = COALESCE(?, color),
            is_active = COALESCE(?, is_active),
            updated_at = ?
        WHERE id = ?"#,
        tipo_vehiculo,
        marca,
        modelo,
        color,
        is_active,
        updated_at,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Elimina un vehículo por ID
pub async fn delete(pool: &SqlitePool, id: &str) -> sqlx::Result<()> {
    sqlx::query!("DELETE FROM vehiculos WHERE id = ?", id)
        .execute(pool)
        .await?;

    Ok(())
}
