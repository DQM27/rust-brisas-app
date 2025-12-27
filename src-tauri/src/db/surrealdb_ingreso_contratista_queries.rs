// src/db/surrealdb_ingreso_contratista_queries.rs
use crate::models::ingreso::{CreateIngresoContratistaInput, Ingreso};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use chrono::Utc;

pub async fn insert(
    input: CreateIngresoContratistaInput,
    contratista_data: &serde_json::Value,
) -> Result<Option<Ingreso>, SurrealDbError> {
    let client = get_db().await?;
    let now = Utc::now().to_rfc3339();

    // Extraer datos del contratista para desnormalizar (backup)
    let nombre = contratista_data["nombre"].as_str().unwrap_or("").to_string();
    let apellido = contratista_data["apellido"].as_str().unwrap_or("").to_string();
    let cedula = contratista_data["cedula"].as_str().unwrap_or("").to_string();
    let empresa_nombre = contratista_data["empresa_nombre"].as_str().unwrap_or("").to_string();
    let praind_vigente = contratista_data["praind_vigente"].as_bool();
    let estado = contratista_data["estado"].as_str().map(|s| s.to_string());

    let contratista_id = format!("contratistas:{}", input.contratista_id);
    let vehiculo_id = input.vehiculo_id.map(|v| format!("vehiculos:{}", v));
    let usuario_id = format!("users:{}", input.usuario_ingreso_id);

    let mut result = client
        .query(
            r#"
            CREATE ingresos CONTENT {
                contratista_id: $contratista_id,
                usuario_ingreso_id: $usuario_id,
                cedula: $cedula,
                nombre: $nombre,
                apellido: $apellido,
                empresa_nombre: $empresa_nombre,
                tipo_ingreso: 'contratista',
                tipo_autorizacion: $tipo_autorizacion,
                modo_ingreso: $modo_ingreso,
                vehiculo_id: $vehiculo_id,
                gafete_numero: $gafete_numero,
                gafete_tipo: $gafete_tipo,
                fecha_hora_ingreso: $now,
                observaciones: $observaciones,
                praind_vigente_al_ingreso: $praind_vigente,
                estado_contratista_al_ingreso: $estado,
                created_at: $now,
                updated_at: $now
            };
        "#,
        )
        .bind(("contratista_id", contratista_id))
        .bind(("usuario_id", usuario_id))
        .bind(("cedula", cedula))
        .bind(("nombre", nombre))
        .bind(("apellido", apellido))
        .bind(("empresa_nombre", empresa_nombre))
        .bind(("tipo_autorizacion", input.tipo_autorizacion))
        .bind(("modo_ingreso", input.modo_ingreso))
        .bind(("vehiculo_id", vehiculo_id))
        .bind(("gafete_numero", input.gafete_numero))
        .bind(("gafete_tipo", input.gafete_tipo))
        .bind(("now", now))
        .bind(("observaciones", input.observaciones))
        .bind(("praind_vigente", praind_vigente))
        .bind(("estado", estado))
        .await?;

    Ok(result.take(0)?)
}

pub async fn find_ingreso_abierto_by_contratista(
    contratista_id: &str,
) -> Result<Option<Ingreso>, SurrealDbError> {
    let client = get_db().await?;
    let contratista_link = format!("contratistas:{}", contratista_id);

    let mut result = client
        .query(
            r#"
            SELECT * FROM ingresos 
            WHERE contratista_id = $contratista_link 
            AND fecha_hora_salida IS NONE
            LIMIT 1
        "#,
        )
        .bind(("contratista_link", contratista_link))
        .await?;

    Ok(result.take(0)?)
}

pub async fn update_salida(
    ingreso_id: &str,
    usuario_salida_id: &str,
    observaciones: Option<String>,
) -> Result<Option<Ingreso>, SurrealDbError> {
    let client = get_db().await?;
    let now = Utc::now().to_rfc3339();
    let usuario_link = format!("users:{}", usuario_salida_id);

    let mut result = client
        .query(
            r#"
            UPDATE type::thing('ingresos', $id) MERGE {
                fecha_hora_salida: $now,
                usuario_salida_id: $usuario_link,
                observaciones_salida: $observaciones,
                updated_at: $now
            };
        "#,
        )
        .bind(("id", ingreso_id.to_string()))
        .bind(("usuario_link", usuario_link))
        .bind(("observaciones", observaciones))
        .bind(("now", now))
        .await?;

    Ok(result.take(0)?)
}

pub async fn find_by_id(id: &str) -> Result<Option<Ingreso>, SurrealDbError> {
    let client = get_db().await?;
    let mut result = client
        .query("SELECT * FROM type::thing('ingresos', $id)")
        .bind(("id", id.to_string()))
        .await?;
    Ok(result.take(0)?)
}
