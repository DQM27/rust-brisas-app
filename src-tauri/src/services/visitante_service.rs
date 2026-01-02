//! # Servicio: Gestión de Visitantes Ocasionales
//!
//! Este servicio orquesta el registro y seguimiento de personas externas.
//! Implementa filtros de seguridad (Lista Negra) y vinculación opcional
//! de activos móviles (Vehículos).

use crate::db::surrealdb_lista_negra_queries as ln_db;
use crate::db::surrealdb_vehiculo_queries as veh_db;
use crate::db::surrealdb_visitante_queries as db;
use crate::domain::errors::VisitanteError;
use crate::domain::vehiculo as vehiculo_domain;
use crate::domain::visitante as domain;
use crate::models::vehiculo::{TipoVehiculo, VehiculoCreateDTO};
use crate::models::visitante::{
    CreateVisitanteInput, VisitanteCreateDTO, VisitanteResponse, VisitanteUpdateDTO,
};
use crate::services::surrealdb_service::SurrealDbError;
use chrono::Utc;
use log::{debug, error, info, warn};
use surrealdb::RecordId;

/// Mapeo de errores técnicos a errores de negocio.
fn map_db_error(e: SurrealDbError) -> VisitanteError {
    error!("Fallo en SurrealDB durante operación de visitantes: {}", e);
    VisitanteError::Database(e.to_string())
}

/// Normalización de IDs de visitante.
fn parse_visitante_id(id_str: &str) -> RecordId {
    if id_str.contains(':') {
        let parts: Vec<&str> = id_str.split(':').collect();
        RecordId::from_table_key(parts[0], parts[1])
    } else {
        RecordId::from_table_key("visitante", id_str)
    }
}

/// Normalización de IDs de empresa.
fn parse_empresa_id(id_str: &str) -> RecordId {
    if id_str.contains(':') {
        let parts: Vec<&str> = id_str.split(':').collect();
        RecordId::from_table_key(parts[0], parts[1])
    } else {
        RecordId::from_table_key("empresa", id_str)
    }
}

/// Registra un nuevo visitante.
///
/// El flujo garantiza:
/// 1. Validación y Normalización de Identidad.
/// 2. Filtro de Seguridad: Bloqueo si aparece en la lista negra.
/// 3. Registro de Propiedad Vehicular: Si el visitante ingresa con vehículo.
pub async fn create_visitante(
    mut input: CreateVisitanteInput,
) -> Result<VisitanteResponse, VisitanteError> {
    debug!("Iniciando registro de nuevo visitante para cédula: {}", input.cedula);
    domain::validar_create_input(&input)?;

    // Normalización de datos para evitar duplicados por formato.
    input.cedula = domain::normalizar_cedula(&input.cedula);
    input.nombre = domain::normalizar_nombre(&input.nombre);
    input.apellido = domain::normalizar_nombre(&input.apellido);
    if let Some(s) = input.segundo_nombre.as_ref() {
        input.segundo_nombre = Some(domain::normalizar_nombre(s));
    }
    if let Some(s) = input.segundo_apellido.as_ref() {
        input.segundo_apellido = Some(domain::normalizar_nombre(s));
    }

    // Seguridad: Chequeo preventivo obligatorio en Lista Negra.
    debug!("Verificando Lista Negra para cédula: {}", input.cedula);
    let block_status =
        ln_db::check_if_blocked_by_cedula(&input.cedula).await.map_err(map_db_error)?;

    if block_status.is_blocked {
        let nivel = block_status.nivel_severidad.unwrap_or_else(|| "BAJO".to_string());
        warn!(
            "🚨 BLOQUEO DE SEGURIDAD: Visitante {} intentó ingresar con nivel de riesgo: {}",
            input.cedula, nivel
        );
        return Err(VisitanteError::Validation(format!(
            "BLOQUEO DE SEGURIDAD: Cédula {} en lista negra (Nivel: {}).",
            input.cedula, nivel
        )));
    }

    if db::get_visitante_by_cedula(&input.cedula).await.map_err(map_db_error)?.is_some() {
        warn!("Intento de registro duplicado para cédula: {}", input.cedula);
        return Err(VisitanteError::CedulaExists);
    }

    let dto = VisitanteCreateDTO {
        cedula: input.cedula,
        nombre: input.nombre,
        apellido: input.apellido,
        segundo_nombre: input.segundo_nombre,
        segundo_apellido: input.segundo_apellido,
        empresa: parse_empresa_id(&input.empresa_id),
        has_vehicle: input.has_vehicle,
    };

    info!("✅ Registrando nuevo visitante: {} {} (ID: {})", dto.nombre, dto.apellido, dto.cedula);
    let visitante = db::create_visitante(dto).await.map_err(map_db_error)?;

    // Gestión del vehículo asociado al visitante para control de acceso.
    if input.has_vehicle {
        if let (Some(tipo), Some(placa)) = (&input.tipo_vehiculo, &input.placa) {
            if !tipo.is_empty() && !placa.is_empty() {
                debug!("Registrando activo móvil vinculado: Placa {}", placa);
                let tipo_norm = vehiculo_domain::validar_tipo_vehiculo(tipo)
                    .map_err(|e| VisitanteError::Validation(e.to_string()))?
                    .as_str()
                    .to_string();

                let placa_norm = vehiculo_domain::normalizar_placa(placa);

                let dto_vehiculo = VehiculoCreateDTO {
                    propietario: visitante.id.clone(),
                    tipo_vehiculo: tipo_norm
                        .parse::<TipoVehiculo>()
                        .map_err(|e| VisitanteError::Validation(e))?,
                    placa: placa_norm,
                    marca: input.marca.as_ref().map(|s| s.trim().to_string()),
                    modelo: input.modelo.as_ref().map(|s| s.trim().to_string()),
                    color: input.color.as_ref().map(|s| s.trim().to_string()),
                    is_active: true,
                };

                match veh_db::insert(dto_vehiculo).await {
                    Ok(_) => info!("🚗 Vehículo Placa {} registrado y vinculado con éxito", placa),
                    Err(e) => error!("Fallo al vincular vehículo en registro de visitante: {}", e),
                }
            }
        }
    }

    // Retornamos el perfil completo (incluyendo resolución de empresa si aplica).
    if let Ok(Some(fetched)) = db::find_by_id_fetched(&visitante.id).await {
        return Ok(VisitanteResponse::from_fetched(fetched));
    }

    Ok(VisitanteResponse::from(visitante))
}

