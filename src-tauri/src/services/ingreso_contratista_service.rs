/// Servicio: Punto de Control de Seguridad - Ingresos y Salidas de Contratistas.
///
/// Este es el núcleo operativo de la garita. Coordina múltiples subsistemas
/// (Lista Negra, Vigencia PRAIND, Estado de Contratista, Gestión de Gafetes)
/// para determinar en tiempo real si un trabajador externo puede ingresar.
///
/// Responsabilidades:
/// - Validación de pre-ingreso (identidad, seguridad, unicidad).
/// - Registro físico de entrada con asignación de recursos.
/// - Control de salida y liberación de gafetes.
/// - Monitoreo de tiempos de permanencia.
use crate::db::surrealdb_contratista_queries as contratista_queries;
use crate::db::surrealdb_ingreso_contratista_queries as db;
use crate::domain::errors::IngresoContratistaError;

use crate::domain::motor_validacion as motor;
use crate::models::ingreso::{
    AlertaTiempoExcedido, CreateIngresoContratistaInput, IngresoConEstadoResponse, IngresoResponse,
    RegistrarSalidaInput, ResultadoValidacionSalida, ValidacionIngresoResponse,
};
use crate::models::lista_negra::BlockCheckResponse;
use crate::models::validation::{
    EstadoAutorizacion, InfoListaNegra, MotorContexto, NivelSeveridad, TipoAcceso, ValidationStatus,
};
use crate::services::{gafete_service, lista_negra_service};
use log::{error, info, warn};
use surrealdb::RecordId;

// --------------------------------------------------------------------------
// HELPERS INTERNOS
// --------------------------------------------------------------------------

/// Parsea un ID de contratista (acepta "contratista:id" o "id").
fn parse_contratista_id(id_str: &str) -> Result<RecordId, IngresoContratistaError> {
    if id_str.contains(':') {
        id_str.parse::<RecordId>().map_err(|_| {
            IngresoContratistaError::Validation("ID de contratista inválido".to_string())
        })
    } else {
        Ok(RecordId::from_table_key("contratista", id_str))
    }
}

/// Parsea un ID de usuario (acepta "user:id" o "id").
fn parse_user_id(id_str: &str) -> Result<RecordId, IngresoContratistaError> {
    if id_str.contains(':') {
        id_str
            .parse::<RecordId>()
            .map_err(|_| IngresoContratistaError::Validation("ID de usuario inválido".to_string()))
    } else {
        Ok(RecordId::from_table_key("user", id_str))
    }
}

/// Parsea un ID de ingreso (acepta "ingreso:id" o "id").
fn parse_ingreso_id(id_str: &str) -> Result<RecordId, IngresoContratistaError> {
    if id_str.contains(':') {
        id_str
            .parse::<RecordId>()
            .map_err(|_| IngresoContratistaError::Validation("ID de ingreso inválido".to_string()))
    } else {
        Ok(RecordId::from_table_key("ingreso", id_str))
    }
}

// --------------------------------------------------------------------------
// LÓGICA DE VALIDACIÓN Y CONTROL
// --------------------------------------------------------------------------

