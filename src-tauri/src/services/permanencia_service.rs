// ==========================================
// src/services/permanencia_service.rs
// ==========================================
// Orquesta DB + Dominio para la fase de PERMANENCIA (monitoreo)

use crate::db::ingreso_queries as db;
use crate::db::lista_negra_queries;
use crate::domain::ingreso::validaciones_permanencia as domain;
use crate::domain::ingreso::tipos::{AlertaTiempo, EstadoPermanencia};
use crate::models::ingreso::IngresoResponse;
use serde::Serialize;
use sqlx::SqlitePool;

// ==========================================
// DTOs ESPECÍFICOS DE PERMANENCIA
// ==========================================

/// Respuesta de ingreso con información de estado de permanencia
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IngresoConEstadoResponse {
    #[serde(flatten)]
    pub ingreso: IngresoResponse,
    pub alerta_tiempo: AlertaTiempo,
}

/// Alerta de tiempo excedido
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
    pub estado: EstadoPermanencia,
}

/// Alerta de cambio en lista negra
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertaListaNegra {
    pub ingreso_id: String,
    pub cedula: String,
    pub nombre_completo: String,
    pub bloqueado: bool,
    pub motivo: Option<String>,
}

// ==========================================
// CONSULTAS CON ESTADO DE PERMANENCIA
// ==========================================

/// Obtiene un ingreso por ID con su estado de permanencia calculado
pub async fn get_ingreso_con_estado(
    pool: &SqlitePool,
    ingreso_id: String,
) -> Result<IngresoConEstadoResponse, String> {
    // 1. Obtener ingreso básico
    let ingreso = db::find_by_id(pool, &ingreso_id).await?;
    let details = db::find_details_by_id(pool, &ingreso_id).await?;

    let mut ingreso_response = IngresoResponse::from(ingreso.clone());
    ingreso_response.usuario_ingreso_nombre = details.usuario_ingreso_nombre.unwrap_or_default();
    ingreso_response.usuario_salida_nombre = details.usuario_salida_nombre;
    ingreso_response.vehiculo_placa = details.vehiculo_placa;

    // 2. Calcular estado de permanencia si está abierto
    let alerta_tiempo = if ingreso.fecha_hora_salida.is_none() {
        let minutos = domain::calcular_tiempo_transcurrido(&ingreso.fecha_hora_ingreso)?;
        domain::construir_alerta_tiempo(minutos)
    } else {
        // Ya salió, usar tiempo de permanencia registrado
        let minutos = ingreso.tiempo_permanencia_minutos.unwrap_or(0);
        domain::construir_alerta_tiempo(minutos)
    };

    Ok(IngresoConEstadoResponse {
        ingreso: ingreso_response,
        alerta_tiempo,
    })
}

// ==========================================
// FUNCIÓN: get_ingresos_abiertos_con_alertas
// ==========================================

pub async fn get_ingresos_abiertos_con_alertas(
    pool: &SqlitePool,
) -> Result<Vec<IngresoConEstadoResponse>, String> {
    let ingresos = db::find_ingresos_abiertos(pool).await?;

    let mut responses = Vec::new();
    for ingreso in ingresos {
        let minutos = domain::calcular_tiempo_transcurrido(&ingreso.fecha_hora_ingreso)?;
        let alerta_tiempo = domain::construir_alerta_tiempo(minutos);

        let ingreso_response = IngresoResponse::from(ingreso);  // ✅ SIN mut

        responses.push(IngresoConEstadoResponse {
            ingreso: ingreso_response,
            alerta_tiempo,
        });
    }

    Ok(responses)
}

// ==========================================
// FUNCIÓN: verificar_tiempos_excedidos
// ==========================================

