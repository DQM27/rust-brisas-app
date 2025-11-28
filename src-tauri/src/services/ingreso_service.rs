// src/services/ingreso_service.rs
// ==========================================
// Orquesta dominio, db y otros servicios - Lógica de negocio completa

use crate::db::alerta_gafete_queries as alerta_db;
use crate::db::contratista_queries;
use crate::db::ingreso_queries as db;
use crate::db::lista_negra_queries;
use crate::domain::ingreso as domain;
use crate::models::ingreso::{
    AlertaGafeteResponse, CreateIngresoContratistaInput, IngresoListResponse, IngresoResponse,
    ModoIngreso, RegistrarSalidaInput, ResolverAlertaInput, TipoAutorizacion, TipoIngreso,
    ValidacionIngresoResponse,
};
use crate::services::gafete_service;
use chrono::{NaiveDateTime, Utc};
use sqlx::SqlitePool;
use uuid::Uuid;

// ==========================================
// VALIDAR INGRESO CONTRATISTA
// ==========================================

pub async fn validar_ingreso_contratista(
    pool: &SqlitePool,
    contratista_id: String,
) -> Result<ValidacionIngresoResponse, String> {
    // 1. Verificar lista negra
    let block_status = lista_negra_queries::check_if_blocked(pool, &contratista_id).await?;

    if block_status.blocked {
        let motivo = block_status.motivo.unwrap_or_default();
        return Ok(ValidacionIngresoResponse {
            puede_ingresar: false,
            motivo_rechazo: Some(format!("Contratista bloqueado: {}", motivo)),
            alertas: vec![],
            contratista: None,
            tiene_ingreso_abierto: false,
            ingreso_abierto: None,
        });
    }

    // 2. Verificar que no tenga ingreso abierto
    let ingreso_abierto = db::find_ingreso_abierto_by_contratista(pool, &contratista_id).await;
    if let Ok(ingreso) = ingreso_abierto {
        return Ok(ValidacionIngresoResponse {
            puede_ingresar: false,
            motivo_rechazo: Some("El contratista ya tiene un ingreso abierto".to_string()),
            alertas: vec![],
            contratista: None,
            tiene_ingreso_abierto: true,
            ingreso_abierto: Some(IngresoResponse::from(ingreso)),
        });
    }

    // 3. Verificar datos del contratista
    let contratista_opt = contratista_queries::find_basic_info_by_id(pool, &contratista_id).await?;

    match contratista_opt {
        None => Ok(ValidacionIngresoResponse {
            puede_ingresar: false,
            motivo_rechazo: Some("Contratista no encontrado".to_string()),
            alertas: vec![],
            contratista: None,
            tiene_ingreso_abierto: false,
            ingreso_abierto: None,
        }),
        Some(contratista) => {
            // Verificar estado
            if contratista.estado.to_lowercase() != "activo" {
                return Ok(ValidacionIngresoResponse {
                    puede_ingresar: false,
                    motivo_rechazo: Some(format!("Contratista con estado: {}", contratista.estado)),
                    alertas: vec![],
                    contratista: Some(serde_json::json!({
                        "id": contratista.id,
                        "cedula": contratista.cedula,
                        "nombre": contratista.nombre,
                        "apellido": contratista.apellido,
                        "empresa_nombre": contratista.empresa_nombre,
                        "estado": contratista.estado,
                    })),
                    tiene_ingreso_abierto: false,
                    ingreso_abierto: None,
                });
            }

            // Verificar PRAIND
            let praind_vigente =
                domain::verificar_praind_vigente(&contratista.fecha_vencimiento_praind)?;
            if !praind_vigente {
                return Ok(ValidacionIngresoResponse {
                    puede_ingresar: false,
                    motivo_rechazo: Some("PRAIND vencido".to_string()),
                    alertas: vec![],
                    contratista: Some(serde_json::json!({
                        "id": contratista.id,
                        "cedula": contratista.cedula,
                        "nombre": contratista.nombre,
                        "apellido": contratista.apellido,
                        "empresa_nombre": contratista.empresa_nombre,
                        "fecha_vencimiento_praind": contratista.fecha_vencimiento_praind,
                    })),
                    tiene_ingreso_abierto: false,
                    ingreso_abierto: None,
                });
            }

            // 4. Verificar alertas de gafetes pendientes (CORREGIDO: usar cédula correcta)
            let alertas_db = alerta_db::find_pendientes_by_cedula(pool, &contratista.cedula).await?;
            let mut alertas_msgs = Vec::new();
            if !alertas_db.is_empty() {
                alertas_msgs.push(format!("Tiene {} gafete(s) sin devolver", alertas_db.len()));
            }

            Ok(ValidacionIngresoResponse {
                puede_ingresar: true,
                motivo_rechazo: None,
                alertas: alertas_msgs,
                contratista: Some(serde_json::json!({
                    "id": contratista.id,
                    "cedula": contratista.cedula,
                    "nombre": contratista.nombre,
                    "apellido": contratista.apellido,
                    "empresa_nombre": contratista.empresa_nombre,
                    "praind_vigente": true,
                    "estado": contratista.estado,
                })),
                tiene_ingreso_abierto: false,
                ingreso_abierto: None,
            })
        }
    }
}

