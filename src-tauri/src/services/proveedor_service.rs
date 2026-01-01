/// Gestión Estratégica de Proveedores Recurrentes.
///
/// Este servicio gestiona a los proveedores que ingresan con frecuencia a las instalaciones.
/// A diferencia de los contratistas, los proveedores suelen tener un ciclo de rotación
/// diferente, pero comparten la necesidad de validación de seguridad (Lista Negra)
/// y trazabilidad vehicular.
use crate::db::{
    surrealdb_empresa_queries as empresa_db, surrealdb_lista_negra_queries as lista_negra_db,
    surrealdb_proveedor_queries as db, surrealdb_vehiculo_queries as vehiculo_db,
};
use crate::domain::errors::ProveedorError;
use crate::domain::proveedor as proveedor_domain;
use crate::domain::vehiculo as vehiculo_domain;
use crate::models::proveedor::{
    CreateProveedorInput, EstadoProveedor, ProveedorCreateDTO, ProveedorResponse,
    ProveedorUpdateDTO, UpdateProveedorInput,
};
use crate::models::vehiculo::{TipoVehiculo, VehiculoCreateDTO};
use crate::services::search_service::SearchService;
use chrono::Utc;
use log::{error, info, warn};
use std::sync::Arc;
use surrealdb::RecordId;

/// Helper para parsear ID de proveedor (acepta con o sin prefijo)
fn parse_proveedor_id(id_str: &str) -> RecordId {
    if id_str.contains(':') {
        let parts: Vec<&str> = id_str.split(':').collect();
        RecordId::from_table_key(parts[0], parts[1])
    } else {
        RecordId::from_table_key("proveedor", id_str)
    }
}

/// Helper para parsear ID de empresa (acepta con o sin prefijo)
fn parse_empresa_id(id_str: &str) -> RecordId {
    if id_str.contains(':') {
        let parts: Vec<&str> = id_str.split(':').collect();
        RecordId::from_table_key(parts[0], parts[1])
    } else {
        RecordId::from_table_key("empresa", id_str)
    }
}

