// use crate::db::contratista_queries as db;
// use crate::db::empresa_queries;
// use crate::db::lista_negra_queries;
use crate::db::surrealdb_audit_queries as audit_db;
use crate::db::surrealdb_contratista_queries as db;
use crate::db::surrealdb_empresa_queries as empresa_db;
use crate::db::surrealdb_lista_negra_queries as ln_db;
use crate::db::surrealdb_vehiculo_queries as veh_db;

use crate::domain::contratista as domain;
use crate::domain::errors::ContratistaError;
use crate::models::contratista::{
    CambiarEstadoInput, ContratistaListResponse, ContratistaResponse, CreateContratistaInput,
    EstadoContratista, UpdateContratistaInput,
};
use crate::services::search_service::SearchService;
use crate::services::surrealdb_service::SurrealDbError;
use log::{error, info};
use std::sync::Arc;

// Helper para mapear errores de SurrealDB a ContratistaError
fn map_db_error(e: SurrealDbError) -> ContratistaError {
    ContratistaError::Database(sqlx::Error::Protocol(e.to_string()))
}

// ==========================================
// CREAR CONTRATISTA
// ==========================================

pub async fn create_contratista(
    _search_service: &Arc<SearchService>, // Mantenemos firma por compatibilidad, pero está stubbed
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
    info!(
        "Creando contratista con cédula {} para empresa {}",
        cedula_normalizada, input.empresa_id
    );
    let empresa_opt =
        empresa_db::get_empresa_by_id(&input.empresa_id).await.map_err(map_db_error)?;
    if empresa_opt.is_none() {
        return Err(ContratistaError::EmpresaNotFound);
    }

    // 6. Insertar en DB
    let contratista = db::create(input).await.map_err(|e| {
        error!("Error de base de datos al crear contratista {}: {}", cedula_normalizada, e);
        map_db_error(e)
    })?;

    info!("Contratista {} creado exitosamente con ID {}", cedula_normalizada, contratista.id);

    // 7. Retornar respuesta
    build_response(contratista).await
}

// ==========================================
// OBTENER CONTRATISTA POR ID
// ==========================================

pub async fn get_contratista_by_id(id: &str) -> Result<ContratistaResponse, ContratistaError> {
    let contratista =
        db::find_by_id(id).await.map_err(map_db_error)?.ok_or(ContratistaError::NotFound)?;
    build_response(contratista).await
}

// ==========================================
// OBTENER CONTRATISTA POR CÉDULA
// ==========================================

pub async fn get_contratista_by_cedula(
    cedula: &str,
) -> Result<ContratistaResponse, ContratistaError> {
    let contratista = db::find_by_cedula(cedula)
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
    let raw_list = db::find_all().await.map_err(map_db_error)?; // Simplificado para este paso

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
    id: String,
    input: UpdateContratistaInput,
) -> Result<ContratistaResponse, ContratistaError> {
    // 1. Validar input
    domain::validar_update_input(&input)?;

    info!("Actualizando contratista con ID {}", id);

    // 2. Verificar que el contratista existe
    let _ = db::find_by_id(&id).await.map_err(map_db_error)?.ok_or(ContratistaError::NotFound)?;

    // 3. Verificar que la empresa exista si viene
    if let Some(ref empresa_id) = input.empresa_id {
        let empresa_opt = empresa_db::get_empresa_by_id(empresa_id).await.map_err(map_db_error)?;
        if empresa_opt.is_none() {
            return Err(ContratistaError::EmpresaNotFound);
        }
    }

    // 4. Actualizar en DB
    let updated = db::update(&id, input).await.map_err(|e| {
        error!("Error al actualizar contratista {}: {}", id, e);
        map_db_error(e)
    })?;

    info!("Contratista {} actualizado exitosamente", id);

    // 5. Retornar
    build_response(updated).await
}

// ==========================================
// CAMBIAR ESTADO
// ==========================================