// ==========================================
// CREAR INGRESO CONTRATISTA
// ==========================================

pub async fn create_ingreso_contratista(
    pool: &SqlitePool,
    input: CreateIngresoContratistaInput,
    usuario_id: String,
) -> Result<IngresoResponse, String> {
    // 1. Validar input
    domain::validar_create_contratista_input(&input)?;

    // 2. Verificar que NO tenga ingreso abierto (NUEVO - evita duplicados)
    if db::find_ingreso_abierto_by_contratista(pool, &input.contratista_id)
        .await
        .is_ok()
    {
        return Err("El contratista ya tiene un ingreso abierto".to_string());
    }

    // 3. Obtener datos del contratista
    let contratista = contratista_queries::find_basic_info_by_id(pool, &input.contratista_id)
        .await?
        .ok_or_else(|| "Contratista no encontrado".to_string())?;

    // 4. Validaciones de dominio
    let praind_vigente = domain::verificar_praind_vigente(&contratista.fecha_vencimiento_praind)?;

    // 5. Validar gafete si se proporciona
    let gafete_normalizado = if let Some(ref g) = input.gafete_numero {
        let normalizado = domain::normalizar_numero_gafete(g);
        let disponible = gafete_service::is_gafete_disponible(pool, &normalizado).await?;
        if !disponible {
            return Err(format!("Gafete {} no está disponible", normalizado));
        }
        Some(normalizado)
    } else {
        None
    };

    // 6. Parsear enums
    let tipo_autorizacion = TipoAutorizacion::from_str(&input.tipo_autorizacion)?;
    let modo_ingreso = ModoIngreso::from_str(&input.modo_ingreso)?;

    // 7. Crear registro
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    db::insert(
        pool,
        &id,
        Some(input.contratista_id.as_str()),
        &contratista.cedula,
        &contratista.nombre,
        &contratista.apellido,
        &contratista.empresa_nombre,
        TipoIngreso::Contratista.as_str(),
        tipo_autorizacion.as_str(),
        modo_ingreso.as_str(),
        input.vehiculo_id.as_deref(),
        None, // placa_temporal - solo para contratistas
        gafete_normalizado.as_deref(),
        &now,
        &usuario_id,
        Some(praind_vigente),
        Some(contratista.estado.as_str()),
        input.observaciones.as_deref(),
        &now,
        &now,
    )
    .await?;

    // 8. Retornar
    get_ingreso_by_id(pool, id).await
}

// ==========================================
// REGISTRAR SALIDA
// ==========================================

pub async fn registrar_salida(
    pool: &SqlitePool,
    input: RegistrarSalidaInput,
    usuario_id: String,
) -> Result<IngresoResponse, String> {
    // 1. Obtener ingreso
    let ingreso = db::find_by_id(pool, &input.ingreso_id).await?;

    // 2. Verificar que esté abierto
    if ingreso.fecha_hora_salida.is_some() {
        return Err("El ingreso ya fue cerrado".to_string());
    }

    // 3. Calcular tiempo de permanencia
    let fecha_ingreso = NaiveDateTime::parse_from_str(&ingreso.fecha_hora_ingreso, "%+")
        .map_err(|e| format!("Error parseando fecha: {}", e))?;
    let fecha_salida = Utc::now().naive_utc();
    let minutos = (fecha_salida - fecha_ingreso).num_minutes();

    // 4. Registrar salida
    let now = Utc::now().to_rfc3339();
    db::registrar_salida(
        pool,
        &input.ingreso_id,
        &now,
        minutos,
        &usuario_id,
        input.observaciones_salida.as_deref(),
        &now,
    )
    .await?;

    // 5. Si NO devolvió el gafete y tenía gafete asignado, crear alerta
    if !input.devolvio_gafete && ingreso.gafete_numero.is_some() {
        let gafete_numero = ingreso.gafete_numero.unwrap();
        let nombre_completo = format!("{} {}", ingreso.nombre, ingreso.apellido);

        let alerta_id = Uuid::new_v4().to_string();
        alerta_db::insert(
            pool,
            &alerta_id,
            ingreso.contratista_id.as_deref(),
            &ingreso.cedula,
            &nombre_completo,
            &gafete_numero,
            &input.ingreso_id,
            &now,
            input.observaciones_salida.as_deref(),
            &usuario_id,
            &now,
            &now,
        )
        .await?;
    }

    // 6. Retornar ingreso actualizado
    get_ingreso_by_id(pool, input.ingreso_id).await
}

