/// Servicio: Gestión de Ingresos de Visitas
///
/// Orquestador para el control de acceso de visitantes ocasionales.
/// Responsabilidades:
/// - Registro de entradas con validación de seguridad (Lista Negra, Gafetes).
/// - Control de permanencia (evitar ingresos duplicados por cédula).
/// - Registro de salidas y liberación de recursos.
/// - Trazabilidad de motivos y anfitriones.
use crate::db::surrealdb_ingreso_visita_queries as db;
use crate::domain::errors::IngresoVisitaError;
use crate::models::ingreso::{CreateIngresoVisitaInput, IngresoResponse, IngresoVisitaCreateDTO};
use crate::services::{gafete_service, lista_negra_service};
use log::{error, info, warn};
use surrealdb::RecordId;

// --------------------------------------------------------------------------
// HELPERS & VALIDACIONES PRIVADAS
// --------------------------------------------------------------------------

/// Helper para parsear IDs de `SurrealDB` (Table:Key o solo Key).
fn parse_id(id: &str, table: &str) -> Result<RecordId, IngresoVisitaError> {
    if id.contains(':') {
        id.parse::<RecordId>()
            .map_err(|_| IngresoVisitaError::Validation(format!("ID de {table} inválido")))
    } else {
        Ok(RecordId::from_table_key(table, id))
    }
}

/// Valida disponibilidad de gafete.
async fn validar_gafete_disponible(gafete: Option<i32>) -> Result<(), IngresoVisitaError> {
    if let Some(g) = gafete {
        if g != 0 {
            let disp = gafete_service::is_gafete_disponible(g, "visita")
                .await
                .map_err(|e| IngresoVisitaError::Gafete(e.to_string()))?;
            if !disp {
                return Err(IngresoVisitaError::Validation(
                    "El gafete seleccionado no está disponible".to_string(),
                ));
            }
        }
    }
    Ok(())
}

/// Verifica lista negra.
async fn validar_lista_negra(cedula: String) -> Result<(), IngresoVisitaError> {
    let check = lista_negra_service::check_is_blocked(cedula.clone()).await.unwrap_or_default();

    if check.is_blocked {
        warn!("Intento de ingreso bloqueado por Lista Negra: {cedula}");
        return Err(IngresoVisitaError::Validation(
            "ACCESO RESTRINGIDO: El visitante se encuentra en Lista Negra".to_string(),
        ));
    }
    Ok(())
}

/// Verifica si ya tiene un ingreso activo por cédula (sin ID previo).
async fn validar_ingreso_unico_cedula(cedula: &str) -> Result<(), IngresoVisitaError> {
    let abierto = db::find_ingreso_abierto_by_cedula(cedula)
        .await
        .map_err(|e| IngresoVisitaError::Database(e.to_string()))?;

    if let Some(ingreso) = abierto {
        warn!("Intento de ingreso duplicado para cédula: {cedula}");
        return Err(IngresoVisitaError::Validation(format!(
            "Ya existe un registro de ingreso activo (ID: {}) para esta persona",
            ingreso.id
        )));
    }
    Ok(())
}

// --------------------------------------------------------------------------
// FUNCIONES PÚBLICAS
// --------------------------------------------------------------------------

/// Registra el ingreso de un visitante.
///
/// El flujo de seguridad incluye:
/// 1. Validación de Gafetes: Verifica disponibilidad física del recurso.
/// 2. Filtro de Seguridad: Consulta inmediata a la Lista Negra.
/// 3. Control de Permanencia: Evita ingresos duplicados para la misma identificación.
///
/// # Argumentos
/// * `input` - Datos del visitante y la visita.
/// * `usuario_id_str` - ID del usuario de seguridad.
///
/// # Retorno
/// DTO con el registro creado.
///
/// # Errores
/// - `IngresoVisitaError::Validation`: Reglas de negocio no cumplidas.
/// - `IngresoVisitaError::Database`: fallo de persistencia.
pub async fn registrar_ingreso(
    input: CreateIngresoVisitaInput,
    usuario_id_str: String,
) -> Result<IngresoResponse, IngresoVisitaError> {
    let usuario_id = parse_id(&usuario_id_str, "user")?;

    // Validaciones de Negocio
    validar_gafete_disponible(input.gafete_numero).await?;
    validar_lista_negra(input.cedula.clone()).await?;
    validar_ingreso_unico_cedula(&input.cedula).await?;

    let dto = IngresoVisitaCreateDTO {
        cedula: input.cedula.clone(),
        nombre: input.nombre.clone(),
        apellido: input.apellido.clone(),
        segundo_nombre: input.segundo_nombre.clone(),
        segundo_apellido: input.segundo_apellido.clone(),
        anfitrion: input.anfitrion.clone(),
        area_visitada: input.area_visitada.clone(),
        motivo: input.motivo.clone(),
        modo_ingreso: input.modo_ingreso.clone(),
        placa_vehiculo: input.placa_vehiculo.clone(),
        gafete_numero: input.gafete_numero,
        usuario_ingreso: usuario_id,
        observaciones: input.observaciones.clone(),
    };

    let nuevo_ingreso = db::insert(dto).await.map_err(|e| {
        error!("Error DB al insertar ingreso visita: {e}");
        IngresoVisitaError::Database(e.to_string())
    })?;

    // Marcado de activo físico.
    if let Some(ref g) = nuevo_ingreso.gafete_numero {
        if *g != 0 {
            if let Err(e) = gafete_service::marcar_en_uso(*g, "visita").await {
                error!("Error crítico: Gafete {g} no se pudo marcar en uso: {e:?}");
            }
        }
    }

    info!(
        "Ingreso de visita registrado: {} {} (Gafete: {:?})",
        input.nombre, input.apellido, input.gafete_numero
    );

    Ok(IngresoResponse::from_visita_fetched(nuevo_ingreso))
}

