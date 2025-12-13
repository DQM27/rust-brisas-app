use crate::domain::visitante::{CreateVisitanteInput, Visitante};
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn create_visitante(
    pool: &SqlitePool,
    input: CreateVisitanteInput,
) -> Result<Visitante, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        r#"
        INSERT INTO visitantes (id, cedula, nombre, apellido, segundo_nombre, segundo_apellido, empresa, has_vehicle, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(&id)
    .bind(&input.cedula)
    .bind(&input.nombre)
    .bind(&input.apellido)
    .bind(&input.segundo_nombre)
    .bind(&input.segundo_apellido)
    .bind(&input.empresa)
    .bind(input.has_vehicle)
    .bind(&now)
    .bind(&now)
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
) -> Result<Option<Visitante>, sqlx::Error> {
    sqlx::query_as::<_, Visitante>(
        r#"
        SELECT id, cedula, nombre, apellido, segundo_nombre, segundo_apellido, empresa, has_vehicle, created_at, updated_at
        FROM visitantes
        WHERE cedula = ?
        "#
    )
    .bind(cedula)
    .fetch_optional(pool)
    .await
}

pub async fn get_visitante_by_id(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<Visitante>, sqlx::Error> {
    sqlx::query_as::<_, Visitante>(
        r#"
        SELECT id, cedula, nombre, apellido, segundo_nombre, segundo_apellido, empresa, has_vehicle, created_at, updated_at
        FROM visitantes
        WHERE id = ?
        "#
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn search_visitantes(
    pool: &SqlitePool,
    term: &str,
) -> Result<Vec<Visitante>, sqlx::Error> {
    let pattern = format!("%{}%", term);
    sqlx::query_as::<_, Visitante>(
        r#"
        SELECT id, cedula, nombre, apellido, segundo_nombre, segundo_apellido, empresa, has_vehicle, created_at, updated_at
        FROM visitantes
        WHERE cedula LIKE ? OR nombre LIKE ? OR apellido LIKE ?
        LIMIT 20
        "#
    )
    .bind(&pattern)
    .bind(&pattern)
    .bind(&pattern)
    .fetch_all(pool)
    .await
}
