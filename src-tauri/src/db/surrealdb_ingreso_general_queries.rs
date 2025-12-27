// ==========================================
// src/db/surrealdb_ingreso_general_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::models::ingreso::Ingreso;
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use surrealdb::sql::Thing;

#[derive(Debug, Default)]
pub struct IngresoDetails {
    pub usuario_ingreso_nombre: Option<String>,
    pub usuario_salida_nombre: Option<String>,
    pub vehiculo_placa: Option<String>,
}

pub async fn find_all() -> Result<Vec<Ingreso>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db.query("SELECT * FROM ingreso ORDER BY created_at DESC LIMIT 500").await?;
    Ok(result.take(0)?)
}

pub async fn find_ingresos_abiertos() -> Result<Vec<Ingreso>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM ingreso WHERE fecha_hora_salida IS NONE ORDER BY created_at DESC")
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_by_id(id: &Thing) -> Result<Option<Ingreso>, SurrealDbError> {
    let db = get_db().await?;
    let result: Option<Ingreso> = db.select((id.tb.clone(), id.id.to_string())).await?;
    Ok(result)
}

pub async fn find_details_for_ingreso(ingreso: &Ingreso) -> Result<IngresoDetails, SurrealDbError> {
    let db = get_db().await?;
    let mut details = IngresoDetails::default();

    // Fetch usuario ingreso
    let uid = &ingreso.usuario_ingreso;
    let mut res = db
        .query("SELECT nombre, apellido FROM type::thing($tb, $id)")
        .bind(("tb", uid.tb.clone()))
        .bind(("id", uid.id.to_string()))
        .await?;

    #[derive(serde::Deserialize)]
    struct UserInfo {
        nombre: String,
        apellido: String,
    }

    let u: Option<UserInfo> = res.take(0).ok().flatten();
    if let Some(user) = u {
        details.usuario_ingreso_nombre = Some(format!("{} {}", user.nombre, user.apellido));
    }

    // Fetch usuario salida
    if let Some(uid) = &ingreso.usuario_salida {
        let mut res = db
            .query("SELECT nombre, apellido FROM type::thing($tb, $id)")
            .bind(("tb", uid.tb.clone()))
            .bind(("id", uid.id.to_string()))
            .await?;

        let u: Option<UserInfo> = res.take(0).ok().flatten();
        if let Some(user) = u {
            details.usuario_salida_nombre = Some(format!("{} {}", user.nombre, user.apellido));
        }
    }

    details.vehiculo_placa = ingreso.placa_temporal.clone();
    Ok(details)
}

pub async fn find_all_with_details() -> Result<Vec<(Ingreso, IngresoDetails)>, SurrealDbError> {
    let ingresos = find_all().await?;
    let mut result = Vec::new();
    for ing in ingresos {
        let details = find_details_for_ingreso(&ing).await?;
        result.push((ing, details));
    }
    Ok(result)
}

pub async fn find_ingresos_abiertos_with_details(
) -> Result<Vec<(Ingreso, IngresoDetails)>, SurrealDbError> {
    let ingresos = find_ingresos_abiertos().await?;
    let mut result = Vec::new();
    for ing in ingresos {
        let details = find_details_for_ingreso(&ing).await?;
        result.push((ing, details));
    }
    Ok(result)
}

pub async fn find_ingreso_by_gafete(gafete: &str) -> Result<Option<Ingreso>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM ingreso WHERE gafete_numero = $gafete AND fecha_hora_salida IS NONE LIMIT 1")
        .bind(("gafete", gafete.to_string()))
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_salidas_in_range_with_details(
    start: &str,
    end: &str,
) -> Result<Vec<(Ingreso, IngresoDetails)>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM ingreso WHERE fecha_hora_salida >= $start AND fecha_hora_salida <= $end ORDER BY fecha_hora_salida DESC")
        .bind(("start", start.to_string()))
        .bind(("end", end.to_string()))
        .await?;

    let ingresos: Vec<Ingreso> = result.take(0)?;
    let mut final_result = Vec::new();
    for ing in ingresos {
        let details = find_details_for_ingreso(&ing).await?;
        final_result.push((ing, details));
    }
    Ok(final_result)
}
