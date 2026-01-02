/// Servicio: Gestión de Citas y Pre-registros
///
/// Orquestador de la lógica de negocio para el agendamiento y recepción de visitantes.
/// Responsabilidades:
/// - Agendar citas vinculando visitantes (existentes o nuevos).
/// - Consultar la agenda diaria y estados de citas.
/// - Procesar el ingreso físico de visitantes con cita previa.
use crate::db::surrealdb_cita_queries as db;
use crate::domain::errors::CitaError;
use crate::models::cita::{CitaCreateDTO, CitaResponse, CreateCitaInput};
use crate::models::visitante::CreateVisitanteInput;
use crate::services::visitante_service;
use chrono::{Local, Utc};
use surrealdb::RecordId;

// --------------------------------------------------------------------------
// API PÚBLICA DE CITAS: ORQUESTACIÓN
// --------------------------------------------------------------------------

/// Datos consolidados para la operación de agendado.
pub struct AgendarCitaParams {
    pub cita: CreateCitaInput,
    pub visitante_extra: Option<CreateVisitanteInput>,
    pub usuario_id: String,
}

/// Agenda una nueva cita en el calendario institucional.
///
/// # Argumentos
/// * `params` - Estructura DTO con los datos de la cita, visitante opcional y usuario.
///
/// # Retorno
/// Retorna la estructura `CitaResponse` con los datos de la cita agendada.
///
/// # Errores
/// - `CitaError::Validation`: Si los datos de entrada no cumplen las reglas de dominio.
/// - `CitaError::Database`: Si ocurre un error de persistencia o red con `SurrealDB`.
pub async fn agendar_cita(params: AgendarCitaParams) -> Result<CitaResponse, CitaError> {
    // 1. Validación de Dominio (Capa Pura)
    crate::domain::cita::validar_create_input(&params.cita)?;

    let usuario_id = parse_record_id(&params.usuario_id, "user")?;

    // 2. Orquestación de Identidad del Visitante
    let visitante_id: Option<RecordId> = if let Some(ref id_str) = params.cita.visitante_id {
        if id_str.is_empty() {
            None
        } else {
            Some(parse_record_id(id_str, "visitante")?)
        }
    } else {
        None
    };

    let final_visitante_id = if let Some(vid) = visitante_id {
        Some(vid)
    } else if let Some(v_input) = params.visitante_extra {
        let existente = visitante_service::get_visitante_by_cedula(&v_input.cedula)
            .await
            .map_err(|e| CitaError::Database(e.to_string()))?;

        if let Some(v) = existente { Some(parse_record_id(&v.id, "visitante")?) } else {
            let nuevo = visitante_service::create_visitante(v_input)
                .await
                .map_err(|e| CitaError::Database(e.to_string()))?;
            Some(parse_record_id(&nuevo.id, "visitante")?)
        }
    } else {
        None
    };

    let fecha_inicio = parse_datetime(&params.cita.fecha_cita)?;

    // 3. Persistencia
    let dto = CitaCreateDTO {
        visitante_id: final_visitante_id,
        usuario_id,
        motivo: params.cita.motivo,
        fecha_inicio: fecha_inicio.clone(),
        fecha_fin: fecha_inicio,
        anfitrion: Some(params.cita.anfitrion),
        area_visitada: Some(params.cita.area_visitada),
        visitante_nombre: params.cita.visitante_nombre,
        visitante_cedula: params.cita.visitante_cedula,
    };

    let nueva_cita = db::insert(dto).await.map_err(|e| CitaError::Database(e.to_string()))?;

    let fetched = db::find_by_id_fetched(&nueva_cita.id)
        .await
        .map_err(|e| CitaError::Database(e.to_string()))?
        .ok_or(CitaError::Database(
            "Fallo crítico al recuperar la cita recién creada".to_string(),
        ))?;

    Ok(CitaResponse::from_fetched(fetched))
}

