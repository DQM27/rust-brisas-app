// src/services/ingreso_contratista_service.rs

use crate::db::contratista_queries;
use crate::db::lista_negra_queries;
use crate::services::alerta_service;
// Usamos nuestro nuevo modulo de queries
use crate::db::ingreso_contratista_queries as db;

// Usamos nuestro nuevo modulo de dominio
use crate::domain::ingreso_contratista as domain;

use crate::models::ingreso::{
    CreateIngresoContratistaInput, IngresoResponse, ModoIngreso, RegistrarSalidaInput,
    TipoAutorizacion, TipoIngreso, ValidacionIngresoResponse,
};
use crate::services::gafete_service;
use chrono::Utc;
use serde::Serialize;
use sqlx::SqlitePool;

use uuid::Uuid;

// ==========================================
// DTOs HELPER PARA SERVICIO
// ==========================================

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultadoValidacionSalida {
    pub puede_salir: bool,
    pub errores: Vec<String>,
    pub advertencias: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IngresoConEstadoResponse {
    #[serde(flatten)]
    pub ingreso: IngresoResponse,
    pub alerta_tiempo: domain::AlertaTiempo,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertaTiempoExcedido {
    pub ingreso_id: String,
    pub cedula: String,
    pub nombre_completo: String,
    pub empresa_nombre: String,
    pub fecha_hora_ingreso: String,
    pub minutos_transcurridos: i64,
    pub minutos_excedidos: i64,
    pub estado: domain::EstadoPermanencia,
}

// Implementación del trait helper para validación
impl domain::InputEntrada for CreateIngresoContratistaInput {
    fn tipo_ingreso(&self) -> &str {
        "contratista"
    }
}

// ==========================================
// 1. FASE DE ENTRADA
// ==========================================

pub async fn validar_ingreso_contratista(
    pool: &SqlitePool,
    contratista_id: String,
) -> Result<ValidacionIngresoResponse, String> {
    // A. Verificar lista negra
    let block_status = lista_negra_queries::check_if_blocked(pool, &contratista_id).await?;

    // B. Verificar ingreso abierto
    let tiene_ingreso_abierto = db::find_ingreso_abierto_by_contratista(pool, &contratista_id)
        .await
        .is_ok();

    if tiene_ingreso_abierto {
        let ingreso = db::find_ingreso_abierto_by_contratista(pool, &contratista_id).await?;
        return Ok(ValidacionIngresoResponse {
            puede_ingresar: false,
            motivo_rechazo: Some("El contratista ya tiene un ingreso abierto".to_string()),
            alertas: vec![],
            contratista: None,
            tiene_ingreso_abierto: true,
            ingreso_abierto: Some(IngresoResponse::from(ingreso)),
        });
    }

    // C. Datos del contratista
    let contratista_opt = contratista_queries::find_basic_info_by_id(pool, &contratista_id)
        .await
        .map_err(|e| e.to_string())?;
    let contratista = match contratista_opt {
        None => {
            return Ok(ValidacionIngresoResponse {
                puede_ingresar: false,
                motivo_rechazo: Some("Contratista no encontrado".to_string()),
                alertas: vec![],
                contratista: None,
                tiene_ingreso_abierto: false,
                ingreso_abierto: None,
            });
        }
        Some(c) => c,
    };

    // D. Validaciones de Dominio
    let praind_vigente = domain::verificar_praind_vigente(&contratista.fecha_vencimiento_praind)?;
    let alertas_db = alerta_service::find_pendientes_by_cedula(pool, &contratista.cedula)
        .await
        .map_err(|e| e.to_string())?;

    let resultado = domain::evaluar_elegibilidad_entrada(
        block_status.blocked,
        block_status.motivo,
        tiene_ingreso_abierto,
        &contratista.estado,
        praind_vigente,
        alertas_db.len(),
    );

    // E. Vehículos (para frontend)
    let vehiculos = crate::db::vehiculo_queries::find_by_contratista(pool, &contratista_id)
        .await
        .unwrap_or_default();

    // F. Construir JSON Seguro
    let contratista_json = if resultado.puede_ingresar || resultado.motivo_rechazo.is_some() {
        Some(serde_json::json!({
            "id": contratista.id,
            "cedula": contratista.cedula,
            "nombre": contratista.nombre,
            "apellido": contratista.apellido,
            "empresa_nombre": contratista.empresa_nombre,
            "estado": contratista.estado,
            "praind_vigente": praind_vigente,
            "fecha_vencimiento_praind": contratista.fecha_vencimiento_praind,
            "vehiculos": vehiculos,
            "alertas": alertas_db.iter().cloned().map(crate::models::ingreso::AlertaGafeteResponse::from).collect::<Vec<_>>()
        }))
    } else {
        None
    };

    Ok(ValidacionIngresoResponse {
        puede_ingresar: resultado.puede_ingresar,
        motivo_rechazo: resultado.motivo_rechazo,
        alertas: resultado.alertas,
        contratista: contratista_json,
        tiene_ingreso_abierto: false,
        ingreso_abierto: None,
    })
}

pub async fn crear_ingreso_contratista(
    pool: &SqlitePool,
    input: CreateIngresoContratistaInput,
    usuario_id: String,
) -> Result<IngresoResponse, String> {
    // 1. Validar input básico
    domain::validar_input_entrada(&input)?;

    // 2. Verificar duplicados (DB check final)
    if db::find_ingreso_abierto_by_contratista(pool, &input.contratista_id)
        .await
        .is_ok()
    {
        return Err("El contratista ya tiene un ingreso abierto".to_string());
    }

    // 3. Obtener Datos
    let contratista = contratista_queries::find_basic_info_by_id(pool, &input.contratista_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Contratista no encontrado".to_string())?;

    let praind_vigente = domain::verificar_praind_vigente(&contratista.fecha_vencimiento_praind)?;

    // 4. Gestionar Gafete
    let gafete_normalizado = if let Some(ref g) = input.gafete_numero {
        let normalizado = domain::normalizar_numero_gafete(g);
        let disponible = gafete_service::is_gafete_disponible(pool, &normalizado, "contratista")
            .await
            .map_err(|e| e.to_string())?;
        if !disponible {
            return Err(format!("Gafete {} no está disponible", normalizado));
        }
        Some(normalizado)
    } else {
        None
    };

    // 5. Insertar
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let tipo_autorizacion = TipoAutorizacion::from_str(&input.tipo_autorizacion)?;
    let modo_ingreso = ModoIngreso::from_str(&input.modo_ingreso)?;

    db::insert(
        pool,
        &id,
        &input.contratista_id,
        &contratista.cedula,
        &contratista.nombre,
        &contratista.apellido,
        &contratista.empresa_nombre,
        TipoIngreso::Contratista.as_str(),
        tipo_autorizacion.as_str(),
        modo_ingreso.as_str(),
        input.vehiculo_id.as_deref(),
        None,
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

    get_ingreso_by_id(pool, id).await
}

// ==========================================
// 2. FASE DE SALIDA
// ==========================================

pub async fn validar_puede_salir(
    pool: &SqlitePool,
    ingreso_id: &str,
    gafete_devuelto: Option<&str>,
) -> Result<ResultadoValidacionSalida, String> {
    let mut errores = Vec::new();

    match db::find_by_id(pool, ingreso_id).await {
        Ok(ingreso) => {
            if let Err(e) = domain::validar_ingreso_abierto(&ingreso.fecha_hora_salida) {
                errores.push(e);
            }
            if let Some(devuelto) = gafete_devuelto {
                if let Err(e) = domain::validar_gafete_coincide(
                    ingreso.gafete_numero.as_deref(),
                    Some(devuelto),
                ) {
                    errores.push(e);
                }
            }
        }
        Err(e) => errores.push(e),
    }

    Ok(ResultadoValidacionSalida {
        puede_salir: errores.is_empty(),
        errores,
        advertencias: vec![],
    })
}

pub async fn registrar_salida(
    pool: &SqlitePool,
    input: RegistrarSalidaInput,
    usuario_id: String,
) -> Result<IngresoResponse, String> {
    let ingreso = db::find_by_id(pool, &input.ingreso_id).await?;
    domain::validar_ingreso_abierto(&ingreso.fecha_hora_salida)?;

    let now = Utc::now().to_rfc3339();
    domain::validar_tiempo_salida(&ingreso.fecha_hora_ingreso, &now)?;
    let minutos_permanencia =
        domain::calcular_tiempo_permanencia(&ingreso.fecha_hora_ingreso, &now)?;

    // Evaluar reporte de gafete
    let decision = domain::evaluar_devolucion_gafete(
        ingreso.gafete_numero.is_some(),
        ingreso.gafete_numero.as_deref(),
        input.devolvio_gafete,
        // Si dice que devolvio, asumimos que es el mismo (frontend simple)
        if input.devolvio_gafete {
            ingreso.gafete_numero.as_deref()
        } else {
            None
        },
    )?;

    // Actualizar DB
    db::registrar_salida(
        pool,
        &input.ingreso_id,
        &now,
        minutos_permanencia,
        &usuario_id,
        input.observaciones_salida.as_deref(),
        &now,
    )
    .await?;

    // Generar Alerta si aplica
    if decision.debe_generar_reporte {
        if let Some(num) = decision.gafete_numero {
            let alerta_id = Uuid::new_v4().to_string();
            let nombre_completo = format!("{} {}", ingreso.nombre, ingreso.apellido);
            alerta_service::insert(
                pool,
                &alerta_id,
                ingreso.contratista_id.as_deref(),
                &ingreso.cedula,
                &nombre_completo,
                &num,
                Some(&input.ingreso_id),
                None,
                &now,
                decision.motivo.as_deref(),
                &usuario_id,
                &now,
                &now,
            )
            .await
            .map_err(|e| e.to_string())?;
        }
    }

    get_ingreso_by_id(pool, input.ingreso_id).await
}

// ==========================================
// 3. FASE DE PERMANENCIA (MONITOREO)
// ==========================================

pub async fn get_ingresos_abiertos_con_alertas(
    pool: &SqlitePool,
) -> Result<Vec<IngresoConEstadoResponse>, String> {
    let ingresos = db::find_ingresos_abiertos(pool).await?;
    let mut responses = Vec::new();

    for ingreso in ingresos {
        let minutos = domain::calcular_tiempo_transcurrido(&ingreso.fecha_hora_ingreso)?;
        let alerta_tiempo = domain::construir_alerta_tiempo(minutos);
        let details = db::find_details_by_id(pool, &ingreso.id).await?;

        let mut ingreso_resp = IngresoResponse::from(ingreso);
        ingreso_resp.usuario_ingreso_nombre = details.usuario_ingreso_nombre.unwrap_or_default();
        ingreso_resp.vehiculo_placa = details.vehiculo_placa;

        responses.push(IngresoConEstadoResponse {
            ingreso: ingreso_resp,
            alerta_tiempo,
        });
    }
    Ok(responses)
}

pub async fn verificar_tiempos_excedidos(
    pool: &SqlitePool,
) -> Result<Vec<AlertaTiempoExcedido>, String> {
    let ingresos = db::find_ingresos_abiertos(pool).await?;
    let mut alertas = Vec::new();

    for ingreso in ingresos {
        let minutos = domain::calcular_tiempo_transcurrido(&ingreso.fecha_hora_ingreso)?;
        let estado = domain::evaluar_estado_permanencia(minutos);

        if estado == domain::EstadoPermanencia::TiempoExcedido {
            let excedidos = -domain::calcular_minutos_restantes(minutos);
            alertas.push(AlertaTiempoExcedido {
                ingreso_id: ingreso.id,
                cedula: ingreso.cedula.clone(),
                nombre_completo: format!("{} {}", ingreso.nombre, ingreso.apellido),
                empresa_nombre: ingreso.empresa_nombre,
                fecha_hora_ingreso: ingreso.fecha_hora_ingreso,
                minutos_transcurridos: minutos,
                minutos_excedidos: excedidos,
                estado,
            });
        }
    }
    Ok(alertas)
}

// ==========================================
// HELPERS PRIVADOS
// ==========================================

async fn get_ingreso_by_id(pool: &SqlitePool, id: String) -> Result<IngresoResponse, String> {
    let ingreso = db::find_by_id(pool, &id).await?;
    let details = db::find_details_by_id(pool, &id).await?;

    let mut resp = IngresoResponse::from(ingreso);
    resp.usuario_ingreso_nombre = details.usuario_ingreso_nombre.unwrap_or_default();
    resp.usuario_salida_nombre = details.usuario_salida_nombre;
    resp.vehiculo_placa = details.vehiculo_placa;
    Ok(resp)
}