/// Registra un nuevo proveedor en la plataforma.
///
/// El registro de un proveedor es un proceso de confianza que implica:
/// 1. Verificación de Vínculo: Debe pertenecer a una empresa registrada.
/// 2. Filtro de Integridad: Chequeo contra la lista negra institucional.
/// 3. Prevención de Duplicidad: Control estricto de identidad por cédula.
/// 4. Declaración Vehicular: Registro del medio de transporte para control de accesos.
/// 5. Visibilidad Logística: Indexación inmediata para el personal de garita.
pub async fn create_proveedor(
    search_service: &Arc<SearchService>,
    input: CreateProveedorInput,
) -> Result<ProveedorResponse, ProveedorError> {
    proveedor_domain::validar_create_input(&input)?;

    let empresa_id = parse_empresa_id(&input.empresa_id);
    let empresa = empresa_db::find_by_id(&empresa_id)
        .await
        .map_err(|e| ProveedorError::Database(e.to_string()))?
        .ok_or(ProveedorError::EmpresaNotFound)?;

    let empresa_nombre = empresa.nombre;

    // Seguridad: Si existe un veto en lista negra, se deniega el registro.
    let block_status = lista_negra_db::check_if_blocked_by_cedula(&input.cedula)
        .await
        .map_err(|e| ProveedorError::Validation(e.to_string()))?;

    if block_status.is_blocked {
        let nivel = block_status.nivel_severidad.unwrap_or_else(|| "BAJO".to_string());
        return Err(ProveedorError::Validation(format!(
            "BLOQUEO DE SEGURIDAD (Lista Negra): No se puede registrar a la cédula {} (Nivel: {}).",
            input.cedula, nivel
        )));
    }

    if db::find_by_cedula(&input.cedula)
        .await
        .map_err(|e| ProveedorError::Database(e.to_string()))?
        .is_some()
    {
        return Err(ProveedorError::CedulaExists);
    }

    let dto = ProveedorCreateDTO {
        cedula: input.cedula.clone(),
        nombre: input.nombre.trim().to_string(),
        segundo_nombre: input.segundo_nombre.map(|s| s.trim().to_string()),
        apellido: input.apellido.trim().to_string(),
        segundo_apellido: input.segundo_apellido.map(|s| s.trim().to_string()),
        empresa: empresa_id,
        estado: EstadoProveedor::Activo,
    };

    let proveedor = db::create(dto).await.map_err(|e| ProveedorError::Database(e.to_string()))?;

    // Registro automático del vehículo si se proporciona durante el alta.
    if let Some(true) = input.tiene_vehiculo {
        if let (Some(tipo), Some(placa)) = (&input.tipo_vehiculo, &input.placa) {
            if !tipo.is_empty() && !placa.is_empty() {
                let tipo_norm = vehiculo_domain::validar_tipo_vehiculo(tipo)
                    .map_err(|e| ProveedorError::Validation(e.to_string()))?
                    .as_str()
                    .to_string();

                let placa_norm = vehiculo_domain::normalizar_placa(placa);
                let dto_vehiculo = VehiculoCreateDTO {
                    propietario: proveedor.id.clone(),
                    tipo_vehiculo: tipo_norm
                        .parse::<TipoVehiculo>()
                        .map_err(|e| ProveedorError::Validation(e))?,
                    placa: placa_norm,
                    marca: input.marca.as_ref().map(|s| s.trim().to_string()),
                    modelo: input.modelo.as_ref().map(|s| s.trim().to_string()),
                    color: input.color.as_ref().map(|s| s.trim().to_string()),
                    is_active: true,
                };

                let _ = vehiculo_db::insert(dto_vehiculo).await;
            }
        }
    }

    // Sincronización con el motor de búsqueda Tantivy.
    if let Err(e) = search_service.add_proveedor_fetched(&proveedor, &empresa_nombre).await {
        warn!("Aviso: Falló la indexación del proveedor en el buscador: {}", e);
    }

    let resp = populate_response_fetched(proveedor).await.map_err(|e| {
        error!("Error interno al enriquecer respuesta de proveedor: {}", e);
        e
    })?;

    info!("Proveedor {} registrado correctamente.", resp.cedula);
    Ok(resp)
}

/// Busca proveedores
pub async fn search_proveedores(query: &str) -> Result<Vec<ProveedorResponse>, ProveedorError> {
    let proveedores =
        db::search(query, 100).await.map_err(|e| ProveedorError::Database(e.to_string()))?;

    let mut responses = Vec::with_capacity(proveedores.len());
    for prov in proveedores {
        responses.push(populate_response_fetched(prov).await?);
    }
    Ok(responses)
}

/// Obtiene proveedor por cédula
pub async fn get_proveedor_by_cedula(
    cedula: &str,
) -> Result<Option<ProveedorResponse>, ProveedorError> {
    let p =
        db::find_by_cedula(cedula).await.map_err(|e| ProveedorError::Database(e.to_string()))?;

    if let Some(proveedor) = p {
        Ok(Some(populate_response_fetched(proveedor).await?))
    } else {
        Ok(None)
    }
}

/// Cambia el estado operativo de un proveedor y sincroniza su estado para búsquedas.
pub async fn change_status(
    search_service: &Arc<SearchService>,
    id_str: &str,
    new_status: &str,
) -> Result<ProveedorResponse, ProveedorError> {
    let id = parse_proveedor_id(id_str);

    info!("Cambiando estado de proveedor {} a {}", id_str, new_status);

    let mut dto = ProveedorUpdateDTO::default();
    dto.estado = Some(new_status.parse::<EstadoProveedor>().map_err(ProveedorError::Validation)?);
    dto.updated_at = Some(surrealdb::Datetime::from(Utc::now()));

    let proveedor = db::update(&id, dto).await.map_err(|e| {
        error!("Error técnico al actualizar estado del proveedor {}: {}", id_str, e);
        ProveedorError::Database(e.to_string())
    })?;

    let empresa_nombre = proveedor.empresa.nombre.clone();

    // Es vital que el buscador refleje el nuevo estado para evitar ingresos de proveedores inactivos.
    if let Err(e) = search_service.update_proveedor_fetched(&proveedor, &empresa_nombre).await {
        warn!("Aviso: Falló la actualización del proveedor en el índice: {}", e);
    }

    populate_response_fetched(proveedor).await
}

