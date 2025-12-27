// ==========================================
// src/services/vehiculo_service.rs
// ==========================================
use crate::db::surrealdb_contratista_queries as contratista_db;
use crate::db::surrealdb_vehiculo_queries as db;
use crate::domain::errors::VehiculoError;
use crate::domain::vehiculo as domain;
use crate::models::vehiculo::{
    CreateVehiculoInput, TipoVehiculo, TipoVehiculoStats, UpdateVehiculoInput,
    VehiculoListResponse, VehiculoResponse,
};
use crate::services::surrealdb_service::SurrealDbError;
use log::error;

fn map_db_error(e: SurrealDbError) -> VehiculoError {
    error!("Error de base de datos (SurrealDB): {}", e);
    VehiculoError::Database(e.to_string())
}

pub async fn create_vehiculo(
    input: CreateVehiculoInput,
) -> Result<VehiculoResponse, VehiculoError> {
    domain::validar_create_input(&input)?;
    let placa_normalizada = domain::normalizar_placa(&input.placa);
    let tipo_vehiculo = domain::validar_tipo_vehiculo(&input.tipo_vehiculo)?;
    let marca_normalizada =
        input.marca.as_ref().map(|m| domain::normalizar_texto(m)).filter(|m| !m.is_empty());
    let modelo_normalizado =
        input.modelo.as_ref().map(|m| domain::normalizar_texto(m)).filter(|m| !m.is_empty());
    let color_normalizado =
        input.color.as_ref().map(|c| domain::normalizar_texto(c)).filter(|c| !c.is_empty());

    // Verificar que el contratista existe si se proporciona un contratista_id
    if let Some(ref cid) = input.contratista_id {
        let exists = contratista_db::find_by_id(cid).await.map_err(map_db_error)?.is_some();
        if !exists {
            return Err(VehiculoError::Validation(
                "El contratista especificado no existe".to_string(),
            ));
        }
    }

    let count = db::count_by_placa(&placa_normalizada).await.map_err(map_db_error)?;
    if count > 0 {
        return Err(VehiculoError::PlacaExists);
    }

    let input_db = CreateVehiculoInput {
        contratista_id: input.contratista_id.clone(),
        proveedor_id: None,
        tipo_vehiculo: tipo_vehiculo.as_str().to_string(),
        placa: placa_normalizada,
        marca: marca_normalizada,
        modelo: modelo_normalizado,
        color: color_normalizado,
    };

    let vehiculo_creado = db::insert(input_db).await.map_err(map_db_error)?;
    get_vehiculo_by_id(&vehiculo_creado.id).await
}

pub async fn get_vehiculo_by_id(id: &str) -> Result<VehiculoResponse, VehiculoError> {
    let vehiculo =
        db::find_by_id(id).await.map_err(map_db_error)?.ok_or(VehiculoError::NotFound)?;
    let mut response = VehiculoResponse::from(vehiculo.clone());
    if let Some(cid) = &vehiculo.contratista_id {
        if let Some(c) = contratista_db::find_by_id(cid).await.map_err(map_db_error)? {
            response.contratista_nombre = format!("{} {}", c.nombre, c.apellido);
            response.contratista_cedula = c.cedula;
            if let Ok(emp_nombre) = contratista_db::get_empresa_nombre(&c.empresa_id).await {
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
    get_vehiculo_by_id(&vehiculo.id).await
}

pub async fn get_all_vehiculos() -> Result<VehiculoListResponse, VehiculoError> {
    let vehiculos = db::find_all().await.map_err(map_db_error)?;
    let mut vehiculo_responses = Vec::new();
    for vehiculo in vehiculos {
        if let Ok(res) = get_vehiculo_by_id(&vehiculo.id).await {
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
        if let Ok(res) = get_vehiculo_by_id(&vehiculo.id).await {
            vehiculo_responses.push(res);
        }
    }
    Ok(vehiculo_responses)
}

pub async fn get_vehiculos_by_contratista(
    contratista_id: String,
) -> Result<Vec<VehiculoResponse>, VehiculoError> {
    let vehiculos = db::find_by_contratista(&contratista_id).await.map_err(map_db_error)?;
    let mut vehiculo_responses = Vec::new();
    for vehiculo in vehiculos {
        if let Ok(res) = get_vehiculo_by_id(&vehiculo.id).await {
            vehiculo_responses.push(res);
        }
    }
    Ok(vehiculo_responses)
}

pub async fn update_vehiculo(
    id: String,
    input: UpdateVehiculoInput,
) -> Result<VehiculoResponse, VehiculoError> {
    domain::validar_update_input(&input)?;
    let _ = db::find_by_id(&id).await.map_err(map_db_error)?.ok_or(VehiculoError::NotFound)?;
    let tipo_str = if let Some(ref t) = input.tipo_vehiculo {
        Some(domain::validar_tipo_vehiculo(t)?.as_str().to_string())
    } else {
        None
    };
    let marca_normalizada =
        input.marca.as_ref().map(|m| domain::normalizar_texto(m)).filter(|m| !m.is_empty());
    let modelo_normalizado =
        input.modelo.as_ref().map(|m| domain::normalizar_texto(m)).filter(|m| !m.is_empty());
    let color_normalizado =
        input.color.as_ref().map(|c| domain::normalizar_texto(c)).filter(|c| !c.is_empty());

    let input_db = UpdateVehiculoInput {
        tipo_vehiculo: tipo_str,
        marca: marca_normalizada,
        modelo: modelo_normalizado,
        color: color_normalizado,
        is_active: input.is_active,
    };
    db::update(&id, input_db).await.map_err(map_db_error)?;
    get_vehiculo_by_id(&id).await
}

pub async fn delete_vehiculo(id: String) -> Result<(), VehiculoError> {
    let _ = db::find_by_id(&id).await.map_err(map_db_error)?.ok_or(VehiculoError::NotFound)?;
    db::delete(&id).await.map_err(map_db_error)?;
    Ok(())
}
