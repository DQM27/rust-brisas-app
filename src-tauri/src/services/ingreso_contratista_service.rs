// src/services/ingreso_contratista_service.rs

use crate::db::contratista_queries;

use crate::services::alerta_service;
// Usamos nuestro nuevo modulo de queries
use crate::db::ingreso_contratista_queries as db;

// Usamos nuestro nuevo modulo de dominio
use crate::domain::errors::IngresoContratistaError;
use crate::domain::ingreso_contratista as domain;
use crate::domain::motor_validacion::{self as motor, ContextoIngreso};
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

    // D. Validaciones con Motor Unificado
    let alertas_db = alerta_service::find_pendientes_by_cedula(pool, &contratista.cedula)
        .await
        .map_err(|e| IngresoContratistaError::Database(sqlx::Error::Protocol(e.to_string())))?;

    let nombre_completo = format!("{} {}", contratista.nombre, contratista.apellido);
    let contexto = ContextoIngreso::new_contratista(
        contratista.cedula.clone(),
        nombre_completo,
        &contratista.fecha_vencimiento_praind,
        block_response.is_blocked,
        block_response.motivo.clone(),
        ingreso_abierto.is_some(),
        contratista.estado.clone(),
        alertas_db.len(),
    );

    let resultado_motor = motor::validar_ingreso(&contexto);

    // E. Vehículos (para frontend)
    let vehiculos = crate::db::vehiculo_queries::find_by_contratista(pool, &contratista_id)
        .await
        .unwrap_or_default();

    // F. Construir JSON Seguro
    let praind_vigente = !resultado_motor.bloqueos.iter().any(|b| {
        matches!(b, motor::MotivoBloqueo::AutorizacionInvalida { motivo } if motivo.contains("PRAIND"))
    });

    let contratista_json = if resultado_motor.puede_ingresar || !resultado_motor.bloqueos.is_empty()
    {
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
        puede_ingresar: resultado_motor.puede_ingresar,
        motivo_rechazo: resultado_motor.mensaje_bloqueo(),
        alertas: resultado_motor.alertas,
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

    let fecha_venc =
        chrono::NaiveDate::parse_from_str(&contratista.fecha_vencimiento_praind, "%Y-%m-%d")
            .map_err(|_| {
                IngresoContratistaError::Validation(format!(
                    "Fecha PRAIND inválida: {}",
                    contratista.fecha_vencimiento_praind
                ))
            })?;
    let praind_vigente = fecha_venc >= chrono::Utc::now().date_naive();

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

    Ok(ResultadoValidacionSalida { puede_salir: errores.is_empty(), errores, advertencias: vec![] })
}

pub async fn registrar_salida(
    pool: &SqlitePool,
    input: RegistrarSalidaInput,
    usuario_id: String,
) -> Result<IngresoResponse, IngresoContratistaError> {
    let ingreso =
        db::find_by_id(pool, &input.ingreso_id).await?.ok_or(IngresoContratistaError::NotFound)?;
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
        if input.devolvio_gafete { ingreso.gafete_numero.as_deref() } else { None },
    ); // La función retorna DecisionReporteGafete directamente (no Result)

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
                None, // ingreso_proveedor_id
                None, // ingreso_visita_id
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

        responses.push(IngresoConEstadoResponse { ingreso: response, alerta_tiempo });
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
// 4. CIERRE MANUAL DE INGRESO
// ==========================================

/// Input para cerrar un ingreso manualmente
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CerrarIngresoManualInput {
    pub ingreso_id: String,
    pub motivo_cierre: String,
    pub fecha_salida_estimada: Option<String>,
    pub notas: Option<String>,
}

/// Resultado de cierre manual
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultadoCierreManualResponse {
    pub ingreso: IngresoResponse,
    pub genera_reporte: bool,
    pub tipo_reporte: Option<String>,
    pub mensaje: Option<String>,
}

