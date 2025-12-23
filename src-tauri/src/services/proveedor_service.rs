// ==========================================
// src/services/proveedor_service.rs
// ==========================================
use crate::db::{empresa_queries, proveedor_queries, vehiculo_queries};
use crate::domain::errors::ProveedorError;
use crate::domain::proveedor as proveedor_domain;
use crate::domain::vehiculo as vehiculo_domain;
use crate::models::proveedor::{CreateProveedorInput, ProveedorResponse, UpdateProveedorInput};
use crate::services::search_service::SearchService;
use chrono::Utc;
use log::{error, info, warn};
use sqlx::SqlitePool;
use std::sync::Arc;
use uuid::Uuid;

/// Crea un nuevo proveedor
pub async fn create_proveedor(
    pool: &SqlitePool,
    search_service: &Arc<SearchService>,
    input: CreateProveedorInput,
) -> Result<ProveedorResponse, ProveedorError> {
    // 0. Validar Input de Dominio
    proveedor_domain::validar_create_input(&input)?;

    // 1. Validar que la empresa existe
    let empresa = empresa_queries::find_by_id(pool, &input.empresa_id)
        .await?
        .ok_or(ProveedorError::EmpresaNotFound)?;

    let empresa_nombre = empresa.nombre;

    // 2. Validar duplicidad
    info!("Creando proveedor con cédula '{}' para empresa '{}'", input.cedula, input.empresa_id);
    if proveedor_queries::find_by_cedula(pool, &input.cedula).await?.is_some() {
        return Err(ProveedorError::CedulaExists);
    }

    // 3. Crear
    let proveedor = proveedor_queries::create(pool, input.clone()).await?;

    // 4. Crear Vehículo si aplica
    if let Some(true) = input.tiene_vehiculo {
        if let (Some(tipo), Some(placa)) = (&input.tipo_vehiculo, &input.placa) {
            if !tipo.is_empty() && !placa.is_empty() {
                // Normalizar tipo
                let tipo_norm = vehiculo_domain::validar_tipo_vehiculo(tipo)
                    .map_err(|e| ProveedorError::Validation(e.to_string()))?
                    .as_str()
                    .to_string();

                let vid = Uuid::new_v4().to_string();
                let now = Utc::now().to_rfc3339();

                vehiculo_queries::insert(
                    pool,
                    &vid,
                    None,                // Contratista ID
                    Some(&proveedor.id), // Proveedor ID
                    &tipo_norm,
                    placa,
                    input.marca.as_deref(),
                    input.modelo.as_deref(),
                    input.color.as_deref(),
                    &now,
                    &now,
                )
                .await?;
            }
        }
    }

    // 5. Indexar en búsqueda
    if let Err(e) = search_service.add_proveedor(&proveedor, &empresa_nombre).await {
        warn!("Error indexando proveedor: {}", e);
        // No falla la operación, solo logging
    }

    // 6. Enriquecer respuesta (nombre empresa y vehículo)
    let resp = populate_response(pool, proveedor).await.map_err(|e| {
        error!("Error al poblar respuesta para proveedor: {}", e);
        e
    })?;

    info!("Proveedor creado exitosamente con ID {}", resp.id);
    Ok(resp)
}

/// Busca proveedores
pub async fn search_proveedores(
    pool: &SqlitePool,
    query: &str,
) -> Result<Vec<ProveedorResponse>, ProveedorError> {
    let proveedores = proveedor_queries::search(pool, query, 100).await?;

    let mut responses = Vec::with_capacity(proveedores.len());
    for prov in proveedores {
        responses.push(populate_response(pool, prov).await?);
    }
    Ok(responses)
}

/// Obtiene proveedor por cédula
pub async fn get_proveedor_by_cedula(
    pool: &SqlitePool,
    cedula: &str,
) -> Result<Option<ProveedorResponse>, ProveedorError> {
    let p = proveedor_queries::find_by_cedula(pool, cedula).await?;

    if let Some(proveedor) = p {
        Ok(Some(populate_response(pool, proveedor).await?))
    } else {
        Ok(None)
    }
}

/// Cambia el estado de un proveedor
pub async fn change_status(
    pool: &SqlitePool,
    search_service: &Arc<SearchService>,
    id: &str,
    new_status: &str,
) -> Result<ProveedorResponse, ProveedorError> {
    let input = UpdateProveedorInput {
        nombre: None,
        segundo_nombre: None,
        apellido: None,
        segundo_apellido: None,
        empresa_id: None,
        estado: Some(new_status.to_string()),
        tiene_vehiculo: None,
        tipo_vehiculo: None,
        placa: None,
        marca: None,
        modelo: None,
        color: None,
    };

    info!("Cambiando estado de proveedor {} a {}", id, new_status);
    let proveedor = proveedor_queries::update(pool, id, input).await.map_err(|e| {
        error!("Error al actualizar estado del proveedor {}: {}", id, e);
        ProveedorError::Database(e)
    })?;

    // Obtener nombre de empresa para indexación
    let empresa = empresa_queries::find_by_id(pool, &proveedor.empresa_id).await?;
    let empresa_nombre = empresa.map(|e| e.nombre).unwrap_or_else(|| "Desconocida".to_string());

    // Actualizar en índice de búsqueda
    if let Err(e) = search_service.update_proveedor(&proveedor, &empresa_nombre).await {
        warn!("Error actualizando proveedor en índice: {}", e);
    }

    populate_response(pool, proveedor).await
}

