// ==========================================
// src/db/surrealdb_vehiculo_queries.rs
// ==========================================
use crate::models::vehiculo::{CreateVehiculoInput, UpdateVehiculoInput, Vehiculo};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use chrono::Utc;
use serde_json::json;

pub async fn insert(input: CreateVehiculoInput) -> Result<Vehiculo, SurrealDbError> {
    let client = get_db().await?;
    let now = Utc::now().to_rfc3339();

    let contratista_link = input.contratista_id.map(|id| format!("contratistas:{}", id));
    let proveedor_link = input.proveedor_id.map(|id| format!("proveedores:{}", id));

    let res: Option<Vehiculo> = client
        .create("vehiculos")
        .content(json!({
            "placa": input.placa,
            "marca": input.marca,
            "modelo": input.modelo,
            "color": input.color,
            "tipo_vehiculo": input.tipo_vehiculo,
            "is_active": true,
            "contratista_id": contratista_link,
            "proveedor_id": proveedor_link,
            "created_at": now,
            "updated_at": now
        }))
        .await?;

    res.ok_or(SurrealDbError::TransactionError("Error al insertar vehículo".to_string()))
}

pub async fn find_by_id(id: &str) -> Result<Option<Vehiculo>, SurrealDbError> {
    let client = get_db().await?;
    let id_only = id.strip_prefix("vehiculos:").unwrap_or(id);
    let res: Option<Vehiculo> = client.select(("vehiculos", id_only.to_string())).await?;
    Ok(res)
}

pub async fn find_by_placa(placa: &str) -> Result<Option<Vehiculo>, SurrealDbError> {
    let client = get_db().await?;
    let mut result = client
        .query("SELECT * FROM vehiculos WHERE placa = $placa AND is_active = true")
        .bind(("placa", placa.to_string()))
        .await?;
    let vehiculo: Option<Vehiculo> = result.take(0)?;
    Ok(vehiculo)
}

pub async fn find_all() -> Result<Vec<Vehiculo>, SurrealDbError> {
    let client = get_db().await?;
    let res: Vec<Vehiculo> = client.select("vehiculos").await?;
    Ok(res)
}

pub async fn find_activos() -> Result<Vec<Vehiculo>, SurrealDbError> {
    let client = get_db().await?;
    let mut result = client.query("SELECT * FROM vehiculos WHERE is_active = true").await?;
    let res: Vec<Vehiculo> = result.take(0)?;
    Ok(res)
}

pub async fn update(id: &str, input: UpdateVehiculoInput) -> Result<Vehiculo, SurrealDbError> {
    let client = get_db().await?;
    let id_only = id.strip_prefix("vehiculos:").unwrap_or(id);
    let now = Utc::now().to_rfc3339();

    let mut update_data = json!({ "updated_at": now });
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

    let res: Option<Vehiculo> =
        client.update(("vehiculos", id_only.to_string())).merge(update_data).await?;

    res.ok_or(SurrealDbError::TransactionError("Error al actualizar vehículo".to_string()))
}

pub async fn delete(id: &str) -> Result<(), SurrealDbError> {
    let client = get_db().await?;
    let id_only = id.strip_prefix("vehiculos:").unwrap_or(id);
    let _: Option<Vehiculo> = client.delete(("vehiculos", id_only.to_string())).await?;
    Ok(())
}

pub async fn count_by_placa(placa: &str) -> Result<i64, SurrealDbError> {
    let client = get_db().await?;
    let mut result = client
        .query("SELECT count() FROM vehiculos WHERE placa = $placa AND is_active = true")
        .bind(("placa", placa.to_string()))
        .await?;
    let count_obj: Option<serde_json::Value> = result.take(0)?;
    let count = count_obj.and_then(|v| v.get("count").and_then(|c| c.as_i64())).unwrap_or(0);
    Ok(count)
}

pub async fn find_by_contratista(contratista_id: &str) -> Result<Vec<Vehiculo>, SurrealDbError> {
    let client = get_db().await?;
    let id_only = contratista_id.strip_prefix("contratistas:").unwrap_or(contratista_id);
    let contratista_link = format!("contratistas:{}", id_only);
    let mut result = client
        .query(
            "SELECT * FROM vehiculos WHERE contratista_id = $contratista_id AND is_active = true",
        )
        .bind(("contratista_id", contratista_link))
        .await?;
    let vehiculos: Vec<Vehiculo> = result.take(0)?;
    Ok(vehiculos)
}

pub async fn find_by_proveedor(proveedor_id: &str) -> Result<Vec<Vehiculo>, SurrealDbError> {
    let client = get_db().await?;
    let mut result = client
        .query("SELECT * FROM vehiculos WHERE proveedor_id = type::thing('proveedores', $pid) AND is_active = true")
        .bind(("pid", proveedor_id.to_string()))
        .await?;
    let vehiculos: Vec<Vehiculo> = result.take(0)?;
    Ok(vehiculos)
}