// Helper para llenar datos de empresa y vehículos
async fn populate_response_fetched(
    proveedor: crate::models::proveedor::ProveedorFetched,
) -> Result<ProveedorResponse, ProveedorError> {
    let mut response = ProveedorResponse::from_fetched(proveedor.clone());

    // Buscar vehículos
    let vehiculos = vehiculo_db::find_by_propietario(&proveedor.id).await.unwrap_or_default();

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
pub async fn get_proveedor_by_id(id_str: &str) -> Result<ProveedorResponse, ProveedorError> {
    let id = parse_proveedor_id(id_str);
    let proveedor = db::find_by_id_fetched(&id)
        .await
        .map_err(|e| ProveedorError::Database(e.to_string()))?
        .ok_or(ProveedorError::NotFound)?;

    populate_response_fetched(proveedor).await
}

/// Actualiza los datos de un proveedor, incluyendo la gestión reactiva de su vehículo.
pub async fn update_proveedor(
    search_service: &Arc<SearchService>,
    id_str: String,
    input: UpdateProveedorInput,
) -> Result<ProveedorResponse, ProveedorError> {
    let id = parse_proveedor_id(&id_str);
    proveedor_domain::validar_update_input(&input)?;

    info!("Actualizando perfil del proveedor {}", id_str);

    db::find_by_id(&id)
        .await
        .map_err(|e| ProveedorError::Database(e.to_string()))?
        .ok_or(ProveedorError::NotFound)?;

    let mut dto = ProveedorUpdateDTO::default();
    if let Some(v) = input.nombre {
        dto.nombre = Some(v.trim().to_string());
    }
    if let Some(v) = input.segundo_nombre {
        dto.segundo_nombre = Some(v.trim().to_string());
    }
    if let Some(v) = input.apellido {
        dto.apellido = Some(v.trim().to_string());
    }
    if let Some(v) = input.segundo_apellido {
        dto.segundo_apellido = Some(v.trim().to_string());
    }
    if let Some(v) = input.empresa_id {
        let emp_id = parse_empresa_id(&v);
        if empresa_db::find_by_id(&emp_id).await.unwrap_or_default().is_none() {
            return Err(ProveedorError::EmpresaNotFound);
        }
        dto.empresa = Some(emp_id);
    }
    if let Some(v) = input.estado {
        dto.estado = Some(v.parse::<EstadoProveedor>().map_err(ProveedorError::Validation)?);
    }
    dto.updated_at = Some(surrealdb::Datetime::from(Utc::now()));

    let proveedor = db::update(&id, dto).await.map_err(|e| {
        error!("Error en DB al actualizar proveedor {}: {}", id_str, e);
        ProveedorError::Database(e.to_string())
    })?;

    // Lógica de sincronización vehicular:
    // Permite añadir o eliminar el vehículo asociado durante la actualización del proveedor.
    if let Some(tiene) = input.tiene_vehiculo {
        let vehiculos = vehiculo_db::find_by_propietario(&id).await.unwrap_or_default();
        let vehiculo_existente = vehiculos.first();

        if tiene {
            if let (Some(tipo), Some(placa)) = (&input.tipo_vehiculo, &input.placa) {
                if !tipo.is_empty() && !placa.is_empty() {
                    let tipo_norm = vehiculo_domain::validar_tipo_vehiculo(tipo)
                        .map_err(|e| ProveedorError::Validation(e.to_string()))?
                        .as_str()
                        .to_string();

                    if let Some(v) = vehiculo_existente {
                        let mut veh_dto = crate::models::vehiculo::VehiculoUpdateDTO::default();
                        veh_dto.tipo_vehiculo = Some(
                            tipo_norm
                                .parse::<crate::models::vehiculo::TipoVehiculo>()
                                .map_err(|e| ProveedorError::Validation(e))?,
                        );
                        veh_dto.marca = input.marca.clone();
                        veh_dto.modelo = input.modelo.clone();
                        veh_dto.color = input.color.clone();
                        veh_dto.is_active = Some(true);
                        veh_dto.updated_at = Some(surrealdb::Datetime::from(Utc::now()));

                        let _ = vehiculo_db::update(&v.id, veh_dto).await;
                    } else {
                        let dto_vehiculo = VehiculoCreateDTO {
                            propietario: id.clone(),
                            tipo_vehiculo: tipo_norm
                                .parse::<TipoVehiculo>()
                                .map_err(|e| ProveedorError::Validation(e))?,
                            placa: placa.clone(),
                            marca: input.marca.clone(),
                            modelo: input.modelo.clone(),
                            color: input.color.clone(),
                            is_active: true,
                        };

                        let _ = vehiculo_db::insert(dto_vehiculo).await;
                    }
                }
            }
        } else {
            // Si el proveedor deja de traer vehículo, lo removemos de su perfil.
            if let Some(v) = vehiculo_existente {
                let _ = vehiculo_db::delete(&v.id).await;
            }
        }
    }

    let empresa_nombre = proveedor.empresa.nombre.clone();
    if let Err(e) = search_service.update_proveedor_fetched(&proveedor, &empresa_nombre).await {
        warn!("Aviso: Falló la sincronización con el motor de búsqueda: {}", e);
    }

    get_proveedor_by_id(&id_str).await
}

/// Elimina a un proveedor (Borrado lógico para preservar integridad histórica).
pub async fn delete_proveedor(
    search_service: &Arc<SearchService>,
    id_str: &str,
) -> Result<(), ProveedorError> {
    let id = parse_proveedor_id(id_str);

    db::find_by_id_fetched(&id)
        .await
        .map_err(|e| ProveedorError::Database(e.to_string()))?
        .ok_or(ProveedorError::NotFound)?;

    db::delete(&id).await.map_err(|e| ProveedorError::Database(e.to_string()))?;

    // Es fundamental removerlo del motor de búsqueda para que no aparezca en los resultados activos.
    let _ = search_service.delete_proveedor(&id.to_string()).await;

    info!("Proveedor archivado (borrado lógico): {}", id_str);
    Ok(())
}

/// Restaura a un proveedor previamente archivado y lo re-habilita para búsquedas.
pub async fn restore_proveedor(
    search_service: &Arc<SearchService>,
    id_str: &str,
) -> Result<ProveedorResponse, ProveedorError> {
    let id = parse_proveedor_id(id_str);

    let proveedor_fetched =
        db::restore(&id).await.map_err(|e| ProveedorError::Database(e.to_string()))?;

    let empresa_nombre = proveedor_fetched.empresa.nombre.clone();

    // Al restaurar, debemos volver a indexarlo para que sea visible en garitas.
    if let Ok(Some(p)) = db::find_by_id_fetched(&id).await {
        let _ = search_service.add_proveedor_fetched(&p, &empresa_nombre).await;
    }

    populate_response_fetched(proveedor_fetched).await
}

/// Obtiene proveedores archivados
pub async fn get_archived_proveedores() -> Result<Vec<ProveedorResponse>, ProveedorError> {
    let proveedores =
        db::find_archived().await.map_err(|e| ProveedorError::Database(e.to_string()))?;

    let mut responses = Vec::with_capacity(proveedores.len());
    for prov in proveedores {
        responses.push(populate_response_fetched(prov).await?);
    }
    Ok(responses)
}