// Helper para llenar datos de empresa y vehículos
async fn populate_response(
    pool: &SqlitePool,
    proveedor: crate::models::proveedor::Proveedor,
) -> Result<ProveedorResponse, ProveedorError> {
    let empresa = empresa_queries::find_by_id(pool, &proveedor.empresa_id).await?;

    let proveedor_id = proveedor.id.clone();
    let mut response: ProveedorResponse = proveedor.into();
    if let Some(e) = empresa {
        response.empresa_nombre = e.nombre;
    } else {
        response.empresa_nombre = "Empresa no encontrada".to_string();
    }

    // Buscar vehículos
    let vehiculos =
        vehiculo_queries::find_by_proveedor(pool, &proveedor_id).await.unwrap_or_default();

    if let Some(v) = vehiculos.first() {
        response.vehiculo_tipo = Some(v.tipo_vehiculo.to_string());
        response.vehiculo_placa = Some(v.placa.clone());
        response.vehiculo_marca = v.marca.clone();
        response.vehiculo_modelo = v.modelo.clone();
        response.vehiculo_color = v.color.clone();
    }

    Ok(response)
}

/// Obtiene un proveedor por ID con todos sus datos
pub async fn get_proveedor_by_id(
    pool: &SqlitePool,
    id: &str,
) -> Result<ProveedorResponse, ProveedorError> {
    let row = proveedor_queries::find_by_id_with_empresa(pool, id)
        .await?
        .ok_or(ProveedorError::NotFound)?;

    let mut response = ProveedorResponse::from(row.proveedor);
    response.empresa_nombre = row.empresa_nombre;
    response.vehiculo_tipo = row.vehiculo_tipo;
    response.vehiculo_placa = row.vehiculo_placa;
    response.vehiculo_marca = row.vehiculo_marca;
    response.vehiculo_modelo = row.vehiculo_modelo;
    response.vehiculo_color = row.vehiculo_color;

    Ok(response)
}

/// Actualiza un proveedor
pub async fn update_proveedor(
    pool: &SqlitePool,
    search_service: &Arc<SearchService>,
    id: String,
    input: UpdateProveedorInput,
) -> Result<ProveedorResponse, ProveedorError> {
    // 0. Validar Input de Dominio
    proveedor_domain::validar_update_input(&input)?;

    info!("Actualizando proveedor con ID {}", id);

    // 1. Verificar existencia
    let _ = proveedor_queries::find_by_id(pool, &id).await?.ok_or(ProveedorError::NotFound)?;

    // 2. Validar Empresa si cambia
    if let Some(ref eid) = input.empresa_id {
        if empresa_queries::find_by_id(pool, eid).await?.is_none() {
            return Err(ProveedorError::EmpresaNotFound);
        }
    }

    // 3. Actualizar Proveedor en DB
    let proveedor = proveedor_queries::update(pool, &id, input.clone()).await.map_err(|e| {
        error!("Error al actualizar proveedor {}: {}", id, e);
        ProveedorError::Database(e)
    })?;

    // 4. Gestionar Vehículo
    if let Some(tiene) = input.tiene_vehiculo {
        let vehiculos = vehiculo_queries::find_by_proveedor(pool, &id).await.unwrap_or_default();
        let vehiculo_existente = vehiculos.first();
        let now = Utc::now().to_rfc3339();

        if tiene {
            // Actualizar o Crear
            if let (Some(tipo), Some(placa)) = (&input.tipo_vehiculo, &input.placa) {
                if !tipo.is_empty() && !placa.is_empty() {
                    // Normalizar antes de usar
                    // Convertimos Result<TipoVehiculo, String> -> TipoVehiculo -> &str -> String
                    // El ? propagará el error string si la validación falla
                    let tipo_norm = vehiculo_domain::validar_tipo_vehiculo(tipo)
                        .map_err(|e| ProveedorError::Validation(e.to_string()))?
                        .as_str()
                        .to_string();

                    if let Some(v) = vehiculo_existente {
                        // Update
                        vehiculo_queries::update(
                            pool,
                            &v.id,
                            Some(&tipo_norm),
                            input.marca.as_deref(),
                            input.modelo.as_deref(),
                            input.color.as_deref(),
                            Some(true),
                            &now,
                        )
                        .await?;
                    } else {
                        // Create
                        let vid = Uuid::new_v4().to_string();
                        vehiculo_queries::insert(
                            pool,
                            &vid,
                            None,
                            Some(&id),
                            &tipo_norm, // Use normalized type
                            placa,
                            input.marca.as_deref(),
                            input.modelo.as_deref(),
                            input.color.as_deref(),
                            &now,
                            &now,
                        )
                        .await?;
                    }
                }
            }
        } else {
            // Eliminar si existe
            if let Some(v) = vehiculo_existente {
                vehiculo_queries::delete(pool, &v.id).await?;
            }
        }
    }

    // 5. Actualizar Search Index
    // Necesitamos el nombre de la empresa
    let empresa_nombre = if let Some(e) =
        empresa_queries::find_by_id(pool, &proveedor.empresa_id).await.unwrap_or(None)
    {
        e.nombre
    } else {
        "Desconocida".to_string()
    };

    if let Err(e) = search_service.update_proveedor(&proveedor, &empresa_nombre).await {
        warn!("Error actualizando índice: {}", e);
    }

    // 6. Retornar actualizado
    get_proveedor_by_id(pool, &id).await
}
