// ==========================================
// src/db/surrealdb_role_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::models::role::{Role, RoleCreateDTO};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use surrealdb::RecordId;

pub async fn find_all() -> Result<Vec<Role>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db.query("SELECT * FROM role ORDER BY is_system DESC, name ASC").await?;
    Ok(result.take(0)?)
}

pub async fn find_by_id(id: &RecordId) -> Result<Option<Role>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db.query("SELECT * FROM $id").bind(("id", id.clone())).await?;
    Ok(result.take(0)?)
}

pub async fn get_permissions(role_id: &RecordId) -> Result<Vec<String>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db.query("SELECT permissions FROM $id").bind(("id", role_id.clone())).await?;

    #[derive(serde::Deserialize)]
    struct Perms {
        permissions: Option<Vec<String>>,
    }

    let row: Option<Perms> = result.take(0)?;
    Ok(row.and_then(|r| r.permissions).unwrap_or_default())
}

pub async fn create(id: &str, dto: RoleCreateDTO) -> Result<Role, SurrealDbError> {
    let db = get_db().await?;

    let mut result = db
        .query(
            r#"
            CREATE type::thing('role', $id) CONTENT $dto
        "#,
        )
        .bind(("id", id.to_string()))
        .bind(("dto", dto))
        .await?;

    let created: Option<Role> = result.take(0)?;
    created.ok_or(SurrealDbError::Query("Error creando rol".to_string()))
}

pub async fn update(
    id: &RecordId,
    data: serde_json::Value,
) -> Result<Option<Role>, SurrealDbError> {
    let db = get_db().await?;
    let mut result =
        db.query("UPDATE $id MERGE $data").bind(("id", id.clone())).bind(("data", data)).await?;

    Ok(result.take(0)?)
}

pub async fn delete(id: &RecordId) -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    db.query("DELETE $id").bind(("id", id.clone())).await?;
    Ok(())
}

pub async fn exists_by_name(name: &str) -> Result<bool, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT count() FROM role WHERE name = $name GROUP ALL")
        .bind(("name", name.to_string()))
        .await?;

    #[derive(serde::Deserialize)]
    struct Count {
        count: i64,
    }

    let rows: Vec<Count> = result.take(0)?;
    Ok(rows.first().map(|c| c.count > 0).unwrap_or(false))
}
