/// Punto de Control: Admisión de Visitantes Ocasionales.
///
/// Gestiona la entrada y salida de personas que no tienen un vínculo permanente con
/// la institución (visitas familiares, mensajería, técnicos externos).
/// Se prioriza la agilidad del registro sin sacrificar la seguridad perimetral.
use crate::db::surrealdb_ingreso_visita_queries as db;
use crate::domain::errors::IngresoVisitaError;
use crate::models::ingreso::{CreateIngresoVisitaInput, IngresoResponse, IngresoVisitaCreateDTO};
use crate::services::{gafete_service, lista_negra_service};
use surrealdb::RecordId;

/// Registra el ingreso de un visitante.
///
/// El flujo de seguridad incluye:
/// 1. Validación de Gafetes: Verifica disponibilidad física del recurso.
/// 2. Filtro de Seguridad: Consulta inmediata a la Lista Negra.
/// 3. Control de Permanencia: Evita ingresos duplicados para la misma identificación.
pub async fn registrar_ingreso(
    input: CreateIngresoVisitaInput,
    usuario_id_str: String,
) -> Result<IngresoResponse, IngresoVisitaError> {
    let usuario_id = if usuario_id_str.contains(':') {
        usuario_id_str
            .parse::<RecordId>()
            .map_err(|_| IngresoVisitaError::Validation("ID de usuario inválido".to_string()))?
    } else {
        RecordId::from_table_key("user", &usuario_id_str)
    };

    if let Some(ref g) = input.gafete_numero {
        if g != "S/G" && !g.is_empty() {
            let disp = gafete_service::is_gafete_disponible(g, "visita")
                .await
                .map_err(|e| IngresoVisitaError::Validation(e.to_string()))?;
            if !disp {
                return Err(IngresoVisitaError::Validation(
                    "El gafete seleccionado no está disponible".to_string(),
                ));
            }
        }
    }

    let check =
        lista_negra_service::check_is_blocked(input.cedula.clone()).await.unwrap_or_default();
    if check.is_blocked {
        return Err(IngresoVisitaError::Validation(
            "ACCESO RESTRINGIDO: El visitante se encuentra en Lista Negra".to_string(),
        ));
    }

    let abierto = db::find_ingreso_abierto_by_cedula(&input.cedula)
        .await
        .map_err(|e| IngresoVisitaError::Database(e.to_string()))?;
    if abierto.is_some() {
        return Err(IngresoVisitaError::Validation(
            "Ya existe un registro de ingreso activo para esta persona".to_string(),
        ));
    }

    let dto = IngresoVisitaCreateDTO {
        cedula: input.cedula,
        nombre: input.nombre,
        apellido: input.apellido,
        anfitrion: input.anfitrion,
        area_visitada: input.area_visitada,
        motivo: input.motivo,
        modo_ingreso: input.modo_ingreso,
        placa_vehiculo: input.placa_vehiculo,
        gafete_numero: input.gafete_numero,
        usuario_ingreso: usuario_id,
        observaciones: input.observaciones,
    };

    let nuevo_ingreso =
        db::insert(dto).await.map_err(|e| IngresoVisitaError::Database(e.to_string()))?;

    // Marcado de activo físico.
    if let Some(ref g) = nuevo_ingreso.gafete_numero {
        if g != "S/G" && !g.is_empty() {
            let _ = gafete_service::marcar_en_uso(g, "visita").await;
        }
    }

    Ok(IngresoResponse::from_visita_fetched(nuevo_ingreso))
}

/// Registra la salida del visitante y libera los recursos.
pub async fn registrar_salida(
    ingreso_id_str: String,
    usuario_id_str: String,
    devolvio_gafete: bool,
    observaciones: Option<String>,
) -> Result<IngresoResponse, IngresoVisitaError> {
    let ingreso_id = if ingreso_id_str.contains(':') {
        ingreso_id_str
            .parse::<RecordId>()
            .map_err(|_| IngresoVisitaError::Validation("ID de ingreso inválido".to_string()))?
    } else {
        RecordId::from_table_key("ingreso_visita", &ingreso_id_str)
    };

    let usuario_id = if usuario_id_str.contains(':') {
        usuario_id_str
            .parse::<RecordId>()
            .map_err(|_| IngresoVisitaError::Validation("ID de usuario inválido".to_string()))?
    } else {
        RecordId::from_table_key("user", &usuario_id_str)
    };

    let actualizado = db::update_salida(&ingreso_id, &usuario_id, observaciones)
        .await
        .map_err(|e| IngresoVisitaError::Database(e.to_string()))?;

    if devolvio_gafete {
        if let Some(ref g) = actualizado.gafete_numero {
            if g != "S/G" {
                let _ = gafete_service::liberar_gafete(g, "visita").await;
            }
        }
    }

    Ok(IngresoResponse::from_visita_fetched(actualizado))
}

pub async fn get_activos(
) -> Result<Vec<crate::domain::ingreso_visita::IngresoVisitaPopulated>, IngresoVisitaError> {
    Ok(vec![])
}

/// Valida si un visitante es apto para entrar antes de proceder al registro manual.
pub async fn validar_ingreso(
    _visitante_id: &str,
) -> Result<crate::domain::ingreso_visita::ValidacionIngresoVisitaResponse, IngresoVisitaError> {
    // TODO: Implementar validación real consultando DB y motor de validación
    Ok(crate::domain::ingreso_visita::ValidacionIngresoVisitaResponse {
        puede_ingresar: true,
        cedula: String::new(),
        nombre: String::new(),
        apellido: String::new(),
        motivo_rechazo: None,
        alertas_gafete: vec![],
        tiene_gafetes_pendientes: false,
    })
}
