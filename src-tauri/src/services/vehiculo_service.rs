//! # Servicio: Gestión de Vehículos (Activos Móviles)
//!
//! Este servicio orquesta el parque vehicular de la plataforma, gestionando
//! la vinculación de unidades móviles con sus propietarios (Personal,
//! Proveedores o Visitantes) y asegurando la integridad de las placas.
//!
//! ## Responsabilidades
//! - Registro y normalización de placas vehiculares.
//! - Validación cross-table de propietarios en SurrealDB.
//! - Gestión de estatus y estadísticas de flota.
//! - Auditoría de cambios en activos móviles.
use crate::db::surrealdb_contratista_queries as contratista_db;
use crate::db::surrealdb_proveedor_queries as proveedor_db;
use crate::db::surrealdb_vehiculo_queries as db;
use crate::db::surrealdb_visitante_queries as visitante_db;
use crate::domain::errors::VehiculoError;
use crate::domain::vehiculo as domain;
use crate::models::vehiculo::{
    TipoVehiculo, TipoVehiculoStats, UpdateVehiculoInput, VehiculoCreateDTO, VehiculoListResponse,
    VehiculoResponse, VehiculoUpdateDTO,
};
use crate::services::surrealdb_service::SurrealDbError;
use chrono::Utc;
use log::{debug, error, info, warn};
use surrealdb::RecordId;

// ==========================================
// HELPERS DE IDENTIDAD Y PARSEO
// ==========================================

/// Mapeo de errores de infraestructura a dominio.
fn map_db_error(e: SurrealDbError) -> VehiculoError {
    error!("❌ Fallo técnico en persistencia de vehículos: {}", e);
    VehiculoError::Database(e.to_string())
}

/// Normalización de IDs de vehículo.
fn parse_vehiculo_id(id_str: &str) -> RecordId {
    if id_str.contains(':') {
        let parts: Vec<&str> = id_str.split(':').collect();
        RecordId::from_table_key(parts[0], parts[1])
    } else {
        RecordId::from_table_key("vehiculo", id_str)
    }
}

/// Identifica al propietario del vehículo analizando el prefijo de la tabla.
fn parse_propietario_id(id_str: &str) -> RecordId {
    if id_str.contains(':') {
        let parts: Vec<&str> = id_str.split(':').collect();
        RecordId::from_table_key(parts[0], parts[1])
    } else {
        // Por defecto asume contratista si no hay contexto de tabla explícito.
        RecordId::from_table_key("contratista", id_str)
    }
}

// ==========================================
// SERVICIOS DE ORQUESTACIÓN
// ==========================================

/// Registra un nuevo vehículo garantizando la unicidad de su placa.
///
/// El flujo de validación asegura:
/// 1. Existencia del Propietario: El vehículo debe estar vinculado a una persona válida.
/// 2. Integridad de la Placa: No se permiten duplicados para evitar suplantaciones.
/// 3. Normalización: La placa se guarda en un formato uniforme para facilitar búsquedas.
pub async fn create_vehiculo(
    input: crate::models::vehiculo::CreateVehiculoInput,
) -> Result<VehiculoResponse, VehiculoError> {
    domain::validar_create_input(&input)?;
    let placa_normalizada = domain::normalizar_placa(&input.placa);
    let tipo_vehiculo = domain::validar_tipo_vehiculo(&input.tipo_vehiculo)?;

    let propietario_id = parse_propietario_id(&input.propietario_id);
    info!("🚗 Iniciando registro de unidad móvil para {}...", propietario_id);

    // Validación Cross-Table: Comprueba la existencia física del dueño en su respectiva tabla.
    debug!("🔍 Verificando existencia del propietario en tabla {}", propietario_id.table());
    let exists = match propietario_id.table() {
        "contratista" => {
            contratista_db::find_by_id(&propietario_id).await.map_err(map_db_error)?.is_some()
        }
        "proveedor" => {
            proveedor_db::find_by_id(&propietario_id).await.map_err(map_db_error)?.is_some()
        }
        "visitante" => {
            visitante_db::find_by_id(&propietario_id).await.map_err(map_db_error)?.is_some()
        }
        _ => {
            warn!(
                "⚠️ Intento de registro con tipo de propietario inválido: {}",
                propietario_id.table()
            );
            return Err(VehiculoError::Validation(
                "Tipo de ente propietario no reconocido".to_string(),
            ));
        }
    };

    if !exists {
        warn!("🚨 Protocolo de identidad fallido: El propietario {} no existe", propietario_id);
        return Err(VehiculoError::Validation(format!(
            "Protocolo de identidad fallido: El propietario no existe en la base de datos de {}",
            propietario_id.table()
        )));
    }

    let count = db::count_by_placa(&placa_normalizada).await.map_err(map_db_error)?;
    if count > 0 {
        warn!("⚠️ Intento de duplicar placa ya registrada: {}", placa_normalizada);
        return Err(VehiculoError::PlacaExists);
    }

    let dto = VehiculoCreateDTO {
        propietario: propietario_id.clone(),
        tipo_vehiculo,
        placa: placa_normalizada.clone(),
        marca: input.marca.as_ref().map(|s| s.trim().to_string()),
        modelo: input.modelo.as_ref().map(|s| s.trim().to_string()),
        color: input.color.as_ref().map(|s| s.trim().to_string()),
        is_active: true,
    };

    let vehiculo_creado = db::insert(dto).await.map_err(map_db_error)?;
    info!("✅ Vehículo [{}] registrado exitosamente para {}", placa_normalizada, propietario_id);
    Ok(VehiculoResponse::from(vehiculo_creado))
}

