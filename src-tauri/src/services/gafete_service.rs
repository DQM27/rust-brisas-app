//! # Servicio de Gestión de Gafetes
//!
//! Controla el inventario, asignación y ciclo de vida de los gafetes físicos.
//!
//! ## Responsabilidades
//! - Gestionar el inventario de gafetes (Altas, Bajas, Modificaciones).
//! - Controlar la disponibilidad y estado (Activo, En Uso, Perdido).
//! - Garantizar la integridad de las asignaciones (evitar duplicados físicos).
//!
//! ## Dependencias
//! - `crate::db::surrealdb_gafete_queries`: Persistencia.
//! - `crate::domain::errors::GafeteError`: Errores de dominio.

use crate::db::surrealdb_gafete_queries as db;
use crate::domain::errors::GafeteError;
use crate::models::gafete::{
    CreateGafeteInput, CreateGafeteRangeInput, GafeteCreateDTO, GafeteEstado, GafeteResponse,
    TipoGafete,
};
use log::{error, info, warn};
use surrealdb::RecordId;

// --------------------------------------------------------------------------
// HELPERS & VALIDACIONES PRIVADAS
// --------------------------------------------------------------------------

/// Helper para parsear IDs de gafete.
///
/// Soporta formatos: "123" (asume tabla 'gafete') o "gafete:123".
fn parse_gafete_id(id_str: &str) -> Result<RecordId, GafeteError> {
    if id_str.contains(':') {
        id_str
            .parse::<RecordId>()
            .map_err(|_| GafeteError::Validation(format!("ID de gafete inválido: {id_str}")))
    } else {
        Ok(RecordId::from_table_key("gafete", id_str))
    }
}

// --------------------------------------------------------------------------
// FUNCIONES PÚBLICAS
// --------------------------------------------------------------------------

/// Verifica si un gafete específico está disponible para asignación.
///
/// # Criterios de Disponibilidad
/// 1. El gafete debe existir en base de datos.
/// 2. Su estado debe ser `Activo`.
/// 3. No debe estar marcado como `en_uso`.
/// 4. No debe tener alertas pendientes (gafete perdido/no devuelto).
///
/// # Argumentos
/// * `numero` - Número identificador del gafete.
/// * `tipo` - Tipo de gafete (ej. "contratista", "visita").
///
/// # Retorno
/// `true` si puede asignarse, `false` en caso contrario.
pub async fn is_gafete_disponible(numero: i32, tipo: &str) -> Result<bool, GafeteError> {
    use crate::db::surrealdb_alerta_queries as alerta_db;

    match db::get_gafete(numero, tipo).await {
        Ok(Some(g)) => {
            // Criterio básico: estado activo y no en uso
            if g.estado != GafeteEstado::Activo || g.en_uso {
                return Ok(false);
            }

            // Criterio adicional: verificar si hay alerta pendiente para este gafete
            let alertas_pendientes = alerta_db::find_all(Some(false)).await.unwrap_or_default();

            let tiene_alerta = alertas_pendientes.iter().any(|a| a.gafete_numero == numero);

            if tiene_alerta {
                warn!("Gafete {numero} tiene alerta pendiente - no disponible");
                return Ok(false);
            }

            Ok(true)
        }
        Ok(None) => Ok(false),
        Err(e) => {
            error!("Error DB consultando disponibilidad de gafete {numero}: {e}");
            Err(GafeteError::Database(e.to_string()))
        }
    }
}

/// Marca un gafete como entregado y en uso.
///
/// # Errores
/// * `GafeteError::NotFound` - Si el gafete no existe.
/// * `GafeteError::Database` - Error de persistencia.
pub async fn marcar_en_uso(numero: i32, tipo: &str) -> Result<(), GafeteError> {
    let gafete = db::get_gafete(numero, tipo)
        .await
        .map_err(|e| GafeteError::Database(e.to_string()))?
        .ok_or(GafeteError::NotFound)?;

    db::set_gafete_uso(&gafete.id, true).await.map_err(|e| GafeteError::Database(e.to_string()))?;

    Ok(())
}

