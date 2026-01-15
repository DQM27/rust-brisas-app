/// Servicio: Gestión de Ingresos de Proveedores
///
/// Orquestador para el control de acceso de proveedores y personal externo.
/// Responsabilidades:
/// - Registro de entradas con validación de seguridad (Lista Negra).
/// - Asignación y control de gafetes provisionales.
/// - Registro de salidas y liberación de recursos.
/// - Trazabilidad de áreas visitadas y motivos.
use crate::db::surrealdb_ingreso_proveedor_queries as db;
use crate::db::surrealdb_proveedor_queries as proveedor_queries;
use crate::domain::errors::IngresoProveedorError;
use crate::models::ingreso::{
    CreateIngresoProveedorInput, IngresoProveedorCreateDTO, IngresoResponse,
    ValidacionIngresoProveedorResponse,
};
use crate::services::{gafete_service, lista_negra_service, proveedor_service};
use log::{error, info, warn};
use surrealdb::RecordId;

// --------------------------------------------------------------------------
// HELPERS & VALIDACIONES PRIVADAS
// --------------------------------------------------------------------------

/// Helper para parsear IDs de `SurrealDB` (Table:Key o solo Key).
fn parse_id(id: &str, table: &str) -> Result<RecordId, IngresoProveedorError> {
    if id.contains(':') {
        id.parse::<RecordId>()
            .map_err(|_| IngresoProveedorError::Validation(format!("ID de {table} inválido")))
    } else {
        Ok(RecordId::from_table_key(table, id))
    }
}

/// Valida disponibilidad de gafete.
async fn validar_gafete_disponible(gafete: Option<i32>) -> Result<(), IngresoProveedorError> {
    if let Some(g) = gafete {
        if g != 0 {
            let disp = gafete_service::is_gafete_disponible(g, "proveedor")
                .await
                .map_err(|e| IngresoProveedorError::Gafete(e.to_string()))?;
            if !disp {
                return Err(IngresoProveedorError::Validation(
                    "El gafete seleccionado ya está en uso".to_string(),
                ));
            }
        }
    }
    Ok(())
}

/// Verifica lista negra.
async fn validar_lista_negra(cedula: String) -> Result<(), IngresoProveedorError> {
    let check = lista_negra_service::check_is_blocked(cedula.clone()).await.map_err(|_| {
        IngresoProveedorError::Validation(
            "Error al consultar protocolos de seguridad (Lista Negra)".to_string(),
        )
    })?;

    if check.is_blocked {
        warn!("Intento de ingreso bloqueado por Lista Negra: {cedula}");
        return Err(IngresoProveedorError::Validation(
            "ACCESO DENEGADO: El proveedor se encuentra en Lista Negra".to_string(),
        ));
    }
    Ok(())
}

/// Verifica si ya tiene un ingreso activo.
async fn validar_ingreso_unico(proveedor_id: &RecordId) -> Result<(), IngresoProveedorError> {
    let abierto = db::find_ingreso_abierto_by_proveedor(proveedor_id)
        .await
        .map_err(|e| IngresoProveedorError::Database(e.to_string()))?;

    if let Some(ingreso) = abierto {
        warn!("Intento de ingreso duplicado para proveedor: {proveedor_id}");
        return Err(IngresoProveedorError::Validation(format!(
            "Ya existe un registro de ingreso abierto (ID: {}) para esta persona",
            ingreso.id
        )));
    }
    Ok(())
}

// --------------------------------------------------------------------------
// FUNCIONES PÚBLICAS
// --------------------------------------------------------------------------

