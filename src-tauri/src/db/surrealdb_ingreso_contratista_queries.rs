// ==========================================
// src/db/surrealdb_ingreso_contratista_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::models::ingreso::{CreateIngresoContratistaInput, Ingreso};
use crate::services::surrealdb_service::{get_db, SurrealDbError};

pub async fn insert(
    input: CreateIngresoContratistaInput,
    contratista_data: &serde_json::Value,
) -> Result<Option<Ingreso>, SurrealDbError> {
    let db = get_db().await?;

    // Extract data from contratista for denormalization
    let nombre = contratista_data["nombre"].as_str().unwrap_or("").to_string();
    let apellido = contratista_data["apellido"].as_str().unwrap_or("").to_string();
    let cedula = contratista_data["cedula"].as_str().unwrap_or("").to_string();
    let empresa_nombre = contratista_data["empresa_nombre"].as_str().unwrap_or("").to_string();
    let praind_vigente = contratista_data["praind_vigente"].as_bool();
    let estado = contratista_data["estado"].as_str().map(|s| s.to_string());

    let contratista_id = {
        let id = &input.contratista_id;
        let id_only = id.strip_prefix("contratista:").unwrap_or(id);
        format!("contratista:{}", id_only)
    };

    let vehiculo_id = input.vehiculo_id.map(|v| {
        let v_only = v.strip_prefix("vehiculo:").unwrap_or(&v);
        format!("vehiculo:{}", v_only)
    });

    let usuario_id = {
        let id = &input.usuario_ingreso_id;
        let id_only = id.strip_prefix("user:").unwrap_or(id);
        format!("user:{}", id_only)
    };

    let mut result = db
        .query(
            r#"
            CREATE ingreso CONTENT {
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
                fecha_hora_ingreso: time::now(),
                observaciones: $observaciones,
                praind_vigente_al_ingreso: $praind_vigente,
                estado_contratista_al_ingreso: $estado,
                created_at: time::now(),
                updated_at: time::now()
            }
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
        .bind(("observaciones", input.observaciones))
        .bind(("praind_vigente", praind_vigente))
        .bind(("estado", estado))
        .await?;

    Ok(result.take(0)?)
}

pub async fn find_ingreso_abierto_by_contratista(
    contratista_id: &str,
) -> Result<Option<Ingreso>, SurrealDbError> {
    let db = get_db().await?;
    let id_only = contratista_id.strip_prefix("contratista:").unwrap_or(contratista_id);
    let contratista_link = format!("contratista:{}", id_only);

    let mut result = db
        .query(
            r#"
            SELECT * FROM ingreso 
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
    let db = get_db().await?;

    let id_only = ingreso_id.strip_prefix("ingreso:").unwrap_or(ingreso_id).to_string();
    let usuario_id_only = usuario_salida_id.strip_prefix("user:").unwrap_or(usuario_salida_id);
    let usuario_link = format!("user:{}", usuario_id_only);

    let mut result = db
        .query(
            r#"
            UPDATE type::thing('ingreso', $id) MERGE {
                fecha_hora_salida: time::now(),
                usuario_salida_id: $usuario_link,
                observaciones_salida: $observaciones,
                updated_at: time::now()
            }
        "#,
        )
        .bind(("id", id_only))
        .bind(("usuario_link", usuario_link))
        .bind(("observaciones", observaciones))
        .await?;

    Ok(result.take(0)?)
}

pub async fn find_by_id(id: &str) -> Result<Option<Ingreso>, SurrealDbError> {
    let db = get_db().await?;
    let id_only = id.strip_prefix("ingreso:").unwrap_or(id).to_string();

    let mut result =
        db.query("SELECT * FROM type::thing('ingreso', $id)").bind(("id", id_only)).await?;

    Ok(result.take(0)?)
}