pub async fn search_visitantes(term: &str) -> Result<Vec<VisitanteResponse>, VisitanteError> {
    let visitantes = db::search_visitantes(term).await.map_err(map_db_error)?;
    Ok(visitantes.into_iter().map(VisitanteResponse::from_fetched).collect())
}

pub async fn get_visitante_by_id(
    id_str: &str,
) -> Result<Option<VisitanteResponse>, VisitanteError> {
    let id_thing = parse_visitante_id(id_str);
    let opt = db::find_by_id_fetched(&id_thing).await.map_err(map_db_error)?;
    Ok(opt.map(VisitanteResponse::from_fetched))
}

pub async fn get_visitante_by_cedula(
    cedula: &str,
) -> Result<Option<VisitanteResponse>, VisitanteError> {
    let cedula_norm = domain::normalizar_cedula(cedula);
    let opt = db::get_visitante_by_cedula(&cedula_norm).await.map_err(map_db_error)?;
    Ok(opt.map(VisitanteResponse::from))
}

/// Actualiza los datos de un visitante.
pub async fn update_visitante(
    id_str: &str,
    mut input: CreateVisitanteInput,
) -> Result<VisitanteResponse, VisitanteError> {
    let id_thing = parse_visitante_id(id_str);
    debug!("Actualizando perfil de visitante: {}", id_str);

    db::find_by_id(&id_thing).await.map_err(map_db_error)?.ok_or(VisitanteError::NotFound)?;

    input.nombre = domain::normalizar_nombre(&input.nombre);
    input.apellido = domain::normalizar_nombre(&input.apellido);

    let mut dto = VisitanteUpdateDTO::default();
    dto.nombre = Some(input.nombre);
    dto.apellido = Some(input.apellido);
    dto.segundo_nombre = Some(input.segundo_nombre);
    dto.segundo_apellido = Some(input.segundo_apellido);
    dto.empresa = Some(parse_empresa_id(&input.empresa_id));
    dto.has_vehicle = Some(input.has_vehicle);
    dto.updated_at = Some(surrealdb::Datetime::from(Utc::now()));

    info!("📝 Actualizando datos del visitante ID: {}", id_str);
    let visitante = db::update(&id_thing, dto).await.map_err(map_db_error)?;

    if let Ok(Some(fetched)) = db::find_by_id_fetched(&visitante.id).await {
        return Ok(VisitanteResponse::from_fetched(fetched));
    }

    Ok(VisitanteResponse::from(visitante))
}

pub async fn delete_visitante(id_str: &str) -> Result<(), VisitanteError> {
    let id_thing = parse_visitante_id(id_str);
    db::find_by_id(&id_thing).await.map_err(map_db_error)?.ok_or(VisitanteError::NotFound)?;

    info!("🗑️ Archivando visitante: {}", id_str);
    db::delete(&id_thing).await.map_err(map_db_error)
}

pub async fn restore_visitante(id_str: &str) -> Result<VisitanteResponse, VisitanteError> {
    let id_thing = parse_visitante_id(id_str);
    info!("♻️ Restaurando visitante: {}", id_str);
    let visitante = db::restore(&id_thing).await.map_err(map_db_error)?;

    if let Ok(Some(fetched)) = db::find_by_id_fetched(&visitante.id).await {
        return Ok(VisitanteResponse::from_fetched(fetched));
    }

    Ok(VisitanteResponse::from(visitante))
}

pub async fn get_archived_visitantes() -> Result<Vec<VisitanteResponse>, VisitanteError> {
    debug!("Consultando catálogo de visitantes archivados");
    let visitantes = db::find_archived().await.map_err(map_db_error)?;
    Ok(visitantes.into_iter().map(VisitanteResponse::from_fetched).collect())
}

pub async fn get_all_visitantes() -> Result<Vec<VisitanteResponse>, VisitanteError> {
    debug!("Consultando listado total de visitantes");
    let visitantes = db::find_all().await.map_err(map_db_error)?;
    Ok(visitantes.into_iter().map(VisitanteResponse::from_fetched).collect())
}

// --------------------------------------------------------------------------
// PRUEBAS UNITARIAS
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_visitante_id() {
        let id = parse_visitante_id("123");
        assert_eq!(id.table(), "visitante");
        assert_eq!(id.key().to_string(), "123");

        let id_comp = parse_visitante_id("visitante:abc");
        assert_eq!(id_comp.table(), "visitante");
        assert_eq!(id_comp.key().to_string(), "abc");
    }

    #[test]
    fn test_parse_empresa_id() {
        let id = parse_empresa_id("brisa");
        assert_eq!(id.table(), "empresa");
        assert_eq!(id.key().to_string(), "brisa");

        let id_comp = parse_empresa_id("empresa:x");
        assert_eq!(id_comp.table(), "empresa");
        assert_eq!(id_comp.key().to_string(), "x");
    }
}
