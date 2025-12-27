// ==========================================
// src/db/surrealdb_contratista_queries.rs
// ==========================================

use crate::models::contratista::{Contratista, CreateContratistaInput, UpdateContratistaInput};
use crate::services::surrealdb_service::{get_db, SurrealDbError};

pub async fn create(input: CreateContratistaInput) -> Result<Contratista, SurrealDbError> {
    let db = get_db().await?;
    let empresa_id = input.empresa_id.strip_prefix("empresa:").unwrap_or(&input.empresa_id);

    let result: Option<Contratista> = db
        .query(
            r#"
            CREATE contratista CONTENT {
                cedula: $cedula,
                nombre: $nombre,
                segundo_nombre: $segundo_nombre,
                apellido: $apellido,
                segundo_apellido: $segundo_apellido,
                empresa_id: $empresa_id,
                fecha_vencimiento_praind: $fecha_vencimiento_praind,
                estado: "activo",
                created_at: time::now(),
                updated_at: time::now()
            }
        "#,
        )
        .bind(("cedula", input.cedula))
        .bind(("nombre", input.nombre))
        .bind(("segundo_nombre", input.segundo_nombre))
        .bind(("apellido", input.apellido))
        .bind(("segundo_apellido", input.segundo_apellido))
        .bind(("empresa_id", format!("empresa:{}", empresa_id)))
        .bind(("fecha_vencimiento_praind", input.fecha_vencimiento_praind))
        .await?
        .take(0)?;

    result.ok_or(SurrealDbError::Query("No se pudo crear el contratista".to_string()))
}

pub async fn find_by_id(id: &str) -> Result<Option<Contratista>, SurrealDbError> {
    let db = get_db().await?;
    let id_only = id.strip_prefix("contratista:").unwrap_or(id);
    let result: Option<Contratista> = db.select(("contratista", id_only)).await?;
    Ok(result)
}

pub async fn find_by_cedula(cedula: &str) -> Result<Option<Contratista>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM contratista WHERE cedula = $cedula")
        .bind(("cedula", cedula.to_string()))
        .await?;
    let contratista: Option<Contratista> = result.take(0)?;
    Ok(contratista)
}

pub async fn find_all() -> Result<Vec<Contratista>, SurrealDbError> {
    let db = get_db().await?;
    let result: Vec<Contratista> = db.select("contratista").await?;
    Ok(result)
}

pub async fn find_by_empresa(empresa_id: &str) -> Result<Vec<Contratista>, SurrealDbError> {
    let db = get_db().await?;
    let id_only = empresa_id.strip_prefix("empresa:").unwrap_or(empresa_id);
    let mut result = db
        .query("SELECT * FROM contratista WHERE empresa_id = $empresa_id")
        .bind(("empresa_id", format!("empresa:{}", id_only)))
        .await?;
    let contratistas: Vec<Contratista> = result.take(0)?;
    Ok(contratistas)
}

pub async fn update(
    id: &str,
    input: UpdateContratistaInput,
) -> Result<Contratista, SurrealDbError> {
    let db = get_db().await?;
    let id_only = id.strip_prefix("contratista:").unwrap_or(id);

    let mut update_data = serde_json::Map::new();
    if let Some(v) = input.nombre {
        update_data.insert("nombre".to_string(), serde_json::json!(v));
    }
    if let Some(v) = input.segundo_nombre {
        update_data.insert("segundo_nombre".to_string(), serde_json::json!(v));
    }
    if let Some(v) = input.apellido {
        update_data.insert("apellido".to_string(), serde_json::json!(v));
    }
    if let Some(v) = input.segundo_apellido {
        update_data.insert("segundo_apellido".to_string(), serde_json::json!(v));
    }
    if let Some(v) = input.empresa_id {
        let emp_id = v.strip_prefix("empresa:").unwrap_or(&v);
        update_data
            .insert("empresa_id".to_string(), serde_json::json!(format!("empresa:{}", emp_id)));
    }
    if let Some(v) = input.fecha_vencimiento_praind {
        update_data.insert("fecha_vencimiento_praind".to_string(), serde_json::json!(v));
    }

    let result: Option<Contratista> =
        db.update(("contratista", id_only)).merge(serde_json::Value::Object(update_data)).await?;

    result.ok_or(SurrealDbError::Query("No se pudo actualizar el contratista".to_string()))
}

pub async fn update_status(id: &str, estado: &str) -> Result<Contratista, SurrealDbError> {
    let db = get_db().await?;
    let id_only = id.strip_prefix("contratista:").unwrap_or(id);

    let result: Option<Contratista> =
        db.update(("contratista", id_only)).merge(serde_json::json!({ "estado": estado })).await?;

    result.ok_or(SurrealDbError::Query("No se pudo actualizar el estado".to_string()))
}

pub async fn delete(id: &str) -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    let id_only = id.strip_prefix("contratista:").unwrap_or(id);
    let _: Option<Contratista> = db.delete(("contratista", id_only)).await?;
    Ok(())
}

pub async fn get_empresa_nombre(empresa_id: &str) -> Result<String, SurrealDbError> {
    let db = get_db().await?;
    let id_only = empresa_id.strip_prefix("empresa:").unwrap_or(empresa_id);

    let mut result = db
        .query("SELECT nombre FROM type::thing('empresa', $id)")
        .bind(("id", id_only.to_string()))
        .await?;

    #[derive(serde::Deserialize)]
    struct NombreResult {
        nombre: String,
    }

    let res: Option<NombreResult> = result.take(0)?;
    Ok(res.map(|r| r.nombre).unwrap_or_else(|| "Empresa desconocida".to_string()))
}
