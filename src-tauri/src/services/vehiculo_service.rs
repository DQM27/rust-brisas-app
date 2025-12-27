// ==========================================
// src/services/vehiculo_service.rs
// ==========================================
use crate::db::surrealdb_contratista_queries as contratista_db;
use crate::db::surrealdb_proveedor_queries as proveedor_db;
use crate::db::surrealdb_vehiculo_queries as db;
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

/// Helper para parsear ID de contratista
fn parse_contratista_id(id_str: &str) -> RecordId {
    if id_str.contains(':') {
        let parts: Vec<&str> = id_str.split(':').collect();
        RecordId::from_table_key(parts[0], parts[1])
    } else {
        RecordId::from_table_key("contratista", id_str)
    }
}

/// Helper para parsear ID de proveedor
fn parse_proveedor_id(id_str: &str) -> RecordId {
    if id_str.contains(':') {
        let parts: Vec<&str> = id_str.split(':').collect();
        RecordId::from_table_key(parts[0], parts[1])
    } else {
        RecordId::from_table_key("proveedor", id_str)
    }
}

pub async fn create_vehiculo(
    input: crate::models::vehiculo::CreateVehiculoInput,
) -> Result<VehiculoResponse, VehiculoError> {
    domain::validar_create_input(&input)?;
    let placa_normalizada = domain::normalizar_placa(&input.placa);
    let tipo_vehiculo = domain::validar_tipo_vehiculo(&input.tipo_vehiculo)?;

    let contratista = input.contratista_id.as_ref().map(|id| parse_contratista_id(id));
    let proveedor = input.proveedor_id.as_ref().map(|id| parse_proveedor_id(id));

    // Verificar que el contratista existe si se proporciona
    if let Some(ref c_id) = contratista {
        if contratista_db::find_by_id(c_id).await.map_err(map_db_error)?.is_none() {
            return Err(VehiculoError::Validation(
                "El contratista especificado no existe".to_string(),
            ));
        }
    }

    // Verificar que el proveedor existe si se proporciona
    if let Some(ref p_id) = proveedor {
        if proveedor_db::find_by_id(p_id).await.map_err(map_db_error)?.is_none() {
            return Err(VehiculoError::Validation(
                "El proveedor especificado no existe".to_string(),
            ));
        }
    }

    let count = db::count_by_placa(&placa_normalizada).await.map_err(map_db_error)?;
    if count > 0 {
        return Err(VehiculoError::PlacaExists);
    }

    let dto = VehiculoCreateDTO {
        contratista,
        proveedor,
        visitante: None,
        tipo_vehiculo,
        placa: placa_normalizada,
        marca: input.marca.as_ref().map(|s| s.trim().to_uppercase()),
        modelo: input.modelo.as_ref().map(|s| s.trim().to_uppercase()),
        color: input.color.as_ref().map(|s| s.trim().to_uppercase()),
        is_active: true,
    };

    let vehiculo_creado = db::insert(dto).await.map_err(map_db_error)?;
    get_vehiculo_by_id(&vehiculo_creado.id.to_string()).await
}

pub async fn get_vehiculo_by_id(id_str: &str) -> Result<VehiculoResponse, VehiculoError> {
    let id = parse_vehiculo_id(id_str);
    let vehiculo =
        db::find_by_id(&id).await.map_err(map_db_error)?.ok_or(VehiculoError::NotFound)?;

    let mut response = VehiculoResponse::from(vehiculo.clone());

    // Enriquecer según el dueño
    if let Some(c_id) = &vehiculo.contratista {
        if let Some(c) = contratista_db::find_by_id(c_id).await.map_err(map_db_error)? {
            response.contratista_nombre = format!("{} {}", c.nombre, c.apellido);
            response.contratista_cedula = c.cedula;
            if let Ok(emp_nombre) = contratista_db::get_empresa_nombre(&c.empresa).await {
                response.empresa_nombre = emp_nombre;
            }
        }
    } else if let Some(p_id) = &vehiculo.proveedor {
        if let Some(p) = proveedor_db::find_by_id(p_id).await.map_err(map_db_error)? {
            response.contratista_nombre = format!("{} {}", p.nombre, p.apellido);
            response.contratista_cedula = p.cedula;
            // Para proveedores, podríamos buscar el nombre de la empresa también
            if let Ok(emp_nombre) = proveedor_db::get_empresa_nombre(&p.empresa).await {
                response.empresa_nombre = emp_nombre;
            }
        }
    }

    Ok(response)
}

pub async fn get_vehiculo_by_placa(placa: String) -> Result<VehiculoResponse, VehiculoError> {
    let placa_normalizada = domain::normalizar_placa(&placa);
    let vehiculo = db::find_by_placa(&placa_normalizada)
        .await
        .map_err(map_db_error)?
        .ok_or(VehiculoError::NotFound)?;
    get_vehiculo_by_id(&vehiculo.id.to_string()).await
}

pub async fn get_all_vehiculos() -> Result<VehiculoListResponse, VehiculoError> {
    let vehiculos = db::find_all().await.map_err(map_db_error)?;
    let mut vehiculo_responses = Vec::new();
    for vehiculo in vehiculos {
        if let Ok(res) = get_vehiculo_by_id(&vehiculo.id.to_string()).await {
            vehiculo_responses.push(res);
        }
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
    let vehiculos = db::find_activos().await.map_err(map_db_error)?;
    let mut vehiculo_responses = Vec::new();
    for vehiculo in vehiculos {
        if let Ok(res) = get_vehiculo_by_id(&vehiculo.id.to_string()).await {
            vehiculo_responses.push(res);
        }
    }
    Ok(vehiculo_responses)
}

pub async fn get_vehiculos_by_contratista(
    id_str: String,
) -> Result<Vec<VehiculoResponse>, VehiculoError> {
    let id = parse_contratista_id(&id_str);
    let vehiculos = db::find_by_contratista(&id).await.map_err(map_db_error)?;
    let mut vehiculo_responses = Vec::new();
    for vehiculo in vehiculos {
        if let Ok(res) = get_vehiculo_by_id(&vehiculo.id.to_string()).await {
            vehiculo_responses.push(res);
        }
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

    db::update(&id, dto).await.map_err(map_db_error)?;
    get_vehiculo_by_id(&id_str).await
}

pub async fn delete_vehiculo(id_str: String) -> Result<(), VehiculoError> {
    let id = parse_vehiculo_id(&id_str);
    db::find_by_id(&id).await.map_err(map_db_error)?.ok_or(VehiculoError::NotFound)?;
    db::delete(&id).await.map_err(map_db_error)?;
    Ok(())
}
