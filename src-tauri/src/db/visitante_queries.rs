// ==========================================
// src/db/visitante_queries.rs
// ==========================================
// Capa de data access: queries SQL puras
// Strict Mode: Uso de query_as! para validación y DTO intermedio

use crate::domain::visitante::{CreateVisitanteInput, Visitante};
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

// ==========================================
// DTO & CONVERSION
// ==========================================

#[derive(sqlx::FromRow)]
struct VisitanteRow {
    id: Option<String>,
    cedula: Option<String>,
    nombre: Option<String>,
    apellido: Option<String>,
    segundo_nombre: Option<String>,
    segundo_apellido: Option<String>,
    empresa: Option<String>,
    has_vehicle: Option<bool>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

impl From<VisitanteRow> for Visitante {
    fn from(row: VisitanteRow) -> Self {
        Visitante {
            id: row.id.unwrap_or_default(),
            cedula: row.cedula.unwrap_or_default(),
            nombre: row.nombre.unwrap_or_default(),
            apellido: row.apellido.unwrap_or_default(),
            segundo_nombre: row.segundo_nombre,
            segundo_apellido: row.segundo_apellido,
            empresa: row.empresa, // Option<String> stays Option
            has_vehicle: row.has_vehicle.unwrap_or(false),
            created_at: row.created_at.unwrap_or_default(),
            updated_at: row.updated_at.unwrap_or_default(),
        }
    }
}

// ==========================================
// QUERIES
// ==========================================

pub async fn create_visitante(
    pool: &SqlitePool,
    input: CreateVisitanteInput,
) -> sqlx::Result<Visitante> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    sqlx::query!(
        r#"
        INSERT INTO visitantes (id, cedula, nombre, apellido, segundo_nombre, segundo_apellido, empresa, has_vehicle, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        id,
        input.cedula,
        input.nombre,
        input.apellido,
        input.segundo_nombre,
        input.segundo_apellido,
        input.empresa,
        input.has_vehicle,
        now,
        now
    )
    .execute(pool)
    .await?;

    Ok(Visitante {
        id,
        cedula: input.cedula,
        nombre: input.nombre,
        apellido: input.apellido,
        segundo_nombre: input.segundo_nombre,
        segundo_apellido: input.segundo_apellido,
        empresa: input.empresa,
        has_vehicle: input.has_vehicle,
        created_at: now.clone(),
        updated_at: now,
    })
}

pub async fn get_visitante_by_cedula(
    pool: &SqlitePool,
    cedula: &str,
) -> sqlx::Result<Option<Visitante>> {
    let row = sqlx::query_as!(
        VisitanteRow,
        r#"
        SELECT 
            id, cedula, nombre, apellido, segundo_nombre, segundo_apellido, 
            empresa, 
            has_vehicle as "has_vehicle: bool",
            CAST(created_at AS TEXT) as created_at,
            CAST(updated_at AS TEXT) as updated_at
        FROM visitantes
        WHERE cedula = ?
        "#,
        cedula
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| r.into()))
}

pub async fn get_visitante_by_id(pool: &SqlitePool, id: &str) -> sqlx::Result<Option<Visitante>> {
    let row = sqlx::query_as!(
        VisitanteRow,
        r#"
        SELECT 
            id, cedula, nombre, apellido, segundo_nombre, segundo_apellido, 
            empresa, 
            has_vehicle as "has_vehicle: bool",
            CAST(created_at AS TEXT) as created_at,
            CAST(updated_at AS TEXT) as updated_at
        FROM visitantes
        WHERE id = ?
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| r.into()))
}

pub async fn search_visitantes(pool: &SqlitePool, term: &str) -> sqlx::Result<Vec<Visitante>> {
    let pattern = format!("%{}%", term);
    let rows = sqlx::query_as!(
        VisitanteRow,
        r#"
        SELECT 
            id, cedula, nombre, apellido, segundo_nombre, segundo_apellido, 
            empresa, 
            has_vehicle as "has_vehicle: bool",
            CAST(created_at AS TEXT) as created_at,
            CAST(updated_at AS TEXT) as updated_at
        FROM visitantes
        WHERE cedula LIKE ? OR nombre LIKE ? OR apellido LIKE ?
        LIMIT 20
        "#,
        pattern,
        pattern,
        pattern
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| r.into()).collect())
}
