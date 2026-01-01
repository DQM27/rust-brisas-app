// ==========================================
// src/db/surrealdb_gafete_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

// ==========================================
// src/db/surrealdb_gafete_queries.rs
// ==========================================

use crate::models::gafete::{Gafete, GafeteCreateDTO};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use surrealdb::RecordId;

pub async fn get_gafete(numero: i32, tipo: &str) -> Result<Option<Gafete>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM gafete WHERE numero = $n AND tipo = $t LIMIT 1")
        .bind(("n", numero))
        .bind(("t", tipo.to_string()))
        .await?;
    Ok(result.take(0)?)
}

pub async fn get_all_gafetes() -> Result<Vec<Gafete>, SurrealDbError> {
    let db = get_db().await?;
    Ok(db.select("gafete").await?)
}

pub async fn set_gafete_uso(id: &RecordId, en_uso: bool) -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    db.query("UPDATE $id SET en_uso = $uso, updated_at = time::now()")
        .bind(("id", id.clone()))
        .bind(("uso", en_uso))
        .await?;
    Ok(())
}

pub async fn create_gafete(dto: GafeteCreateDTO) -> Result<Gafete, SurrealDbError> {
    let db = get_db().await?;
    let result: Option<Gafete> =
        db.query("CREATE gafete CONTENT $dto").bind(("dto", dto)).await?.take(0)?;
    result.ok_or(SurrealDbError::Query("No se pudo crear el gafete".to_string()))
}

pub async fn find_by_id(id: &RecordId) -> Result<Option<Gafete>, SurrealDbError> {
    let db = get_db().await?;
    Ok(db.select(id.clone()).await?)
}

pub async fn update_estado(id: &RecordId, estado: &str) -> Result<Gafete, SurrealDbError> {
    let db = get_db().await?;
    let result: Option<Gafete> = db
        .query("UPDATE $id SET estado = $estado, updated_at = time::now()")
        .bind(("id", id.clone()))
        .bind(("estado", estado.to_string()))
        .await?
        .take(0)?;
    result.ok_or(SurrealDbError::Query("No se pudo actualizar el estado del gafete".to_string()))
}

pub async fn delete_gafete_by_id(id: &RecordId) -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    let _: Option<Gafete> = db.delete(id.clone()).await?;
    Ok(())
}

pub async fn get_gafetes_disponibles(tipo: &str) -> Result<Vec<Gafete>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM gafete WHERE tipo = $tipo AND en_uso = false AND estado = 'activo' ORDER BY numero")
        .bind(("tipo", tipo.to_string()))
        .await?;
    Ok(result.take(0)?)
}
