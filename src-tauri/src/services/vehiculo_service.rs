// ==========================================
// src/services/vehiculo_service.rs
// ==========================================
use crate::db::surrealdb_contratista_queries as contratista_db;
use crate::db::surrealdb_proveedor_queries as proveedor_db;
use crate::db::surrealdb_vehiculo_queries as db;
use crate::db::surrealdb_visitante_queries as visitante_db;
use crate::domain::errors::VehiculoError;
use crate::domain::vehiculo as domain;
use crate::models::vehiculo::{
    TipoVehiculo, TipoVehiculoStats, UpdateVehiculoInput, VehiculoCreateDTO, VehiculoListResponse,
    VehiculoResponse, VehiculoUpdateDTO,
};
use crate::services::surrealdb_service::SurrealDbError;
use chrono::Utc;
use log::error;
use surrealdb::RecordId;

fn map_db_error(e: SurrealDbError) -> VehiculoError {
    error!("Error de base de datos (SurrealDB): {}", e);
    VehiculoError::Database(e.to_string())
}

/// Helper para parsear ID de vehículo (acepta con o sin prefijo)
fn parse_vehiculo_id(id_str: &str) -> RecordId {
    if id_str.contains(':') {
        let parts: Vec<&str> = id_str.split(':').collect();
        RecordId::from_table_key(parts[0], parts[1])
    } else {
        RecordId::from_table_key("vehiculo", id_str)
    }
}

/// Helper para parsear cualquier ID de propietario (contratista, proveedor, visitante)
fn parse_propietario_id(id_str: &str) -> RecordId {
    if id_str.contains(':') {
        let parts: Vec<&str> = id_str.split(':').collect();
        RecordId::from_table_key(parts[0], parts[1])
    } else {
        // Fallback or error? For now assume it's a RecordId string or we need to know the table.
        // If it's just a key, this might fail if we don't know the table.
        // But usually we receive full RecordId strings from frontend.
        RecordId::from_table_key("contratista", id_str)
    }
}

pub async fn create_vehiculo(
    input: crate::models::vehiculo::CreateVehiculoInput,
) -> Result<VehiculoResponse, VehiculoError> {
    domain::validar_create_input(&input)?;
    let placa_normalizada = domain::normalizar_placa(&input.placa);
    let tipo_vehiculo = domain::validar_tipo_vehiculo(&input.tipo_vehiculo)?;

    let propietario_id = parse_propietario_id(&input.propietario_id);

    // Verificar que el propietario existe
    let exists = match propietario_id.table() {
        "contratista" => {
            contratista_db::find_by_id(&propietario_id).await.map_err(map_db_error)?.is_some()
        }
        "proveedor" => {
            proveedor_db::find_by_id(&propietario_id).await.map_err(map_db_error)?.is_some()
        }
        "visitante" => {
            visitante_db::find_by_id(&propietario_id).await.map_err(map_db_error)?.is_some()
        }
        _ => return Err(VehiculoError::Validation("Tipo de propietario no válido".to_string())),
    };

    if !exists {
        return Err(VehiculoError::Validation(format!(
            "El propietario ({}) no existe",
            propietario_id.table()
        )));
    }

    let count = db::count_by_placa(&placa_normalizada).await.map_err(map_db_error)?;
    if count > 0 {
        return Err(VehiculoError::PlacaExists);
    }

    let dto = VehiculoCreateDTO {
        propietario: propietario_id,
        tipo_vehiculo,
        placa: placa_normalizada,
        marca: input.marca.as_ref().map(|s| s.trim().to_uppercase()),
        modelo: input.modelo.as_ref().map(|s| s.trim().to_uppercase()),
        color: input.color.as_ref().map(|s| s.trim().to_uppercase()),
        is_active: true,
    };

    let vehiculo_creado = db::insert(dto).await.map_err(map_db_error)?;
    Ok(VehiculoResponse::from(vehiculo_creado))
}

pub async fn get_vehiculo_by_id(id_str: &str) -> Result<VehiculoResponse, VehiculoError> {
    let id = parse_vehiculo_id(id_str);
    let vehiculo =
        db::find_by_id_fetched(&id).await.map_err(map_db_error)?.ok_or(VehiculoError::NotFound)?;

    Ok(VehiculoResponse::from_fetched(vehiculo))
}