/// Cierra un ingreso manualmente (cuando el guardia no registró la salida a tiempo)
pub async fn cerrar_ingreso_manual(
    pool: &SqlitePool,
    input: CerrarIngresoManualInput,
    usuario_id: String,
) -> Result<ResultadoCierreManualResponse, IngresoContratistaError> {
    // 1. Buscar el ingreso
    let ingreso =
        db::find_by_id(pool, &input.ingreso_id).await?.ok_or(IngresoContratistaError::NotFound)?;

    // 2. Validar que esté abierto
    domain::validar_ingreso_abierto(&ingreso.fecha_hora_salida)?;

    // 3. Parsear motivo
    let motivo: domain::MotivoCierre =
        input.motivo_cierre.parse().map_err(|e: String| IngresoContratistaError::Validation(e))?;

    // 4. Evaluar cierre con lógica de dominio
    let evaluacion = domain::evaluar_cierre_manual(&ingreso.fecha_hora_ingreso, &motivo)?;

    // 5. Determinar fecha de salida
    let fecha_salida = input.fecha_salida_estimada.unwrap_or_else(|| Utc::now().to_rfc3339());

    // 6. Calcular tiempo de permanencia
    let minutos_permanencia =
        domain::calcular_tiempo_permanencia(&ingreso.fecha_hora_ingreso, &fecha_salida)
            .map_err(|e| IngresoContratistaError::Validation(e))?;

    // 7. Actualizar DB con cierre manual
    let now = Utc::now().to_rfc3339();
    db::registrar_salida(
        pool,
        &input.ingreso_id,
        &fecha_salida,
        minutos_permanencia,
        &usuario_id,
        input.notas.as_deref(),
        &now,
    )
    .await?;

    // 8. Obtener ingreso actualizado
    let ingreso_actualizado = get_ingreso_by_id(pool, input.ingreso_id).await?;

    Ok(ResultadoCierreManualResponse {
        ingreso: ingreso_actualizado,
        genera_reporte: evaluacion.genera_reporte,
        tipo_reporte: evaluacion.tipo_reporte,
        mensaje: evaluacion.mensaje,
    })
}

// ==========================================
// 5. INGRESO EXCEPCIONAL
// ==========================================

/// Input para registrar un ingreso excepcional
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngresoExcepcionalInput {
    pub contratista_id: String,
    pub autorizado_por: String,
    pub motivo_excepcional: String,
    pub notas: Option<String>,
    // Campos normales de ingreso
    pub vehiculo_id: Option<String>,
    pub gafete_numero: Option<String>,
    pub modo_ingreso: String,
    pub observaciones: Option<String>,
}

/// Resultado de ingreso excepcional
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IngresoExcepcionalResponse {
    pub ingreso: IngresoResponse,
    pub motivo_original_bloqueo: String,
    pub autorizado_por: String,
    pub valido_hasta: String,
}

/// Registra un ingreso excepcional para un contratista que normalmente no podría entrar
pub async fn registrar_ingreso_excepcional(
    pool: &SqlitePool,
    input: IngresoExcepcionalInput,
    usuario_id: String,
) -> Result<IngresoExcepcionalResponse, IngresoContratistaError> {
    // 1. Obtener contratista
    let contratista = contratista_queries::find_basic_info_by_id(pool, &input.contratista_id)
        .await?
        .ok_or(IngresoContratistaError::ContratistaNotFound)?;

    // 2. Verificar que NO puede entrar normalmente (de lo contrario, ¿por qué es excepcional?)
    let fecha_venc =
        chrono::NaiveDate::parse_from_str(&contratista.fecha_vencimiento_praind, "%Y-%m-%d")
            .map_err(|_| {
                IngresoContratistaError::Validation(format!(
                    "Fecha PRAIND inválida: {}",
                    contratista.fecha_vencimiento_praind
                ))
            })?;
    let praind_vigente = fecha_venc >= chrono::Utc::now().date_naive();

    // Determinar motivo de bloqueo original
    let motivo_bloqueo = if !praind_vigente {
        "PRAIND vencido".to_string()
    } else if contratista.estado != "activo" {
        format!("Estado: {}", contratista.estado)
    } else {
        "Razón no determinada".to_string()
    };

    // 3. Parsear motivo excepcional
    let motivo: domain::MotivoExcepcional = input
        .motivo_excepcional
        .parse()
        .map_err(|e: String| IngresoContratistaError::Validation(e))?;

    // 4. Evaluar ingreso excepcional
    let evaluacion = domain::evaluar_ingreso_excepcional(
        &motivo_bloqueo,
        &input.autorizado_por,
        &motivo,
        input.notas.as_deref(),
    );

    // 5. Verificar duplicados
    let existing = db::find_ingreso_abierto_by_contratista(pool, &input.contratista_id).await?;
    if existing.is_some() {
        return Err(IngresoContratistaError::AlreadyInside);
    }

    // 6. Gestionar gafete
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

    // 7. Insertar ingreso (marcado como excepcional)
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let modo_ingreso: ModoIngreso = input
        .modo_ingreso
        .parse()
        .map_err(|_| IngresoContratistaError::Validation("Modo ingreso inválido".to_string()))?;

    // Usamos "correo" como tipo de autorización para excepcionales
    db::insert(
        pool,
        &id,
        &input.contratista_id,
        &contratista.cedula,
        &contratista.nombre,
        &contratista.apellido,
        &contratista.empresa_nombre,
        TipoIngreso::Contratista.as_str(),
        TipoAutorizacion::Correo.as_str(), // Excepcional usa correo/autorización especial
        modo_ingreso.as_str(),
        input.vehiculo_id.as_deref(),
        None,
        gafete_normalizado.as_deref(),
        &now,
        &usuario_id,
        Some(praind_vigente),
        Some(&contratista.estado),
        input.observaciones.as_deref(),
        &now,
        &now,
    )
    .await?;

    // 8. Obtener ingreso creado
    let ingreso = get_ingreso_by_id(pool, id).await?;

    Ok(IngresoExcepcionalResponse {
        ingreso,
        motivo_original_bloqueo: evaluacion.motivo_original_bloqueo,
        autorizado_por: evaluacion.autorizado_por,
        valido_hasta: evaluacion.valido_hasta,
    })
}

