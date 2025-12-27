// src/db/surrealdb_ingreso_proveedor_queries.rs
use crate::models::ingreso::{CreateIngresoProveedorInput, Ingreso};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use chrono::Utc;

pub async fn insert(
    input: CreateIngresoProveedorInput,
    proveedor_data: &serde_json::Value,
) -> Result<Option<Ingreso>, SurrealDbError> {
    let client = get_db().await?;
    let now = Utc::now().to_rfc3339();

    // Extraer datos del proveedor para desnormalizar
    let nombre = input.nombre;
    let apellido = input.apellido;
    let cedula = input.cedula;
    let empresa_nombre = proveedor_data["nombre"].as_str().unwrap_or("").to_string(); // proveedor_data podría ser la empresa en este contexto?
                                                                                      // Nota: en el modelo original, el proveedor suele ser una persona externa, y "empresa_id" es la empresa a la que visita o representa.
                                                                                      // Asumiremos que proveedor_data trae info relevante de la empresa.

    let empresa_id = format!("empresas:{}", input.empresa_id); // Asumiendo que empresas están en tabla empresas, si no, se guarda string.
                                                               // Si empresa_id es solo referencia, guardamos el nombre obtenido previamente via SQL o similar.

    let usuario_id = format!("users:{}", input.usuario_ingreso_id);

    // Ajuste: El input tiene `empresa_id`. Si queremos guardar el nombre denormalizado, el servicio debe pasarlo.
    // Por simplicidad, guardamos empresa_id como referencia si es una tabla, o string si es un campo texto.
    // En el modelo `Ingreso`, campo `empresa_nombre` es string.

    let mut result = client
        .query(
            r#"
            CREATE ingresos CONTENT {
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
                fecha_hora_ingreso: $now,
                observaciones: $observaciones,
                motivo: $motivo,
                area_visitada: $area_visitada,
                created_at: $now,
                updated_at: $now
            };
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
        .bind(("now", now))
        .bind(("observaciones", input.observaciones))
        .bind(("motivo", input.motivo))
        .bind(("area_visitada", input.area_visitada))
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
