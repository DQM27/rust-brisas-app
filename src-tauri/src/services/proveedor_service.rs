// ==========================================
// src/services/proveedor_service.rs
// ==========================================
use crate::db::{
    surrealdb_empresa_queries as empresa_db, surrealdb_lista_negra_queries as lista_negra_db,
    surrealdb_proveedor_queries as db, surrealdb_vehiculo_queries as vehiculo_db,
};
use crate::domain::errors::ProveedorError;
use crate::domain::proveedor as proveedor_domain;
use crate::domain::vehiculo as vehiculo_domain;
use crate::models::proveedor::{
    CreateProveedorInput, Proveedor, ProveedorResponse, UpdateProveedorInput,
};
use crate::models::vehiculo::{CreateVehiculoInput, UpdateVehiculoInput};
use crate::services::search_service::SearchService;
use chrono::Utc;
use log::{error, info, warn};
use std::sync::Arc;

/// Crea un nuevo proveedor
pub async fn create_proveedor(
    search_service: &Arc<SearchService>,
    input: CreateProveedorInput,
) -> Result<ProveedorResponse, ProveedorError> {
    // 0. Validar Input de Dominio
    proveedor_domain::validar_create_input(&input)?;

    // 1. Validar que la empresa existe
    let empresa = empresa_db::find_by_id(&input.empresa_id)
        .await
        .map_err(|e| ProveedorError::Database(sqlx::Error::Protocol(e.to_string())))?
        .ok_or(ProveedorError::EmpresaNotFound)?;

    let empresa_nombre = empresa.nombre;

    // 2. Verificar que NO esté en lista negra
    let block_status = lista_negra_db::check_if_blocked_by_cedula(&input.cedula)
        .await
        .map_err(|e| ProveedorError::Validation(e.to_string()))?;

    if block_status.is_blocked {
        let nivel = block_status.nivel_severidad.unwrap_or_else(|| "BAJO".to_string());
        return Err(ProveedorError::Validation(format!(
            "No se puede registrar. La persona con cédula {} está en lista negra. Nivel: {}",
            input.cedula, nivel
        )));
    }

    // 3. Validar duplicidad
    info!("Creando proveedor con cédula '{}' para empresa '{}'", input.cedula, input.empresa_id);
    if db::find_by_cedula(&input.cedula)
        .await
        .map_err(|e| ProveedorError::Database(sqlx::Error::Protocol(e.to_string())))?
        .is_some()
    {
        return Err(ProveedorError::CedulaExists);
    }

    // 3. Crear
    let proveedor = db::create(input.clone())
        .await
        .map_err(|e| ProveedorError::Database(sqlx::Error::Protocol(e.to_string())))?;

    // 4. Crear Vehículo si aplica
    if let Some(true) = input.tiene_vehiculo {
        if let (Some(tipo), Some(placa)) = (&input.tipo_vehiculo, &input.placa) {
            if !tipo.is_empty() && !placa.is_empty() {
                // Normalizar tipo
                let tipo_norm = vehiculo_domain::validar_tipo_vehiculo(tipo)
                    .map_err(|e| ProveedorError::Validation(e.to_string()))?
                    .as_str()
                    .to_string();

                let vehiculo_input = CreateVehiculoInput {
                    contratista_id: None,
                    proveedor_id: Some(proveedor.id.clone()),
                    tipo_vehiculo: tipo_norm,
                    placa: placa.clone(),
                    marca: input.marca.clone(),
                    modelo: input.modelo.clone(),
                    color: input.color.clone(),
                };

                vehiculo_db::insert(vehiculo_input)
                    .await
                    .map_err(|e| ProveedorError::Database(sqlx::Error::Protocol(e.to_string())))?;
            }
        }
    }

    // 5. Indexar en búsqueda
    if let Err(e) = search_service.add_proveedor(&proveedor, &empresa_nombre).await {
        warn!("Error indexando proveedor: {}", e);
        // No falla la operación, solo logging
    }

    // 6. Enriquecer respuesta (nombre empresa y vehículo)
    let resp = populate_response(proveedor).await.map_err(|e| {
        error!("Error al poblar respuesta para proveedor: {}", e);
        e
    })?;

    info!("Proveedor creado exitosamente con ID {}", resp.id);
    Ok(resp)
}

/// Busca proveedores
pub async fn search_proveedores(query: &str) -> Result<Vec<ProveedorResponse>, ProveedorError> {
    let proveedores = db::search(query, 100)
        .await
        .map_err(|e| ProveedorError::Database(sqlx::Error::Protocol(e.to_string())))?;

    let mut responses = Vec::with_capacity(proveedores.len());
    for prov in proveedores {
        responses.push(populate_response(prov).await?);
    }
    Ok(responses)
}

/// Obtiene proveedor por cédula
pub async fn get_proveedor_by_cedula(
    cedula: &str,
) -> Result<Option<ProveedorResponse>, ProveedorError> {
    let p = db::find_by_cedula(cedula)
        .await
        .map_err(|e| ProveedorError::Database(sqlx::Error::Protocol(e.to_string())))?;

    if let Some(proveedor) = p {
        Ok(Some(populate_response(proveedor).await?))
    } else {
        Ok(None)
    }
}