pub async fn cambiar_estado_contratista(
    _search_service: &Arc<SearchService>,
    id: String,
    input: CambiarEstadoInput,
) -> Result<ContratistaResponse, ContratistaError> {
    // 1. Validar estado
    let estado = domain::validar_estado(&input.estado)?;

    info!("Cambiando estado de contratista {} a {}", id, input.estado);

    // 2. Verificar que el contratista existe
    let _ = db::find_by_id(&id).await.map_err(map_db_error)?.ok_or(ContratistaError::NotFound)?;

    // 3. Actualizar estado en DB
    let updated = db::update_status(&id, estado.as_str()).await.map_err(map_db_error)?;

    // 4. Retornar
    build_response(updated).await
}

// ==========================================
// ELIMINAR CONTRATISTA
// ==========================================

pub async fn delete_contratista(
    _search_service: &Arc<SearchService>,
    id: String,
) -> Result<(), ContratistaError> {
    info!("Eliminando contratista con ID {}", id);

    // Verificar que existe
    let _ = db::find_by_id(&id).await.map_err(map_db_error)?.ok_or(ContratistaError::NotFound)?;

    // Eliminar de DB
    db::delete(&id).await.map_err(|e| {
        error!("Error al eliminar contratista {}: {}", id, e);
        map_db_error(e)
    })?;

    info!("Contratista {} eliminado exitosamente", id);
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
    // 1. Obtener contratista actual
    let contratista = db::find_by_id(&input.contratista_id)
        .await
        .map_err(map_db_error)?
        .ok_or(ContratistaError::NotFound)?;

    let fecha_anterior = contratista.fecha_vencimiento_praind.clone();

    // 2. Validar nueva fecha
    crate::models::contratista::validaciones::validar_fecha(&input.nueva_fecha_praind)
        .map_err(|e| ContratistaError::Validation(e))?;

    // 3. Actualizar contratista
    info!(
        "Actualizando PRAIND para contratista {} -> {}",
        input.contratista_id, input.nueva_fecha_praind
    );
    let updated = db::update(
        &input.contratista_id,
        UpdateContratistaInput {
            fecha_vencimiento_praind: Some(input.nueva_fecha_praind.clone()),
            ..Default::default()
        },
    )
    .await
    .map_err(map_db_error)?;

    // 4. Registrar en historial
    audit_db::insert_praind_historial(
        &input.contratista_id,
        Some(&fecha_anterior),
        &input.nueva_fecha_praind,
        &usuario_id,
        input.motivo.as_deref(),
    )
    .await
    .map_err(map_db_error)?;

    // 5. Retornar
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
    // 1. Obtener contratista actual
    let contratista = db::find_by_id(&input.contratista_id)
        .await
        .map_err(map_db_error)?
        .ok_or(ContratistaError::NotFound)?;

    let estado_anterior = contratista.estado.as_str().to_string();

    // 2. Validar nuevo estado
    let nuevo_estado: crate::models::contratista::EstadoContratista =
        input.nuevo_estado.parse().map_err(|e: String| ContratistaError::Validation(e))?;

    // 3. Actualizar estado
    let updated = db::update_status(&input.contratista_id, nuevo_estado.as_str())
        .await
        .map_err(map_db_error)?;

    // 4. Registrar en historial
    audit_db::insert_historial_estado(
        &input.contratista_id,
        &estado_anterior,
        nuevo_estado.as_str(),
        Some(&usuario_id),
        &input.motivo,
    )
    .await
    .map_err(map_db_error)?;

    // 5. Retornar
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
        db::get_empresa_nombre(&contratista.empresa_id).await.map_err(map_db_error)?;

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

#[derive(Default)]
pub struct DefaultUpdateContratistaInput {
    pub nombre: Option<String>,
    pub segundo_nombre: Option<String>,
    pub apellido: Option<String>,
    pub segundo_apellido: Option<String>,
    pub empresa_id: Option<String>,
    pub fecha_vencimiento_praind: Option<String>,
}

impl From<DefaultUpdateContratistaInput> for UpdateContratistaInput {
    fn from(d: DefaultUpdateContratistaInput) -> Self {
        Self {
            nombre: d.nombre,
            segundo_nombre: d.segundo_nombre,
            apellido: d.apellido,
            segundo_apellido: d.segundo_apellido,
            empresa_id: d.empresa_id,
            fecha_vencimiento_praind: d.fecha_vencimiento_praind,
            tiene_vehiculo: None,
            tipo_vehiculo: None,
            placa: None,
            marca: None,
            modelo: None,
            color: None,
        }
    }
}
