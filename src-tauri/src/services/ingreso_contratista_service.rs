// src/services/ingreso_contratista_service.rs
use crate::domain::errors::IngresoContratistaError;
use crate::models::ingreso::{
    CreateIngresoContratistaInput, IngresoResponse, RegistrarSalidaInput, ValidacionIngresoResponse,
};
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
// FUNCIONES DE SERVICIO (STUBS)
// ==========================================

pub async fn validar_ingreso_contratista(
    _contratista_id: String,
) -> Result<ValidacionIngresoResponse, IngresoContratistaError> {
    Err(IngresoContratistaError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

pub async fn crear_ingreso_contratista(
    _input: CreateIngresoContratistaInput,
    _usuario_id: String,
) -> Result<IngresoResponse, IngresoContratistaError> {
    Err(IngresoContratistaError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

pub async fn registrar_salida(
    _input: RegistrarSalidaInput,
    _usuario_id: String,
) -> Result<IngresoResponse, IngresoContratistaError> {
    Err(IngresoContratistaError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

pub async fn validar_puede_salir(
    _ingreso_id: &str,
    _gafete: Option<&str>,
) -> Result<ResultadoValidacionSalida, String> {
    Err("No implementado".to_string())
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
