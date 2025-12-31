use crate::db::surrealdb_audit_queries as audit_db;
use crate::db::surrealdb_contratista_queries as db;
use crate::db::surrealdb_empresa_queries as empresa_db;
use crate::db::surrealdb_lista_negra_queries as ln_db;
use crate::db::surrealdb_vehiculo_queries as veh_db;

use crate::domain::contratista as domain;
use crate::domain::errors::ContratistaError;
use crate::domain::vehiculo as vehiculo_domain; // NEW IMPORT
use crate::models::contratista::{
    CambiarEstadoInput, ContratistaCreateDTO, ContratistaListResponse, ContratistaResponse,
    CreateContratistaInput, EstadoContratista, UpdateContratistaInput,
};
use crate::models::vehiculo::{TipoVehiculo, VehiculoCreateDTO}; // NEW IMPORTS
use crate::services::search_service::SearchService;
use crate::services::surrealdb_service::SurrealDbError;
use log::{error, info};
use std::sync::Arc;
use surrealdb::RecordId;

// Helper para mapear errores de SurrealDB a ContratistaError
fn map_db_error(e: SurrealDbError) -> ContratistaError {
    ContratistaError::Database(e.to_string())
}

/// Helper para parsear ID de contratista (acepta con o sin prefijo)
fn parse_contratista_id(id_str: &str) -> RecordId {
    if id_str.contains(':') {
        let parts: Vec<&str> = id_str.split(':').collect();
        RecordId::from_table_key(parts[0], parts[1])
    } else {
        RecordId::from_table_key("contratista", id_str)
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

// ==========================================
// CREAR CONTRATISTA
// ==========================================

pub async fn create_contratista(
    search_service: &Arc<SearchService>,
    input: CreateContratistaInput,
) -> Result<ContratistaResponse, ContratistaError> {
    // 1. Validar input
    domain::validar_create_input(&input)?;

    // 2. Normalizar datos
    let cedula_normalizada = domain::normalizar_cedula(&input.cedula);

    // 3. Verificar que NO esté en lista negra
    let block_status =
        ln_db::check_if_blocked_by_cedula(&cedula_normalizada).await.map_err(map_db_error)?;

    if block_status.is_blocked {
        let nivel = block_status.nivel_severidad.unwrap_or_else(|| "BAJO".to_string());
        return Err(ContratistaError::Validation(format!(
            "No se puede registrar. La persona con cédula {} está en lista negra. Nivel: {}",
            cedula_normalizada, nivel
        )));
    }

    // 4. Verificar que la cédula no exista
    let existing = db::find_by_cedula(&cedula_normalizada).await.map_err(map_db_error)?;
    if existing.is_some() {
        return Err(ContratistaError::CedulaExists);
    }

    // 5. Verificar que la empresa exista
    let empresa_id = parse_empresa_id(&input.empresa_id);
    let empresa_opt = empresa_db::find_by_id(&empresa_id).await.map_err(map_db_error)?;
    if empresa_opt.is_none() {
        return Err(ContratistaError::EmpresaNotFound);
    }

    // 6. Preparar DTO
    let fecha_vencimiento =
        crate::models::contratista::validaciones::validar_fecha(&input.fecha_vencimiento_praind)
            .map_err(ContratistaError::Validation)?;

    let dto = ContratistaCreateDTO {
        cedula: cedula_normalizada.clone(),
        nombre: input.nombre.trim().to_string(),
        segundo_nombre: input.segundo_nombre.map(|s| s.trim().to_string()),
        apellido: input.apellido.trim().to_string(),
        segundo_apellido: input.segundo_apellido.map(|s| s.trim().to_string()),
        empresa: empresa_id,
        fecha_vencimiento_praind: surrealdb::Datetime::from(fecha_vencimiento),
        estado: EstadoContratista::Activo,
    };

    // 7. Insertar en DB
    let contratista = db::create(dto).await.map_err(|e| {
        error!("Error de base de datos al crear contratista {}: {}", cedula_normalizada, e);
        map_db_error(e)
    })?;

    info!("Contratista {} creado exitosamente con ID {}", cedula_normalizada, contratista.id);

    // 7.5. Gestionar Vehículo (Opcional)
    if let Some(true) = input.tiene_vehiculo {
        if let (Some(tipo), Some(placa)) = (&input.tipo_vehiculo, &input.placa) {
            if !tipo.is_empty() && !placa.is_empty() {
                // Validaciones de dominio de vehículo
                let tipo_norm = vehiculo_domain::validar_tipo_vehiculo(tipo)
                    .map_err(|e| ContratistaError::Validation(e.to_string()))?
                    .as_str()
                    .to_string();

                let placa_norm = vehiculo_domain::normalizar_placa(placa);

                // Validar que placa no exista (opcional, o dejar que la DB falle)
                // db::find_vehiculo_by_placa ...

                let dto_vehiculo = VehiculoCreateDTO {
                    propietario: contratista.id.clone(),
                    tipo_vehiculo: tipo_norm
                        .parse::<TipoVehiculo>()
                        .map_err(|e| ContratistaError::Validation(e))?,
                    placa: placa_norm,
                    marca: input.marca.as_ref().map(|s| s.trim().to_string()),
                    modelo: input.modelo.as_ref().map(|s| s.trim().to_string()),
                    color: input.color.as_ref().map(|s| s.trim().to_string()),
                    is_active: true,
                };

                if let Err(e) = veh_db::insert(dto_vehiculo).await {
                    error!("Error al crear vehículo para contratista {}: {}", contratista.id, e);
                    // No fallamos la creación del contratista, solo logueamos (o podríamos retornar error)
                    // Para consistencia con Proveedor, si falla vehículo, quizás deberíamos advertir.
                    // Por ahora log error.
                } else {
                    info!("Vehículo creado para contratista {}", contratista.id);
                }
            }
        }
    }

    // 8. Indexar en búsqueda
    let empresa_nombre = contratista.empresa.nombre.clone();
    if let Err(e) = search_service.add_contratista_fetched(&contratista, &empresa_nombre).await {
        log::warn!("Error indexando contratista en búsqueda: {}", e);
    }

    // 9. Retornar respuesta
    build_response_fetched(contratista).await
}

// ==========================================
// OBTENER CONTRATISTA POR ID
// ==========================================

pub async fn get_contratista_by_id(id_str: &str) -> Result<ContratistaResponse, ContratistaError> {
    let id = parse_contratista_id(id_str);
    let contratista = db::find_by_id_fetched(&id)
        .await
        .map_err(map_db_error)?
        .ok_or(ContratistaError::NotFound)?;
    build_response_fetched(contratista).await
}

// ==========================================
// OBTENER CONTRATISTA POR CÉDULA
// ==========================================

pub async fn get_contratista_by_cedula(
    cedula: &str,
) -> Result<ContratistaResponse, ContratistaError> {
    let cedula_norm = domain::normalizar_cedula(cedula);
    let contratista = db::find_by_cedula(&cedula_norm)
        .await
        .map_err(map_db_error)?
        .ok_or(ContratistaError::NotFound)?;
    build_response_fetched(contratista).await
}

// ==========================================
// OBTENER TODOS LOS CONTRATISTAS
// ==========================================

pub async fn get_all_contratistas() -> Result<ContratistaListResponse, ContratistaError> {
    let raw_list = db::find_all_fetched().await.map_err(map_db_error)?;

    let mut contratistas = Vec::new();
    for c in raw_list {
        contratistas.push(build_response_fetched(c).await?);
    }

    let total = contratistas.len();
    let activos = contratistas.iter().filter(|c| c.estado == EstadoContratista::Activo).count();
    let con_praind_vencido = contratistas.iter().filter(|c| c.praind_vencido).count();
    let requieren_atencion = contratistas.iter().filter(|c| c.requiere_atencion).count();

    Ok(ContratistaListResponse {
        contratistas,
        total,
        activos,
        con_praind_vencido,
        requieren_atencion,
    })
}

// ==========================================
// OBTENER CONTRATISTAS ACTIVOS
// ==========================================

pub async fn get_contratistas_activos() -> Result<Vec<ContratistaResponse>, ContratistaError> {
    let raw_list = db::find_all_fetched().await.map_err(map_db_error)?;

    let mut contratistas = Vec::new();
    for c in raw_list {
        let res = build_response_fetched(c).await?;
        if res.estado == EstadoContratista::Activo {
            contratistas.push(res);
        }
    }

    Ok(contratistas)
}

// ==========================================
// ACTUALIZAR CONTRATISTA
// ==========================================

pub async fn update_contratista(
    search_service: &Arc<SearchService>,
    id_str: String,
    input: UpdateContratistaInput,
) -> Result<ContratistaResponse, ContratistaError> {
    use crate::models::contratista::ContratistaUpdateDTO;

    let id = parse_contratista_id(&id_str);
    println!(">>> DEBUG: update_contratista INPUT: {:?}", input);

    // 1. Validar input
    domain::validar_update_input(&input)?;

    // 2. Verificar que el contratista existe y obtener datos actuales
    let existing =
        db::find_by_id(&id).await.map_err(map_db_error)?.ok_or(ContratistaError::NotFound)?;

    // 3. Construir DTO tipado para update
    let mut dto = ContratistaUpdateDTO::default();

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

    // 4. Empresa: solo si cambió
    if let Some(empresa_id_str) = &input.empresa_id {
        let empresa_id = parse_empresa_id(empresa_id_str);
        if empresa_id != existing.empresa {
            // Verify empresa exists
            if empresa_db::find_by_id(&empresa_id).await.map_err(map_db_error)?.is_none() {
                return Err(ContratistaError::EmpresaNotFound);
            }
            dto.empresa = Some(empresa_id);
            println!(">>> DEBUG: Empresa changed to {:?}", dto.empresa);
        } else {
            println!(">>> DEBUG: Empresa unchanged, skipping");
        }
    }

    // 5. Fecha PRAIND
    if let Some(v) = input.fecha_vencimiento_praind {
        let fecha = crate::models::contratista::validaciones::validar_fecha(&v)
            .map_err(ContratistaError::Validation)?;
        dto.fecha_vencimiento_praind = Some(fecha.into());
    }

    // 6. Actualizar en DB con DTO tipado
    println!(">>> DEBUG: Updating contratista {} with DTO: {:?}", id, dto);
    let updated = db::update(&id, dto).await.map_err(|e| {
        error!("Error de base de datos al actualizar contratista {}: {}", id, e);
        map_db_error(e)
    })?;
    println!(">>> DEBUG: Update result: {:?}", updated);

    // 7. Gestionar Vehículo (si tiene_vehiculo está presente)
    if let Some(true) = input.tiene_vehiculo {
        if let (Some(tipo), Some(placa)) = (&input.tipo_vehiculo, &input.placa) {
            if !tipo.is_empty() && !placa.is_empty() {
                // Validaciones de dominio de vehículo
                let tipo_norm = vehiculo_domain::validar_tipo_vehiculo(tipo)
                    .map_err(|e| ContratistaError::Validation(e.to_string()))?
                    .as_str()
                    .to_string();

                let placa_norm = vehiculo_domain::normalizar_placa(placa);

                // Buscar si ya existe un vehículo con esta placa
                let existing_vehiculo =
                    veh_db::find_by_placa(&placa_norm).await.map_err(map_db_error)?;

                if let Some(vehiculo) = existing_vehiculo {
                    // Actualizar vehículo existente
                    use crate::models::vehiculo::VehiculoUpdateDTO;
                    let update_dto = VehiculoUpdateDTO {
                        tipo_vehiculo: Some(
                            tipo_norm
                                .parse::<TipoVehiculo>()
                                .map_err(|e| ContratistaError::Validation(e))?,
                        ),
                        marca: input.marca.as_ref().map(|s| s.trim().to_string()),
                        modelo: input.modelo.as_ref().map(|s| s.trim().to_string()),
                        color: input.color.as_ref().map(|s| s.trim().to_string()),
                        ..Default::default()
                    };

                    if let Err(e) = veh_db::update(&vehiculo.id, update_dto).await {
                        error!("Error al actualizar vehículo {}: {}", placa_norm, e);
                    } else {
                        info!(
                            "Vehículo {} actualizado para contratista {}",
                            placa_norm, updated.id
                        );
                    }
                } else {
                    // Crear nuevo vehículo
                    let dto_vehiculo = VehiculoCreateDTO {
                        propietario: updated.id.clone(),
                        tipo_vehiculo: tipo_norm
                            .parse::<TipoVehiculo>()
                            .map_err(|e| ContratistaError::Validation(e))?,
                        placa: placa_norm.clone(),
                        marca: input.marca.as_ref().map(|s| s.trim().to_string()),
                        modelo: input.modelo.as_ref().map(|s| s.trim().to_string()),
                        color: input.color.as_ref().map(|s| s.trim().to_string()),
                        is_active: true,
                    };

                    if let Err(e) = veh_db::insert(dto_vehiculo).await {
                        error!(
                            "Error al crear vehículo {} para contratista {}: {}",
                            placa_norm, updated.id, e
                        );
                    } else {
                        info!("Vehículo {} creado para contratista {}", placa_norm, updated.id);
                    }
                }
            }
        }
    }

    // 8. Indexar en búsqueda
    let empresa_nombre = updated.empresa.nombre.clone();
    if let Err(e) = search_service.update_contratista_fetched(&updated, &empresa_nombre).await {
        log::warn!("Error actualizando contratista en búsqueda: {}", e);
    }

    // 9. Retornar
    build_response_fetched(updated).await
}

// ==========================================
// CAMBIAR ESTADO
// ==========================================

pub async fn cambiar_estado_contratista(
    _search_service: &Arc<SearchService>,
    id_str: String,
    input: CambiarEstadoInput,
) -> Result<ContratistaResponse, ContratistaError> {
    let id = parse_contratista_id(&id_str);
    let estado = domain::validar_estado(&input.estado)?;

    db::find_by_id(&id).await.map_err(map_db_error)?.ok_or(ContratistaError::NotFound)?;

    let updated = db::update_status(&id, estado.as_str()).await.map_err(map_db_error)?;
    build_response_fetched(updated).await
}

// ==========================================
// ELIMINAR CONTRATISTA
// ==========================================

pub async fn delete_contratista(
    _search_service: &Arc<SearchService>,
    id_str: String,
) -> Result<(), ContratistaError> {
    let id = parse_contratista_id(&id_str);

    db::find_by_id(&id).await.map_err(map_db_error)?.ok_or(ContratistaError::NotFound)?;

    db::delete(&id).await.map_err(|e| {
        error!("Error al eliminar contratista {}: {}", id_str, e);
        map_db_error(e)
    })?;

    Ok(())
}

// ==========================================
// ACTUALIZAR PRAIND CON HISTORIAL
// ==========================================

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActualizarPraindInput {
    pub contratista_id: String,
    pub nueva_fecha_praind: String,
    pub motivo: Option<String>,
}

pub async fn actualizar_praind_con_historial(
    _search_service: &Arc<SearchService>,
    input: ActualizarPraindInput,
    usuario_id: String,
) -> Result<ContratistaResponse, ContratistaError> {
    let id = parse_contratista_id(&input.contratista_id);

    let contratista =
        db::find_by_id(&id).await.map_err(map_db_error)?.ok_or(ContratistaError::NotFound)?;

    // Parse formatted string from Datetime (Day-Month-Year)
    let dt: chrono::DateTime<chrono::Utc> = contratista
        .fecha_vencimiento_praind
        .to_string()
        .parse()
        .unwrap_or_else(|_| chrono::Utc::now());
    let fecha_anterior = dt.format("%d-%m-%Y").to_string();

    let nueva_fecha =
        crate::models::contratista::validaciones::validar_fecha(&input.nueva_fecha_praind)
            .map_err(ContratistaError::Validation)?;

    // Use typed DTO for update
    use crate::models::contratista::ContratistaUpdateDTO;
    let dto = ContratistaUpdateDTO {
        fecha_vencimiento_praind: Some(nueva_fecha.into()),
        ..Default::default()
    };

    let updated = db::update(&id, dto).await.map_err(map_db_error)?;

    audit_db::insert_praind_historial(
        &input.contratista_id,
        Some(&fecha_anterior),
        &input.nueva_fecha_praind,
        &usuario_id,
        input.motivo.as_deref(),
    )
    .await
    .map_err(map_db_error)?;

    build_response_fetched(updated).await
}

// ==========================================
// CAMBIAR ESTADO CON HISTORIAL
// ==========================================

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CambiarEstadoConHistorialInput {
    pub contratista_id: String,
    pub nuevo_estado: String,
    pub motivo: String,
}

pub async fn cambiar_estado_con_historial(
    _search_service: &Arc<SearchService>,
    input: CambiarEstadoConHistorialInput,
    usuario_id: String,
) -> Result<ContratistaResponse, ContratistaError> {
    let id = parse_contratista_id(&input.contratista_id);

    let contratista =
        db::find_by_id(&id).await.map_err(map_db_error)?.ok_or(ContratistaError::NotFound)?;

    let estado_anterior = contratista.estado.as_str().to_string();

    let nuevo_estado: EstadoContratista =
        input.nuevo_estado.parse().map_err(ContratistaError::Validation)?;

    let updated = db::update_status(&id, nuevo_estado.as_str()).await.map_err(map_db_error)?;

    audit_db::insert_historial_estado(
        &input.contratista_id,
        &estado_anterior,
        nuevo_estado.as_str(),
        Some(&usuario_id),
        &input.motivo,
    )
    .await
    .map_err(map_db_error)?;

    build_response_fetched(updated).await
}

// ==========================================
// HELPERS
// ==========================================

async fn build_response_fetched(
    contratista: crate::models::contratista::ContratistaFetched,
) -> Result<ContratistaResponse, ContratistaError> {
    let mut response = ContratistaResponse::from_fetched(contratista.clone());

    // Obtener vehículo
    let vehiculos = veh_db::find_by_propietario(&contratista.id).await.map_err(map_db_error)?;
    if let Some(v) = vehiculos.first() {
        response.vehiculo_tipo = Some(v.tipo_vehiculo.to_string());
        response.vehiculo_placa = Some(v.placa.clone());
        response.vehiculo_marca = v.marca.clone();
        response.vehiculo_modelo = v.modelo.clone();
        response.vehiculo_color = v.color.clone();
    }

    // Verificar si está bloqueado
    let block_status =
        ln_db::check_if_blocked_by_cedula(&contratista.cedula).await.map_err(map_db_error)?;
    response.esta_bloqueado = block_status.is_blocked;

    if block_status.is_blocked {
        response.puede_ingresar = false;
    }

    Ok(response)
}

// ==========================================
// RESTORE & ARCHIVED
// ==========================================

pub async fn restore_contratista(
    search_service: &Arc<SearchService>,
    id_str: String,
) -> Result<(), ContratistaError> {
    let id = parse_contratista_id(&id_str);

    // Verify it exists (even if deleted, find_by_id checks physical existence)
    let exists = db::find_by_id(&id).await.map_err(map_db_error)?;
    if exists.is_none() {
        return Err(ContratistaError::NotFound);
    }

    db::restore(&id).await.map_err(map_db_error)?;

    // Re-index
    if let Some(contratista) = db::find_by_id_fetched(&id).await.map_err(map_db_error)? {
        let empresa_nombre = contratista.empresa.nombre.clone();
        if let Err(e) = search_service.add_contratista_fetched(&contratista, &empresa_nombre).await
        {
            log::warn!("Error re-indexing restored contratista: {}", e);
        }
    }

    Ok(())
}

pub async fn get_archived_contratistas() -> Result<Vec<ContratistaResponse>, ContratistaError> {
    let raw_list = db::find_archived().await.map_err(map_db_error)?;

    let mut contratistas = Vec::new();
    for c in raw_list {
        contratistas.push(build_response_fetched(c).await?);
    }

    Ok(contratistas)
}
