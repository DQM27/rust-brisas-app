// src/db/surrealdb_gafete_queries.rs
use crate::models::gafete::Gafete;
use crate::services::surrealdb_service::{get_db, SurrealDbError};

pub async fn get_gafete(numero: &str, tipo: &str) -> Result<Option<Gafete>, SurrealDbError> {
    let client = get_db().await?;
    let numero_owned = numero.to_string();
    let tipo_owned = tipo.to_string();
    let mut result = client
        .query("SELECT * FROM gafetes WHERE numero = $n AND tipo = $t LIMIT 1")
        .bind(("n", numero_owned))
        .bind(("t", tipo_owned))
        .await?;
    Ok(result.take(0)?)
}

pub async fn get_all_gafetes() -> Result<Vec<Gafete>, SurrealDbError> {
    let client = get_db().await?;
    Ok(client.select("gafetes").await?)
}
