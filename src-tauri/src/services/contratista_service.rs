use crate::db::surrealdb_audit_queries as audit_db;
use crate::db::surrealdb_contratista_queries as db;
use crate::db::surrealdb_empresa_queries as empresa_db;
use crate::db::surrealdb_lista_negra_queries as ln_db;
use crate::db::surrealdb_vehiculo_queries as veh_db;

use crate::domain::contratista as domain;
use crate::domain::errors::ContratistaError;
use crate::models::contratista::{
    CambiarEstadoInput, ContratistaCreateDTO, ContratistaListResponse, ContratistaResponse,
    CreateContratistaInput, EstadoContratista, UpdateContratistaInput,
};
use crate::services::search_service::SearchService;
use crate::services::surrealdb_service::SurrealDbError;
use chrono::Utc;
use log::{error, info};
use std::sync::Arc;
use surrealdb::sql::Thing;

// Helper para mapear errores de SurrealDB a ContratistaError
fn map_db_error(e: SurrealDbError) -> ContratistaError {
    ContratistaError::Database(e.to_string())
}

/// Helper para parsear ID de contratista (acepta con o sin prefijo)
fn parse_contratista_id(id_str: &str) -> Thing {
    if id_str.contains(':') {
        let parts: Vec<&str> = id_str.split(':').collect();
        Thing::from((parts[0], parts[1]))
    } else {
        Thing::from(("contratista", id_str))
    }
}

/// Helper para parsear ID de empresa (acepta con o sin prefijo)
fn parse_empresa_id(id_str: &str) -> Thing {
    if id_str.contains(':') {
        let parts: Vec<&str> = id_str.split(':').collect();
        Thing::from((parts[0], parts[1]))
    } else {
        Thing::from(("empresa", id_str))
    }
}

// ==========================================
// CREAR CONTRATISTA
// ==========================================

pub async fn create_contratista(
    _search_service: &Arc<SearchService>,
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
        nombre: input.nombre.trim().to_uppercase(),
        segundo_nombre: input.segundo_nombre.map(|s| s.trim().to_uppercase()),
        apellido: input.apellido.trim().to_uppercase(),
        segundo_apellido: input.segundo_apellido.map(|s| s.trim().to_uppercase()),
        empresa: empresa_id,
        fecha_vencimiento_praind: fecha_vencimiento,
        estado: EstadoContratista::Activo,
    };

    // 7. Insertar en DB
    let contratista = db::create(dto).await.map_err(|e| {
        error!("Error de base de datos al crear contratista {}: {}", cedula_normalizada, e);
        map_db_error(e)
    })?;

    info!("Contratista {} creado exitosamente con ID {}", cedula_normalizada, contratista.id);

    // 8. Retornar respuesta
    build_response(contratista).await
}

// ==========================================
// OBTENER CONTRATISTA POR ID
// ==========================================

pub async fn get_contratista_by_id(id_str: &str) -> Result<ContratistaResponse, ContratistaError> {
    let id = parse_contratista_id(id_str);
    let contratista =
        db::find_by_id(&id).await.map_err(map_db_error)?.ok_or(ContratistaError::NotFound)?;
    build_response(contratista).await
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
    build_response(contratista).await
}

// ==========================================
// OBTENER TODOS LOS CONTRATISTAS
// ==========================================

pub async fn get_all_contratistas() -> Result<ContratistaListResponse, ContratistaError> {
    let raw_list = db::find_all().await.map_err(map_db_error)?;

    let mut contratistas = Vec::new();
    for c in raw_list {
        contratistas.push(build_response(c).await?);
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
    let raw_list = db::find_all().await.map_err(map_db_error)?;

    let mut contratistas = Vec::new();
    for c in raw_list {
        let res = build_response(c).await?;
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
    _search_service: &Arc<SearchService>,
    id_str: String,
    input: UpdateContratistaInput,
) -> Result<ContratistaResponse, ContratistaError> {
    let id = parse_contratista_id(&id_str);

    // 1. Validar input
    domain::validar_update_input(&input)?;

    // 2. Verificar que el contratista existe
    db::find_by_id(&id).await.map_err(map_db_error)?.ok_or(ContratistaError::NotFound)?;

    // 3. Preparar datos de actualización
    let mut update_data = serde_json::Map::new();
    if let Some(v) = input.nombre {
        update_data.insert("nombre".to_string(), serde_json::json!(v.trim().to_uppercase()));
    }
    if let Some(v) = input.segundo_nombre {
        update_data
            .insert("segundo_nombre".to_string(), serde_json::json!(v.trim().to_uppercase()));
    }
    if let Some(v) = input.apellido {
        update_data.insert("apellido".to_string(), serde_json::json!(v.trim().to_uppercase()));
    }
    if let Some(v) = input.segundo_apellido {
        update_data
            .insert("segundo_apellido".to_string(), serde_json::json!(v.trim().to_uppercase()));
    }
    if let Some(v) = input.empresa_id {
        let emp_id = parse_empresa_id(&v);
        // Verificar que la empresa exista
        if empresa_db::find_by_id(&emp_id).await.map_err(map_db_error)?.is_none() {
            return Err(ContratistaError::EmpresaNotFound);
        }
        update_data.insert("empresa".to_string(), serde_json::json!(emp_id));
    }
    if let Some(v) = input.fecha_vencimiento_praind {
        let fecha = crate::models::contratista::validaciones::validar_fecha(&v)
            .map_err(ContratistaError::Validation)?;
        update_data.insert("fecha_vencimiento_praind".to_string(), serde_json::json!(fecha));
    }
    update_data.insert("updated_at".to_string(), serde_json::json!(Utc::now()));

    // 4. Actualizar en DB
    let updated = db::update(&id, serde_json::Value::Object(update_data)).await.map_err(|e| {
        error!("Error al actualizar contratista {}: {}", id_str, e);
        map_db_error(e)
    })?;

    // 5. Retornar
    build_response(updated).await
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
    build_response(updated).await
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

    let fecha_anterior = contratista.fecha_vencimiento_praind.format("%Y-%m-%d").to_string();

    let nueva_fecha =
        crate::models::contratista::validaciones::validar_fecha(&input.nueva_fecha_praind)
            .map_err(ContratistaError::Validation)?;

    let mut update_data = serde_json::Map::new();
    update_data.insert("fecha_vencimiento_praind".to_string(), serde_json::json!(nueva_fecha));
    update_data.insert("updated_at".to_string(), serde_json::json!(Utc::now()));

    let updated =
        db::update(&id, serde_json::Value::Object(update_data)).await.map_err(map_db_error)?;

    audit_db::insert_praind_historial(
        &input.contratista_id,
        Some(&fecha_anterior),
        &input.nueva_fecha_praind,
        &usuario_id,
        input.motivo.as_deref(),
    )
    .await
    .map_err(map_db_error)?;

    build_response(updated).await
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

    build_response(updated).await
}

// ==========================================
// HELPERS
// ==========================================

async fn build_response(
    contratista: crate::models::contratista::Contratista,
) -> Result<ContratistaResponse, ContratistaError> {
    let mut response = ContratistaResponse::from(contratista.clone());

    // Obtener nombre de empresa
    response.empresa_nombre =
        db::get_empresa_nombre(&contratista.empresa).await.map_err(map_db_error)?;

    // Obtener vehículo
    let vehiculos = veh_db::find_by_contratista(&contratista.id).await.map_err(map_db_error)?;
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
