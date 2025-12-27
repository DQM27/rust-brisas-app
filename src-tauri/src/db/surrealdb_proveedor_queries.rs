use crate::models::proveedor::{
    CreateProveedorInput, EstadoProveedor, Proveedor, UpdateProveedorInput,
};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use chrono::Utc;
use uuid::Uuid;

pub async fn create(input: CreateProveedorInput) -> Result<Proveedor, SurrealDbError> {
    let client = get_db().await?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let estado = EstadoProveedor::Activo;

    // Asignar nombres
    let nombre = input.nombre.to_uppercase();
    let segundo_nombre = input.segundo_nombre.map(|s| s.to_uppercase());
    let apellido = input.apellido.to_uppercase();
    let segundo_apellido = input.segundo_apellido.map(|s| s.to_uppercase());

    let sql = r#"
        CREATE type::thing('proveedores', $id) CONTENT {
            id: $id,
            cedula: $cedula,
            nombre: $nombre,
            segundo_nombre: $segundo_nombre,
            apellido: $apellido,
            segundo_apellido: $segundo_apellido,
            empresa_id: $empresa_id,
            estado: $estado,
            created_at: $now,
            updated_at: $now
        }
    "#;

    let mut result = client
        .query(sql)
        .bind(("id", id.to_string()))
        .bind(("cedula", input.cedula.to_string()))
        .bind(("nombre", nombre))
        .bind(("segundo_nombre", segundo_nombre))
        .bind(("apellido", apellido))
        .bind(("segundo_apellido", segundo_apellido))
        .bind(("empresa_id", input.empresa_id.to_string()))
        .bind(("estado", estado))
        .bind(("now", now))
        .await?;

    let created: Option<Proveedor> = result.take(0)?;
    created.ok_or(SurrealDbError::Query("Error creando proveedor".to_string()))
}

pub async fn find_by_id(id: &str) -> Result<Option<Proveedor>, SurrealDbError> {
    let client = get_db().await?;
    let sql = "SELECT * FROM type::thing('proveedores', $id)";
    let mut result = client.query(sql).bind(("id", id.to_string())).await?;
    Ok(result.take(0)?)
}

pub async fn find_by_cedula(cedula: &str) -> Result<Option<Proveedor>, SurrealDbError> {
    let client = get_db().await?;
    let sql = "SELECT * FROM proveedores WHERE cedula = $cedula LIMIT 1";
    let mut result = client.query(sql).bind(("cedula", cedula.to_string())).await?;
    Ok(result.take(0)?)
}

pub async fn find_all() -> Result<Vec<Proveedor>, SurrealDbError> {
    let client = get_db().await?;
    let sql = "SELECT * FROM proveedores ORDER BY created_at DESC";
    let mut result = client.query(sql).await?;
    Ok(result.take(0)?)
}

pub async fn search(query: &str, limit: usize) -> Result<Vec<Proveedor>, SurrealDbError> {
    let client = get_db().await?;
    let sql = r#"
        SELECT * FROM proveedores 
        WHERE 
            nombre CONTAINS $q OR 
            apellido CONTAINS $q OR 
            cedula CONTAINS $q
        ORDER BY created_at DESC 
        LIMIT $limit
    "#;

    let mut result =
        client.query(sql).bind(("q", query.to_uppercase())).bind(("limit", limit)).await?;

    Ok(result.take(0)?)
}

pub async fn update(id: &str, input: UpdateProveedorInput) -> Result<Proveedor, SurrealDbError> {
    let client = get_db().await?;
    let now = Utc::now().to_rfc3339();

    let mut map = serde_json::Map::new();
    map.insert("updated_at".to_string(), serde_json::json!(now));

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
        map.insert("empresa_id".to_string(), serde_json::json!(v));
    }

    if let Some(v) = input.estado {
        // Validar parsing si se requiere, o confiar en que es un string válido para el enum
        // El input viene como String. El enum espera ACTIVO/INACTIVO/SUSPENDIDO.
        map.insert("estado".to_string(), serde_json::json!(v));
    }

    let sql = "UPDATE type::thing('proveedores', $id) MERGE $data";

    let mut result = client.query(sql).bind(("id", id.to_string())).bind(("data", map)).await?;

    let updated: Option<Proveedor> = result.take(0)?;
    updated
        .ok_or(SurrealDbError::Query("Proveedor no encontrado o error al actualizar".to_string()))
}
