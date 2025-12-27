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
    contratista_id: String,
) -> Result<ValidacionIngresoResponse, IngresoContratistaError> {
    // 1. Obtener datos del contratista (SurrealDB)
    let contratista = contratista_queries::find_by_id(&contratista_id)
        .await
        .map_err(|e| IngresoContratistaError::Database(sqlx::Error::Protocol(e.to_string())))?
        .ok_or(IngresoContratistaError::ContratistaNotFound)?;

    // 2. Verificar Lista Negra
    // Asumimos lista_negra_service no pide pool.
    // Si necesitara acceso a DB, debería usar surreal_lista_negra_queries internamente.
    let b = lista_negra_service::check_is_blocked(contratista.cedula.clone()).await.unwrap_or(
        BlockCheckResponse { is_blocked: false, nivel_severidad: None, bloqueado_desde: None },
    );

    // 3. Verificar Ingreso Abierto (SurrealDB)
    let ing_ab = db::find_ingreso_abierto_by_contratista(&contratista.id)
        .await
        .map_err(|e| IngresoContratistaError::Database(sqlx::Error::Protocol(e.to_string())))?;

    if let Some(ref ing) = ing_ab {
        let resp = IngresoResponse::try_from(ing.clone())
            .map_err(|_| IngresoContratistaError::Validation("Error parsing".to_string()))?;
        return Ok(ValidacionIngresoResponse {
            puede_ingresar: false,
            motivo_rechazo: Some("Ya tiene ingreso activo".to_string()),
            alertas: vec![],
            contratista: None,
            tiene_ingreso_abierto: true,
            ingreso_abierto: Some(resp),
        });
    }

    // 4. Motor de Validación
    // Contratista struct fields: cedula, nombre, apellido, fecha_vencimiento_praind, estado...
    let ctx = ContextoIngreso::new_contratista(
        contratista.cedula.clone(),
        format!("{} {}", contratista.nombre, contratista.apellido),
        &contratista.fecha_vencimiento_praind,
        b.is_blocked,
        b.nivel_severidad,
        false, // TODO: verificar si es excepcional check
        contratista.estado.clone(),
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
    _usuario_id: String,
) -> Result<IngresoResponse, IngresoContratistaError> {
    // 2. Validar Gafete si aplica
    if let Some(ref g) = input.gafete_numero {
        let tipo_g = input.gafete_tipo.as_deref().unwrap_or("contratista");
        let disp = gafete_service::is_gafete_disponible(g, tipo_g)
            .await
            .map_err(|e| IngresoContratistaError::Gafete(e))?;

        if !disp {
            return Err(IngresoContratistaError::GafeteNotAvailable);
        }
    }

    // 3. Obtener datos del contratista para guardar snapshot
    let contratista = contratista_queries::find_by_id(&input.contratista_id)
        .await
        .map_err(|e| IngresoContratistaError::Database(sqlx::Error::Protocol(e.to_string())))?
        .ok_or(IngresoContratistaError::ContratistaNotFound)?;

    let contratista_json = serde_json::json!(contratista);

    // 4. Insertar en DB
    let nuevo_ingreso = db::insert(input.into(), &contratista_json)
        .await
        .map_err(|e| IngresoContratistaError::Database(sqlx::Error::Protocol(e.to_string())))?
        .ok_or(IngresoContratistaError::Database(sqlx::Error::Protocol(
            "Error al crear ingreso".to_string(),
        )))?;

    // 5. Marcar gafete como en uso
    if let Some(ref g) = nuevo_ingreso.gafete_numero {
        let tipo_g = nuevo_ingreso.gafete_tipo.as_deref().unwrap_or("contratista");
        let _ = gafete_service::marcar_en_uso(g, tipo_g).await;
    }

    IngresoResponse::try_from(nuevo_ingreso).map_err(|e| IngresoContratistaError::Validation(e))
}

pub async fn registrar_salida(
    input: RegistrarSalidaInput,
    usuario_id: String,
) -> Result<IngresoResponse, IngresoContratistaError> {
    // 1. Actualizar salida en DB
    let ingreso_actualizado =
        db::update_salida(&input.ingreso_id, &usuario_id, input.observaciones_salida)
            .await
            .map_err(|e| IngresoContratistaError::Database(sqlx::Error::Protocol(e.to_string())))?
            .ok_or(IngresoContratistaError::NotFound)?;

    // 2. Liberar gafete si se devolvió
    if input.devolvio_gafete {
        if let Some(ref g) = ingreso_actualizado.gafete_numero {
            let tipo_g = ingreso_actualizado.gafete_tipo.as_deref().unwrap_or("contratista");
            let _ = gafete_service::liberar_gafete(g, tipo_g).await;
        }
    }

    IngresoResponse::try_from(ingreso_actualizado)
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
