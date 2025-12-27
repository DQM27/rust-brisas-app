// ==========================================
// src/db/surrealdb_gafete_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::models::gafete::Gafete;
use crate::services::surrealdb_service::{get_db, SurrealDbError};

pub async fn get_gafete(numero: &str, tipo: &str) -> Result<Option<Gafete>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM gafete WHERE numero = $n AND tipo = $t LIMIT 1")
        .bind(("n", numero.to_string()))
        .bind(("t", tipo.to_string()))
        .await?;
    Ok(result.take(0)?)
}

pub async fn get_all_gafetes() -> Result<Vec<Gafete>, SurrealDbError> {
    let db = get_db().await?;
    Ok(db.select("gafete").await?)
}

pub async fn set_gafete_uso(numero: &str, tipo: &str, en_uso: bool) -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    db.query(
        "UPDATE gafete SET en_uso = $uso, updated_at = time::now() WHERE numero = $n AND tipo = $t",
    )
    .bind(("uso", en_uso))
    .bind(("n", numero.to_string()))
    .bind(("t", tipo.to_string()))
    .await?;
    Ok(())
}

pub async fn create_gafete(numero: &str, tipo: &str) -> Result<Option<Gafete>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query(
            r#"
            CREATE gafete CONTENT {
                numero: $numero,
                tipo: $tipo,
                estado: "activo",
                en_uso: false,
                created_at: time::now(),
                updated_at: time::now()
            }
        "#,
        )
        .bind(("numero", numero.to_string()))
        .bind(("tipo", tipo.to_string()))
        .await?;
    Ok(result.take(0)?)
}

pub async fn get_gafetes_disponibles(tipo: &str) -> Result<Vec<Gafete>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM gafete WHERE tipo = $tipo AND en_uso = false AND estado = 'activo' ORDER BY numero")
        .bind(("tipo", tipo.to_string()))
        .await?;
    Ok(result.take(0)?)
}
