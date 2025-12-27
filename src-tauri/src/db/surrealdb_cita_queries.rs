// ==========================================
// src/db/surrealdb_cita_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::services::surrealdb_service::{get_db, SurrealDbError};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cita {
    pub id: RecordId,
    pub visitante_id: Option<RecordId>,
    pub usuario_id: RecordId,
    pub motivo: String,
    pub fecha_inicio: surrealdb::Datetime,
    pub fecha_fin: surrealdb::Datetime,
    pub estado: String,
    pub activa: bool,
    pub created_at: surrealdb::Datetime,
    pub updated_at: surrealdb::Datetime,
    pub visitante_nombre: Option<String>,
    pub visitante_cedula: Option<String>,
}

pub async fn insert(cita: serde_json::Value) -> Result<Option<Cita>, SurrealDbError> {
    let db = get_db().await?;

    let mut result = db
        .query(
            r#"
            CREATE cita CONTENT $data MERGE {
                created_at: time::now(),
                updated_at: time::now()
            }
        "#,
        )
        .bind(("data", cita))
        .await?;

    Ok(result.take(0)?)
}

pub async fn find_activas_by_fecha(
    fecha_inicio: &str,
    fecha_fin: &str,
) -> Result<Vec<Cita>, SurrealDbError> {
    let db = get_db().await?;

    let mut result = db
        .query(
            r#"
            SELECT * FROM cita 
            WHERE fecha_inicio >= $f_inicio 
            AND fecha_inicio <= $f_fin 
            AND activa = true
            ORDER BY fecha_inicio ASC
        "#,
        )
        .bind(("f_inicio", fecha_inicio.to_string()))
        .bind(("f_fin", fecha_fin.to_string()))
        .await?;

    Ok(result.take(0)?)
}

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

pub async fn cancel(id: &RecordId) -> Result<Option<Cita>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query(
            r#"
            UPDATE $id MERGE {
                activa: false,
                estado: 'cancelada',
                updated_at: time::now()
            }
        "#,
        )
        .bind(("id", id.clone()))
        .await?;

    Ok(result.take(0)?)
}

pub async fn completar(id: &RecordId) -> Result<Option<Cita>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query(
            r#"
            UPDATE $id MERGE {
                activa: false,
                estado: 'completada',
                updated_at: time::now()
            }
        "#,
        )
        .bind(("id", id.clone()))
        .await?;

    Ok(result.take(0)?)
}