// ==========================================
// OBTENER INGRESO POR ID
// ==========================================

pub async fn get_ingreso_by_id(pool: &SqlitePool, id: String) -> Result<IngresoResponse, String> {
    let ingreso = db::find_by_id(pool, &id).await?;

    // Obtener detalles adicionales
    let details = db::find_details_by_id(pool, &id).await?;

    let mut response = IngresoResponse::from(ingreso);
    response.usuario_ingreso_nombre = details.usuario_ingreso_nombre.unwrap_or_default();
    response.usuario_salida_nombre = details.usuario_salida_nombre;
    response.vehiculo_placa = details.vehiculo_placa;

    Ok(response)
}

// ==========================================
// OBTENER TODOS LOS INGRESOS
// ==========================================

pub async fn get_all_ingresos(pool: &SqlitePool) -> Result<IngresoListResponse, String> {
    let ingresos = db::find_all(pool).await?;

    let mut responses = Vec::new();
    for ingreso in ingresos {
        let response = IngresoResponse::from(ingreso);
        responses.push(response);
    }

    let total = responses.len();
    let adentro = responses
        .iter()
        .filter(|i| i.fecha_hora_salida.is_none())
        .count();
    let salieron = total - adentro;

    Ok(IngresoListResponse {
        ingresos: responses,
        total,
        adentro,
        salieron,
    })
}

// ==========================================
// OBTENER INGRESOS ABIERTOS
// ==========================================

pub async fn get_ingresos_abiertos(pool: &SqlitePool) -> Result<Vec<IngresoResponse>, String> {
    let ingresos = db::find_ingresos_abiertos(pool).await?;

    let mut responses = Vec::new();
    for ingreso in ingresos {
        let response = IngresoResponse::from(ingreso);
        responses.push(response);
    }

    Ok(responses)
}

// ==========================================
// OBTENER INGRESO POR GAFETE
// ==========================================

pub async fn get_ingreso_by_gafete(
    pool: &SqlitePool,
    gafete_numero: String,
) -> Result<IngresoResponse, String> {
    let ingreso = db::find_ingreso_by_gafete(pool, &gafete_numero).await?;
    let response = IngresoResponse::from(ingreso);
    Ok(response)
}

// ==========================================
// ALERTAS DE GAFETES
// ==========================================

pub async fn get_alertas_pendientes_by_cedula(
    pool: &SqlitePool,
    cedula: String,
) -> Result<Vec<AlertaGafeteResponse>, String> {
    let alertas = alerta_db::find_pendientes_by_cedula(pool, &cedula).await?;
    let responses: Vec<_> = alertas
        .into_iter()
        .map(AlertaGafeteResponse::from)
        .collect();
    Ok(responses)
}

pub async fn get_all_alertas_gafetes(
    pool: &SqlitePool,
) -> Result<Vec<AlertaGafeteResponse>, String> {
    let alertas = alerta_db::find_all(pool, None).await?;
    let responses: Vec<_> = alertas
        .into_iter()
        .map(AlertaGafeteResponse::from)
        .collect();
    Ok(responses)
}

pub async fn resolver_alerta_gafete(
    pool: &SqlitePool,
    input: ResolverAlertaInput,
) -> Result<AlertaGafeteResponse, String> {
    let now = Utc::now().to_rfc3339();
    alerta_db::resolver(pool, &input.alerta_id, &now, input.notas.as_deref(), &now).await?;

    let alerta = alerta_db::find_by_id(pool, &input.alerta_id).await?;
    Ok(AlertaGafeteResponse::from(alerta))
}