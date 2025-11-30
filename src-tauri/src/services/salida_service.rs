// ==========================================
// src/services/salida_service.rs
// ==========================================
// Orquesta DB + Dominio para la fase de SALIDA

use crate::db::{alerta_gafete_queries as alerta_db, ingreso_queries as db};
use crate::domain::ingreso::validaciones_salida as domain;
use crate::models::ingreso::{IngresoResponse, RegistrarSalidaInput};
use chrono::Utc;
use serde::Serialize;
use sqlx::SqlitePool;
use uuid::Uuid;

// ==========================================
// DTOs ESPECÍFICOS DE SALIDA
// ==========================================

/// Resultado de validación pre-salida
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultadoValidacionSalida {
    pub puede_salir: bool,
    pub errores: Vec<String>,
    pub advertencias: Vec<String>,
}

// ==========================================
// VALIDACIÓN PRE-SALIDA
// ==========================================

/// Valida que se puede registrar la salida
/// 
/// Verifica:
/// 1. Ingreso existe
/// 2. Ingreso está abierto
/// 3. Gafete devuelto coincide (si aplica)
pub async fn validar_puede_salir(
    pool: &SqlitePool,
    ingreso_id: &str,
    gafete_devuelto: Option<&str>,
) -> Result<ResultadoValidacionSalida, String> {
    let mut errores = Vec::new();
    let advertencias = Vec::new();

    // 1. Verificar que el ingreso existe
    let ingreso = match db::find_by_id(pool, ingreso_id).await {
        Ok(i) => i,
        Err(e) => {
            errores.push(e);
            return Ok(ResultadoValidacionSalida {
                puede_salir: false,
                errores,
                advertencias,
            });
        }
    };

    // 2. Verificar que está abierto (usando dominio)
    if let Err(e) = domain::validar_ingreso_abierto(&ingreso.fecha_hora_salida) {
        errores.push(e);
    }

    // 3. Validar gafete si se devolvió algo
    if let Some(devuelto) = gafete_devuelto {
        if let Err(e) =
            domain::validar_gafete_coincide(ingreso.gafete_numero.as_deref(), Some(devuelto))
        {
            errores.push(e);
        }
    }

    Ok(ResultadoValidacionSalida {
        puede_salir: errores.is_empty(),
        errores,
        advertencias,
    })
}

// ==========================================
// REGISTRAR SALIDA
// ==========================================

/// Registra la salida de un contratista
/// 
/// Orquesta:
/// 1. Obtener ingreso actual
/// 2. Validaciones de dominio (estado abierto, gafete)
/// 3. Calcular tiempo de permanencia
/// 4. Actualizar registro en DB
/// 5. Generar reporte de gafete si aplica
/// 6. Retornar ingreso actualizado
pub async fn registrar_salida(
    pool: &SqlitePool,
    input: RegistrarSalidaInput,
    usuario_id: String,
) -> Result<IngresoResponse, String> {
    // 1. Obtener ingreso actual
    let ingreso = db::find_by_id(pool, &input.ingreso_id).await?;

    // 2. Validar con dominio que está abierto
    domain::validar_ingreso_abierto(&ingreso.fecha_hora_salida)?;

    // 3. Preparar datos de salida
    let now = Utc::now().to_rfc3339();

    // 4. Validar tiempo de salida (no puede ser antes del ingreso)
    domain::validar_tiempo_salida(&ingreso.fecha_hora_ingreso, &now)?;

    // 5. Calcular tiempo de permanencia con dominio
    let minutos_permanencia = domain::calcular_tiempo_permanencia(&ingreso.fecha_hora_ingreso, &now)?;

    // 6. Evaluar devolución de gafete con dominio
    let tenia_gafete = ingreso.gafete_numero.is_some();
    let gafete_asignado = ingreso.gafete_numero.as_deref();

    // Determinar gafete devuelto (None si no devolvió)
    let gafete_devuelto = if input.devolvio_gafete {
        gafete_asignado // Si dice que devolvió, asumimos que devolvió el mismo
    } else {
        None
    };

    let decision = domain::evaluar_devolucion_gafete(
        tenia_gafete,
        gafete_asignado,
        input.devolvio_gafete,
        gafete_devuelto,
    )?;

    // 7. Actualizar registro de salida en DB
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

    // 8. Generar reporte de gafete si es necesario
    if decision.debe_generar_reporte {
        let gafete_numero = decision
            .gafete_numero
            .ok_or_else(|| "Error: debe generar reporte pero no hay número de gafete".to_string())?;

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
            decision.motivo.as_deref(),
            &usuario_id,
            &now,
            &now,
        )
        .await?;
    }

    // 9. Retornar ingreso actualizado
    get_ingreso_by_id(pool, input.ingreso_id).await
}

// ==========================================
// REGISTRAR SALIDA CON GAFETE ESPECÍFICO
// ==========================================

