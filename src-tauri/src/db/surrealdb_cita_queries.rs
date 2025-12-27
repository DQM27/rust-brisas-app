// ==========================================
// src/db/surrealdb_cita_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::services::surrealdb_service::{get_db, SurrealDbError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Cita {
    pub id: String,
    pub visitante_id: Option<String>,
    pub usuario_id: String,
    pub motivo: String,
    pub fecha_inicio: String,
    pub fecha_fin: String,
    pub estado: String,
    pub activa: bool,
    pub created_at: String,
    pub updated_at: String,
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

pub async fn find_by_id(id: &str) -> Result<Option<Cita>, SurrealDbError> {
    let db = get_db().await?;
    let id_only = id.strip_prefix("cita:").unwrap_or(id).to_string();

    let mut result =
        db.query("SELECT * FROM type::thing('cita', $id)").bind(("id", id_only)).await?;

    Ok(result.take(0)?)
}

pub async fn find_by_visitante(visitante_id: &str) -> Result<Vec<Cita>, SurrealDbError> {
    let db = get_db().await?;
    let id_only = visitante_id.strip_prefix("visitante:").unwrap_or(visitante_id);
    let visitante_link = format!("visitante:{}", id_only);

    let mut result = db
        .query("SELECT * FROM cita WHERE visitante_id = $visitante_id ORDER BY fecha_inicio DESC")
        .bind(("visitante_id", visitante_link))
        .await?;

    Ok(result.take(0)?)
}

pub async fn cancel(id: &str) -> Result<Option<Cita>, SurrealDbError> {
    let db = get_db().await?;
    let id_only = id.strip_prefix("cita:").unwrap_or(id).to_string();

    let mut result = db
        .query(
            r#"
            UPDATE type::thing('cita', $id) MERGE {
                activa: false,
                estado: 'cancelada',
                updated_at: time::now()
            }
        "#,
        )
        .bind(("id", id_only))
        .await?;

    Ok(result.take(0)?)
}

pub async fn completar(id: &str) -> Result<Option<Cita>, SurrealDbError> {
    let db = get_db().await?;
    let id_only = id.strip_prefix("cita:").unwrap_or(id).to_string();

    let mut result = db
        .query(
            r#"
            UPDATE type::thing('cita', $id) MERGE {
                activa: false,
                estado: 'completada',
                updated_at: time::now()
            }
        "#,
        )
        .bind(("id", id_only))
        .await?;

    Ok(result.take(0)?)
}
