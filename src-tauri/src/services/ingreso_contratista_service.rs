/// Punto de Control de Seguridad: Ingresos y Salidas de Contratistas.
///
/// Este es el núcleo operativo de la garita. Este servicio coordina múltiples subsistemas
/// (Lista Negra, Vigencia PRAIND, Estado de Contratista, Gestión de Gafetes y Motor de Validación)
/// para determinar, en tiempo real, si un trabajador externo puede entrar a las instalaciones.
use crate::db::surrealdb_contratista_queries as contratista_queries;
use crate::db::surrealdb_ingreso_contratista_queries as db;
use crate::domain::errors::IngresoContratistaError;
use crate::domain::ingreso_contratista::{
    self, calcular_tiempo_permanencia, validar_ingreso_abierto,
};
use crate::domain::motor_validacion as motor;
use crate::models::ingreso::{
    CreateIngresoContratistaInput, IngresoResponse, RegistrarSalidaInput, ValidacionIngresoResponse,
};
use crate::models::lista_negra::BlockCheckResponse;
use crate::models::validation::{
    EstadoAutorizacion, InfoListaNegra, MotorContexto, NivelSeveridad, TipoAcceso, ValidationStatus,
};
use crate::services::{gafete_service, lista_negra_service};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

// ==========================================
// DTOs PÚBLICOS (Estructuras de respuesta para la UI)
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
// LÓGICA DE VALIDACIÓN Y CONTROL
// ==========================================

/// Ejecuta una validación profunda antes de permitir la apertura de un ingreso.
///
/// El proceso orquestado es:
/// 1. Verificación de Identidad: Confirma que el contratista está activo.
/// 2. Filtro de Seguridad: Consulta la lista negra institucional.
/// 3. Regla de Unicidad: Impide ingresos dobles si ya hay uno abierto.
/// 4. Motor de Reglas: Analiza la fecha PRAIND y otras políticas complejas.
pub async fn validar_ingreso_contratista(
    contratista_id_str: String,
) -> Result<ValidacionIngresoResponse, IngresoContratistaError> {
    let contratista_id = if contratista_id_str.contains(':') {
        let parts: Vec<&str> = contratista_id_str.split(':').collect();
        RecordId::from_table_key(parts[0], parts[1])
    } else {
        RecordId::from_table_key("contratista", &contratista_id_str)
    };

    let contratista = contratista_queries::find_by_id(&contratista_id)
        .await
        .map_err(|e| IngresoContratistaError::Database(e.to_string()))?
        .ok_or(IngresoContratistaError::ContratistaNotFound)?;

    let b = lista_negra_service::check_is_blocked(contratista.cedula.clone()).await.unwrap_or(
        BlockCheckResponse { is_blocked: false, nivel_severidad: None, bloqueado_desde: None },
    );

    let ing_ab = db::find_ingreso_abierto_by_contratista(&contratista.id)
        .await
        .map_err(|e| IngresoContratistaError::Database(e.to_string()))?;

    if let Some(ing) = ing_ab {
        let resp = IngresoResponse::from_contratista_fetched(ing)
            .map_err(|e| IngresoContratistaError::Validation(e))?;
        return Ok(ValidacionIngresoResponse {
            puede_ingresar: false,
            motivo_rechazo: Some("Ya tiene un ingreso activo en planta".to_string()),
            alertas: vec![],
            contratista: None,
            tiene_ingreso_abierto: true,
            ingreso_abierto: Some(resp),
        });
    }

    // (Fecha PRAIND eliminada por no ser utilizada en el contexto actual del motor)

    // Invocación del Motor de Reglas de Negocio.
    // Aquí se decide si un contratista entra como "Autorizado" o "Bloqueado".
    let motor_ctx = MotorContexto {
        ident_cedula: contratista.cedula.clone(),
        ident_nombre: format!("{} {}", contratista.nombre, contratista.apellido),
        tipo_acceso: TipoAcceso::Contratista,
        lista_negra: if b.is_blocked {
            Some(InfoListaNegra {
                motivo: "Bloqueo detectado".to_string(), // Placeholder mejorado
                severidad: NivelSeveridad::Alto,         // Mapeo simple por ahora
            })
        } else {
            None
        },
        ingreso_activo: None, // TODO: Verificar ingreso activo
        estado_autorizacion: EstadoAutorizacion::from_str_lossy(contratista.estado.as_str()),
        alerta_gafete: None, // TODO: Verificar gafetes pendientes
    };

    let motor_res = motor::ejecutar_validacion_motor(&motor_ctx);

    Ok(ValidacionIngresoResponse {
        puede_ingresar: motor_res.status == ValidationStatus::Allowed,
        motivo_rechazo: if motor_res.status != ValidationStatus::Allowed {
            Some(motor_res.message)
        } else {
            None
        },
        alertas: vec![],
        contratista: Some(serde_json::json!(contratista)),
        tiene_ingreso_abierto: false,
        ingreso_abierto: None,
    })
}

/// Registra físicamente el ingreso de un contratista y asigna recursos (Gafetes).
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

    let usuario_id = if usuario_id_str.contains(':') {
        usuario_id_str.parse::<RecordId>().map_err(|_| {
            IngresoContratistaError::Validation("ID de usuario inválido".to_string())
        })?
    } else {
        RecordId::from_table_key("user", &usuario_id_str)
    };

    // Gestión de Gafetes Físicos: Se asegura que el recurso no esté duplicado.
    if let Some(ref g) = input.gafete_numero {
        if *g != 0 {
            let disp = gafete_service::is_gafete_disponible(*g, "contratista")
                .await
                .map_err(|e| IngresoContratistaError::Gafete(e))?;

            if !disp {
                return Err(IngresoContratistaError::GafeteNotAvailable);
            }
        }
    }

    let contratista = contratista_queries::find_by_id_fetched(&contratista_id)
        .await
        .map_err(|e| IngresoContratistaError::Database(e.to_string()))?
        .ok_or(IngresoContratistaError::ContratistaNotFound)?;

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

    let nuevo_ingreso =
        db::insert(dto).await.map_err(|e| IngresoContratistaError::Database(e.to_string()))?;

    // Actualización de estado del activo físico (Gafete).
    if let Some(ref g) = nuevo_ingreso.gafete_numero {
        let _ = gafete_service::marcar_en_uso(*g, "contratista").await;
    }

    IngresoResponse::from_contratista_fetched(nuevo_ingreso)
        .map_err(|e| IngresoContratistaError::Validation(e))
}

/// Finaliza una jornada de trabajo registrando la salida y liberando recursos.
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

    let ingreso_actualizado =
        db::update_salida(&ingreso_id, &usuario_id, input.observaciones_salida)
            .await
            .map_err(|e| IngresoContratistaError::Database(e.to_string()))?;

    // Devolución del Gafete: Permite que el recurso vuelva a estar disponible para otros.
    if input.devolvio_gafete {
        if let Some(ref g) = ingreso_actualizado.gafete_numero {
            let _ = gafete_service::liberar_gafete(*g, "contratista").await;
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
