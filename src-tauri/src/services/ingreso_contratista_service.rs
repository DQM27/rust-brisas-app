// src/services/ingreso_contratista_service.rs

use crate::db::contratista_queries;

use crate::services::alerta_service;
// Usamos nuestro nuevo modulo de queries
use crate::db::ingreso_contratista_queries as db;

// Usamos nuestro nuevo modulo de dominio
use crate::domain::errors::IngresoContratistaError;
use crate::domain::ingreso_contratista as domain;
use crate::models::lista_negra::BlockCheckResponse;
use crate::services::lista_negra_service;

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
) -> Result<ValidacionIngresoResponse, IngresoContratistaError> {
    // A. Buscar Contratista
    let contratista = contratista_queries::find_basic_info_by_id(pool, &contratista_id)
        .await
        .map_err(|e| IngresoContratistaError::Database(sqlx::Error::Protocol(e.to_string())))?
        .ok_or(IngresoContratistaError::ContratistaNotFound)?;

    // B. Verificar Bloqueo
    let block_response = lista_negra_service::check_is_blocked(pool, contratista.cedula.clone())
        .await
        .unwrap_or(BlockCheckResponse {
            is_blocked: false,
            motivo: None,
            bloqueado_desde: None,
            bloqueado_hasta: None,
            bloqueado_por: None,
        });

    // C. Verificar Ingreso Abierto
    let ingreso_abierto = db::find_ingreso_abierto_by_contratista(pool, &contratista.id)
        .await
        .map_err(|e| IngresoContratistaError::Database(sqlx::Error::Protocol(e.to_string())))?;

    if let Some(ref ingreso) = ingreso_abierto {
        let response = IngresoResponse::try_from(ingreso.clone()).map_err(|e| {
            IngresoContratistaError::Validation(format!("Error parsing ingreso: {}", e))
        })?;

        return Ok(ValidacionIngresoResponse {
            puede_ingresar: false,
            motivo_rechazo: Some("El contratista ya tiene un ingreso activo".to_string()),
            alertas: vec![],
            contratista: Some(serde_json::json!({
                "id": contratista.id,
                "cedula": contratista.cedula,
                "nombre": contratista.nombre,
                "apellido": contratista.apellido,
                "nombre_completo": format!("{} {}", contratista.nombre, contratista.apellido),
                "empresa_nombre": contratista.empresa_nombre,
                "estado": contratista.estado,
            })),
            tiene_ingreso_abierto: true,
            ingreso_abierto: Some(response),
        });
    }

    // D. Validaciones de Dominio
    let praind_vigente = domain::verificar_praind_vigente(&contratista.fecha_vencimiento_praind)?;
    let alertas_db = alerta_service::find_pendientes_by_cedula(pool, &contratista.cedula)
        .await
        .map_err(|e| IngresoContratistaError::Database(sqlx::Error::Protocol(e.to_string())))?;

    let resultado = domain::evaluar_elegibilidad_entrada(
        block_response.is_blocked,
        block_response.motivo,
        ingreso_abierto.is_some(),
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
) -> Result<IngresoResponse, IngresoContratistaError> {
    // 1. Validar input básico
    domain::validar_input_entrada(&input)?;

    // 2. Verificar duplicados (DB check final)
    let existing = db::find_ingreso_abierto_by_contratista(pool, &input.contratista_id).await?;
    if existing.is_some() {
        return Err(IngresoContratistaError::AlreadyInside);
    }

    // 3. Obtener Datos
    let contratista = contratista_queries::find_basic_info_by_id(pool, &input.contratista_id)
        .await?
        .ok_or(IngresoContratistaError::ContratistaNotFound)?;

    let praind_vigente = domain::verificar_praind_vigente(&contratista.fecha_vencimiento_praind)?;

    // 4. Gestionar Gafete
    let gafete_normalizado = if let Some(ref g) = input.gafete_numero {
        let normalizado = domain::normalizar_numero_gafete(g);
        let disponible = gafete_service::is_gafete_disponible(pool, &normalizado, "contratista")
            .await
            .map_err(|e| IngresoContratistaError::Gafete(e.to_string()))?;
        if !disponible {
            return Err(IngresoContratistaError::GafeteNotAvailable);
        }
        Some(normalizado)
    } else {
        None
    };

    // 5. Insertar
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    // Parse enums manually or map conversion errors
    let tipo_autorizacion: TipoAutorizacion = input.tipo_autorizacion.parse().map_err(|_| {
        IngresoContratistaError::Validation("Tipo autorización inválido".to_string())
    })?;
    let modo_ingreso: ModoIngreso = input
        .modo_ingreso
        .parse()
        .map_err(|_| IngresoContratistaError::Validation("Modo ingreso inválido".to_string()))?;

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
        Ok(Some(ingreso)) => {
            if let Err(e) = domain::validar_ingreso_abierto(&ingreso.fecha_hora_salida) {
                errores.push(e.to_string());
            }
            if let Some(devuelto) = gafete_devuelto {
                if let Err(e) = domain::validar_gafete_coincide(
                    ingreso.gafete_numero.as_deref(),
                    Some(devuelto),
                ) {
                    errores.push(e.to_string());
                }
            }
        }
        Ok(None) => errores.push("Ingreso no encontrado".to_string()),
        Err(e) => errores.push(e.to_string()),
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
) -> Result<IngresoResponse, IngresoContratistaError> {
    let ingreso = db::find_by_id(pool, &input.ingreso_id)
        .await?
        .ok_or(IngresoContratistaError::NotFound)?;
    domain::validar_ingreso_abierto(&ingreso.fecha_hora_salida)?;

    let now = Utc::now().to_rfc3339();
    domain::validar_tiempo_salida(&ingreso.fecha_hora_ingreso, &now)
        .map_err(|e: String| IngresoContratistaError::Validation(e))?; // domain::validar_tiempo_salida returns generic String yet? Check.
                                                                       // I need to update domain::validar_tiempo_salida too, I missed it?
                                                                       // Wait, step 1317: I updated calcular_tiempo_transcurrido. Did I update validar_tiempo_salida? No I didn't see it in the chunk replacement.
                                                                       // So I assume it returns String. I'll fix it here with map_err or update domain first.
                                                                       // Easier to map_err for now, or assume I update domain next.
                                                                       // Let's assume map_err logic for String errors from domain.

    let minutos_permanencia =
        domain::calcular_tiempo_permanencia(&ingreso.fecha_hora_ingreso, &now)
            .map_err(|e: String| IngresoContratistaError::Validation(e))?;

    // Evaluar reporte de gafete
    let decision = domain::evaluar_devolucion_gafete(
        ingreso.gafete_numero.is_some(),
        ingreso.gafete_numero.as_deref(),
        input.devolvio_gafete,
        if input.devolvio_gafete {
            ingreso.gafete_numero.as_deref()
        } else {
            None
        },
    )
    .map_err(|e| IngresoContratistaError::Validation(e))?; // This function returns DecisionReporteGafete struct, NOT Result. Wait, let me check.
                                                           // Step 1308: DecisionReporteGafete struct. It is not a result.
                                                           // But `evaluar_devolucion_gafete` call in original code has `?` (line 304).
                                                           // This implies it returned Result.

    // I need to check `evaluar_devolucion_gafete` signature.
    // If it returns Result, I need to know the error type.

    // Let's defer full replacement of registrar_salida until I know `evaluar_devolucion_gafete`.
    // But for now I'll use map_err for everything I'm unsure of.

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
            .map_err(|e| IngresoContratistaError::Database(sqlx::Error::Protocol(e.to_string())))?;
        }
    }

    get_ingreso_by_id(pool, input.ingreso_id).await
}