/// Recupera la agenda del día de hoy para la visualización rápida en garita.
///
/// # Retorno
/// Lista de citas programadas para el ciclo diario actual.
///
/// # Errores
/// Retorna `CitaError::Database` en caso de fallo en la consulta.
pub async fn get_citas_hoy() -> Result<Vec<CitaResponse>, CitaError> {
    let now = Local::now();
    let hoy_inicio = now.format("%Y-%m-%dT00:00:00").to_string();
    let hoy_fin = now.format("%Y-%m-%dT23:59:59").to_string();

    let citas = db::find_activas_by_fecha_fetched(&hoy_inicio, &hoy_fin)
        .await
        .map_err(|e| CitaError::Database(e.to_string()))?;

    Ok(citas.into_iter().map(CitaResponse::from_fetched).collect())
}

/// Recupera todas las citas pendientes de procesamiento.
///
/// # Retorno
/// Lista de citas con estado 'Programada'.
pub async fn get_citas_pendientes() -> Result<Vec<CitaResponse>, CitaError> {
    let citas =
        db::find_pendientes_fetched().await.map_err(|e| CitaError::Database(e.to_string()))?;

    Ok(citas.into_iter().map(CitaResponse::from_fetched).collect())
}

/// Obtiene el detalle completo de una cita específica por su ID.
///
/// # Argumentos
/// * `id_str` - El identificador de la cita.
///
/// # Retorno
/// El detalle de la cita.
///
/// # Errores
/// - `CitaError::NotFound`: Si el ID no corresponde a ninguna cita.
/// - `CitaError::Database`: Error de acceso a datos.
pub async fn get_cita_by_id(id_str: String) -> Result<CitaResponse, CitaError> {
    let id = parse_record_id(&id_str, "cita")?;

    let cita = db::find_by_id_fetched(&id)
        .await
        .map_err(|e| CitaError::Database(e.to_string()))?
        .ok_or(CitaError::NotFound)?;

    Ok(CitaResponse::from_fetched(cita))
}

/// Cancela una cita planificada, impidiendo su futuro procesamiento.
///
/// # Argumentos
/// * `id_str` - ID de la cita a anular.
///
/// # Retorno
/// Retorna `Ok(())` si la anulación fue exitosa en la base de datos.
pub async fn cancelar_cita(id_str: String) -> Result<(), CitaError> {
    let id = parse_record_id(&id_str, "cita")?;
    db::cancel(&id).await.map_err(|e| CitaError::Database(e.to_string()))?;
    Ok(())
}

/// Acción de Entrada: Convierte una cita planificada en un ingreso real en planta.
///
/// Este es el paso final del flujo de pre-registro. Valida que la cita esté vigente
/// y utiliza el servicio de ingresos para abrir el registro de permanencia física.
///
/// # Argumentos
/// * `cita_id_str` - ID de la cita programada.
/// * `gafete_numero` - Número de gafete físico asignado en portería.
/// * `usuario_id_str` - ID del oficial de seguridad que procesa la entrada.
///
/// # Retorno
/// La cita actualizada o error de procesamiento.
///
/// # Errores
/// - `CitaError::Unauthorized`: Si el usuario no tiene permisos de actualización.
/// - `CitaError::NotFound`: Si la cita no existe.
/// - `CitaError::Validation`: Si la cita ya fue procesada o el gafete es inválido.
/// - `CitaError::Database`: Fallo al registrar el ingreso o actualizar la cita.
pub async fn procesar_ingreso_cita(
    cita_id_str: String,
    gafete_numero: Option<String>,
    usuario_id_str: String,
) -> Result<CitaResponse, CitaError> {
    let cita_id = parse_record_id(&cita_id_str, "cita")?;
    let cita = db::find_by_id_fetched(&cita_id)
        .await
        .map_err(|e| CitaError::Database(e.to_string()))?
        .ok_or(CitaError::NotFound)?;

    if cita.estado != crate::models::cita::EstadoCita::Programada {
        return Err(CitaError::Validation(format!(
            "La cita no se puede procesar porque ya está en estado: {}",
            cita.estado
        )));
    }

    let visitante = cita.visitante_id.as_ref();

    let gafete_int = if let Some(g_str) = gafete_numero {
        if g_str.trim().is_empty() {
            None
        } else {
            Some(
                crate::domain::common::normalizar_gafete_a_int(&g_str)
                    .map_err(CitaError::Validation)?,
            )
        }
    } else {
        None
    };

    let ingreso_input = crate::models::ingreso::CreateIngresoVisitaInput {
        cedula: visitante.as_ref().map(|v| v.cedula.clone()).unwrap_or_default(),
        nombre: visitante.as_ref().map(|v| v.nombre.clone()).unwrap_or_default(),
        apellido: visitante.as_ref().map(|v| v.apellido.clone()).unwrap_or_default(),
        segundo_nombre: visitante.as_ref().and_then(|v| v.segundo_nombre.clone()),
        segundo_apellido: visitante.as_ref().and_then(|v| v.segundo_apellido.clone()),
        anfitrion: cita.anfitrion.clone().unwrap_or_default(),
        area_visitada: cita.area_visitada.clone().unwrap_or_default(),
        motivo: cita.motivo.clone(),
        modo_ingreso: "caminando".to_string(),
        gafete_numero: gafete_int,
        placa_vehiculo: None,
        observaciones: Some(format!(
            "INGRESO AUTOMÁTICO: Procesado desde cita programada #{cita_id_str}"
        )),
    };

    crate::services::ingreso_visita_service::registrar_ingreso(ingreso_input, usuario_id_str)
        .await
        .map_err(|e| CitaError::Database(e.to_string()))?;

    let completed = db::completar(&cita_id)
        .await
        .map_err(|e| CitaError::Database(e.to_string()))?
        .ok_or(CitaError::Database(
            "Fallo al actualizar el estado de la cita a 'completada'".to_string(),
        ))?;

    Ok(CitaResponse::from_fetched(completed))
}

