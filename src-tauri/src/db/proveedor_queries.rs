// ==========================================
// src/db/proveedor_queries.rs
// ==========================================
// Capa de data access: queries SQL puras
// Strict Mode: Uso de query_as! para validación en tiempo de compilación

use crate::models::proveedor::{
    CreateProveedorInput, EstadoProveedor, Proveedor, UpdateProveedorInput,
};
use chrono::Utc;
use serde::Serialize;
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ProveedorEnhancedRow {
    pub proveedor: Proveedor,
    pub empresa_nombre: String,
    pub vehiculo_tipo: Option<String>,
    pub vehiculo_placa: Option<String>,
    pub vehiculo_marca: Option<String>,
    pub vehiculo_modelo: Option<String>,
    pub vehiculo_color: Option<String>,
}

// ==========================================
// QUERIES DE LECTURA
// ==========================================

/// Obtiene un proveedor por ID con su empresa y vehículo asociado
pub async fn find_by_id_with_empresa(
    pool: &SqlitePool,
    id: &str,
) -> sqlx::Result<Option<ProveedorEnhancedRow>> {
    // DTO intermedio para el query complejo
    #[derive(sqlx::FromRow)]
    struct EnhancedRow {
        id: String,
        cedula: String,
        nombre: String,
        segundo_nombre: Option<String>,
        apellido: String,
        segundo_apellido: Option<String>,
        empresa_id: String,
        estado: EstadoProveedor,
        created_at: String,
        updated_at: String,
        empresa_nombre: String,
        tipo_vehiculo: Option<String>,
        placa: Option<String>,
        marca: Option<String>,
        modelo: Option<String>,
        color: Option<String>,
    }

    let row = sqlx::query_as!(
        EnhancedRow,
        r#"
        SELECT 
            p.id as "id!",
            p.cedula as "cedula!",
            p.nombre as "nombre!",
            p.segundo_nombre,
            p.apellido as "apellido!",
            p.segundo_apellido,
            p.empresa_id as "empresa_id!",
            p.estado as "estado!: EstadoProveedor",
            CAST(p.created_at AS TEXT) as "created_at!",
            CAST(p.updated_at AS TEXT) as "updated_at!",
            e.nombre as "empresa_nombre!",
            v.tipo_vehiculo,
            v.placa,
            v.marca,
            v.modelo,
            v.color
        FROM proveedores p
        LEFT JOIN empresas e ON p.empresa_id = e.id
        LEFT JOIN vehiculos v ON p.id = v.proveedor_id
        WHERE p.id = ?
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| ProveedorEnhancedRow {
        proveedor: Proveedor {
            id: r.id,
            cedula: r.cedula,
            nombre: r.nombre,
            segundo_nombre: r.segundo_nombre,
            apellido: r.apellido,
            segundo_apellido: r.segundo_apellido,
            empresa_id: r.empresa_id,
            estado: r.estado,
            created_at: r.created_at,
            updated_at: r.updated_at,
        },
        empresa_nombre: r.empresa_nombre,
        vehiculo_tipo: r.tipo_vehiculo,
        vehiculo_placa: r.placa,
        vehiculo_marca: r.marca,
        vehiculo_modelo: r.modelo,
        vehiculo_color: r.color,
    }))
}

pub async fn create(pool: &SqlitePool, input: CreateProveedorInput) -> sqlx::Result<Proveedor> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let estado = EstadoProveedor::Activo;
    let estado_str = estado.as_str();

    sqlx::query!(
        r#"
        INSERT INTO proveedores (id, cedula, nombre, segundo_nombre, apellido, segundo_apellido, empresa_id, estado, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        id,
        input.cedula,
        input.nombre,
        input.segundo_nombre,
        input.apellido,
        input.segundo_apellido,
        input.empresa_id,
        estado_str,
        now,
        now
    )
    .execute(pool)
    .await?;

    Ok(Proveedor {
        id,
        cedula: input.cedula,
        nombre: input.nombre,
        segundo_nombre: input.segundo_nombre,
        apellido: input.apellido,
        segundo_apellido: input.segundo_apellido,
        empresa_id: input.empresa_id,
        estado,
        created_at: now.clone(),
        updated_at: now,
    })
}

pub async fn find_by_cedula(pool: &SqlitePool, cedula: &str) -> sqlx::Result<Option<Proveedor>> {
    sqlx::query_as!(
        Proveedor,
        r#"
        SELECT 
            id as "id!",
            cedula as "cedula!",
            nombre as "nombre!",
            segundo_nombre,
            apellido as "apellido!",
            segundo_apellido,
            empresa_id as "empresa_id!",
            estado as "estado!: EstadoProveedor",
            CAST(created_at AS TEXT) as "created_at!",
            CAST(updated_at AS TEXT) as "updated_at!"
        FROM proveedores 
        WHERE cedula = ?
        "#,
        cedula
    )
    .fetch_optional(pool)
    .await
}

