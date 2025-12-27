// ==========================================
// src/db/surrealdb_proveedor_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::models::proveedor::{
    CreateProveedorInput, EstadoProveedor, Proveedor, UpdateProveedorInput,
};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use uuid::Uuid;

pub async fn create(input: CreateProveedorInput) -> Result<Proveedor, SurrealDbError> {
    let db = get_db().await?;
    let id = Uuid::new_v4().to_string();
    let estado = EstadoProveedor::Activo;

    // Normalize names to uppercase
    let nombre = input.nombre.to_uppercase();
    let segundo_nombre = input.segundo_nombre.map(|s| s.to_uppercase());
    let apellido = input.apellido.to_uppercase();
    let segundo_apellido = input.segundo_apellido.map(|s| s.to_uppercase());
    let empresa_id =
        input.empresa_id.strip_prefix("empresa:").unwrap_or(&input.empresa_id).to_string();

    let mut result = db
        .query(
            r#"
            CREATE type::thing('proveedor', $id) CONTENT {
                id: $id,
                cedula: $cedula,
                nombre: $nombre,
                segundo_nombre: $segundo_nombre,
                apellido: $apellido,
                segundo_apellido: $segundo_apellido,
                empresa_id: $empresa_id,
                estado: $estado,
                created_at: time::now(),
                updated_at: time::now()
            }
        "#,
        )
        .bind(("id", id))
        .bind(("cedula", input.cedula))
        .bind(("nombre", nombre))
        .bind(("segundo_nombre", segundo_nombre))
        .bind(("apellido", apellido))
        .bind(("segundo_apellido", segundo_apellido))
        .bind(("empresa_id", format!("empresa:{}", empresa_id)))
        .bind(("estado", estado))
        .await?;

    let created: Option<Proveedor> = result.take(0)?;
    created.ok_or(SurrealDbError::Query("Error creando proveedor".to_string()))
}

pub async fn find_by_id(id: &str) -> Result<Option<Proveedor>, SurrealDbError> {
    let db = get_db().await?;
    let id_only = id.strip_prefix("proveedor:").unwrap_or(id).to_string();
    let mut result =
        db.query("SELECT * FROM type::thing('proveedor', $id)").bind(("id", id_only)).await?;
    Ok(result.take(0)?)
}

pub async fn find_by_cedula(cedula: &str) -> Result<Option<Proveedor>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM proveedor WHERE cedula = $cedula LIMIT 1")
        .bind(("cedula", cedula.to_string()))
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_all() -> Result<Vec<Proveedor>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db.query("SELECT * FROM proveedor ORDER BY created_at DESC").await?;
    Ok(result.take(0)?)
}

pub async fn search(query: &str, limit: usize) -> Result<Vec<Proveedor>, SurrealDbError> {
    let db = get_db().await?;
    let query_upper = query.to_uppercase();

    let mut result = db
        .query(
            r#"
            SELECT * FROM proveedor 
            WHERE 
                nombre CONTAINS $q OR 
                apellido CONTAINS $q OR 
                cedula CONTAINS $q
            ORDER BY created_at DESC 
            LIMIT $limit
        "#,
        )
        .bind(("q", query_upper))
        .bind(("limit", limit))
        .await?;

    Ok(result.take(0)?)
}

pub async fn update(id: &str, input: UpdateProveedorInput) -> Result<Proveedor, SurrealDbError> {
    let db = get_db().await?;
    let id_only = id.strip_prefix("proveedor:").unwrap_or(id).to_string();

    let mut map = serde_json::Map::new();

    if let Some(v) = input.nombre {
        map.insert("nombre".to_string(), serde_json::json!(v.to_uppercase()));
    }
    if let Some(v) = input.segundo_nombre {
        map.insert("segundo_nombre".to_string(), serde_json::json!(v.to_uppercase()));
    }
    if let Some(v) = input.apellido {
        map.insert("apellido".to_string(), serde_json::json!(v.to_uppercase()));
    }
    if let Some(v) = input.segundo_apellido {
        map.insert("segundo_apellido".to_string(), serde_json::json!(v.to_uppercase()));
    }
    if let Some(v) = input.empresa_id {
        let emp_id = v.strip_prefix("empresa:").unwrap_or(&v);
        map.insert("empresa_id".to_string(), serde_json::json!(format!("empresa:{}", emp_id)));
    }
    if let Some(v) = input.estado {
        map.insert("estado".to_string(), serde_json::json!(v));
    }

    let mut result = db
        .query("UPDATE type::thing('proveedor', $id) MERGE $data")
        .bind(("id", id_only))
        .bind(("data", map))
        .await?;

    let updated: Option<Proveedor> = result.take(0)?;
    updated
        .ok_or(SurrealDbError::Query("Proveedor no encontrado o error al actualizar".to_string()))
}
