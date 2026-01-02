// ==========================================
// src/db/surrealdb_cita_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::models::cita::{Cita, CitaCreateDTO, CitaFetched, EstadoCita};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use surrealdb::RecordId;

// ==========================================
// QUERIES CON FETCH (pre-pobladas)
// ==========================================

pub async fn find_all_fetched() -> Result<Vec<CitaFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query(
            r"
            SELECT * FROM cita 
            ORDER BY fecha_inicio DESC
            FETCH visitante_id, usuario_id
        ",
        )
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_by_id_fetched(id: &RecordId) -> Result<Option<CitaFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query(
            r"
            SELECT * FROM $id
            FETCH visitante_id, usuario_id
        ",
        )
        .bind(("id", id.clone()))
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_activas_by_fecha_fetched(
    fecha_inicio: &str,
    fecha_fin: &str,
) -> Result<Vec<CitaFetched>, SurrealDbError> {
    let db = get_db().await?;

    let mut result = db
        .query(
            r"
            SELECT * FROM cita 
            WHERE fecha_inicio >= $f_inicio 
            AND fecha_inicio <= $f_fin 
            AND activa = true
            ORDER BY fecha_inicio ASC
            FETCH visitante_id, usuario_id
        ",
        )
        .bind(("f_inicio", fecha_inicio.to_string()))
        .bind(("f_fin", fecha_fin.to_string()))
        .await?;

    Ok(result.take(0)?)
}

pub async fn find_pendientes_fetched() -> Result<Vec<CitaFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query(
            r"
            SELECT * FROM cita 
            WHERE activa = true AND estado = $estado
            ORDER BY fecha_inicio ASC
            FETCH visitante_id, usuario_id
        ",
        )
        .bind(("estado", EstadoCita::Programada))
        .await?;
    Ok(result.take(0)?)
}

// ==========================================
// QUERIES BÁSICAS (sin FETCH)
// ==========================================

pub async fn find_by_id(id: &RecordId) -> Result<Option<Cita>, SurrealDbError> {
    let db = get_db().await?;
    let result = db.select(id.clone()).await?;
    Ok(result)
}

pub async fn find_by_visitante(visitante_id: &RecordId) -> Result<Vec<Cita>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM cita WHERE visitante_id = $visitante_id ORDER BY fecha_inicio DESC")
        .bind(("visitante_id", visitante_id.clone()))
        .await?;

    Ok(result.take(0)?)
}

// ==========================================
// MUTACIONES
// ==========================================

pub async fn insert(dto: CitaCreateDTO) -> Result<Cita, SurrealDbError> {
    let db = get_db().await?;

    let mut result = db
        .query(
            r"
            CREATE cita CONTENT {
                visitante_id: $visitante_id,
                usuario_id: $usuario_id,
                motivo: $motivo,
                fecha_inicio: $fecha_inicio,
                fecha_fin: $fecha_fin,
                anfitrion: $anfitrion,
                area_visitada: $area_visitada,
                visitante_nombre: $visitante_nombre,
                visitante_cedula: $visitante_cedula,
                estado: $estado,
                activa: true,
                created_at: time::now(),
                updated_at: time::now()
            }
        ",
        )
        .bind(("visitante_id", dto.visitante_id))
        .bind(("usuario_id", dto.usuario_id))
        .bind(("motivo", dto.motivo))
        .bind(("fecha_inicio", dto.fecha_inicio))
        .bind(("fecha_fin", dto.fecha_fin))
        .bind(("anfitrion", dto.anfitrion))
        .bind(("area_visitada", dto.area_visitada))
        .bind(("visitante_nombre", dto.visitante_nombre))
        .bind(("visitante_cedula", dto.visitante_cedula))
        .bind(("estado", EstadoCita::Programada))
        .await?;

    result.take::<Option<Cita>>(0)?.ok_or(SurrealDbError::Query("Error creando cita".to_string()))
}

pub async fn cancel(id: &RecordId) -> Result<Option<Cita>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query(
            r"
            UPDATE $id MERGE {
                activa: false,
                estado: $estado,
                updated_at: time::now()
            }
        ",
        )
        .bind(("id", id.clone()))
        .bind(("estado", EstadoCita::Cancelada))
        .await?;

    Ok(result.take(0)?)
}

pub async fn completar(id: &RecordId) -> Result<Option<CitaFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query(
            r"
            UPDATE $id MERGE {
                activa: false,
                estado: $estado,
                updated_at: time::now()
            };
            SELECT * FROM $id FETCH visitante_id, usuario_id
        ",
        )
        .bind(("id", id.clone()))
        .bind(("estado", EstadoCita::Finalizada))
        .await?;

    // Take the second statement result (the SELECT)
    Ok(result.take(1)?)
}