pub async fn get_vehiculo_by_id(id_str: &str) -> Result<VehiculoResponse, VehiculoError> {
    let id = parse_vehiculo_id(id_str);
    let vehiculo =
        db::find_by_id_fetched(&id).await.map_err(map_db_error)?.ok_or(VehiculoError::NotFound)?;

    Ok(VehiculoResponse::from_fetched(vehiculo))
}

pub async fn get_vehiculo_by_placa(placa: String) -> Result<VehiculoResponse, VehiculoError> {
    let placa_normalizada = domain::normalizar_placa(&placa);
    let vehiculo = db::find_by_placa(&placa_normalizada)
        .await
        .map_err(map_db_error)?
        .ok_or(VehiculoError::NotFound)?;
    Ok(VehiculoResponse::from_fetched(vehiculo))
}

/// Obtiene todos los vehículos con estadísticas de composición de flota.
pub async fn get_all_vehiculos() -> Result<VehiculoListResponse, VehiculoError> {
    let vehiculos = db::find_all_fetched().await.map_err(map_db_error)?;
    let mut vehiculo_responses = Vec::with_capacity(vehiculos.len());
    for vehiculo in vehiculos {
        vehiculo_responses.push(VehiculoResponse::from_fetched(vehiculo));
    }
    let total = vehiculo_responses.len();
    let activos = vehiculo_responses.iter().filter(|v| v.is_active).count();
    let inactivos = total - activos;
    let motocicletas =
        vehiculo_responses.iter().filter(|v| v.tipo_vehiculo == TipoVehiculo::Motocicleta).count();
    let automoviles =
        vehiculo_responses.iter().filter(|v| v.tipo_vehiculo == TipoVehiculo::Automovil).count();
    Ok(VehiculoListResponse {
        vehiculos: vehiculo_responses,
        total,
        activos,
        inactivos,
        por_tipo: TipoVehiculoStats { motocicletas, automoviles },
    })
}

pub async fn get_vehiculos_activos() -> Result<Vec<VehiculoResponse>, VehiculoError> {
    let vehiculos = db::find_activos_fetched().await.map_err(map_db_error)?;
    let mut vehiculo_responses = Vec::with_capacity(vehiculos.len());
    for vehiculo in vehiculos {
        vehiculo_responses.push(VehiculoResponse::from_fetched(vehiculo));
    }
    Ok(vehiculo_responses)
}

/// Filtra los vehículos pertenecientes a una persona específica.
pub async fn get_vehiculos_by_propietario(
    id_str: String,
) -> Result<Vec<VehiculoResponse>, VehiculoError> {
    let id = parse_propietario_id(&id_str);
    let vehiculos = db::find_by_propietario(&id).await.map_err(map_db_error)?;
    let mut vehiculo_responses = Vec::with_capacity(vehiculos.len());
    for vehiculo in vehiculos {
        vehiculo_responses.push(VehiculoResponse::from(vehiculo));
    }
    Ok(vehiculo_responses)
}

/// Actualiza los detalles de un vehículo, como cambio de color o estado operativo.
pub async fn update_vehiculo(
    id_str: String,
    input: UpdateVehiculoInput,
) -> Result<VehiculoResponse, VehiculoError> {
    let id = parse_vehiculo_id(&id_str);
    domain::validar_update_input(&input)?;

    db::find_by_id(&id).await.map_err(map_db_error)?.ok_or(VehiculoError::NotFound)?;

    let mut dto = VehiculoUpdateDTO::default();
    if let Some(t) = input.tipo_vehiculo {
        let tipo = domain::validar_tipo_vehiculo(&t)?;
        dto.tipo_vehiculo = Some(tipo);
    }
    if let Some(m) = input.marca {
        dto.marca = Some(m.trim().to_string());
    }
    if let Some(m) = input.modelo {
        dto.modelo = Some(m.trim().to_string());
    }
    if let Some(c) = input.color {
        dto.color = Some(c.trim().to_string());
    }
    if let Some(a) = input.is_active {
        dto.is_active = Some(a);
    }
    dto.updated_at = Some(surrealdb::Datetime::from(Utc::now()));

    let updated = db::update(&id, dto).await.map_err(map_db_error)?;
    info!("📝 Perfil de vehículo {} actualizado correctamente.", id_str);
    Ok(VehiculoResponse::from_fetched(updated))
}

pub async fn delete_vehiculo(id_str: String) -> Result<(), VehiculoError> {
    let id = parse_vehiculo_id(&id_str);
    db::find_by_id(&id).await.map_err(map_db_error)?.ok_or(VehiculoError::NotFound)?;

    info!("🗑️ Procesando baja del vehículo {}...", id_str);
    db::delete(&id).await.map_err(map_db_error)?;
    info!("✅ Vehículo {} eliminado del sistema de control.", id_str);
    Ok(())
}

// --------------------------------------------------------------------------
// PRUEBAS UNITARIAS
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_vehiculo_id() {
        let id = parse_vehiculo_id("vehiculo:v123");
        assert_eq!(id.table(), "vehiculo");
        assert_eq!(id.key().to_string(), "v123");

        let id_clean = parse_vehiculo_id("v456");
        assert_eq!(id_clean.table(), "vehiculo");
        assert_eq!(id_clean.key().to_string(), "v456");
    }

    #[test]
    fn test_parse_propietario_id() {
        // Casos con tabla explícita
        let prov = parse_propietario_id("proveedor:p1");
        assert_eq!(prov.table(), "proveedor");

        let visit = parse_propietario_id("visitante:v1");
        assert_eq!(visit.table(), "visitante");

        // Caso por defecto (contratista)
        let cont = parse_propietario_id("c1");
        assert_eq!(cont.table(), "contratista");
    }
}