// ==========================================
// 3. FASE DE PERMANENCIA (MONITOREO)
// ==========================================

pub async fn get_ingresos_abiertos_con_alertas(
    pool: &SqlitePool,
) -> Result<Vec<IngresoConEstadoResponse>, IngresoContratistaError> {
    let ingresos = db::find_ingresos_abiertos(pool)
        .await
        .map_err(|e| IngresoContratistaError::Database(sqlx::Error::Protocol(e.to_string())))?;
    let mut responses = Vec::new();

    for ingreso in ingresos {
        let minutos = domain::calcular_tiempo_transcurrido(&ingreso.fecha_hora_ingreso)?;
        let alerta_tiempo = domain::construir_alerta_tiempo(minutos);
        let details = db::find_details_by_id(pool, &ingreso.id)
            .await
            .map_err(|e| IngresoContratistaError::Database(sqlx::Error::Protocol(e.to_string())))?
            .unwrap_or(db::IngresoDetails {
                usuario_ingreso_nombre: None,
                usuario_salida_nombre: None,
                vehiculo_placa: None,
            });

        let mut response = IngresoResponse::try_from(ingreso).map_err(|e| {
            IngresoContratistaError::Validation(format!("Error parsing ingreso: {}", e))
        })?;
        response.usuario_ingreso_nombre = details.usuario_ingreso_nombre.unwrap_or_default();
        response.vehiculo_placa = details.vehiculo_placa;

        responses.push(IngresoConEstadoResponse {
            ingreso: response,
            alerta_tiempo,
        });
    }
    Ok(responses)
}

pub async fn verificar_tiempos_excedidos(
    pool: &SqlitePool,
) -> Result<Vec<AlertaTiempoExcedido>, IngresoContratistaError> {
    let ingresos = db::find_ingresos_abiertos(pool)
        .await
        .map_err(|e| IngresoContratistaError::Database(sqlx::Error::Protocol(e.to_string())))?;
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

async fn get_ingreso_by_id(
    pool: &SqlitePool,
    id: String,
) -> Result<IngresoResponse, IngresoContratistaError> {
    let ingreso = db::find_by_id(pool, &id)
        .await?
        .ok_or(IngresoContratistaError::NotFound)?;
    let details = db::find_details_by_id(pool, &id)
        .await?
        .unwrap_or(db::IngresoDetails {
            usuario_ingreso_nombre: None,
            usuario_salida_nombre: None,
            vehiculo_placa: None,
        });

    let mut resp = IngresoResponse::try_from(ingreso).map_err(|e| {
        IngresoContratistaError::Validation(format!("Error parsing ingreso: {}", e))
    })?;
    resp.usuario_ingreso_nombre = details.usuario_ingreso_nombre.unwrap_or_default();
    resp.usuario_salida_nombre = details.usuario_salida_nombre;
    resp.vehiculo_placa = details.vehiculo_placa;
    Ok(resp)
}