/// Registra la salida del visitante y libera los recursos.
///
/// Finaliza la visita marcando la hora de salida. Si se le prestó un gafete,
/// se intenta liberar; si no se devuelve, se alerta el incidente.
///
/// # Argumentos
/// * `ingreso_id_str` - ID del ingreso a cerrar.
/// * `usuario_id_str` - ID del usuario de seguridad.
/// * `devolvio_gafete` - Indica si el activo físico fue retornado.
/// * `observaciones` - Notas finales sobre la visita.
///
/// # Retorno
/// DTO con el estado final de la visita.
///
/// # Errores
/// - `IngresoVisitaError::Validation`: IDs mal formados.
/// - `IngresoVisitaError::Database`: Error al actualizar.
pub async fn registrar_salida(
    ingreso_id_str: String,
    usuario_id_str: String,
    devolvio_gafete: bool,
    observaciones: Option<String>,
) -> Result<IngresoResponse, IngresoVisitaError> {
    let ingreso_id = parse_id(&ingreso_id_str, "ingreso_visita")?;
    let usuario_id = parse_id(&usuario_id_str, "user")?;

    let actualizado =
        db::update_salida(&ingreso_id, &usuario_id, observaciones).await.map_err(|e| {
            error!("Error DB al registrar salida de visita: {e}");
            IngresoVisitaError::Database(e.to_string())
        })?;

    if devolvio_gafete {
        if let Some(ref g) = actualizado.gafete_numero {
            if *g != 0 {
                if let Err(e) = gafete_service::liberar_gafete(*g, "visita").await {
                    error!("Error no fatal: No se pudo liberar gafete {g} en salida visita: {e:?}");
                } else {
                    info!("Gafete {g} (visita) liberado correctamente.");
                }
            }
        }
    } else if let Some(g) = actualizado.gafete_numero {
        if g != 0 {
            warn!("ALERTA: Visitante salió SIN devolver gafete {g}");
        }
    }

    info!("Salida de visita registrada: {}", actualizado.nombre);

    Ok(IngresoResponse::from_visita_fetched(actualizado))
}

/// Consulta las visitas que permanecen activas en planta.
///
/// # Retorno
/// Lista de ingresos de visita sin fecha de salida.
pub async fn get_activos() -> Result<Vec<IngresoResponse>, IngresoVisitaError> {
    let activos = db::find_activos_fetched()
        .await
        .map_err(|e| IngresoVisitaError::Database(e.to_string()))?;

    Ok(activos.into_iter().map(IngresoResponse::from_visita_fetched).collect())
}

/// Valida si un visitante es apto para entrar antes de proceder al registro manual.
///
/// Verifica lista negra y existencia de ingresos previos abiertos.
///
/// # Argumentos
/// * `cedula`: Identificación del visitante.
pub async fn validar_ingreso(
    cedula: &str,
) -> Result<crate::domain::ingreso_visita::ValidacionIngresoVisitaResponse, IngresoVisitaError> {
    // 1. Check Lista Negra
    let check = lista_negra_service::check_is_blocked(cedula.to_string()).await.unwrap_or_default();

    // 2. Check Ingreso Activo
    let abierto = db::find_ingreso_abierto_by_cedula(cedula).await.ok().flatten();

    let puede_ingresar = !check.is_blocked && abierto.is_none();
    let motivo = if check.is_blocked {
        Some("Visitante en Lista Negra".to_string())
    } else if abierto.is_some() {
        Some("Ya tiene un ingreso activo".to_string())
    } else {
        None
    };

    Ok(crate::domain::ingreso_visita::ValidacionIngresoVisitaResponse {
        puede_ingresar,
        cedula: cedula.to_string(),
        nombre: String::new(), // En visita a veces no tenemos el nombre previo si es nuevo
        apellido: String::new(),
        segundo_nombre: None,
        segundo_apellido: None,
        motivo_rechazo: motivo,
        alertas_gafete: vec![],
        tiene_gafetes_pendientes: false,
    })
}

// --------------------------------------------------------------------------
// TESTS UNITARIOS (Helpers)
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_id_visita_valido() {
        let res = parse_id("ingreso_visita:123", "ingreso_visita");
        assert!(res.is_ok());
        assert_eq!(res.unwrap().to_string(), "ingreso_visita:123");
    }

    #[test]
    fn test_parse_id_sin_prefijo_agrega_tabla() {
        let res = parse_id("abc", "user");
        assert!(res.is_ok());
        assert_eq!(res.unwrap().to_string(), "user:abc");
    }
}
