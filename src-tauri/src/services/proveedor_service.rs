// ==========================================
// src/services/proveedor_service.rs
// ==========================================
use crate::db::{empresa_queries, proveedor_queries, vehiculo_queries};
use crate::models::proveedor::{CreateProveedorInput, ProveedorResponse, UpdateProveedorInput};
use crate::services::search_service::SearchService;
use chrono::Utc;
use sqlx::SqlitePool;
use std::sync::Arc;
use uuid::Uuid;

/// Crea un nuevo proveedor
pub async fn create_proveedor(
    pool: &SqlitePool,
    search_service: &Arc<SearchService>,
    input: CreateProveedorInput,
) -> Result<ProveedorResponse, String> {
    // 1. Validar que la empresa existe
    let empresa = empresa_queries::find_by_id(pool, &input.empresa_id)
        .await
        .map_err(|e| e.to_string())?;

    if empresa.is_none() {
        return Err("La empresa seleccionada no existe".to_string());
    }
    let empresa_nombre = empresa.unwrap().nombre;

    // 2. Validar duplicidad
    if proveedor_queries::find_by_cedula(pool, &input.cedula)
        .await
        .map_err(|e| e.to_string())?
        .is_some()
    {
        return Err("Ya existe un proveedor con esa cédula".to_string());
    }

    // 3. Crear
    let proveedor = proveedor_queries::create(pool, input.clone())
        .await
        .map_err(|e| e.to_string())?;

    // 4. Crear Vehículo si aplica
    if let Some(true) = input.tiene_vehiculo {
        if let (Some(tipo), Some(placa)) = (&input.tipo_vehiculo, &input.placa) {
            if !tipo.is_empty() && !placa.is_empty() {
                let vid = Uuid::new_v4().to_string();
                let now = Utc::now().to_rfc3339();

                vehiculo_queries::insert(
                    pool,
                    &vid,
                    None,                // Contratista ID
                    Some(&proveedor.id), // Proveedor ID
                    tipo,
                    placa,
                    input.marca.as_deref(),
                    input.modelo.as_deref(),
                    input.color.as_deref(),
                    &now,
                    &now,
                )
                .await
                .map_err(|e| e.to_string())?;
            }
        }
    }

    // 5. Indexar en búsqueda
    if let Err(e) = search_service
        .add_proveedor(&proveedor, &empresa_nombre)
        .await
    {
        eprintln!("Error indexando proveedor: {}", e);
        // No falla la operación, solo logging
    }

    // 6. Enriquecer respuesta (nombre empresa y vehículo)
    populate_response(pool, proveedor).await
}

/// Busca proveedores
pub async fn search_proveedores(
    pool: &SqlitePool,
    query: &str,
) -> Result<Vec<ProveedorResponse>, String> {
    let proveedores = proveedor_queries::search(pool, query, 100)
        .await
        .map_err(|e| e.to_string())?;

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
) -> Result<Option<ProveedorResponse>, String> {
    let p = proveedor_queries::find_by_cedula(pool, cedula)
        .await
        .map_err(|e| e.to_string())?;

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
) -> Result<ProveedorResponse, String> {
    let input = UpdateProveedorInput {
        nombre: None,
        segundo_nombre: None,
        apellido: None,
        segundo_apellido: None,
        empresa_id: None,
        estado: Some(new_status.to_string()),
    };

    let proveedor = proveedor_queries::update(pool, id, input)
        .await
        .map_err(|e| e.to_string())?;

    // Obtener nombre de empresa para indexación
    let empresa = empresa_queries::find_by_id(pool, &proveedor.empresa_id)
        .await
        .map_err(|e| e.to_string())?;
    let empresa_nombre = empresa
        .map(|e| e.nombre)
        .unwrap_or_else(|| "Desconocida".to_string());

    // Actualizar en índice de búsqueda
    if let Err(e) = search_service
        .update_proveedor(&proveedor, &empresa_nombre)
        .await
    {
        eprintln!("Error actualizando proveedor en índice: {}", e);
    }

    populate_response(pool, proveedor).await
}

// Helper para llenar datos de empresa y vehículos
async fn populate_response(
    pool: &SqlitePool,
    proveedor: crate::models::proveedor::Proveedor,
) -> Result<ProveedorResponse, String> {
    let empresa = empresa_queries::find_by_id(pool, &proveedor.empresa_id)
        .await
        .map_err(|e| e.to_string())?;

    let proveedor_id = proveedor.id.clone();
    let mut response: ProveedorResponse = proveedor.into();
    if let Some(e) = empresa {
        response.empresa_nombre = e.nombre;
    } else {
        response.empresa_nombre = "Empresa no encontrada".to_string();
    }

    // Buscar vehículos
    let vehiculos = vehiculo_queries::find_by_proveedor(pool, &proveedor_id)
        .await
        .unwrap_or_default();

    if let Some(v) = vehiculos.first() {
        response.vehiculo_tipo = Some(v.tipo_vehiculo.to_string());
        response.vehiculo_placa = Some(v.placa.clone());
        response.vehiculo_marca = v.marca.clone();
        response.vehiculo_modelo = v.modelo.clone();
        response.vehiculo_color = v.color.clone();
    }

    Ok(response)
}
