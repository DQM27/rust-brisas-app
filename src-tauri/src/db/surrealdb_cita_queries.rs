// src/db/surrealdb_cita_queries.rs
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use chrono::Utc;
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
    // Desnormalizados opcionales
    pub visitante_nombre: Option<String>,
    pub visitante_cedula: Option<String>,
}

pub async fn insert(cita: serde_json::Value) -> Result<Option<Cita>, SurrealDbError> {
    let client = get_db().await?;
    let now = Utc::now().to_rfc3339();

    // Asumimos que el Value trae los campos necesarios.
    // Lo ideal seria usar un struct CreateCitaInput pero por simplicidad y tiempo usaremos Value + merge de fechas

    let mut result = client
        .query(
            r#"
            CREATE citas CONTENT $data MERGE {
                created_at: $now,
                updated_at: $now
            };
        "#,
        )
        .bind(("data", cita))
        .bind(("now", now))
        .await?;

    Ok(result.take(0)?)
}

pub async fn find_activas_by_fecha(
    fecha_inicio: &str,
    fecha_fin: &str,
) -> Result<Vec<Cita>, SurrealDbError> {
    let client = get_db().await?;

    let mut result = client
        .query(
            r#"
            SELECT * FROM citas 
            WHERE fecha_inicio >= $f_inicio 
            AND fecha_inicio <= $f_fin 
            AND activa = true
            ORDER BY fecha_inicio ASC;
        "#,
        )
        .bind(("f_inicio", fecha_inicio.to_string()))
        .bind(("f_fin", fecha_fin.to_string()))
        .await?;

    Ok(result.take(0)?)
}

pub async fn cancel(id: &str) -> Result<Option<Cita>, SurrealDbError> {
    let client = get_db().await?;
    let now = Utc::now().to_rfc3339();

    let mut result = client
        .query(
            r#"
            UPDATE type::thing('citas', $id) MERGE {
                activa: false,
                estado: 'cancelada',
                updated_at: $now
            };
        "#,
        )
        .bind(("id", id.to_string()))
        .bind(("now", now))
        .await?;

    Ok(result.take(0)?)
}