// --------------------------------------------------------------------------
// HELPERS INTERNOS
// --------------------------------------------------------------------------

/// Convierte un ID de texto en un `RecordId` de `SurrealDB`.
fn parse_record_id(id_str: &str, table: &str) -> Result<RecordId, CitaError> {
    if id_str.contains(':') {
        id_str
            .parse()
            .map_err(|_| CitaError::Validation(format!("Formato de ID inválido: {id_str}")))
    } else {
        Ok(RecordId::from_table_key(table, id_str))
    }
}

/// Parsea una cadena de fecha a formato Datetime de `SurrealDB`.
///
/// Soporta RFC 3339 (fecha con hora) y YYYY-MM-DD (fecha simple).
/// Para la fecha simple, se asume el mediodía como hora por defecto.
fn parse_datetime(date_str: &str) -> Result<surrealdb::Datetime, CitaError> {
    // Intenta parsear como timestamp completo (Estándar para almacenamiento detallado)
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_str) {
        return Ok(surrealdb::Datetime::from(dt.with_timezone(&Utc)));
    }

    // Fallback: Intenta parsear como fecha simple YYYY-MM-DD
    if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        let dt = date
            .and_hms_opt(12, 0, 0) // Por defecto al mediodía si no hay hora especificada.
            .ok_or(CitaError::Validation("Fecha con formato incompleto".to_string()))?;
        return Ok(surrealdb::Datetime::from(chrono::DateTime::<Utc>::from_naive_utc_and_offset(
            dt, Utc,
        )));
    }

    Err(CitaError::Validation(format!(
        "El formato de fecha '{date_str}' no es reconocido (ISO 8601 esperado o YYYY-MM-DD)"
    )))
}

// --------------------------------------------------------------------------
// PRUEBAS UNITARIAS (Lógica Pura)
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_datetime_iso() {
        let result = parse_datetime("2026-01-15T08:30:00Z");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_datetime_simple() {
        let result = parse_datetime("2026-01-15");
        assert!(result.is_ok());
        // Verificamos que no de error
    }

    #[test]
    fn test_parse_datetime_error() {
        let result = parse_datetime("fecha-invalida");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_record_id() {
        let id = parse_record_id("cita:123", "cita").unwrap();
        assert_eq!(id.to_string(), "cita:123");

        let id_raw = parse_record_id("xyz", "visitante").unwrap();
        assert_eq!(id_raw.to_string(), "visitante:xyz");
    }
}
