use crate::models::role::Role;
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use chrono::Utc;
use serde::Deserialize;

pub async fn find_all() -> Result<Vec<Role>, SurrealDbError> {
    let client = get_db().await?;
    let sql = "SELECT * FROM roles ORDER BY is_system DESC, name ASC";
    let mut result = client.query(sql).await?;
    Ok(result.take(0)?)
}

pub async fn find_by_id(id: &str) -> Result<Option<Role>, SurrealDbError> {
    let client = get_db().await?;
    let sql = r#"SELECT * FROM type::thing('roles', $id)"#;
    let mut result = client.query(sql).bind(("id", id.to_string())).await?;
    Ok(result.take(0)?)
}

// Retorna permisos as array of strings
pub async fn get_permissions(role_id: &str) -> Result<Vec<String>, SurrealDbError> {
    let client = get_db().await?;
    // Asumiendo que ahora guardamos permisos en un campo array 'permissions' dentro del rol
    let sql = "SELECT permissions FROM type::thing('roles', $id)";
    let mut result = client.query(sql).bind(("id", role_id.to_string())).await?;

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
    let client = get_db().await?;
    let now = Utc::now().to_rfc3339();

    let sql = r#"
        CREATE type::thing('roles', $id) CONTENT {
            id: $id,
            name: $name,
            description: $description,
            is_system: $is_system,
            permissions: $permissions,
            created_at: $now,
            updated_at: $now
        }
    "#;

    let mut result = client
        .query(sql)
        .bind(("id", id.to_string()))
        .bind(("name", name.to_string()))
        .bind(("description", description.map(String::from)))
        .bind(("is_system", is_system))
        .bind(("permissions", permissions.clone()))
        .bind(("now", now))
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
    let client = get_db().await?;
    let now = Utc::now().to_rfc3339();

    let mut map = serde_json::Map::new();
    map.insert("updated_at".to_string(), serde_json::json!(now));

    if let Some(v) = name {
        map.insert("name".to_string(), serde_json::json!(v));
    }
    if let Some(v) = description {
        map.insert("description".to_string(), serde_json::json!(v));
    }
    if let Some(v) = permissions {
        map.insert("permissions".to_string(), serde_json::json!(v));
    }

    let sql = "UPDATE type::thing('roles', $id) MERGE $data";
    let mut result = client.query(sql).bind(("id", id.to_string())).bind(("data", map)).await?;

    Ok(result.take(0)?)
}

pub async fn delete(id: &str) -> Result<(), SurrealDbError> {
    let client = get_db().await?;
    let sql = "DELETE type::thing('roles', $id)";
    let mut _result = client.query(sql).bind(("id", id.to_string())).await?;
    Ok(())
}

pub async fn exists_by_name(name: &str) -> Result<bool, SurrealDbError> {
    let client = get_db().await?;
    let sql = "SELECT count() FROM roles WHERE name = $name GROUP ALL";
    let mut result = client.query(sql).bind(("name", name.to_string())).await?;
    #[derive(Deserialize)]
    struct Count {
        count: i64,
    }
    let rows: Vec<Count> = result.take(0)?;
    Ok(rows.first().map(|c| c.count > 0).unwrap_or(false))
}
