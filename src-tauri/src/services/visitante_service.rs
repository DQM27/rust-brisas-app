//! # Servicio: Gestión de Visitantes Ocasionales
//!
//! Este servicio orquesta el registro y seguimiento de personas externas.
//! Implementa filtros de seguridad (Lista Negra) y vinculación opcional
//! de activos móviles (Vehículos).
//!
//! ## Responsabilidades
//! - Validar datos de visitantes mediante `domain::visitante`
//! - Verificar bloqueos de seguridad en Lista Negra
//! - Registrar vehículos asociados al visitante
//!
//! ## Flujo de Creación
//! 1. Validación y normalización de datos
//! 2. Verificación de Lista Negra (bloqueo preventivo)
//! 3. Verificación de duplicados por cédula
//! 4. Creación del visitante
//! 5. Registro de vehículo (si aplica)

use crate::db::surrealdb_lista_negra_queries as ln_db;
use crate::db::surrealdb_vehiculo_queries as veh_db;
use crate::db::surrealdb_visitante_queries as db;
use crate::domain::common::parse_record_id;
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

// --------------------------------------------------------------------------
// UTILIDADES INTERNAS
// --------------------------------------------------------------------------

/// Mapeo de errores técnicos a errores de negocio.
fn map_db_error(e: SurrealDbError) -> VisitanteError {
    error!("Fallo en SurrealDB durante operación de visitantes: {e}");
    VisitanteError::Database(e.to_string())
}

// --------------------------------------------------------------------------
// OPERACIONES CRUD
// --------------------------------------------------------------------------

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

    // Capturamos datos de vehículo antes de mover el input
    let vehicle_data = if input.has_vehicle {
        Some((
            input.tipo_vehiculo.take(),
            input.placa.take(),
            input.marca.take(),
            input.modelo.take(),
            input.color.take(),
        ))
    } else {
        None
    };

    let dto = VisitanteCreateDTO {
        cedula: input.cedula,
        nombre: input.nombre,
        apellido: input.apellido,
        segundo_nombre: input.segundo_nombre,
        segundo_apellido: input.segundo_apellido,
        empresa: parse_record_id(&input.empresa_id, "empresa"),
        has_vehicle: input.has_vehicle,
    };

    info!("✅ Registrando nuevo visitante: {} {} (ID: {})", dto.nombre, dto.apellido, dto.cedula);
    let visitante = db::create_visitante(dto).await.map_err(map_db_error)?;

    // Gestión del vehículo asociado al visitante para control de acceso.
    // NOTA: Error en vehículo no bloquea la creación del visitante (error parcial).
    let mut vehicle_warning: Option<String> = None;

    if let Some((tipo_opt, placa_opt, marca, modelo, color)) = vehicle_data {
        if let (Some(tipo), Some(placa)) = (tipo_opt, placa_opt) {
            if !tipo.is_empty() && !placa.is_empty() {
                debug!("Registrando activo móvil vinculado: Placa {placa}");

                match registrar_vehiculo_visitante(
                    &visitante.id,
                    &tipo,
                    &placa,
                    marca,
                    modelo,
                    color,
                )
                .await
                {
                    Ok(()) => info!("🚗 Vehículo Placa {placa} registrado y vinculado con éxito"),
                    Err(e) => {
                        let msg = format!("Visitante creado pero falló registro de vehículo: {e}");
                        warn!("⚠️ {msg}");
                        vehicle_warning = Some(msg);
                    }
                }
            }
        }
    }

    // Retornamos el perfil completo (incluyendo resolución de empresa si aplica).
    let mut response = if let Ok(Some(fetched)) = db::find_by_id_fetched(&visitante.id).await {
        VisitanteResponse::from_fetched(fetched)
    } else {
        VisitanteResponse::from(visitante)
    };

    // Adjuntar warning si hubo error parcial con el vehículo
    response.warning = vehicle_warning;

    Ok(response)
}

/// Registra un vehículo asociado a un visitante (función auxiliar).
async fn registrar_vehiculo_visitante(
    visitante_id: &surrealdb::RecordId,
    tipo: &str,
    placa: &str,
    marca: Option<String>,
    modelo: Option<String>,
    color: Option<String>,
) -> Result<(), String> {
    let tipo_norm = vehiculo_domain::validar_tipo_vehiculo(tipo)
        .map_err(|e| e.to_string())?
        .as_str()
        .to_string();

    let placa_norm = vehiculo_domain::normalizar_placa(placa);

    let dto_vehiculo = VehiculoCreateDTO {
        propietario: visitante_id.clone(),
        tipo_vehiculo: tipo_norm.parse::<TipoVehiculo>()?,
        placa: placa_norm,
        marca: marca.as_ref().map(|s| s.trim().to_string()),
        modelo: modelo.as_ref().map(|s| s.trim().to_string()),
        color: color.as_ref().map(|s| s.trim().to_string()),
        is_active: true,
    };

    veh_db::insert(dto_vehiculo).await.map_err(|e| e.to_string())?;
    Ok(())
}

