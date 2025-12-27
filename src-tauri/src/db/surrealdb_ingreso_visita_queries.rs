// ==========================================
// src/db/surrealdb_ingreso_visita_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::models::ingreso::{CreateIngresoVisitaInput, Ingreso};
use crate::services::surrealdb_service::{get_db, SurrealDbError};

pub async fn insert(input: CreateIngresoVisitaInput) -> Result<Option<Ingreso>, SurrealDbError> {
    let db = get_db().await?;

    let usuario_id = {
        let id = &input.usuario_ingreso_id;
        let id_only = id.strip_prefix("user:").unwrap_or(id);
        format!("user:{}", id_only)
    };

    let mut result = db
        .query(
            r#"
            CREATE ingreso CONTENT {
                usuario_ingreso_id: $usuario_id,
                cedula: $cedula,
                nombre: $nombre,
                apellido: $apellido,
                empresa_nombre: '',
                tipo_ingreso: 'visita',
                tipo_autorizacion: $tipo_autorizacion,
                modo_ingreso: $modo_ingreso,
                vehiculo_placa: $placa_vehiculo,
                gafete_numero: $gafete_numero,
                gafete_tipo: 'visita',
                fecha_hora_ingreso: time::now(),
                observaciones: $observaciones,
                motivo: $motivo,
                area_visitada: $area,
                anfitrion: $anfitrion,
                created_at: time::now(),
                updated_at: time::now()
            }
        "#,
        )
        .bind(("usuario_id", usuario_id))
        .bind(("cedula", input.cedula))
        .bind(("nombre", input.nombre))
        .bind(("apellido", input.apellido))
        .bind(("tipo_autorizacion", input.tipo_autorizacion))
        .bind(("modo_ingreso", input.modo_ingreso))
        .bind(("placa_vehiculo", input.vehiculo_placa))
        .bind(("gafete_numero", input.gafete_numero))
        .bind(("observaciones", input.observaciones))
        .bind(("motivo", input.motivo_visita))
        .bind(("area", input.area_visitada))
        .bind(("anfitrion", input.anfitrion))
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
            AND tipo_ingreso = 'visita'
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