/// Registra la entrada física de un proveedor a las instalaciones.
///
/// Realiza validaciones críticas de seguridad ante de permitir el acceso
/// (Lista Negra, disponibilidad de Gafete, Unicidad de ingreso).
///
/// # Argumentos
/// * `input` - Datos del intento de ingreso.
/// * `usuario_id_str` - ID del usuario que registra la acción.
///
/// # Retorno
/// DTO con la información del ingreso registrado.
///
/// # Errores
/// - `IngresoProveedorError::Validation`: Si falla alguna regla de negocio.
/// - `IngresoProveedorError::Gafete`: Problemas con el sistema de gafetes.
/// - `IngresoProveedorError::Database`: Fallos de persistencia.
pub async fn registrar_ingreso(
    input: CreateIngresoProveedorInput,
    usuario_id_str: String,
) -> Result<IngresoResponse, IngresoProveedorError> {
    let proveedor_id = parse_id(&input.proveedor_id, "proveedor")?;
    let usuario_id = parse_id(&usuario_id_str, "user")?;

    // Validaciones de Negocio
    validar_gafete_disponible(input.gafete_numero).await?;
    validar_lista_negra(input.cedula.clone()).await?;
    validar_ingreso_unico(&proveedor_id).await?;

    // Obtener datos del proveedor para snapshot
    let proveedor = proveedor_queries::find_by_id(&proveedor_id)
        .await
        .map_err(|e| IngresoProveedorError::Database(e.to_string()))?
        .ok_or(IngresoProveedorError::Validation("Proveedor no encontrado".to_string()))?;

    let dto = IngresoProveedorCreateDTO {
        proveedor: proveedor_id,
        nombre: proveedor.nombre.clone(),
        apellido: proveedor.apellido.clone(),
        segundo_nombre: proveedor.segundo_nombre.clone(),
        segundo_apellido: proveedor.segundo_apellido.clone(),
        cedula: input.cedula.clone(),
        area_visitada: input.area_visitada.clone(),
        motivo: input.motivo.clone(),
        modo_ingreso: input.modo_ingreso.clone(),
        placa_vehiculo: input.placa_vehiculo.clone(),
        gafete_numero: input.gafete_numero,
        usuario_ingreso: usuario_id,
        observaciones: input.observaciones.clone(),
    };

    let nuevo_ingreso = db::insert(dto).await.map_err(|e| {
        error!("Error DB al insertar ingreso proveedor: {e}");
        IngresoProveedorError::Database(e.to_string())
    })?;

    // Actualizar estado de Gafete (Efecto secundario)
    if let Some(g) = input.gafete_numero {
        if g != 0 {
            if let Err(e) = gafete_service::marcar_en_uso(g, "proveedor").await {
                error!("Error crítico: Gafete {g} no se pudo marcar en uso: {e:?}");
                // No fallamos el ingreso, pero logueamos el error de consistencia.
            }
        }
    }

    info!(
        "Ingreso de proveedor registrado: {} {} (Gafete: {:?})",
        proveedor.nombre, proveedor.apellido, input.gafete_numero
    );

    Ok(IngresoResponse::from_proveedor_fetched(nuevo_ingreso))
}

/// Registra la salida del proveedor y libera los recursos asignados.
///
/// Finaliza el ciclo de visita, marcando la hora de salida y liberando el gafete
/// (si fue devuelto) para que pueda ser usado por otro visitante.
///
/// # Argumentos
/// * `ingreso_id_str` - ID del registro de ingreso a cerrar.
/// * `usuario_id_str` - ID del usuario de seguridad que valida la salida.
/// * `observaciones` - Notas adicionales sobre la salida (opcional).
/// * `devolvio_gafete` - Flag crítico: indica si se recuperó el activo físico.
///
/// # Retorno
/// DTO con el estado final del ingreso (incluyendo fechas de cierre).
///
/// # Errores
/// - `IngresoProveedorError::Database`: Si falla la actualización en BD.
/// - `IngresoProveedorError::Validation`: Si los IDs son inválidos.
pub async fn registrar_salida(
    ingreso_id_str: String,
    usuario_id_str: String,
    observaciones: Option<String>,
    devolvio_gafete: bool,
) -> Result<IngresoResponse, IngresoProveedorError> {
    let ingreso_id = parse_id(&ingreso_id_str, "ingreso_proveedor")?;
    let usuario_id = parse_id(&usuario_id_str, "user")?;

    let ingreso_actualizado =
        db::update_salida(&ingreso_id, &usuario_id, observaciones).await.map_err(|e| {
            error!("Error DB al registrar salida de proveedor: {e}");
            IngresoProveedorError::Database(e.to_string())
        })?;

    // Gestión de Gafetes en Salida
    if devolvio_gafete {
        if let Some(ref g) = ingreso_actualizado.gafete_numero {
            if *g != 0 {
                if let Err(e) = gafete_service::liberar_gafete(*g, "proveedor").await {
                    error!("Error no fatal: No se pudo liberar gafete {g} en salida: {e:?}");
                } else {
                    info!("Gafete {g} liberado correctamente.");
                }
            }
        }
    } else if let Some(g) = ingreso_actualizado.gafete_numero {
        if g != 0 {
            warn!("ALERTA: Proveedor salió SIN devolver gafete {g}");
        }
    }

    info!("Salida de proveedor registrada: {}", ingreso_actualizado.nombre);

    Ok(IngresoResponse::from_proveedor_fetched(ingreso_actualizado))
}