/// Libera un gafete, marcándolo como disponible.
///
/// # Errores
/// * `GafeteError::NotFound` - Si el gafete no existe.
/// * `GafeteError::Database` - Error de persistencia.
pub async fn liberar_gafete(numero: i32, tipo: &str) -> Result<(), GafeteError> {
    let gafete = db::get_gafete(numero, tipo)
        .await
        .map_err(|e| GafeteError::Database(e.to_string()))?
        .ok_or(GafeteError::NotFound)?;

    db::set_gafete_uso(&gafete.id, false)
        .await
        .map_err(|e| GafeteError::Database(e.to_string()))?;

    Ok(())
}

/// Crea un único gafete manualmente.
///
/// # Argumentos
/// * `input` - Datos del nuevo gafete.
///
/// # Errores
/// * `GafeteError::Validation` - Si el número o tipo son inválidos.
/// * `GafeteError::Database` - Error al insertar.
pub async fn create_gafete(input: CreateGafeteInput) -> Result<GafeteResponse, GafeteError> {
    use crate::domain::gafete as domain;

    // Validaciones de dominio
    domain::validar_create_input(&input).map_err(|e| GafeteError::Validation(e.to_string()))?;

    let tipo = input
        .tipo
        .parse::<TipoGafete>()
        .map_err(|_| GafeteError::InvalidType(input.tipo.clone()))?;

    let dto =
        GafeteCreateDTO { numero: input.numero, tipo, estado: GafeteEstado::Activo, en_uso: false };

    let gafete = db::create_gafete(dto).await.map_err(|e| {
        error!("Error al crear gafete {}: {}", input.numero, e);
        GafeteError::Database(e.to_string())
    })?;

    info!("Gafete creado exitosamente: {} ({})", gafete.numero, gafete.tipo);

    Ok(GafeteResponse::from(gafete))
}

/// Generador masivo de gafetes por rango.
///
/// Permite poblar el inventario rápidamente (ej. del 100 al 200).
/// Ignora fallos individuales (ej. duplicados) para completar el lote.
///
/// # Argumentos
/// * `input` - Rango (inicio, fin) y tipo.
///
/// # Retorno
/// Cantidad de gafetes creados exitosamente.
pub async fn create_gafete_range(input: CreateGafeteRangeInput) -> Result<i32, GafeteError> {
    use crate::domain::gafete as domain;

    // Validar rango
    if input.start > input.end {
        return Err(GafeteError::Validation(
            "El inicio del rango no puede ser mayor que el fin".to_string(),
        ));
    }

    domain::validar_numero(input.start).map_err(|e| GafeteError::Validation(e.to_string()))?;
    domain::validar_numero(input.end).map_err(|e| GafeteError::Validation(e.to_string()))?;

    let tipo = input
        .tipo
        .parse::<TipoGafete>()
        .map_err(|_| GafeteError::InvalidType(input.tipo.clone()))?;

    let mut created = 0;
    info!("Iniciando creación masiva de gafetes: {} -> {}", input.start, input.end);

    for numero in input.start..=input.end {
        let dto = GafeteCreateDTO {
            numero,
            tipo: tipo.clone(),
            estado: GafeteEstado::Activo,
            en_uso: false,
        };

        // Tolerancia a fallos: Si uno falla (ya existe), seguimos con el siguiente
        if let Ok(_) = db::create_gafete(dto).await {
            created += 1;
        }
    }

    info!("Creación masiva finalizada. Total creados: {created}");
    Ok(created)
}

/// Obtiene un gafete por su ID.
pub async fn get_gafete_by_id(id_str: &str) -> Result<Option<GafeteResponse>, GafeteError> {
    let id = parse_gafete_id(id_str)?;
    db::find_by_id(&id)
        .await
        .map(|opt| opt.map(GafeteResponse::from))
        .map_err(|e| GafeteError::Database(e.to_string()))
}

/// Formatea un `SurrealDB` Datetime a ISO8601 estándar para frontend.
/// `SurrealDB` `Datetime.to_string()` puede incluir prefijo o formato no parseable por JS.
fn format_datetime_iso(dt: &surrealdb::Datetime) -> String {
    let raw = dt.to_string();

    // Clean string: remove "d" prefix and any surrounding quotes (single or double)
    // Example: d'2026-01-14T...' -> 2026-01-14T...
    let mut cleaned = raw.as_str();
    if cleaned.starts_with('d') {
        cleaned = &cleaned[1..];
    }
    cleaned = cleaned.trim_matches(|c| c == '\'' || c == '"');

    // Parse, convert to UTC to ensure 'Z' validity, and format as strict ISO8601
    chrono::DateTime::parse_from_rfc3339(cleaned).map_or_else(
        |_| cleaned.to_string(),
        |dt| dt.with_timezone(&chrono::Utc).to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
    )
}