/// Registra salida verificando que el gafete devuelto coincida EXACTAMENTE
/// 
/// Esta versión es más estricta y requiere que el frontend envíe
/// el número de gafete devuelto explícitamente
pub async fn registrar_salida_con_verificacion_gafete(
    pool: &SqlitePool,
    ingreso_id: String,
    devolvio_gafete: bool,
    gafete_devuelto: Option<String>,
    usuario_id: String,
    observaciones_salida: Option<String>,
) -> Result<IngresoResponse, String> {
    // 1. Obtener ingreso actual
    let ingreso = db::find_by_id(pool, &ingreso_id).await?;

    // 2. Validar con dominio que está abierto
    domain::validar_ingreso_abierto(&ingreso.fecha_hora_salida)?;

    // 3. Preparar datos de salida
    let now = Utc::now().to_rfc3339();

    // 4. Validar tiempo de salida
    domain::validar_tiempo_salida(&ingreso.fecha_hora_ingreso, &now)?;

    // 5. Calcular tiempo de permanencia
    let minutos_permanencia = domain::calcular_tiempo_permanencia(&ingreso.fecha_hora_ingreso, &now)?;

    // 6. Evaluar devolución de gafete (versión estricta)
    let tenia_gafete = ingreso.gafete_numero.is_some();
    let gafete_asignado = ingreso.gafete_numero.as_deref();

    let decision = domain::evaluar_devolucion_gafete(
        tenia_gafete,
        gafete_asignado,
        devolvio_gafete,
        gafete_devuelto.as_deref(),
    )?;

    // 7. Actualizar registro de salida
    db::registrar_salida(
        pool,
        &ingreso_id,
        &now,
        minutos_permanencia,
        &usuario_id,
        observaciones_salida.as_deref(),
        &now,
    )
    .await?;

    // 8. Generar reporte si es necesario
    if decision.debe_generar_reporte {
        let gafete_numero = decision
            .gafete_numero
            .ok_or_else(|| "Error: debe generar reporte pero no hay número de gafete".to_string())?;

        let nombre_completo = format!("{} {}", ingreso.nombre, ingreso.apellido);
        let alerta_id = Uuid::new_v4().to_string();

        alerta_db::insert(
            pool,
            &alerta_id,
            ingreso.contratista_id.as_deref(),
            &ingreso.cedula,
            &nombre_completo,
            &gafete_numero,
            &ingreso_id,
            &now,
            decision.motivo.as_deref(),
            &usuario_id,
            &now,
            &now,
        )
        .await?;
    }

    // 9. Retornar ingreso actualizado
    get_ingreso_by_id(pool, ingreso_id).await
}

// ==========================================
// CONSULTAS AUXILIARES
// ==========================================

/// Obtiene un ingreso por ID con detalles completos
async fn get_ingreso_by_id(pool: &SqlitePool, id: String) -> Result<IngresoResponse, String> {
    let ingreso = db::find_by_id(pool, &id).await?;
    let details = db::find_details_by_id(pool, &id).await?;

    let mut response = IngresoResponse::from(ingreso);
    response.usuario_ingreso_nombre = details.usuario_ingreso_nombre.unwrap_or_default();
    response.usuario_salida_nombre = details.usuario_salida_nombre;
    response.vehiculo_placa = details.vehiculo_placa;

    Ok(response)
}

// ==========================================
// CONSULTAS DE REPORTES
// ==========================================

/// Obtiene todas las salidas de un día específico
pub async fn get_salidas_del_dia(
    pool: &SqlitePool,
    fecha: &str, // formato: "YYYY-MM-DD"
) -> Result<Vec<IngresoResponse>, String> {
    let ingresos = db::find_all(pool).await?;

    let salidas_del_dia: Vec<_> = ingresos
        .into_iter()
        .filter(|i| {
            if let Some(ref salida) = i.fecha_hora_salida {
                salida.starts_with(fecha)
            } else {
                false
            }
        })
        .map(IngresoResponse::from)
        .collect();

    Ok(salidas_del_dia)
}

/// Obtiene estadísticas de salidas
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EstadisticasSalidas {
    pub total_salidas: usize,
    pub con_gafete_devuelto: usize,
    pub sin_gafete_devuelto: usize,
    pub promedio_permanencia_minutos: i64,
}

pub async fn get_estadisticas_salidas(
    pool: &SqlitePool,
    fecha_desde: Option<&str>,
    fecha_hasta: Option<&str>,
) -> Result<EstadisticasSalidas, String> {
    let ingresos = db::find_all(pool).await?;

    // Filtrar salidas en el rango de fechas
    let salidas: Vec<_> = ingresos
        .into_iter()
        .filter(|i| {
            if let Some(ref salida) = i.fecha_hora_salida {
                let mut incluir = true;

                if let Some(desde) = fecha_desde {
                    incluir = incluir && salida.as_str() >= desde;
                }

                if let Some(hasta) = fecha_hasta {
                    incluir = incluir && salida.as_str() <= hasta;
                }

                incluir
            } else {
                false
            }
        })
        .collect();

    let total_salidas = salidas.len();

    // Contar gafetes devueltos
    let con_gafete_devuelto = salidas
        .iter()
        .filter(|i| i.gafete_numero.is_some())
        .count();

    let sin_gafete_devuelto = total_salidas - con_gafete_devuelto;

    // Calcular promedio de permanencia
    let suma_permanencias: i64 = salidas
        .iter()
        .filter_map(|i| i.tiempo_permanencia_minutos)
        .sum();

    let promedio_permanencia_minutos = if total_salidas > 0 {
        suma_permanencias / total_salidas as i64
    } else {
        0
    };

    Ok(EstadisticasSalidas {
        total_salidas,
        con_gafete_devuelto,
        sin_gafete_devuelto,
        promedio_permanencia_minutos,
    })
}