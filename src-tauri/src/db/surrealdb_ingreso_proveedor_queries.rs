// ==========================================
// src/db/surrealdb_ingreso_proveedor_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::models::ingreso::{CreateIngresoProveedorInput, Ingreso};
use crate::services::surrealdb_service::{get_db, SurrealDbError};

pub async fn insert(
    input: CreateIngresoProveedorInput,
    proveedor_data: &serde_json::Value,
) -> Result<Option<Ingreso>, SurrealDbError> {
    let db = get_db().await?;

    // Extract and normalize data
    let nombre = input.nombre;
    let apellido = input.apellido;
    let cedula = input.cedula;
    let empresa_nombre = proveedor_data["nombre"].as_str().unwrap_or("").to_string();

    let empresa_id = {
        let id = &input.empresa_id;
        let id_only = id.strip_prefix("empresa:").unwrap_or(id);
        format!("empresa:{}", id_only)
    };

    let usuario_id = {
        let id = &input.usuario_ingreso_id;
        let id_only = id.strip_prefix("user:").unwrap_or(id);
        format!("user:{}", id_only)
    };

    let mut result = db
        .query(
            r#"
            CREATE ingreso CONTENT {
                empresa_id: $empresa_link,
                usuario_ingreso_id: $usuario_id,
                cedula: $cedula,
                nombre: $nombre,
                apellido: $apellido,
                empresa_nombre: $empresa_nombre,
                tipo_ingreso: 'proveedor',
                tipo_autorizacion: $tipo_autorizacion,
                modo_ingreso: $modo_ingreso,
                vehiculo_placa: $vehiculo_placa,
                gafete_numero: $gafete_numero,
                gafete_tipo: 'proveedor',
                fecha_hora_ingreso: time::now(),
                observaciones: $observaciones,
                motivo: $motivo,
                area_visitada: $area_visitada,
                created_at: time::now(),
                updated_at: time::now()
            }
        "#,
        )
        .bind(("empresa_link", empresa_id))
        .bind(("usuario_id", usuario_id))
        .bind(("cedula", cedula))
        .bind(("nombre", nombre))
        .bind(("apellido", apellido))
        .bind(("empresa_nombre", empresa_nombre))
        .bind(("tipo_autorizacion", input.tipo_autorizacion))
        .bind(("modo_ingreso", input.modo_ingreso))
        .bind(("vehiculo_placa", input.vehiculo_placa))
        .bind(("gafete_numero", input.gafete_numero))
        .bind(("observaciones", input.observaciones))
        .bind(("motivo", input.motivo))
        .bind(("area_visitada", input.area_visitada))
        .await?;

    Ok(result.take(0)?)
}

pub async fn find_ingreso_abierto_by_cedula(
    cedula: &str,
) -> Result<Option<Ingreso>, SurrealDbError> {
    let db = get_db().await?;

    let mut result = db
        .query(
            r#"
            SELECT * FROM ingreso 
            WHERE cedula = $cedula 
            AND tipo_ingreso = 'proveedor'
            AND fecha_hora_salida IS NONE
            LIMIT 1
        "#,
        )
        .bind(("cedula", cedula.to_string()))
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
