// ==========================================
// src/services/vehiculo_service.rs
// ==========================================
// Capa de servicio: orquesta dominio, db
// Contiene la lógica de negocio completa

use crate::db::contratista_queries;
use crate::db::vehiculo_queries as db;
use crate::domain::errors::VehiculoError;
use crate::domain::vehiculo as domain;
use crate::models::vehiculo::{
    CreateVehiculoInput, TipoVehiculo, TipoVehiculoStats, UpdateVehiculoInput,
    VehiculoListResponse, VehiculoResponse,
};
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

// ==========================================
// CREAR VEHÍCULO
// ==========================================

pub async fn create_vehiculo(
    pool: &SqlitePool,
    input: CreateVehiculoInput,
) -> Result<VehiculoResponse, VehiculoError> {
    // 1. Validar input
    domain::validar_create_input(&input)?;

    // 2. Normalizar datos
    let placa_normalizada = domain::normalizar_placa(&input.placa);
    let tipo_vehiculo = domain::validar_tipo_vehiculo(&input.tipo_vehiculo)?;

    let marca_normalizada = input
        .marca
        .as_ref()
        .map(|m| domain::normalizar_texto(m))
        .filter(|m| !m.is_empty());

    let modelo_normalizado = input
        .modelo
        .as_ref()
        .map(|m| domain::normalizar_texto(m))
        .filter(|m| !m.is_empty());

    let color_normalizado = input
        .color
        .as_ref()
        .map(|c| domain::normalizar_texto(c))
        .filter(|c| !c.is_empty());

    // 3. Verificar que el contratista exista
    let exists = db::contratista_exists(pool, &input.contratista_id).await?;
    if !exists {
        // Podríamos usar ContratistaError::NotFound pero la firma retorna VehiculoError.
        // Convertimos a Validation por ahora.
        return Err(VehiculoError::Validation(
            "El contratista especificado no existe".to_string(),
        ));
    }

    // 4. Verificar que la placa no exista
    let count = db::count_by_placa(pool, &placa_normalizada).await?;
    if count > 0 {
        return Err(VehiculoError::PlacaExists);
    }

    // 5. Generar ID y timestamps
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    // 6. Insertar en DB
    db::insert(
        pool,
        &id,
        Some(&input.contratista_id),
        None, // proveedor_id
        tipo_vehiculo.as_str(),
        &placa_normalizada,
        marca_normalizada.as_deref(),
        modelo_normalizado.as_deref(),
        color_normalizado.as_deref(),
        &now,
        &now,
    )
    .await?;

    // 7. Retornar vehículo creado con datos completos
    get_vehiculo_by_id(pool, &id).await
}

// ==========================================
// OBTENER VEHÍCULO POR ID
// ==========================================

pub async fn get_vehiculo_by_id(
    pool: &SqlitePool,
    id: &str,
) -> Result<VehiculoResponse, VehiculoError> {
    // Obtener vehículo de DB
    let vehiculo = db::find_by_id(pool, id)
        .await?
        .ok_or(VehiculoError::NotFound)?;

    // Obtener datos del contratista usando queries ya existentes
    // Nota: find_by_id_with_empresa retorna Option
    let contratista_info = if let Some(cid) = &vehiculo.contratista_id {
        contratista_queries::find_by_id_with_empresa(pool, cid)
            .await
            .map_err(VehiculoError::Database)?
    } else {
        None
    };

    let mut response = VehiculoResponse::from(vehiculo);

    if let Some(info) = contratista_info {
        response.contratista_nombre =
            format!("{} {}", info.contratista.nombre, info.contratista.apellido);
        response.contratista_cedula = info.contratista.cedula;
        response.empresa_nombre = info.empresa_nombre;
    }

    Ok(response)
}

// ==========================================
// OBTENER VEHÍCULO POR PLACA
// ==========================================

pub async fn get_vehiculo_by_placa(
    pool: &SqlitePool,
    placa: String,
) -> Result<VehiculoResponse, VehiculoError> {
    let placa_normalizada = domain::normalizar_placa(&placa);

    // Obtener vehículo de DB
    let vehiculo = db::find_by_placa(pool, &placa_normalizada)
        .await?
        .ok_or(VehiculoError::NotFound)?;

    let contratista_info = if let Some(cid) = &vehiculo.contratista_id {
        contratista_queries::find_by_id_with_empresa(pool, cid)
            .await
            .map_err(VehiculoError::Database)?
    } else {
        None
    };

    let mut response = VehiculoResponse::from(vehiculo);

    if let Some(info) = contratista_info {
        response.contratista_nombre =
            format!("{} {}", info.contratista.nombre, info.contratista.apellido);
        response.contratista_cedula = info.contratista.cedula;
        response.empresa_nombre = info.empresa_nombre;
    }

    Ok(response)
}

// ==========================================
// OBTENER TODOS LOS VEHÍCULOS
// ==========================================

