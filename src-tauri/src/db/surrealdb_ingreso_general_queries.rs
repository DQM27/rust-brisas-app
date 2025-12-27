// src/db/ingreso_general_queries.rs
use crate::models::ingreso::Ingreso;
use crate::services::surrealdb_service::{get_db, SurrealDbError};

// Estructura auxiliar para detalles si fuera necesaria,
// pero en Surreal se puede obtener todo en el Ingreso si se usa fetch
// O devolver Ingreso directamente.
#[derive(Debug, Default)]
pub struct IngresoDetails {
    pub usuario_ingreso_nombre: Option<String>,
    pub usuario_salida_nombre: Option<String>,
    pub vehiculo_placa: Option<String>,
}

pub async fn find_all() -> Result<Vec<Ingreso>, SurrealDbError> {
    let client = get_db().await?;
    // Seleccionar los últimos 500 para no saturar
    let sql = "SELECT * FROM ingresos ORDER BY created_at DESC LIMIT 500";
    let mut result = client.query(sql).await?;
    let ingresos: Vec<Ingreso> = result.take(0)?;
    Ok(ingresos)
}

pub async fn find_ingresos_abiertos() -> Result<Vec<Ingreso>, SurrealDbError> {
    let client = get_db().await?;
    let sql = "SELECT * FROM ingresos WHERE fecha_hora_salida IS NONE ORDER BY created_at DESC";
    let mut result = client.query(sql).await?;
    let ingresos: Vec<Ingreso> = result.take(0)?;
    Ok(ingresos)
}

pub async fn find_by_id(id: &str) -> Result<Option<Ingreso>, SurrealDbError> {
    let client = get_db().await?;
    let id_only = id.strip_prefix("ingresos:").unwrap_or(id);
    let result: Option<Ingreso> = client.select(("ingresos", id_only.to_string())).await?;
    Ok(result)
}

// En SurrealDB, si el modelo Ingreso tiene los campos de nombre desnormalizados, no necesitamos "details" join.
// Si no, deberíamos hacer fetch.
// Asumimos que Ingreso struct tiene datos básicos y si necesitamos nombres de usuario,
// o bien los guardamos al crear (desnormalización) o hacemos fetch.
// En `surrealdb_ingreso_proveedor_queries.rs` create no guardaba nombre usuario.
// Así que deberíamos hacer un SELECT con FETCH usuarios.
// Por simplicidad para MVP "Eliminar SQLite", si no mostramos el nombre del usuario, no es crítico.
// Pero puedo intentar FETCH.
// El struct Ingreso NO tiene campo `usuario_nombre` embebido (tiene `usuario_ingreso_id`).
// Para mantener compatibilidad con `IngresoDetails`, simularé la carga.

pub async fn find_details_for_ingreso(ingreso: &Ingreso) -> Result<IngresoDetails, SurrealDbError> {
    let client = get_db().await?;
    let mut details = IngresoDetails::default();

    // Fetch usuario ingreso - es String no Option
    let uid = ingreso.usuario_ingreso_id.clone();
    if !uid.is_empty() {
        let sql = "SELECT nombre, apellido FROM type::thing($id)";
        let mut res = client.query(sql).bind(("id", uid)).await?;
        #[derive(serde::Deserialize)]
        struct UserInfo {
            nombre: String,
            apellido: String,
        }
        let u: Option<UserInfo> = res.take(0).ok().flatten();
        if let Some(user) = u {
            details.usuario_ingreso_nombre = Some(format!("{} {}", user.nombre, user.apellido));
        }
    }

    // Fetch usuario salida
    let salida_uid = ingreso.usuario_salida_id.clone();
    if let Some(uid) = salida_uid {
        let sql = "SELECT nombre, apellido FROM type::thing($id)";
        let mut res = client.query(sql).bind(("id", uid)).await?;
        #[derive(serde::Deserialize)]
        struct UserInfo {
            nombre: String,
            apellido: String,
        }
        let u: Option<UserInfo> = res.take(0).ok().flatten();
        if let Some(user) = u {
            details.usuario_salida_nombre = Some(format!("{} {}", user.nombre, user.apellido));
        }
    }

    // Vehiculo - usar placa_temporal ya que no hay vehiculo_placa
    details.vehiculo_placa = ingreso.placa_temporal.clone();

    Ok(details)
}

// Wrapper que devuelve tuplas para que `ingreso_general_service` no cambie tanto
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
    let client = get_db().await?;
    // Busca ingreso ABIERTO con ese gafete
    let sql = "SELECT * FROM ingresos WHERE gafete_numero = $gafete AND fecha_hora_salida IS NONE LIMIT 1";
    let mut result = client.query(sql).bind(("gafete", gafete.to_string())).await?;
    Ok(result.take(0)?)
}

pub async fn find_salidas_in_range_with_details(
    start: &str,
    end: &str,
) -> Result<Vec<(Ingreso, IngresoDetails)>, SurrealDbError> {
    let client = get_db().await?;
    // Rango de fechas en fecha_hora_salida
    let sql = "SELECT * FROM ingresos WHERE fecha_hora_salida >= $start AND fecha_hora_salida <= $end ORDER BY fecha_hora_salida DESC";
    let mut result =
        client.query(sql).bind(("start", start.to_string())).bind(("end", end.to_string())).await?;

    let ingresos: Vec<Ingreso> = result.take(0)?;
    let mut final_result = Vec::new();
    for ing in ingresos {
        let details = find_details_for_ingreso(&ing).await?;
        final_result.push((ing, details));
    }
    Ok(final_result)
}
