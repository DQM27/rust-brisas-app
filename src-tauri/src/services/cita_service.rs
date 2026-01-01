/// Gestión Estratégica de Citas y Pre-registros.
///
/// Este servicio permite la planificación anticipada de visitas. Su objetivo es
/// agilizar el trabajo en garita permitiendo que el personal administrativo registre
/// a los visitantes de antemano. Cuando el visitante llega, la cita se "completa"
/// y se transforma automáticamente en un registro de ingreso activo.
use crate::db::surrealdb_cita_queries as db;
use crate::domain::errors::CitaError;
use crate::models::cita::{CitaCreateDTO, CitaResponse};
use crate::models::visitante::CreateVisitanteInput;
use crate::services::visitante_service;
use chrono::{Local, Utc};
use surrealdb::RecordId;

// ==========================================
// API PÚBLICA DE CITAS
// ==========================================

/// Agenda una nueva cita en el calendario institucional.
///
/// Características:
/// 1. Auto-registro: Si el visitante no existe, lo crea automáticamente en el sistema.
/// 2. Integridad: Vincula al usuario que programa la cita para fines de auditoría.
/// 3. Ubicación: Define de antemano el área y el anfitrión que recibirá la visita.
pub async fn agendar_cita(
    visitante_id_str: Option<String>,
    visitante_input: Option<CreateVisitanteInput>,
    fecha_cita: String,
    anfitrion: String,
    area_visitada: String,
    motivo: String,
    usuario_id_str: String,
) -> Result<CitaResponse, CitaError> {
    let usuario_id = parse_record_id(&usuario_id_str, "user")?;

    let visitante_id: Option<RecordId> = if let Some(id_str) = visitante_id_str {
        if !id_str.is_empty() {
            Some(parse_record_id(&id_str, "visitante")?)
        } else {
            None
        }
    } else {
        None
    };

    let final_visitante_id = if let Some(vid) = visitante_id {
        Some(vid)
    } else if let Some(v_input) = visitante_input {
        let existente = visitante_service::get_visitante_by_cedula(&v_input.cedula)
            .await
            .map_err(|e| CitaError::Database(e.to_string()))?;

        match existente {
            Some(v) => Some(parse_record_id(&v.id, "visitante")?),
            None => {
                let nuevo = visitante_service::create_visitante(v_input)
                    .await
                    .map_err(|e| CitaError::Database(e.to_string()))?;
                Some(parse_record_id(&nuevo.id, "visitante")?)
            }
        }
    } else {
        None
    };

    let fecha_inicio = parse_datetime(&fecha_cita)?;

    let dto = CitaCreateDTO {
        visitante_id: final_visitante_id,
        usuario_id,
        motivo,
        fecha_inicio: fecha_inicio.clone(),
        fecha_fin: fecha_inicio,
        anfitrion: Some(anfitrion),
        area_visitada: Some(area_visitada),
        visitante_nombre: None,
        visitante_cedula: None,
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
pub async fn get_citas_hoy() -> Result<Vec<CitaResponse>, CitaError> {
    let now = Local::now();
    let hoy_inicio = now.format("%Y-%m-%dT00:00:00").to_string();
    let hoy_fin = now.format("%Y-%m-%dT23:59:59").to_string();

    let citas = db::find_activas_by_fecha_fetched(&hoy_inicio, &hoy_fin)
        .await
        .map_err(|e| CitaError::Database(e.to_string()))?;

    Ok(citas.into_iter().map(CitaResponse::from_fetched).collect())
}

pub async fn get_citas_pendientes() -> Result<Vec<CitaResponse>, CitaError> {
    let citas =
        db::find_pendientes_fetched().await.map_err(|e| CitaError::Database(e.to_string()))?;

    Ok(citas.into_iter().map(CitaResponse::from_fetched).collect())
}

pub async fn get_cita_by_id(id_str: String) -> Result<CitaResponse, CitaError> {
    let id = parse_record_id(&id_str, "cita")?;

    let cita = db::find_by_id_fetched(&id)
        .await
        .map_err(|e| CitaError::Database(e.to_string()))?
        .ok_or(CitaError::NotFound)?;

    Ok(CitaResponse::from_fetched(cita))
}

pub async fn cancelar_cita(id_str: String) -> Result<(), CitaError> {
    let id = parse_record_id(&id_str, "cita")?;
    db::cancel(&id).await.map_err(|e| CitaError::Database(e.to_string()))?;
    Ok(())
}

/// Acción de Entrada: Convierte una cita planificada en un ingreso real en planta.
///
/// Este es el paso final del flujo de pre-registro. Valida que la cita esté vigente
/// y utiliza el servicio de ingresos para abrir el registro de permanencia física.
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
                    .map_err(|e| CitaError::Validation(e))?,
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
            "INGRESO AUTOMÁTICO: Procesado desde cita programada #{}",
            cita_id_str
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

// ==========================================
// HELPERS INTERNOS
// ==========================================

fn parse_record_id(id_str: &str, table: &str) -> Result<RecordId, CitaError> {
    if id_str.contains(':') {
        id_str
            .parse()
            .map_err(|_| CitaError::Validation(format!("Formato de ID inválido: {}", id_str)))
    } else {
        Ok(RecordId::from_table_key(table, id_str))
    }
}

fn parse_datetime(date_str: &str) -> Result<surrealdb::Datetime, CitaError> {
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_str) {
        return Ok(surrealdb::Datetime::from(dt.with_timezone(&Utc)));
    }

    if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        let dt = date
            .and_hms_opt(12, 0, 0) // Por defecto al mediodía si no hay hora.
            .ok_or(CitaError::Validation("Fecha con formato incompleto".to_string()))?;
        return Ok(surrealdb::Datetime::from(chrono::DateTime::<Utc>::from_naive_utc_and_offset(
            dt, Utc,
        )));
    }

    Err(CitaError::Validation(format!(
        "El formato de fecha '{}' no es reconocido (ISO 8601 esperado)",
        date_str
    )))
}