pub async fn get_all_vehiculos(pool: &SqlitePool) -> Result<VehiculoListResponse, VehiculoError> {
    let vehiculos = db::find_all(pool).await?;

    let mut vehiculo_responses = Vec::new();

    // Optimización: Podríamos hacer un join en query, pero para mantener strictness y reuso
    // llamamos individualmente por ahora. Si el rendimiento sufre, crear query específica.
    // O mejor, precargar todos los contratistas?
    // Dado que find_all es paginado usualmente, aquí trae todo.
    // Asumimos volumen bajo por ahora.

    for vehiculo in vehiculos {
        let mut response = VehiculoResponse::from(vehiculo.clone());

        if let Some(cid) = &vehiculo.contratista_id {
            // Reemplazo de query manual
            if let Some(info) = contratista_queries::find_by_id_with_empresa(pool, cid)
                .await
                .map_err(VehiculoError::Database)?
            {
                response.contratista_nombre =
                    format!("{} {}", info.contratista.nombre, info.contratista.apellido);
                response.contratista_cedula = info.contratista.cedula;
                response.empresa_nombre = info.empresa_nombre;
            }
        }

        vehiculo_responses.push(response);
    }

    // Calcular estadísticas
    let total = vehiculo_responses.len();
    let activos = vehiculo_responses.iter().filter(|v| v.is_active).count();
    let inactivos = total - activos;
    let motocicletas = vehiculo_responses
        .iter()
        .filter(|v| v.tipo_vehiculo == TipoVehiculo::Motocicleta)
        .count();
    let automóviles = vehiculo_responses
        .iter()
        .filter(|v| v.tipo_vehiculo == TipoVehiculo::Automovil)
        .count();

    Ok(VehiculoListResponse {
        vehiculos: vehiculo_responses,
        total,
        activos,
        inactivos,
        por_tipo: TipoVehiculoStats {
            motocicletas,
            automoviles: automóviles,
        },
    })
}

// ==========================================
// OBTENER VEHÍCULOS ACTIVOS
// ==========================================

pub async fn get_vehiculos_activos(
    pool: &SqlitePool,
) -> Result<Vec<VehiculoResponse>, VehiculoError> {
    let vehiculos = db::find_activos(pool).await?;
    let mut vehiculo_responses = Vec::new();

    for vehiculo in vehiculos {
        let mut response = VehiculoResponse::from(vehiculo.clone());
        if let Some(cid) = &vehiculo.contratista_id {
            if let Some(info) = contratista_queries::find_by_id_with_empresa(pool, cid)
                .await
                .map_err(VehiculoError::Database)?
            {
                response.contratista_nombre =
                    format!("{} {}", info.contratista.nombre, info.contratista.apellido);
                response.contratista_cedula = info.contratista.cedula;
                response.empresa_nombre = info.empresa_nombre;
            }
        }
        vehiculo_responses.push(response);
    }

    Ok(vehiculo_responses)
}

// ==========================================
// OBTENER VEHÍCULOS POR CONTRATISTA
// ==========================================

pub async fn get_vehiculos_by_contratista(
    pool: &SqlitePool,
    contratista_id: String,
) -> Result<Vec<VehiculoResponse>, VehiculoError> {
    let vehiculos = db::find_by_contratista(pool, &contratista_id).await?;
    let mut vehiculo_responses = Vec::new();

    for vehiculo in vehiculos {
        let mut response = VehiculoResponse::from(vehiculo.clone());
        // Aquí ya sabemos el contratista_id, pero necesitamos nombre y empresa.
        // Podríamos pasarlo, pero para consistencia buscamos.
        if let Some(info) = contratista_queries::find_by_id_with_empresa(pool, &contratista_id)
            .await
            .map_err(VehiculoError::Database)?
        {
            response.contratista_nombre =
                format!("{} {}", info.contratista.nombre, info.contratista.apellido);
            response.contratista_cedula = info.contratista.cedula;
            response.empresa_nombre = info.empresa_nombre;
        }

        vehiculo_responses.push(response);
    }

    Ok(vehiculo_responses)
}

// ==========================================
// ACTUALIZAR VEHÍCULO
// ==========================================

pub async fn update_vehiculo(
    pool: &SqlitePool,
    id: String,
    input: UpdateVehiculoInput,
) -> Result<VehiculoResponse, VehiculoError> {
    // 1. Validar input
    domain::validar_update_input(&input)?;

    // 2. Verificar que el vehículo existe
    let _ = db::find_by_id(pool, &id)
        .await?
        .ok_or(VehiculoError::NotFound)?;

    // 3. Normalizar y convertir tipo si viene
    let tipo_str = if let Some(ref t) = input.tipo_vehiculo {
        Some(domain::validar_tipo_vehiculo(t)?.as_str().to_string())
    } else {
        None
    };

    // 4. Normalizar textos si vienen
    let marca_normalizada = input
        .marca
        .as_ref()
        .map(|m| domain::normalizar_texto(m))
        .filter(|m| !m.is_empty());

    let modelo_normalizado = input
        .modelo
        .as_ref()
        .map(|m| domain::normalizar_texto(m))
        .filter(|m| !m.is_empty());

    let color_normalizado = input
        .color
        .as_ref()
        .map(|c| domain::normalizar_texto(c))
        .filter(|c| !c.is_empty());

    // 5. Timestamp de actualización
    let now = Utc::now().to_rfc3339();

    // 6. Convertir is_active.
    // db::update ahora espera Option<bool> por mi refactor anterior.
    // input.is_active es Option<bool>. Pasamos directo.
    let is_active = input.is_active;

    // 7. Actualizar en DB
    db::update(
        pool,
        &id,
        tipo_str.as_deref(),
        marca_normalizada.as_deref(),
        modelo_normalizado.as_deref(),
        color_normalizado.as_deref(),
        is_active,
        &now,
    )
    .await?;

    // 8. Retornar vehículo actualizado
    get_vehiculo_by_id(pool, &id).await
}

// ==========================================
// ELIMINAR VEHÍCULO
// ==========================================

pub async fn delete_vehiculo(pool: &SqlitePool, id: String) -> Result<(), VehiculoError> {
    // Verificar que existe antes de eliminar
    let _ = db::find_by_id(pool, &id)
        .await?
        .ok_or(VehiculoError::NotFound)?;

    // Eliminar
    db::delete(pool, &id).await?;

    Ok(())
}
