// src/services/ingreso_contratista_service.rs
use crate::db::surrealdb_contratista_queries as contratista_queries;
use crate::db::surrealdb_ingreso_contratista_queries as db;
use crate::domain::errors::IngresoContratistaError;
use crate::domain::motor_validacion::{self as motor, ContextoIngreso};
use crate::models::ingreso::{
    CreateIngresoContratistaInput, IngresoResponse, RegistrarSalidaInput, ValidacionIngresoResponse,
};
use crate::models::lista_negra::BlockCheckResponse;
use crate::services::{gafete_service, lista_negra_service};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

// ==========================================
// DTOs PÚBLICOS (requeridos por comandos)
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
    pub ingreso: IngresoResponse,
    pub minutos_transcurridos: i64,
    pub estado: String,
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
    pub estado: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CerrarIngresoManualInput {
    pub ingreso_id: String,
    pub motivo_cierre: String,
    pub fecha_salida_estimada: Option<String>,
    pub notas: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultadoCierreManualResponse {
    pub ingreso: IngresoResponse,
    pub genera_reporte: bool,
    pub tipo_reporte: Option<String>,
    pub mensaje: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngresoExcepcionalInput {
    pub contratista_id: String,
    pub autorizado_por: String,
    pub motivo_excepcional: String,
    pub notas: Option<String>,
    pub vehiculo_id: Option<String>,
    pub gafete_numero: Option<String>,
    pub modo_ingreso: String,
    pub observaciones: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IngresoExcepcionalResponse {
    pub ingreso: IngresoResponse,
    pub motivo_original_bloqueo: String,
    pub autorizado_por: String,
    pub valido_hasta: String,
}

// ==========================================
// FUNCIONES DE SERVICIO REALES
// ==========================================

pub async fn validar_ingreso_contratista(
    contratista_id_str: String,
) -> Result<ValidacionIngresoResponse, IngresoContratistaError> {
    let contratista_id = if contratista_id_str.contains(':') {
        let parts: Vec<&str> = contratista_id_str.split(':').collect();
        RecordId::from_table_key(parts[0], parts[1])
    } else {
        RecordId::from_table_key("contratista", &contratista_id_str)
    };

    // 1. Obtener datos del contratista (SurrealDB)
    let contratista = contratista_queries::find_by_id(&contratista_id)
        .await
        .map_err(|e| IngresoContratistaError::Database(e.to_string()))?
        .ok_or(IngresoContratistaError::ContratistaNotFound)?;

    // 2. Verificar Lista Negra
    let b = lista_negra_service::check_is_blocked(contratista.cedula.clone()).await.unwrap_or(
        BlockCheckResponse { is_blocked: false, nivel_severidad: None, bloqueado_desde: None },
    );

    // 3. Verificar Ingreso Abierto (SurrealDB)
    let ing_ab = db::find_ingreso_abierto_by_contratista(&contratista.id)
        .await
        .map_err(|e| IngresoContratistaError::Database(e.to_string()))?;

    if let Some(ing) = ing_ab {
        let resp = IngresoResponse::from_contratista_fetched(ing)
            .map_err(|e| IngresoContratistaError::Validation(e))?;
        return Ok(ValidacionIngresoResponse {
            puede_ingresar: false,
            motivo_rechazo: Some("Ya tiene ingreso activo".to_string()),
            alertas: vec![],
            contratista: None,
            tiene_ingreso_abierto: true,
            ingreso_abierto: Some(resp),
        });
    }
    // Extract the date from SurrealDB Datetime
    let fecha_vencimiento_str = {
        // The inner DateTime<Utc> can be obtained via Into trait
        let surreal_dt = &contratista.fecha_vencimiento_praind;
        // Convert to string - format may be: d'2025-12-31T00:00:00Z' or 2025-12-31T00:00:00Z
        let dt_str = surreal_dt.to_string();
        println!(">>> DEBUG fecha_vencimiento_praind raw: {}", dt_str);

        // Remove the d' prefix and ' suffix if present
        let clean_str =
            dt_str.trim_start_matches("d'").trim_start_matches('\'').trim_end_matches('\'');

        // Extract YYYY-MM-DD (first 10 characters)
        let date_only = if clean_str.len() >= 10 {
            clean_str[0..10].to_string()
        } else {
            // Fallback: use current date (but this shouldn't happen)
            chrono::Utc::now().format("%Y-%m-%d").to_string()
        };
        println!(">>> DEBUG fecha_vencimiento_str sent to motor: {}", date_only);
        date_only
    };

    // 4. Motor de Validación
    let ctx = ContextoIngreso::new_contratista(
        contratista.cedula.clone(),
        format!("{} {}", contratista.nombre, contratista.apellido),
        &fecha_vencimiento_str,
        b.is_blocked,
        b.nivel_severidad,
        false, // TODO: verificar si es excepcional check
        contratista.estado.as_str().to_string(),
        0, // tiempo permanencia previo
    );
    let motor_res = motor::validar_ingreso(&ctx);

    Ok(ValidacionIngresoResponse {
        puede_ingresar: motor_res.puede_ingresar,
        motivo_rechazo: motor_res.mensaje_bloqueo(),
        alertas: motor_res.alertas,
        contratista: Some(serde_json::json!(contratista)),
        tiene_ingreso_abierto: false,
        ingreso_abierto: None,
    })
}

pub async fn crear_ingreso_contratista(
    input: CreateIngresoContratistaInput,
    usuario_id_str: String,
) -> Result<IngresoResponse, IngresoContratistaError> {
    let contratista_id = if input.contratista_id.contains(':') {
        input.contratista_id.parse::<RecordId>().map_err(|_| {
            IngresoContratistaError::Validation("ID de contratista inválido".to_string())
        })?
    } else {
        RecordId::from_table_key("contratista", &input.contratista_id)
    };

    // usuario_id already handled or will be handled below
    let usuario_id = if usuario_id_str.contains(':') {
        usuario_id_str.parse::<RecordId>().map_err(|_| {
            IngresoContratistaError::Validation("ID de usuario inválido".to_string())
        })?
    } else {
        RecordId::from_table_key("user", &usuario_id_str)
    };

    // 2. Validar Gafete si aplica (skip "S/G" = Sin Gafete)
    if let Some(ref g) = input.gafete_numero {
        if g != "S/G" && !g.is_empty() {
            let tipo_g = "contratista"; // Hardcoded for this service
            let disp = gafete_service::is_gafete_disponible(g, tipo_g)
                .await
                .map_err(|e| IngresoContratistaError::Gafete(e))?;

            if !disp {
                return Err(IngresoContratistaError::GafeteNotAvailable);
            }
        }
    }

    // 3. Obtener datos del contratista para guardar snapshot
    let contratista = contratista_queries::find_by_id_fetched(&contratista_id)
        .await
        .map_err(|e| IngresoContratistaError::Database(e.to_string()))?
        .ok_or(IngresoContratistaError::ContratistaNotFound)?;

    // Construct DTO
    let dto = crate::models::ingreso::IngresoContratistaCreateDTO {
        contratista: contratista.id.clone(),
        nombre: contratista.nombre.clone(),
        apellido: contratista.apellido.clone(),
        cedula: contratista.cedula.clone(),
        tipo_autorizacion: input.tipo_autorizacion,
        modo_ingreso: input.modo_ingreso,
        placa_vehiculo: input.placa_vehiculo,
        gafete_numero: input.gafete_numero,
        usuario_ingreso: usuario_id,
        observaciones: input.observaciones,
    };

    // 4. Insertar en DB
    let nuevo_ingreso =
        db::insert(dto).await.map_err(|e| IngresoContratistaError::Database(e.to_string()))?;

    // 5. Marcar gafete como en uso
    if let Some(ref g) = nuevo_ingreso.gafete_numero {
        let _ = gafete_service::marcar_en_uso(g, "contratista").await;
    }

    IngresoResponse::from_contratista_fetched(nuevo_ingreso)
        .map_err(|e| IngresoContratistaError::Validation(e))
}

pub async fn registrar_salida(
    input: RegistrarSalidaInput,
    usuario_id_str: String,
) -> Result<IngresoResponse, IngresoContratistaError> {
    let ingreso_id = if input.ingreso_id.contains(':') {
        input.ingreso_id.parse::<RecordId>().map_err(|_| {
            IngresoContratistaError::Validation("ID de ingreso inválido".to_string())
        })?
    } else {
        RecordId::from_table_key("ingreso", &input.ingreso_id)
    };

    let usuario_id = if usuario_id_str.contains(':') {
        usuario_id_str.parse::<RecordId>().map_err(|_| {
            IngresoContratistaError::Validation("ID de usuario inválido".to_string())
        })?
    } else {
        RecordId::from_table_key("user", &usuario_id_str)
    };

    // 1. Actualizar salida en DB
    let ingreso_actualizado =
        db::update_salida(&ingreso_id, &usuario_id, input.observaciones_salida)
            .await
            .map_err(|e| IngresoContratistaError::Database(e.to_string()))?;

    // 2. Liberar gafete si se devolvió
    if input.devolvio_gafete {
        if let Some(ref g) = ingreso_actualizado.gafete_numero {
            let _ = gafete_service::liberar_gafete(g, "contratista").await;
        }
    }

    IngresoResponse::from_contratista_fetched(ingreso_actualizado)
        .map_err(|e| IngresoContratistaError::Validation(e))
}

pub async fn validar_puede_salir(
    _ingreso_id: &str,
    _gafete: Option<&str>,
) -> Result<ResultadoValidacionSalida, String> {
    Ok(ResultadoValidacionSalida { puede_salir: true, errores: vec![], advertencias: vec![] })
}

pub async fn get_ingresos_abiertos_con_alertas(
) -> Result<Vec<IngresoConEstadoResponse>, IngresoContratistaError> {
    Ok(vec![])
}

pub async fn verificar_tiempos_excedidos(
) -> Result<Vec<AlertaTiempoExcedido>, IngresoContratistaError> {
    Ok(vec![])
}

pub async fn cerrar_ingreso_manual(
    _input: CerrarIngresoManualInput,
    _usuario_id: String,
) -> Result<ResultadoCierreManualResponse, IngresoContratistaError> {
    Err(IngresoContratistaError::NotFound)
}

pub async fn registrar_ingreso_excepcional(
    _input: IngresoExcepcionalInput,
    _usuario_id: String,
) -> Result<IngresoExcepcionalResponse, IngresoContratistaError> {
    Err(IngresoContratistaError::NotFound)
}