/// Busca visitantes por término de búsqueda (cédula, nombre o apellido).
///
/// La búsqueda es case-insensitive y filtra visitantes no archivados.
pub async fn search_visitantes(term: &str) -> Result<Vec<VisitanteResponse>, VisitanteError> {
    let visitantes = db::search_visitantes(term).await.map_err(map_db_error)?;
    Ok(visitantes.into_iter().map(VisitanteResponse::from_fetched).collect())
}

/// Obtiene un visitante por su ID interno.
///
/// Retorna `None` si el visitante no existe.
pub async fn get_visitante_by_id(
    id_str: &str,
) -> Result<Option<VisitanteResponse>, VisitanteError> {
    let id_thing = parse_record_id(id_str, "visitante");
    let opt = db::find_by_id_fetched(&id_thing).await.map_err(map_db_error)?;
    Ok(opt.map(VisitanteResponse::from_fetched))
}

/// Busca un visitante por su número de cédula.
///
/// La cédula se normaliza antes de buscar para garantizar consistencia.
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
    let id_thing = parse_record_id(id_str, "visitante");
    debug!("Actualizando perfil de visitante: {id_str}");

    db::find_by_id(&id_thing).await.map_err(map_db_error)?.ok_or(VisitanteError::NotFound)?;

    input.nombre = domain::normalizar_nombre(&input.nombre);
    input.apellido = domain::normalizar_nombre(&input.apellido);

    let mut dto = VisitanteUpdateDTO::default();
    dto.nombre = Some(input.nombre);
    dto.apellido = Some(input.apellido);
    dto.segundo_nombre = Some(input.segundo_nombre);
    dto.segundo_apellido = Some(input.segundo_apellido);
    dto.empresa = Some(parse_record_id(&input.empresa_id, "empresa"));
    dto.has_vehicle = Some(input.has_vehicle);
    dto.updated_at = Some(surrealdb::Datetime::from(Utc::now()));

    info!("📝 Actualizando datos del visitante ID: {id_str}");
    let visitante = db::update(&id_thing, dto).await.map_err(map_db_error)?;

    if let Ok(Some(fetched)) = db::find_by_id_fetched(&visitante.id).await {
        return Ok(VisitanteResponse::from_fetched(fetched));
    }

    Ok(VisitanteResponse::from(visitante))
}

/// Archiva un visitante (borrado lógico).
///
/// El visitante permanece en la base de datos pero marcado como eliminado.
pub async fn delete_visitante(id_str: &str) -> Result<(), VisitanteError> {
    let id_thing = parse_record_id(id_str, "visitante");
    db::find_by_id(&id_thing).await.map_err(map_db_error)?.ok_or(VisitanteError::NotFound)?;

    info!("🗑️ Archivando visitante: {id_str}");
    db::delete(&id_thing).await.map_err(map_db_error)
}

/// Restaura un visitante previamente archivado.
pub async fn restore_visitante(id_str: &str) -> Result<VisitanteResponse, VisitanteError> {
    let id_thing = parse_record_id(id_str, "visitante");
    info!("♻️ Restaurando visitante: {id_str}");
    let visitante = db::restore(&id_thing).await.map_err(map_db_error)?;

    if let Ok(Some(fetched)) = db::find_by_id_fetched(&visitante.id).await {
        return Ok(VisitanteResponse::from_fetched(fetched));
    }

    Ok(VisitanteResponse::from(visitante))
}

/// Lista todos los visitantes archivados (eliminados lógicamente).
pub async fn get_archived_visitantes() -> Result<Vec<VisitanteResponse>, VisitanteError> {
    debug!("Consultando catálogo de visitantes archivados");
    let visitantes = db::find_archived().await.map_err(map_db_error)?;
    Ok(visitantes.into_iter().map(VisitanteResponse::from_fetched).collect())
}

/// Obtiene todos los visitantes activos (no archivados).
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
    fn test_parse_record_id_visitante() {
        let id = parse_record_id("123", "visitante");
        assert_eq!(id.table(), "visitante");
        assert_eq!(id.key().to_string().replace("⟨", "").replace("⟩", ""), "123");

        let id_comp = parse_record_id("visitante:abc", "visitante");
        assert_eq!(id_comp.table(), "visitante");
        assert_eq!(id_comp.key().to_string().replace("⟨", "").replace("⟩", ""), "abc");
    }

    #[test]
    fn test_parse_record_id_empresa() {
        let id = parse_record_id("brisa", "empresa");
        assert_eq!(id.table(), "empresa");
        assert_eq!(id.key().to_string().replace("⟨", "").replace("⟩", ""), "brisa");

        let id_comp = parse_record_id("empresa:x", "empresa");
        assert_eq!(id_comp.table(), "empresa");
        assert_eq!(id_comp.key().to_string().replace("⟨", "").replace("⟩", ""), "x");
    }
}