pub async fn verificar_tiempos_excedidos(
    pool: &SqlitePool,
) -> Result<Vec<AlertaTiempoExcedido>, String> {
    let ingresos_abiertos = db::find_ingresos_abiertos(pool).await?;
    let mut alertas = Vec::new();

    for ingreso in ingresos_abiertos {
        let minutos = domain::calcular_tiempo_transcurrido(&ingreso.fecha_hora_ingreso)?;
        let estado = domain::evaluar_estado_permanencia(minutos);

        // Solo alertar si excedió el tiempo
        if estado == EstadoPermanencia::TiempoExcedido {
            // Usar calcular_minutos_restantes (que retorna negativo si excede)
            let minutos_excedidos = -domain::calcular_minutos_restantes(minutos);  // ✅ CORREGIDO
            
            alertas.push(AlertaTiempoExcedido {
                ingreso_id: ingreso.id,
                cedula: ingreso.cedula.clone(),
                nombre_completo: format!("{} {}", ingreso.nombre, ingreso.apellido),
                empresa_nombre: ingreso.empresa_nombre,
                fecha_hora_ingreso: ingreso.fecha_hora_ingreso,
                minutos_transcurridos: minutos,
                minutos_excedidos,
                estado,
            });
        }
    }

    Ok(alertas)
}
/// Verifica si hay contratistas próximos al límite (alerta temprana)
/// 
/// Retorna lista de contratistas con tiempo >= 13h 30min y < 14h
pub async fn verificar_alertas_tempranas(
    pool: &SqlitePool,
) -> Result<Vec<AlertaTiempoExcedido>, String> {
    let ingresos_abiertos = db::find_ingresos_abiertos(pool).await?;
    let mut alertas = Vec::new();

    for ingreso in ingresos_abiertos {
        let minutos = domain::calcular_tiempo_transcurrido(&ingreso.fecha_hora_ingreso)?;
        let estado = domain::evaluar_estado_permanencia(minutos);

        // Solo alertar si está en alerta temprana
        if estado == EstadoPermanencia::AlertaTemprana {
            let minutos_restantes = domain::calcular_minutos_restantes(minutos);
            
            alertas.push(AlertaTiempoExcedido {
                ingreso_id: ingreso.id,
                cedula: ingreso.cedula.clone(),
                nombre_completo: format!("{} {}", ingreso.nombre, ingreso.apellido),
                empresa_nombre: ingreso.empresa_nombre,
                fecha_hora_ingreso: ingreso.fecha_hora_ingreso,
                minutos_transcurridos: minutos,
                minutos_excedidos: -minutos_restantes, // negativo = aún no excede
                estado,
            });
        }
    }

    Ok(alertas)
}

/// Verifica si un contratista fue bloqueado mientras estaba dentro
pub async fn verificar_cambio_lista_negra(
    pool: &SqlitePool,
    ingreso_id: String,
) -> Result<Option<AlertaListaNegra>, String> {
    // 1. Obtener el ingreso
    let ingreso = db::find_by_id(pool, &ingreso_id).await?;

    // 2. Verificar si está en lista negra AHORA
    let contratista_id = ingreso
        .contratista_id
        .ok_or_else(|| "Ingreso sin contratista_id asociado".to_string())?;

    let block_status = lista_negra_queries::check_if_blocked(pool, &contratista_id).await?;

    // 3. Si está bloqueado, generar alerta
    if block_status.blocked {
        return Ok(Some(AlertaListaNegra {
            ingreso_id: ingreso.id,
            cedula: ingreso.cedula,
            nombre_completo: format!("{} {}", ingreso.nombre, ingreso.apellido),
            bloqueado: true,
            motivo: block_status.motivo,
        }));
    }

    Ok(None)
}

/// Verifica cambios en lista negra para todos los ingresos abiertos
pub async fn verificar_cambios_lista_negra_masivo(
    pool: &SqlitePool,
) -> Result<Vec<AlertaListaNegra>, String> {
    let ingresos_abiertos = db::find_ingresos_abiertos(pool).await?;
    let mut alertas = Vec::new();

    for ingreso in ingresos_abiertos {
        if let Some(contratista_id) = ingreso.contratista_id {
            let block_status = lista_negra_queries::check_if_blocked(pool, &contratista_id).await?;

            if block_status.blocked {
                alertas.push(AlertaListaNegra {
                    ingreso_id: ingreso.id,
                    cedula: ingreso.cedula.clone(),
                    nombre_completo: format!("{} {}", ingreso.nombre, ingreso.apellido),
                    bloqueado: true,
                    motivo: block_status.motivo,
                });
            }
        }
    }

    Ok(alertas)
}

// ==========================================
// DASHBOARD / ESTADÍSTICAS
// ==========================================

/// Obtiene resumen de estado de todos los ingresos abiertos
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResumenPermanencias {
    pub total_adentro: usize,
    pub normal: usize,
    pub alerta_temprana: usize,
    pub tiempo_excedido: usize,
    pub bloqueados_durante_permanencia: usize,
}

pub async fn get_resumen_permanencias(pool: &SqlitePool) -> Result<ResumenPermanencias, String> {
    let ingresos = db::find_ingresos_abiertos(pool).await?;
    let total_adentro = ingresos.len();

    let mut normal = 0;
    let mut alerta_temprana = 0;
    let mut tiempo_excedido = 0;

    for ingreso in &ingresos {
        let minutos = domain::calcular_tiempo_transcurrido(&ingreso.fecha_hora_ingreso)?;
        let estado = domain::evaluar_estado_permanencia(minutos);

        match estado {
            EstadoPermanencia::Normal => normal += 1,
            EstadoPermanencia::AlertaTemprana => alerta_temprana += 1,
            EstadoPermanencia::TiempoExcedido => tiempo_excedido += 1,
        }
    }

    // Contar bloqueados
    let alertas_bloqueo = verificar_cambios_lista_negra_masivo(pool).await?;
    let bloqueados_durante_permanencia = alertas_bloqueo.len();

    Ok(ResumenPermanencias {
        total_adentro,
        normal,
        alerta_temprana,
        tiempo_excedido,
        bloqueados_durante_permanencia,
    })
}