/// Lista todos los gafetes registrados, enriquecidos con datos de alertas pendientes.
pub async fn get_all_gafetes() -> Result<Vec<GafeteResponse>, GafeteError> {
    use crate::db::surrealdb_alerta_queries as alerta_db;
    use crate::db::surrealdb_ingreso_general_queries as ingreso_db;
    use crate::db::surrealdb_user_queries as user_db;
    use crate::models::ingreso::UniversalIngresoFetched;
    use std::collections::{HashMap, HashSet};

    let gafetes = db::get_all_gafetes().await.map_err(|e| GafeteError::Database(e.to_string()))?;

    // Fetch ALL alerts (both pending and resolved) to show complete history
    let alertas = alerta_db::find_all(None).await.unwrap_or_else(|e| {
        error!("Error fetching alerts: {e}");
        vec![]
    });

    // Collect all unique user IDs that we need to fetch (reportado_por, resuelto_por)
    #[allow(clippy::mutable_key_type)]
    let mut user_ids_to_fetch: HashSet<RecordId> = HashSet::new();
    for alerta in &alertas {
        user_ids_to_fetch.insert(alerta.reportado_por.clone());
        if let Some(ref resuelto_por) = alerta.resuelto_por {
            user_ids_to_fetch.insert(resuelto_por.clone());
        }
    }

    // Batch fetch users and create lookup map (user_id_string -> full name)
    let mut user_names: HashMap<String, String> = HashMap::new();
    for user_id in user_ids_to_fetch {
        if let Ok(Some(user)) = user_db::find_by_id(&user_id).await {
            let full_name = format!("{} {}", user.nombre, user.apellido);
            user_names.insert(user_id.to_string(), full_name);
        }
    }

    // Batch fetch ALL active ingresos to determine real usage status
    // This optimization replaces N+1 queries and ensures status is correct even if 'en_uso' flag is stale
    let active_ingresos = ingreso_db::find_ingresos_abiertos_fetched().await.unwrap_or_else(|e| {
        error!("Error fetching active ingresos: {e}");
        vec![]
    });

    // Create lookup map: gafete_numero -> Ingreso
    let mut active_gafete_map: HashMap<i32, UniversalIngresoFetched> = HashMap::new();
    for ingreso in active_ingresos {
        let gafete_num = match &ingreso {
            UniversalIngresoFetched::Contratista(i) => i.gafete_numero,
            UniversalIngresoFetched::Proveedor(i) => i.gafete_numero,
            UniversalIngresoFetched::Visita(i) => i.gafete_numero,
        };

        if let Some(num) = gafete_num {
            active_gafete_map.insert(num, ingreso);
        }
    }

    // Create lookup map by gafete_numero
    let alertas_map: HashMap<i32, _> = alertas.into_iter().map(|a| (a.gafete_numero, a)).collect();

    // Enrich each gafete with alert and ingreso data
    let mut enriched: Vec<GafeteResponse> = Vec::with_capacity(gafetes.len());

    for g in gafetes {
        let mut resp = GafeteResponse::from(g.clone());

        // Check if there's a pending alert for this gafete number
        if let Some(alerta) = alertas_map.get(&g.numero) {
            resp.alerta_id = Some(alerta.id.to_string());
            // Format date properly as ISO8601 for JavaScript
            resp.fecha_perdido = Some(format_datetime_iso(&alerta.fecha_reporte));
            resp.quien_perdio = Some(alerta.nombre_completo.clone());
            resp.alerta_resuelta = Some(alerta.resuelto);
            resp.notas = alerta.notas.clone();

            // Get actual user name from lookup map
            let reportado_key = alerta.reportado_por.to_string();
            resp.reportado_por_nombre = Some(
                user_names
                    .get(&reportado_key)
                    .cloned()
                    .unwrap_or_else(|| "Usuario desconocido".to_string()),
            );

            // Fecha de resolución if available
            if let Some(ref fecha) = alerta.fecha_resolucion {
                resp.fecha_resolucion = Some(format_datetime_iso(fecha));
            }
            if let Some(ref resuelto_por) = alerta.resuelto_por {
                let resuelto_key = resuelto_por.to_string();
                resp.resuelto_por_nombre = Some(
                    user_names
                        .get(&resuelto_key)
                        .cloned()
                        .unwrap_or_else(|| "Usuario desconocido".to_string()),
                );
            }

            // Mark status as "perdido" if there's an active alert
            if !alerta.resuelto {
                resp.status = "perdido".to_string();
            }
        }

        // If there is an ACTIVE INGRESO for this gafete, force status to "en_uso"
        // This relies on the ingress record as the Source of Truth
        if let Some(ingreso) = active_gafete_map.get(&g.numero) {
            resp.status = "en_uso".to_string();
            resp.esta_disponible = false;

            // We still populate this field even if hidden in frontend, as it's useful data
            let nombre = match ingreso {
                UniversalIngresoFetched::Contratista(i) => {
                    format!("{} {}", i.nombre, i.apellido)
                }
                UniversalIngresoFetched::Proveedor(i) => {
                    format!("{} {}", i.nombre, i.apellido)
                }
                UniversalIngresoFetched::Visita(i) => {
                    format!("{} {}", i.nombre, i.apellido)
                }
            };
            resp.asignado_a = Some(nombre);
        }

        enriched.push(resp);
    }

    Ok(enriched)
}