pub async fn get_vehiculo_by_placa(placa: String) -> Result<VehiculoResponse, VehiculoError> {
    let placa_normalizada = domain::normalizar_placa(&placa);
    let vehiculo = db::find_by_placa(&placa_normalizada)
        .await
        .map_err(map_db_error)?
        .ok_or(VehiculoError::NotFound)?;
    Ok(VehiculoResponse::from_fetched(vehiculo))
}

pub async fn get_all_vehiculos() -> Result<VehiculoListResponse, VehiculoError> {
    let vehiculos = db::find_all_fetched().await.map_err(map_db_error)?;
    let mut vehiculo_responses = Vec::with_capacity(vehiculos.len());
    for vehiculo in vehiculos {
        vehiculo_responses.push(VehiculoResponse::from_fetched(vehiculo));
    }
    let total = vehiculo_responses.len();
    let activos = vehiculo_responses.iter().filter(|v| v.is_active).count();
    let inactivos = total - activos;
    let motocicletas =
        vehiculo_responses.iter().filter(|v| v.tipo_vehiculo == TipoVehiculo::Motocicleta).count();
    let automoviles =
        vehiculo_responses.iter().filter(|v| v.tipo_vehiculo == TipoVehiculo::Automovil).count();
    Ok(VehiculoListResponse {
        vehiculos: vehiculo_responses,
        total,
        activos,
        inactivos,
        por_tipo: TipoVehiculoStats { motocicletas, automoviles },
    })
}

pub async fn get_vehiculos_activos() -> Result<Vec<VehiculoResponse>, VehiculoError> {
    let vehiculos = db::find_activos_fetched().await.map_err(map_db_error)?;
    let mut vehiculo_responses = Vec::with_capacity(vehiculos.len());
    for vehiculo in vehiculos {
        vehiculo_responses.push(VehiculoResponse::from_fetched(vehiculo));
    }
    Ok(vehiculo_responses)
}

pub async fn get_vehiculos_by_propietario(
    id_str: String,
) -> Result<Vec<VehiculoResponse>, VehiculoError> {
    let id = parse_propietario_id(&id_str);
    let vehiculos = db::find_by_propietario(&id).await.map_err(map_db_error)?;
    let mut vehiculo_responses = Vec::with_capacity(vehiculos.len());
    for vehiculo in vehiculos {
        vehiculo_responses.push(VehiculoResponse::from_fetched(vehiculo));
    }
    Ok(vehiculo_responses)
}

pub async fn update_vehiculo(
    id_str: String,
    input: UpdateVehiculoInput,
) -> Result<VehiculoResponse, VehiculoError> {
    let id = parse_vehiculo_id(&id_str);
    domain::validar_update_input(&input)?;

    db::find_by_id(&id).await.map_err(map_db_error)?.ok_or(VehiculoError::NotFound)?;

    let mut dto = VehiculoUpdateDTO::default();
    if let Some(t) = input.tipo_vehiculo {
        let tipo = domain::validar_tipo_vehiculo(&t)?;
        dto.tipo_vehiculo = Some(tipo);
    }
    if let Some(m) = input.marca {
        dto.marca = Some(m.trim().to_uppercase());
    }
    if let Some(m) = input.modelo {
        dto.modelo = Some(m.trim().to_uppercase());
    }
    if let Some(c) = input.color {
        dto.color = Some(c.trim().to_uppercase());
    }
    if let Some(a) = input.is_active {
        dto.is_active = Some(a);
    }
    dto.updated_at = Some(surrealdb::Datetime::from(Utc::now()));

    let updated = db::update(&id, dto).await.map_err(map_db_error)?;
    Ok(VehiculoResponse::from_fetched(updated))
}

pub async fn delete_vehiculo(id_str: String) -> Result<(), VehiculoError> {
    let id = parse_vehiculo_id(&id_str);
    db::find_by_id(&id).await.map_err(map_db_error)?.ok_or(VehiculoError::NotFound)?;
    db::delete(&id).await.map_err(map_db_error)?;
    Ok(())
}