pub async fn find_by_id(pool: &SqlitePool, id: &str) -> sqlx::Result<Option<Proveedor>> {
    sqlx::query_as!(
        Proveedor,
        r#"
        SELECT 
            id as "id!",
            cedula as "cedula!",
            nombre as "nombre!",
            segundo_nombre,
            apellido as "apellido!",
            segundo_apellido,
            empresa_id as "empresa_id!",
            estado as "estado!: EstadoProveedor",
            CAST(created_at AS TEXT) as "created_at!",
            CAST(updated_at AS TEXT) as "updated_at!"
        FROM proveedores 
        WHERE id = ?
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

/// Obtiene todos los proveedores con el nombre de su empresa (para reindexación)
pub async fn find_all_with_empresa(pool: &SqlitePool) -> sqlx::Result<Vec<(Proveedor, String)>> {
    #[derive(sqlx::FromRow)]
    struct ProveedorWithEmpresa {
        id: String,
        cedula: String,
        nombre: String,
        segundo_nombre: Option<String>,
        apellido: String,
        segundo_apellido: Option<String>,
        empresa_id: String,
        estado: EstadoProveedor,
        created_at: String,
        updated_at: String,
        empresa_nombre: String,
    }

    let rows = sqlx::query_as!(
        ProveedorWithEmpresa,
        r#"
        SELECT 
            p.id as "id!",
            p.cedula as "cedula!",
            p.nombre as "nombre!",
            p.segundo_nombre,
            p.apellido as "apellido!",
            p.segundo_apellido,
            p.empresa_id as "empresa_id!",
            p.estado as "estado!: EstadoProveedor",
            CAST(p.created_at AS TEXT) as "created_at!",
            CAST(p.updated_at AS TEXT) as "updated_at!",
            COALESCE(e.nombre, 'Empresa desconocida') as "empresa_nombre!"
        FROM proveedores p
        LEFT JOIN empresas e ON p.empresa_id = e.id
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| {
            (
                Proveedor {
                    id: r.id,
                    cedula: r.cedula,
                    nombre: r.nombre,
                    segundo_nombre: r.segundo_nombre,
                    apellido: r.apellido,
                    segundo_apellido: r.segundo_apellido,
                    empresa_id: r.empresa_id,
                    estado: r.estado,
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                },
                r.empresa_nombre,
            )
        })
        .collect())
}

pub async fn search(pool: &SqlitePool, query: &str, limit: i64) -> sqlx::Result<Vec<Proveedor>> {
    let pattern = format!("%{}%", query);
    sqlx::query_as!(
        Proveedor,
        r#"
        SELECT 
            id as "id!",
            cedula as "cedula!",
            nombre as "nombre!",
            segundo_nombre,
            apellido as "apellido!",
            segundo_apellido,
            empresa_id as "empresa_id!",
            estado as "estado!: EstadoProveedor",
            CAST(created_at AS TEXT) as "created_at!",
            CAST(updated_at AS TEXT) as "updated_at!"
        FROM proveedores
        WHERE cedula LIKE ? OR nombre LIKE ? OR apellido LIKE ?
        ORDER BY created_at DESC
        LIMIT ?
        "#,
        pattern,
        pattern,
        pattern,
        limit
    )
    .fetch_all(pool)
    .await
}

pub async fn update(
    pool: &SqlitePool,
    id: &str,
    input: UpdateProveedorInput,
) -> sqlx::Result<Proveedor> {
    let now = Utc::now().to_rfc3339();

    sqlx::query!(
        r#"
        UPDATE proveedores
        SET nombre = COALESCE(?, nombre),
            segundo_nombre = COALESCE(?, segundo_nombre),
            apellido = COALESCE(?, apellido),
            segundo_apellido = COALESCE(?, segundo_apellido),
            empresa_id = COALESCE(?, empresa_id),
            estado = COALESCE(?, estado),
            updated_at = ?
        WHERE id = ?
        "#,
        input.nombre,
        input.segundo_nombre,
        input.apellido,
        input.segundo_apellido,
        input.empresa_id,
        input.estado,
        now,
        id
    )
    .execute(pool)
    .await?;

    // Fetch the updated record
    find_by_id(pool, id).await?.ok_or_else(|| sqlx::Error::RowNotFound)
}

pub async fn delete(pool: &SqlitePool, id: &str) -> sqlx::Result<()> {
    sqlx::query!("DELETE FROM proveedores WHERE id = ?", id).execute(pool).await?;
    Ok(())
}
