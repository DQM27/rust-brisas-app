// ==========================================
// src/db/surrealdb_vehiculo_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::models::vehiculo::{CreateVehiculoInput, UpdateVehiculoInput, Vehiculo};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use serde_json::json;

pub async fn insert(input: CreateVehiculoInput) -> Result<Vehiculo, SurrealDbError> {
    let db = get_db().await?;

    let contratista_link = input.contratista_id.map(|id| {
        let id_only = id.strip_prefix("contratista:").unwrap_or(&id);
        format!("contratista:{}", id_only)
    });
    let proveedor_link = input.proveedor_id.map(|id| {
        let id_only = id.strip_prefix("proveedor:").unwrap_or(&id);
        format!("proveedor:{}", id_only)
    });

    let res: Option<Vehiculo> = db
        .query(
            r#"
            CREATE vehiculo CONTENT {
                placa: $placa,
                marca: $marca,
                modelo: $modelo,
                color: $color,
                tipo_vehiculo: $tipo_vehiculo,
                is_active: true,
                contratista_id: $contratista_id,
                proveedor_id: $proveedor_id,
                created_at: time::now(),
                updated_at: time::now()
            }
        "#,
        )
        .bind(("placa", input.placa))
        .bind(("marca", input.marca))
        .bind(("modelo", input.modelo))
        .bind(("color", input.color))
        .bind(("tipo_vehiculo", input.tipo_vehiculo))
        .bind(("contratista_id", contratista_link))
        .bind(("proveedor_id", proveedor_link))
        .await?
        .take(0)?;

    res.ok_or(SurrealDbError::TransactionError("Error al insertar vehículo".to_string()))
}

pub async fn find_by_id(id: &str) -> Result<Option<Vehiculo>, SurrealDbError> {
    let db = get_db().await?;
    let id_only = id.strip_prefix("vehiculo:").unwrap_or(id).to_string();
    let res: Option<Vehiculo> = db.select(("vehiculo", id_only)).await?;
    Ok(res)
}

pub async fn find_by_placa(placa: &str) -> Result<Option<Vehiculo>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM vehiculo WHERE placa = $placa AND is_active = true")
        .bind(("placa", placa.to_string()))
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_all() -> Result<Vec<Vehiculo>, SurrealDbError> {
    let db = get_db().await?;
    let res: Vec<Vehiculo> = db.select("vehiculo").await?;
    Ok(res)
}

pub async fn find_activos() -> Result<Vec<Vehiculo>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db.query("SELECT * FROM vehiculo WHERE is_active = true").await?;
    Ok(result.take(0)?)
}

pub async fn update(id: &str, input: UpdateVehiculoInput) -> Result<Vehiculo, SurrealDbError> {
    let db = get_db().await?;
    let id_only = id.strip_prefix("vehiculo:").unwrap_or(id).to_string();

    let mut update_data = json!({});
    if let Some(tipo) = input.tipo_vehiculo {
        update_data["tipo_vehiculo"] = json!(tipo);
    }
    if let Some(marca) = input.marca {
        update_data["marca"] = json!(marca);
    }
    if let Some(modelo) = input.modelo {
        update_data["modelo"] = json!(modelo);
    }
    if let Some(color) = input.color {
        update_data["color"] = json!(color);
    }
    if let Some(active) = input.is_active {
        update_data["is_active"] = json!(active);
    }

    let res: Option<Vehiculo> = db.update(("vehiculo", id_only)).merge(update_data).await?;
    res.ok_or(SurrealDbError::TransactionError("Error al actualizar vehículo".to_string()))
}

pub async fn delete(id: &str) -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    let id_only = id.strip_prefix("vehiculo:").unwrap_or(id).to_string();
    let _: Option<Vehiculo> = db.delete(("vehiculo", id_only)).await?;
    Ok(())
}

pub async fn count_by_placa(placa: &str) -> Result<i64, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT count() FROM vehiculo WHERE placa = $placa AND is_active = true GROUP ALL")
        .bind(("placa", placa.to_string()))
        .await?;

    let count_obj: Option<serde_json::Value> = result.take(0)?;
    let count = count_obj.and_then(|v| v.get("count").and_then(|c| c.as_i64())).unwrap_or(0);
    Ok(count)
}

pub async fn find_by_contratista(contratista_id: &str) -> Result<Vec<Vehiculo>, SurrealDbError> {
    let db = get_db().await?;
    let id_only = contratista_id.strip_prefix("contratista:").unwrap_or(contratista_id);
    let contratista_link = format!("contratista:{}", id_only);

    let mut result = db
        .query("SELECT * FROM vehiculo WHERE contratista_id = $contratista_id AND is_active = true")
        .bind(("contratista_id", contratista_link))
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_by_proveedor(proveedor_id: &str) -> Result<Vec<Vehiculo>, SurrealDbError> {
    let db = get_db().await?;
    let id_only = proveedor_id.strip_prefix("proveedor:").unwrap_or(proveedor_id);
    let proveedor_link = format!("proveedor:{}", id_only);

    let mut result = db
        .query("SELECT * FROM vehiculo WHERE proveedor_id = $proveedor_id AND is_active = true")
        .bind(("proveedor_id", proveedor_link))
        .await?;
    Ok(result.take(0)?)
}
