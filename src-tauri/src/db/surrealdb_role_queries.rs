// ==========================================
// src/db/surrealdb_role_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::models::role::Role;
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use serde::Deserialize;

pub async fn find_all() -> Result<Vec<Role>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db.query("SELECT * FROM role ORDER BY is_system DESC, name ASC").await?;
    Ok(result.take(0)?)
}

pub async fn find_by_id(id: &str) -> Result<Option<Role>, SurrealDbError> {
    let db = get_db().await?;
    let id_only = id.strip_prefix("role:").unwrap_or(id).to_string();
    let mut result =
        db.query("SELECT * FROM type::thing('role', $id)").bind(("id", id_only)).await?;
    Ok(result.take(0)?)
}

pub async fn get_permissions(role_id: &str) -> Result<Vec<String>, SurrealDbError> {
    let db = get_db().await?;
    let id_only = role_id.strip_prefix("role:").unwrap_or(role_id).to_string();

    let mut result =
        db.query("SELECT permissions FROM type::thing('role', $id)").bind(("id", id_only)).await?;

    #[derive(Deserialize)]
    struct Perms {
        permissions: Option<Vec<String>>,
    }

    let row: Option<Perms> = result.take(0)?;
    Ok(row.and_then(|r| r.permissions).unwrap_or_default())
}

pub async fn create(
    id: &str,
    name: &str,
    description: Option<&str>,
    permissions: Vec<String>,
    is_system: bool,
) -> Result<Role, SurrealDbError> {
    let db = get_db().await?;

    // Convert to owned types
    let id_owned = id.to_string();
    let name_owned = name.to_string();
    let description_owned = description.map(String::from);

    let mut result = db
        .query(
            r#"
            CREATE type::thing('role', $id) CONTENT {
                id: $id,
                name: $name,
                description: $description,
                is_system: $is_system,
                permissions: $permissions,
                created_at: time::now(),
                updated_at: time::now()
            }
        "#,
        )
        .bind(("id", id_owned))
        .bind(("name", name_owned))
        .bind(("description", description_owned))
        .bind(("is_system", is_system))
        .bind(("permissions", permissions))
        .await?;

    let created: Option<Role> = result.take(0)?;
    created.ok_or(SurrealDbError::Query("Error creando rol".to_string()))
}

pub async fn update(
    id: &str,
    name: Option<&str>,
    description: Option<&str>,
    permissions: Option<&[String]>,
) -> Result<Option<Role>, SurrealDbError> {
    let db = get_db().await?;
    let id_only = id.strip_prefix("role:").unwrap_or(id).to_string();

    let mut map = serde_json::Map::new();

    if let Some(v) = name {
        map.insert("name".to_string(), serde_json::json!(v));
    }
    if let Some(v) = description {
        map.insert("description".to_string(), serde_json::json!(v));
    }
    if let Some(v) = permissions {
        map.insert("permissions".to_string(), serde_json::json!(v));
    }

    let mut result = db
        .query("UPDATE type::thing('role', $id) MERGE $data")
        .bind(("id", id_only))
        .bind(("data", map))
        .await?;

    Ok(result.take(0)?)
}

pub async fn delete(id: &str) -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    let id_only = id.strip_prefix("role:").unwrap_or(id).to_string();
    db.query("DELETE type::thing('role', $id)").bind(("id", id_only)).await?;
    Ok(())
}

pub async fn exists_by_name(name: &str) -> Result<bool, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT count() FROM role WHERE name = $name GROUP ALL")
        .bind(("name", name.to_string()))
        .await?;

    #[derive(Deserialize)]
    struct Count {
        count: i64,
    }

    let rows: Vec<Count> = result.take(0)?;
    Ok(rows.first().map(|c| c.count > 0).unwrap_or(false))
}
