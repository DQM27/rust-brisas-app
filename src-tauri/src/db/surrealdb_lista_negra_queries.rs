// ==========================================
// src/db/surrealdb_lista_negra_queries.rs
// ==========================================

use crate::models::lista_negra::BlockCheckResponse;
use crate::services::surrealdb_service::{get_surrealdb, SurrealDbError};
use serde::Deserialize;

pub async fn check_if_blocked_by_cedula(
    cedula: &str,
) -> Result<BlockCheckResponse, SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    let mut result = client
        .query("SELECT * FROM lista_negra WHERE cedula = $cedula AND is_active = true")
        .bind(("cedula", cedula.to_string()))
        .await?;

    #[derive(Deserialize)]
    struct LN {
        nivel_severidad: Option<String>,
        created_at: Option<String>,
    }
    let res: Option<LN> = result.take(0)?;

    match res {
        Some(ln) => Ok(BlockCheckResponse {
            is_blocked: true,
            nivel_severidad: ln.nivel_severidad,
            bloqueado_desde: ln.created_at,
        }),
        None => Ok(BlockCheckResponse {
            is_blocked: false,
            nivel_severidad: None,
            bloqueado_desde: None,
        }),
    }
}

pub async fn find_all() -> Result<Vec<crate::models::lista_negra::ListaNegra>, SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;
    let result: Vec<crate::models::lista_negra::ListaNegra> = client.select("lista_negra").await?;
    Ok(result)
}