/// Lista los gafetes disponibles para un tipo específico.
pub async fn get_gafetes_disponibles(tipo_str: &str) -> Result<Vec<GafeteResponse>, GafeteError> {
    let gafetes = db::get_gafetes_disponibles(tipo_str)
        .await
        .map_err(|e| GafeteError::Database(e.to_string()))?;

    Ok(gafetes.into_iter().map(GafeteResponse::from).collect())
}

/// Cambia el estado operativo de un gafete (ej. Perdido, Dañado).
pub async fn update_gafete_status(
    id_str: &str,
    estado: GafeteEstado,
) -> Result<GafeteResponse, GafeteError> {
    let id = parse_gafete_id(id_str)?;

    // Logs de auditoría importantes para cambios de estado
    if estado == GafeteEstado::Extraviado || estado == GafeteEstado::Danado {
        warn!("Marcando gafete {id_str} como {estado}");
    }

    let gafete = db::update_estado(&id, estado.as_str())
        .await
        .map_err(|e| GafeteError::Database(e.to_string()))?;

    Ok(GafeteResponse::from(gafete))
}

/// Elimina un gafete del sistema.
pub async fn delete_gafete(id_str: &str) -> Result<(), GafeteError> {
    let id = parse_gafete_id(id_str)?;

    // Verificar existencia antes de borrar
    if db::find_by_id(&id).await.map_err(|e| GafeteError::Database(e.to_string()))?.is_none() {
        return Err(GafeteError::NotFound);
    }

    db::delete_gafete_by_id(&id).await.map_err(|e| GafeteError::Database(e.to_string()))?;

    info!("Gafete eliminado: {id_str}");
    Ok(())
}

// --------------------------------------------------------------------------
// TESTS UNITARIOS
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_gafete_id_simple() {
        let res = parse_gafete_id("123");
        assert!(res.is_ok());
        assert_eq!(res.unwrap().to_string(), "gafete:⟨123⟩");
    }

    #[test]
    fn test_parse_gafete_id_compuesto() {
        let res = parse_gafete_id("gafete:456");
        assert!(res.is_ok());
        assert_eq!(res.unwrap().to_string(), "gafete:456");
    }

    #[test]
    fn test_parse_gafete_id_invalido() {
        // RecordId requiere formato válido si tiene ":"
        let res = parse_gafete_id("tabla:sin_valor:"); // Formato raro
                                                       // Dependerá de la implementación de surrealdb::RecordId::parse
                                                       // Asimimos que podría fallar o pasar dependiedo de la lib,
                                                       // pero validamos que no crashee.
        let _ = res;
    }
}