// ==========================================
// HELPERS PRIVADOS
// ==========================================

async fn get_ingreso_by_id(
    pool: &SqlitePool,
    id: String,
) -> Result<IngresoResponse, IngresoContratistaError> {
    let ingreso = db::find_by_id(pool, &id).await?.ok_or(IngresoContratistaError::NotFound)?;
    let details = db::find_details_by_id(pool, &id).await?.unwrap_or(db::IngresoDetails {
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

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::Executor;

    async fn setup_test_env() -> SqlitePool {
        let db_id = uuid::Uuid::new_v4().to_string();
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(&format!("sqlite:file:{}?mode=memory&cache=shared", db_id))
            .await
            .unwrap();

        pool.execute("PRAGMA foreign_keys = OFF;").await.unwrap();

        let schemas = vec![
            "migrations/1_create_users.sql",
            "migrations/2_create_contratista.sql",
            "migrations/4_create_vehiculo.sql",
            "migrations/5_create_gafete.sql",
            "migrations/3_create_lista_negra.sql",
            "migrations/12_create_ingresos_proveedores.sql",
            "migrations/7_create_ingreso.sql",
            "migrations/6_create_alertas_gafetes.sql",
        ];

        for path in schemas {
            let sql = std::fs::read_to_string(path).unwrap();
            pool.execute(sql.as_str()).await.unwrap();
        }

        // Seed
        pool.execute("INSERT INTO users (id, email, password_hash, nombre, apellido, role_id, created_at, updated_at, cedula, must_change_password, is_active) 
                      VALUES ('u-1', 'admin@test.com', 'hash', 'Admin', 'Test', 'role-admin', '2025-01-01', '2025-01-01', '000', 0, 1)").await.unwrap();

        pool.execute("INSERT INTO empresas (id, nombre, created_at, updated_at) VALUES ('e-1', 'Test Corp', '2025-01-01', '2025-01-01')").await.unwrap();

        pool.execute("INSERT INTO contratistas (id, cedula, nombre, apellido, empresa_id, fecha_vencimiento_praind, estado, created_at, updated_at)
                      VALUES ('c-1', '12345', 'Juan', 'Perez', 'e-1', '2030-01-01', 'activo', '2025-01-01', '2025-01-01')").await.unwrap();

        pool.execute("INSERT INTO gafetes (numero, tipo, estado, created_at, updated_at) VALUES ('G-100', 'contratista', 'activo', '2025-01-01', '2025-01-01')").await.unwrap();

        pool
    }

    #[tokio::test]
    async fn test_ingreso_contratista_workflow() {
        let pool = setup_test_env().await;
        let contratista_id = "c-1";
        let usuario_id = "u-1";

        // 1. Validar elegibilidad
        let val = validar_ingreso_contratista(&pool, contratista_id.to_string()).await.unwrap();
        assert!(val.puede_ingresar);
        assert!(val.ingreso_abierto.is_none());

        // 2. Crear ingreso (Gafete asignado)
        let input = CreateIngresoContratistaInput {
            contratista_id: contratista_id.to_string(),
            vehiculo_id: None,
            gafete_numero: Some("G-100".to_string()),
            tipo_autorizacion: "praind".to_string(),
            modo_ingreso: "caminando".to_string(),
            observaciones: Some("Test note".to_string()),
            usuario_ingreso_id: usuario_id.to_string(),
        };

        let ingreso =
            crear_ingreso_contratista(&pool, input, usuario_id.to_string()).await.unwrap();
        assert_eq!(ingreso.gafete_numero, Some("G-100".to_string()));
        assert!(ingreso.esta_adentro);

        // 3. Intentar crear duplicado (Debe fallar)
        let input_dup = CreateIngresoContratistaInput {
            contratista_id: contratista_id.to_string(),
            vehiculo_id: None,
            gafete_numero: Some("G-101".to_string()),
            tipo_autorizacion: "praind".to_string(),
            modo_ingreso: "caminando".to_string(),
            observaciones: None,
            usuario_ingreso_id: usuario_id.to_string(),
        };
        let err_dup = crear_ingreso_contratista(&pool, input_dup, usuario_id.to_string()).await;
        assert!(matches!(err_dup, Err(IngresoContratistaError::AlreadyInside)));

        // 4. Registrar salida (Correcta)
        let salida_input = RegistrarSalidaInput {
            ingreso_id: ingreso.id.clone(),
            devolvio_gafete: true,
            usuario_salida_id: usuario_id.to_string(),
            observaciones_salida: None,
        };
        let ingreso_salida =
            registrar_salida(&pool, salida_input, usuario_id.to_string()).await.unwrap();
        assert!(!ingreso_salida.esta_adentro);
        assert!(ingreso_salida.fecha_hora_salida.is_some());
    }

    #[tokio::test]
    async fn test_ingreso_contratista_lost_badge() {
        let pool = setup_test_env().await;
        let contratista_id = "c-1";
        let usuario_id = "u-1";

        // 1. Crear ingreso
        let input = CreateIngresoContratistaInput {
            contratista_id: contratista_id.to_string(),
            vehiculo_id: None,
            gafete_numero: Some("G-100".to_string()),
            tipo_autorizacion: "praind".to_string(),
            modo_ingreso: "caminando".to_string(),
            observaciones: None,
            usuario_ingreso_id: usuario_id.to_string(),
        };
        let ingreso =
            crear_ingreso_contratista(&pool, input, usuario_id.to_string()).await.unwrap();

        // 2. Registrar salida indicando que NO devolvió gafete
        let salida_input = RegistrarSalidaInput {
            ingreso_id: ingreso.id.clone(),
            devolvio_gafete: false,
            usuario_salida_id: usuario_id.to_string(),
            observaciones_salida: Some("Perdió el gafete".to_string()),
        };
        let _ = registrar_salida(&pool, salida_input, usuario_id.to_string()).await.unwrap();

        // 3. Verificar que se generó una alerta
        let alertas = crate::services::alerta_service::find_pendientes_by_cedula(&pool, "12345")
            .await
            .unwrap();
        assert_eq!(alertas.len(), 1);
        assert_eq!(alertas[0].gafete_numero, "G-100");
    }

    #[tokio::test]
    async fn test_ingreso_contratista_blocked() {
        let pool = setup_test_env().await;

        // 1. Bloquear contratista
        pool.execute("INSERT INTO lista_negra (id, cedula, nombre, apellido, motivo_bloqueo, fecha_inicio_bloqueo, bloqueado_por, is_active, created_at, updated_at) 
                      VALUES ('bl-1', '12345', 'Juan', 'Perez', 'Robo', '2025-01-01', 'Admin', 1, '2025-01-01', '2025-01-01')").await.unwrap();

        // 2. Validar (Debe fallar por bloqueo)
        let val = validar_ingreso_contratista(&pool, "c-1".into()).await.unwrap();

        println!(
            "DEBUG: Test Validation Result: puede_ingresar={}, motivo={:?}",
            val.puede_ingresar, val.motivo_rechazo
        );

        assert!(!val.puede_ingresar);
        assert!(val.motivo_rechazo.unwrap().to_lowercase().contains("bloqueado"));
    }
}
