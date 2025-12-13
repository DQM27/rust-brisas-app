// ==========================================
// src/db/proveedor_queries.rs
// ==========================================
use crate::models::proveedor::{
    CreateProveedorInput, EstadoProveedor, Proveedor, UpdateProveedorInput,
};
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn create(
    pool: &SqlitePool,
    input: CreateProveedorInput,
) -> Result<Proveedor, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let estado = EstadoProveedor::Activo;

    sqlx::query(
        r#"
        INSERT INTO proveedores (id, cedula, nombre, segundo_nombre, apellido, segundo_apellido, empresa_id, estado, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(&input.cedula)
    .bind(&input.nombre)
    .bind(&input.segundo_nombre)
    .bind(&input.apellido)
    .bind(&input.segundo_apellido)
    .bind(&input.empresa_id)
    .bind(estado.as_str())
    .bind(&now)
    .bind(&now)
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

pub async fn find_by_cedula(
    pool: &SqlitePool,
    cedula: &str,
) -> Result<Option<Proveedor>, sqlx::Error> {
    sqlx::query_as::<_, Proveedor>("SELECT * FROM proveedores WHERE cedula = ?")
        .bind(cedula)
        .fetch_optional(pool)
        .await
}

pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Proveedor>, sqlx::Error> {
    sqlx::query_as::<_, Proveedor>("SELECT * FROM proveedores WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn search(
    pool: &SqlitePool,
    query: &str,
    limit: i64,
) -> Result<Vec<Proveedor>, sqlx::Error> {
    let pattern = format!("%{}%", query);
    sqlx::query_as::<_, Proveedor>(
        r#"
        SELECT * FROM proveedores
        WHERE cedula LIKE ? OR nombre LIKE ? OR apellido LIKE ?
        ORDER BY created_at DESC
        LIMIT ?
        "#,
    )
    .bind(&pattern)
    .bind(&pattern)
    .bind(&pattern)
    .bind(limit)
    .fetch_all(pool)
    .await
}

pub async fn update(
    pool: &SqlitePool,
    id: &str,
    input: UpdateProveedorInput,
) -> Result<Proveedor, sqlx::Error> {
    let now = Utc::now().to_rfc3339();

    // Construcción dinámica de la query
    // Por simplicidad en MVP, actualizamos campos si vienen, pero SQLx estático prefiere queries fijas.
    // Usaremos COALESCE o lógica condicional en la app.
    // Para simplificar, haremos un fetch previo + update selectivo.

    // NOTA: Para producción robusta, usar query builder. Aquí usaremos un update fijo con COALESCE en SQL.

    sqlx::query_as::<_, Proveedor>(
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
        RETURNING *
        "#,
    )
    .bind(input.nombre)
    .bind(input.segundo_nombre)
    .bind(input.apellido)
    .bind(input.segundo_apellido)
    .bind(input.empresa_id)
    .bind(input.estado)
    .bind(&now)
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn delete(pool: &SqlitePool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM proveedores WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map(|_| ())
}