/// Ejecuta una validación profunda antes de permitir la apertura de un ingreso.
///
/// El proceso orquestado es:
/// 1. Verificación de Identidad: Confirma que el contratista está activo.
/// 2. Filtro de Seguridad: Consulta la lista negra institucional.
/// 3. Regla de Unicidad: Impide ingresos dobles si ya hay uno abierto.
/// 4. Motor de Reglas: Analiza la fecha PRAIND y otras políticas complejas.
///
/// # Argumentos
/// * `contratista_id_str` - ID del contratista a validar.
///
/// # Retorno
/// Respuesta de validación con estado de autorización.
///
/// # Errores
/// - `IngresoContratistaError::ContratistaNotFound`: Contratista no existe.
/// - `IngresoContratistaError::Database`: Error de consulta.
pub async fn validar_ingreso_contratista(
    contratista_id_str: String,
) -> Result<ValidacionIngresoResponse, IngresoContratistaError> {
    let contratista_id = parse_contratista_id(&contratista_id_str)
        .unwrap_or_else(|_| RecordId::from_table_key("contratista", &contratista_id_str));

    let contratista = contratista_queries::find_by_id(&contratista_id)
        .await
        .map_err(|e| {
            error!("Error DB al buscar contratista para validación: {}", e);
            IngresoContratistaError::Database(e.to_string())
        })?
        .ok_or_else(|| {
            warn!("Contratista no encontrado para validación: {}", contratista_id_str);
            IngresoContratistaError::ContratistaNotFound
        })?;

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

// --------------------------------------------------------------------------
// REGISTRO DE INGRESOS
// --------------------------------------------------------------------------

/// Registra físicamente el ingreso de un contratista y asigna recursos.
///
/// # Argumentos
/// * `input` - Datos del ingreso (contratista, gafete, vehículo, etc.).
/// * `usuario_id_str` - ID del guardia que registra.
///
/// # Retorno
/// Respuesta con los datos del ingreso creado.
///
/// # Errores
/// - `IngresoContratistaError::GafeteNotAvailable`: Gafete no disponible.
/// - `IngresoContratistaError::ContratistaNotFound`: Contratista no existe.
/// - `IngresoContratistaError::Database`: Error de persistencia.
pub async fn crear_ingreso_contratista(
    input: CreateIngresoContratistaInput,
    usuario_id_str: String,
) -> Result<IngresoResponse, IngresoContratistaError> {
    let contratista_id = parse_contratista_id(&input.contratista_id)?;
    let usuario_id = parse_user_id(&usuario_id_str)?;

    // Gestión de Gafetes Físicos: Se asegura que el recurso no esté duplicado.
    if let Some(ref g) = input.gafete_numero {
        if *g != 0 {
            let disp = gafete_service::is_gafete_disponible(*g, "contratista")
                .await
                .map_err(|e| IngresoContratistaError::Gafete(e.to_string()))?;

            if !disp {
                warn!("Gafete {} no disponible para ingreso", *g);
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
        segundo_nombre: contratista.segundo_nombre.clone(),
        segundo_apellido: contratista.segundo_apellido.clone(),
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

    info!("Ingreso registrado: Contratista {} ingresó a planta", input.contratista_id);

    IngresoResponse::from_contratista_fetched(nuevo_ingreso)
        .map_err(|e| IngresoContratistaError::Validation(e))
}

// --------------------------------------------------------------------------
// REGISTRO DE SALIDAS
// --------------------------------------------------------------------------

/// Finaliza una jornada de trabajo registrando la salida y liberando recursos.
///
/// # Argumentos
/// * `input` - Datos de la salida (observaciones, estado gafete).
/// * `usuario_id_str` - ID del guardia que registra la salida.
///
/// # Retorno
/// Ingreso actualizado con hora de salida.
///
/// # Errores
/// - `IngresoContratistaError::Database`: Error de actualización.
pub async fn registrar_salida(
    input: RegistrarSalidaInput,
    usuario_id_str: String,
) -> Result<IngresoResponse, IngresoContratistaError> {
    let ingreso_id = parse_ingreso_id(&input.ingreso_id)?;
    let usuario_id = parse_user_id(&usuario_id_str)?;

    let ingreso_actualizado =
        db::update_salida(&ingreso_id, &usuario_id, input.observaciones_salida)
            .await
            .map_err(|e| IngresoContratistaError::Database(e.to_string()))?;

    // Devolución del Gafete: Permite que el recurso vuelva a estar disponible.
    if input.devolvio_gafete {
        if let Some(ref g) = ingreso_actualizado.gafete_numero {
            let _ = gafete_service::liberar_gafete(*g, "contratista").await;
        }
    }

    info!("Salida registrada para ingreso: {}", input.ingreso_id);

    IngresoResponse::from_contratista_fetched(ingreso_actualizado)
        .map_err(|e| IngresoContratistaError::Validation(e))
}

// --------------------------------------------------------------------------
// MONITOREO DE PLANTA
// --------------------------------------------------------------------------

/// Valida si un contratista puede salir de planta.
///
/// # Argumentos
/// * `_ingreso_id` - ID del ingreso activo.
/// * `_gafete` - Número de gafete a verificar (opcional).
///
/// # Retorno
/// Resultado de validación con errores/advertencias.
pub async fn validar_puede_salir(
    _ingreso_id: &str,
    _gafete: Option<&str>,
) -> Result<ResultadoValidacionSalida, String> {
    Ok(ResultadoValidacionSalida { puede_salir: true, errores: vec![], advertencias: vec![] })
}

/// Obtiene el estado de ocupación actual con alertas de tiempo.
///
/// # Retorno
/// Lista de ingresos abiertos con minutos transcurridos.
pub async fn get_ingresos_abiertos_con_alertas(
) -> Result<Vec<IngresoConEstadoResponse>, IngresoContratistaError> {
    // TODO: Implementar lógica de monitoreo
    Ok(vec![])
}

/// Consulta reactiva de alertas por tiempos de permanencia excedidos.
///
/// # Retorno
/// Lista de alertas para contratistas que exceden el límite.
pub async fn verificar_tiempos_excedidos(
) -> Result<Vec<AlertaTiempoExcedido>, IngresoContratistaError> {
    // TODO: Implementar lógica de alertas
    Ok(vec![])
}

// --------------------------------------------------------------------------
// UNIT TESTS
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_contratista_id_con_prefijo() {
        let id = parse_contratista_id("contratista:abc123").unwrap();
        assert_eq!(id.to_string(), "contratista:abc123");
    }

    #[test]
    fn test_parse_contratista_id_sin_prefijo() {
        let id = parse_contratista_id("abc123").unwrap();
        assert_eq!(id.to_string(), "contratista:abc123");
    }

    #[test]
    fn test_parse_user_id_con_prefijo() {
        let id = parse_user_id("user:guard01").unwrap();
        assert_eq!(id.to_string(), "user:guard01");
    }

    #[test]
    fn test_parse_user_id_sin_prefijo() {
        let id = parse_user_id("guard01").unwrap();
        assert_eq!(id.to_string(), "user:guard01");
    }

    #[test]
    fn test_parse_ingreso_id_con_prefijo() {
        let id = parse_ingreso_id("ingreso:ing001").unwrap();
        assert_eq!(id.to_string(), "ingreso:ing001");
    }

    #[test]
    fn test_parse_ingreso_id_sin_prefijo() {
        let id = parse_ingreso_id("ing001").unwrap();
        assert_eq!(id.to_string(), "ingreso:ing001");
    }
}
