// src/db/surrealdb_ingreso_visita_queries.rs
use crate::models::ingreso::{CreateIngresoVisitaInput, Ingreso};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use chrono::Utc;

pub async fn insert(input: CreateIngresoVisitaInput) -> Result<Option<Ingreso>, SurrealDbError> {
    let client = get_db().await?;
    let now = Utc::now().to_rfc3339();
    let usuario_id = format!("users:{}", input.usuario_ingreso_id);

    // Nota: ingreso de visita guarda anfitrión. Puede ser un string o un link.
    // Asumimos string por ahora segun input `anfitrión`

    let mut result = client
        .query(
            r#"
            CREATE ingresos CONTENT {
                usuario_ingreso_id: $usuario_id,
                cedula: $cedula,
                nombre: $nombre,
                apellido: $apellido,
                empresa_nombre: '', // Visitas suelen ser personales, o se podría poner anfitrión/area
                tipo_ingreso: 'visita',
                tipo_autorizacion: $tipo_autorizacion,
                modo_ingreso: $modo_ingreso,
                vehiculo_placa: $placa_vehiculo,
                gafete_numero: $gafete_numero,
                gafete_tipo: 'visita',
                fecha_hora_ingreso: $now,
                observaciones: $observaciones,
                motivo: $motivo,
                area_visitada: $area,
                anfitrion: $anfitrion,
                created_at: $now,
                updated_at: $now
            };
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
        .bind(("now", now))
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
    let client = get_db().await?;

    let mut result = client
        .query(
            r#"
            SELECT * FROM ingresos 
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