/// Consulta los ingresos que permanecen activos (sin fecha de salida).
///
/// Útil para el dashboard de control de planta.
///
/// # Retorno
/// Lista de ingresos abiertos.
pub async fn get_activos() -> Result<Vec<IngresoResponse>, IngresoProveedorError> {
    let activos = db::find_activos_fetched()
        .await
        .map_err(|e| IngresoProveedorError::Database(e.to_string()))?;

    Ok(activos.into_iter().map(IngresoResponse::from_proveedor_fetched).collect())
}

/// Valida si un proveedor es apto para ingresar antes de abrir el formulario de admisión.
///
/// Permite al frontend "pre-validar" una cédula o ID para mostrar alertas tempranas
/// (ej. si ya está dentro o si tiene alertas de seguridad) antes de llenar todo el formulario.
///
/// # Argumentos
/// * `proveedor_id_str`: ID del proveedor a consultar.
///
/// # Retorno
/// DTO con flag `puedeIngresar`, datos del proveedor y alertas.
pub async fn validar_ingreso(
    proveedor_id_str: String,
) -> Result<ValidacionIngresoProveedorResponse, IngresoProveedorError> {
    let p_id = parse_id(&proveedor_id_str, "proveedor")?;

    // 1. Validar que exista
    let p = proveedor_service::get_proveedor_by_id(&p_id.to_string())
        .await
        .map_err(|e| IngresoProveedorError::Validation(e.to_string()))?;

    // 2. Validar que no tenga ingreso abierto
    let abierto = db::find_ingreso_abierto_by_proveedor(&p_id)
        .await
        .map_err(|e| IngresoProveedorError::Database(e.to_string()))?;

    // 3. Consultar alertas de gafetes pendientes
    let alertas = crate::services::alerta_service::find_pendientes_by_cedula(&p.cedula)
        .await
        .unwrap_or_default();

    let tiene_gafetes_pendientes = !alertas.is_empty();
    let alertas_gafete: Vec<String> = alertas
        .into_iter()
        .map(|a| a.notas.unwrap_or_else(|| "Gafete no devuelto".to_string()))
        .collect();

    let puede_ingresar = abierto.is_none();

    Ok(ValidacionIngresoProveedorResponse {
        puede_ingresar,
        cedula: p.cedula.clone(),
        nombre: p.nombre.clone(),
        apellido: p.apellido.clone(),
        segundo_nombre: p.segundo_nombre.clone(),
        segundo_apellido: p.segundo_apellido.clone(),
        empresa_nombre: p.empresa_nombre.clone(),
        motivo_rechazo: if puede_ingresar {
            None
        } else {
            Some("Ya tiene un ingreso activo".to_string())
        },
        alertas_gafete,
        tiene_gafetes_pendientes,
        tiene_ingreso_abierto: !puede_ingresar,
    })
}

// --------------------------------------------------------------------------
// TESTS UNITARIOS (Helpers)
// --------------------------------------------------------------------------

