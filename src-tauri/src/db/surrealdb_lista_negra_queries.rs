// ==========================================
// src/db/surrealdb_lista_negra_queries.rs
// ==========================================

use crate::models::lista_negra::{BlockCheckResponse, ListaNegra};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use serde::Deserialize;
use surrealdb::Datetime;

pub async fn check_if_blocked_by_cedula(
    cedula: &str,
) -> Result<BlockCheckResponse, SurrealDbError> {
    let db = get_db().await?;

    let mut result = db
        .query("SELECT * FROM lista_negra WHERE cedula = $cedula AND is_active = true")
        .bind(("cedula", cedula.to_string()))
        .await?;

    #[derive(Deserialize)]
    struct LN {
        nivel_severidad: Option<String>,
        created_at: Option<Datetime>,
    }
    let res: Option<LN> = result.take(0)?;

    match res {
        Some(ln) => Ok(BlockCheckResponse {
            is_blocked: true,
            nivel_severidad: ln.nivel_severidad,
            bloqueado_desde: ln.created_at.map(|d| d.to_string()),
        }),
        None => Ok(BlockCheckResponse {
            is_blocked: false,
            nivel_severidad: None,
            bloqueado_desde: None,
        }),
    }
}

pub async fn find_all() -> Result<Vec<ListaNegra>, SurrealDbError> {
    let db = get_db().await?;
    let result: Vec<ListaNegra> = db.select("lista_negra").await?;
    Ok(result)
}
