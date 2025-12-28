// src/services/cita_service.rs
// ==========================================
// Refactored to use FETCH joins and new models
// ==========================================

use crate::db::surrealdb_cita_queries as db;
use crate::domain::errors::CitaError;
use crate::models::cita::{CitaCreateDTO, CitaResponse};
use crate::models::visitante::CreateVisitanteInput;
use crate::services::visitante_service;
use chrono::{Local, Utc};
use surrealdb::RecordId;

// ==========================================
// PUBLIC API
// ==========================================

/// Agenda una nueva cita, opcionalmente creando un visitante si no existe
pub async fn agendar_cita(
    visitante_id_str: Option<String>,
    visitante_input: Option<CreateVisitanteInput>,
    fecha_cita: String,
    anfitrion: String,
    area_visitada: String,
    motivo: String,
    usuario_id_str: String,
) -> Result<CitaResponse, CitaError> {
    // 1. Resolver el usuario que registra
    let usuario_id = parse_record_id(&usuario_id_str, "user")?;

    // 2. Resolver Visitante (Existente o Crear)
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
        // Buscar por cédula o crear
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

    // 3. Parse fecha
    let fecha_inicio = parse_datetime(&fecha_cita)?;

    // 4. Construct DTO
    let dto = CitaCreateDTO {
        visitante_id: final_visitante_id,
        usuario_id,
        motivo,
        fecha_inicio: fecha_inicio.clone(),
        fecha_fin: fecha_inicio, // Same as inicio for now
        anfitrion: Some(anfitrion),
        area_visitada: Some(area_visitada),
        visitante_nombre: None,
        visitante_cedula: None,
    };

    // 5. Insertar
    let nueva_cita = db::insert(dto).await.map_err(|e| CitaError::Database(e.to_string()))?;

    // 6. Fetch the created cita with relations
    let fetched = db::find_by_id_fetched(&nueva_cita.id)
        .await
        .map_err(|e| CitaError::Database(e.to_string()))?
        .ok_or(CitaError::Database("Error obteniendo cita creada".to_string()))?;

    Ok(CitaResponse::from_fetched(fetched))
}

/// Obtiene las citas del día actual con datos de visitante y usuario pre-poblados
pub async fn get_citas_hoy() -> Result<Vec<CitaResponse>, CitaError> {
    let now = Local::now();
    let hoy_inicio = now.format("%Y-%m-%dT00:00:00").to_string();
    let hoy_fin = now.format("%Y-%m-%dT23:59:59").to_string();

    let citas = db::find_activas_by_fecha_fetched(&hoy_inicio, &hoy_fin)
        .await
        .map_err(|e| CitaError::Database(e.to_string()))?;

    // Ahora es trivial - FETCH ya nos trajo los datos del visitante y usuario
    Ok(citas.into_iter().map(CitaResponse::from_fetched).collect())
}

/// Obtiene todas las citas pendientes
pub async fn get_citas_pendientes() -> Result<Vec<CitaResponse>, CitaError> {
    let citas =
        db::find_pendientes_fetched().await.map_err(|e| CitaError::Database(e.to_string()))?;

    Ok(citas.into_iter().map(CitaResponse::from_fetched).collect())
}

/// Obtiene una cita por su ID
pub async fn get_cita_by_id(id_str: String) -> Result<CitaResponse, CitaError> {
    let id = parse_record_id(&id_str, "cita")?;

    let cita = db::find_by_id_fetched(&id)
        .await
        .map_err(|e| CitaError::Database(e.to_string()))?
        .ok_or(CitaError::NotFound)?;

    Ok(CitaResponse::from_fetched(cita))
}

/// Cancela una cita
pub async fn cancelar_cita(id_str: String) -> Result<(), CitaError> {
    let id = parse_record_id(&id_str, "cita")?;

    db::cancel(&id).await.map_err(|e| CitaError::Database(e.to_string()))?;

    Ok(())
}

/// Marca una cita como completada (cuando el visitante ingresa)
pub async fn completar_cita(id_str: String) -> Result<CitaResponse, CitaError> {
    let id = parse_record_id(&id_str, "cita")?;

    let cita = db::completar(&id)
        .await
        .map_err(|e| CitaError::Database(e.to_string()))?
        .ok_or(CitaError::NotFound)?;

    Ok(CitaResponse::from_fetched(cita))
}

/// Procesa el ingreso de una cita - convierte la cita en un registro de ingreso
pub async fn procesar_ingreso_cita(
    cita_id_str: String,
    gafete_numero: Option<String>,
    usuario_id_str: String,
) -> Result<CitaResponse, CitaError> {
    // 1. Obtener la cita con datos de visitante
    let cita_id = parse_record_id(&cita_id_str, "cita")?;
    let cita = db::find_by_id_fetched(&cita_id)
        .await
        .map_err(|e| CitaError::Database(e.to_string()))?
        .ok_or(CitaError::NotFound)?;

    // 2. Verificar que la cita esté pendiente
    if cita.estado != "pendiente" {
        return Err(CitaError::Validation(format!("La cita ya está en estado: {}", cita.estado)));
    }

    // 3. Preparar datos del ingreso desde la cita
    let visitante = cita.visitante_id.as_ref();

    let ingreso_input = crate::models::ingreso::CreateIngresoVisitaInput {
        cedula: visitante.map(|v| v.cedula.clone()).unwrap_or_default(),
        nombre: visitante.map(|v| v.nombre.clone()).unwrap_or_default(),
        apellido: visitante.map(|v| v.apellido.clone()).unwrap_or_default(),
        anfitrion: cita.anfitrion.clone().unwrap_or_default(),
        area_visitada: cita.area_visitada.clone().unwrap_or_default(),
        motivo: cita.motivo.clone(),
        modo_ingreso: "caminando".to_string(),
        gafete_numero,
        placa_vehiculo: None,
        observaciones: Some(format!("Ingreso desde cita #{}", cita_id_str)),
    };

    // 4. Registrar el ingreso
    crate::services::ingreso_visita_service::registrar_ingreso(ingreso_input, usuario_id_str)
        .await
        .map_err(|e| CitaError::Database(e.to_string()))?;

    // 5. Marcar la cita como completada
    let completed = db::completar(&cita_id)
        .await
        .map_err(|e| CitaError::Database(e.to_string()))?
        .ok_or(CitaError::Database("Error completando cita".to_string()))?;

    Ok(CitaResponse::from_fetched(completed))
}

// ==========================================
// HELPERS
// ==========================================

fn parse_record_id(id_str: &str, table: &str) -> Result<RecordId, CitaError> {
    if id_str.contains(':') {
        id_str.parse().map_err(|_| CitaError::Validation(format!("ID inválido: {}", id_str)))
    } else {
        Ok(RecordId::from_table_key(table, id_str))
    }
}

fn parse_datetime(date_str: &str) -> Result<surrealdb::Datetime, CitaError> {
    // Try to parse ISO 8601 format
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_str) {
        return Ok(surrealdb::Datetime::from(dt.with_timezone(&Utc)));
    }

    // Try simple date format
    if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        let dt = date
            .and_hms_opt(12, 0, 0)
            .ok_or(CitaError::Validation("Fecha inválida".to_string()))?;
        return Ok(surrealdb::Datetime::from(chrono::DateTime::<Utc>::from_naive_utc_and_offset(
            dt, Utc,
        )));
    }

    Err(CitaError::Validation(format!("Formato de fecha inválido: {}", date_str)))
}