/// Cambia el estado de un proveedor
pub async fn change_status(
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
    let proveedor = db::update(id, input).await.map_err(|e| {
        error!("Error al actualizar estado del proveedor {}: {}", id, e);
        ProveedorError::Database(sqlx::Error::Protocol(e.to_string()))
    })?;

    // Obtener nombre de empresa para indexación
    let empresa = empresa_db::find_by_id(&proveedor.empresa_id)
        .await
        .map_err(|e| ProveedorError::Database(sqlx::Error::Protocol(e.to_string())))?;
    let empresa_nombre = empresa.map(|e| e.nombre).unwrap_or_else(|| "Desconocida".to_string());

    // Actualizar en índice de búsqueda
    if let Err(e) = search_service.update_proveedor(&proveedor, &empresa_nombre).await {
        warn!("Error actualizando proveedor en índice: {}", e);
    }

    populate_response(proveedor).await
}

// Helper para llenar datos de empresa y vehículos
async fn populate_response(proveedor: Proveedor) -> Result<ProveedorResponse, ProveedorError> {
    // Buscar empresa
    let empresa = empresa_db::find_by_id(&proveedor.empresa_id)
        .await
        .map_err(|e| ProveedorError::Database(sqlx::Error::Protocol(e.to_string())))?;

    let proveedor_id = proveedor.id.clone();
    let mut response: ProveedorResponse = proveedor.into();
    if let Some(e) = empresa {
        response.empresa_nombre = e.nombre;
    } else {
        response.empresa_nombre = "Empresa no encontrada".to_string();
    }

    // Buscar vehículos
    let vehiculos = vehiculo_db::find_by_proveedor(&proveedor_id).await.unwrap_or_default();

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
pub async fn get_proveedor_by_id(id: &str) -> Result<ProveedorResponse, ProveedorError> {
    let proveedor = db::find_by_id(id)
        .await
        .map_err(|e| ProveedorError::Database(sqlx::Error::Protocol(e.to_string())))?
        .ok_or(ProveedorError::NotFound)?;

    populate_response(proveedor).await
}

/// Actualiza un proveedor
pub async fn update_proveedor(
    search_service: &Arc<SearchService>,
    id: String,
    input: UpdateProveedorInput,
) -> Result<ProveedorResponse, ProveedorError> {
    // 0. Validar Input de Dominio
    proveedor_domain::validar_update_input(&input)?;

    info!("Actualizando proveedor con ID {}", id);

    // 1. Verificar existencia
    let _ = db::find_by_id(&id)
        .await
        .map_err(|e| ProveedorError::Database(sqlx::Error::Protocol(e.to_string())))?
        .ok_or(ProveedorError::NotFound)?;

    // 2. Validar Empresa si cambia
    if let Some(ref eid) = input.empresa_id {
        if empresa_db::find_by_id(eid)
            .await
            .map_err(|e| ProveedorError::Database(sqlx::Error::Protocol(e.to_string())))?
            .is_none()
        {
            return Err(ProveedorError::EmpresaNotFound);
        }
    }

    // 3. Actualizar Proveedor en DB
    let proveedor = db::update(&id, input.clone()).await.map_err(|e| {
        error!("Error al actualizar proveedor {}: {}", id, e);
        ProveedorError::Database(sqlx::Error::Protocol(e.to_string()))
    })?;

    // 4. Gestionar Vehículo
    if let Some(tiene) = input.tiene_vehiculo {
        let vehiculos = vehiculo_db::find_by_proveedor(&id).await.unwrap_or_default();
        let vehiculo_existente = vehiculos.first();
        let _now = Utc::now().to_rfc3339();

        if tiene {
            // Actualizar o Crear
            if let (Some(tipo), Some(placa)) = (&input.tipo_vehiculo, &input.placa) {
                if !tipo.is_empty() && !placa.is_empty() {
                    let tipo_norm = vehiculo_domain::validar_tipo_vehiculo(tipo)
                        .map_err(|e| ProveedorError::Validation(e.to_string()))?
                        .as_str()
                        .to_string();

                    if let Some(v) = vehiculo_existente {
                        // Update
                        let update_input = UpdateVehiculoInput {
                            tipo_vehiculo: Some(tipo_norm),
                            marca: input.marca.clone(),
                            modelo: input.modelo.clone(),
                            color: input.color.clone(),
                            is_active: Some(true),
                        };

                        vehiculo_db::update(&v.id, update_input).await.map_err(|e| {
                            ProveedorError::Database(sqlx::Error::Protocol(e.to_string()))
                        })?;
                    } else {
                        // Create
                        let vehiculo_input = CreateVehiculoInput {
                            contratista_id: None,
                            proveedor_id: Some(id.clone()),
                            tipo_vehiculo: tipo_norm,
                            placa: placa.clone(),
                            marca: input.marca.clone(),
                            modelo: input.modelo.clone(),
                            color: input.color.clone(),
                        };

                        vehiculo_db::insert(vehiculo_input).await.map_err(|e| {
                            ProveedorError::Database(sqlx::Error::Protocol(e.to_string()))
                        })?;
                    }
                }
            }
        } else {
            // Eliminar si existe
            if let Some(v) = vehiculo_existente {
                vehiculo_db::delete(&v.id)
                    .await
                    .map_err(|e| ProveedorError::Database(sqlx::Error::Protocol(e.to_string())))?;
            }
        }
    }

    // 5. Actualizar Search Index
    let empresa_nombre =
        if let Some(e) = empresa_db::find_by_id(&proveedor.empresa_id).await.unwrap_or(None) {
            e.nombre
        } else {
            "Desconocida".to_string()
        };

    if let Err(e) = search_service.update_proveedor(&proveedor, &empresa_nombre).await {
        warn!("Error actualizando índice: {}", e);
    }

    // 6. Retornar actualizado
    get_proveedor_by_id(&id).await
}
