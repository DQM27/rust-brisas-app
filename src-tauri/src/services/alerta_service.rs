/// Servicio: Gestión de Alertas.
///
/// Orquestador de la lógica de negocio para la gestión de incidencias de seguridad (`AlertaGafete`).
///
/// Responsabilidades:
/// - Registrar nuevas alertas detectadas por el sistema o usuarios.
/// - Consultar historial de alertas (pendientes vs resueltas).
/// - Gestionar la resolución de alertas por parte de seguridad.
use crate::db::surrealdb_alerta_queries as db;
use crate::domain::errors::AlertaError;
use crate::models::ingreso::AlertaGafete;
use crate::services::gafete_service;
use log::{error, info, warn};

/// Localiza una alerta específica para su auditoría o resolución.
///
/// # Arguments
///
/// * `id` - Identificador único de la alerta (`alerta_gafete:UUID`).
///
/// # Returns
///
/// Retorna la alerta encontrada o error `NotFound` si no existe.
pub async fn find_by_id(id: &str) -> Result<AlertaGafete, AlertaError> {
    db::find_by_id(id)
        .await
        .map_err(|e| AlertaError::Database(e.to_string()))?
        .ok_or(AlertaError::NotFound)
}

/// Recupera todas las alertas pendientes asociadas a una persona (Cédula).
///
/// Fundamental para detectar reincidencias durante el proceso de ingreso.
///
/// # Arguments
///
/// * `cedula` - Identificador fiscal/personal del sujeto.
///
/// # Returns
///
/// Lista de alertas NO resueltas (puede estar vacía).
pub async fn find_pendientes_by_cedula(cedula: &str) -> Result<Vec<AlertaGafete>, AlertaError> {
    db::find_pendientes_by_cedula(cedula).await.map_err(|e| AlertaError::Database(e.to_string()))
}

/// Lista alertas filtradas por su estado de resolución, enriquecidas con nombres de usuarios y empresas.
pub async fn find_all(
    resuelto: Option<bool>,
) -> Result<Vec<crate::models::ingreso::AlertaGafeteResponse>, AlertaError> {
    use crate::db::surrealdb_user_queries as user_db;
    use crate::models::ingreso::AlertaGafeteResponse;
    use crate::services::surrealdb_service::get_db;
    use std::collections::{HashMap, HashSet};
    use surrealdb::RecordId;

    let alertas = db::find_all(resuelto).await.map_err(|e| AlertaError::Database(e.to_string()))?;
    let db = get_db().await.map_err(|e| AlertaError::Database(e.to_string()))?;

    // 1. Recolectar IDs para batch fetch
    #[allow(clippy::mutable_key_type)]
    let mut user_ids: HashSet<RecordId> = HashSet::new();
    #[allow(clippy::mutable_key_type)]
    let mut contractor_ids: HashSet<RecordId> = HashSet::new();
    #[allow(clippy::mutable_key_type)]
    let mut provider_ids: HashSet<RecordId> = HashSet::new();
    #[allow(clippy::mutable_key_type)]
    let mut visitor_ids: HashSet<RecordId> = HashSet::new();

    for a in &alertas {
        user_ids.insert(a.reportado_por.clone());
        if let Some(ref rid) = a.resuelto_por {
            user_ids.insert(rid.clone());
        }
        if let Some(ref rid) = a.ingreso_contratista {
            contractor_ids.insert(rid.clone());
        }
        if let Some(ref rid) = a.ingreso_proveedor {
            provider_ids.insert(rid.clone());
        }
        if let Some(ref rid) = a.ingreso_visita {
            visitor_ids.insert(rid.clone());
        }
    }

    // 2. Mapa de Usuarios
    let mut user_names: HashMap<String, String> = HashMap::new();
    for id in user_ids {
        if let Ok(Some(u)) = user_db::find_by_id(&id).await {
            user_names.insert(id.to_string(), format!("{} {}", u.nombre, u.apellido));
        }
    }

    // 3. Mapa de Empresas (Normalización de IDs para el matching)
    let mut company_map: HashMap<String, String> = HashMap::new();

    // Batch para Contratistas
    if !contractor_ids.is_empty() {
        let q = "SELECT id, contratista.empresa.nombre as nombre FROM ingreso_contratista WHERE id IN $ids FETCH contratista, contratista.empresa";
        if let Ok(mut res) =
            db.query(q).bind(("ids", contractor_ids.into_iter().collect::<Vec<_>>())).await
        {
            if let Ok(rows) = res.take::<Vec<serde_json::Value>>(0) {
                for row in rows {
                    if let (Some(id), Some(n)) =
                        (row.get("id"), row.get("nombre").and_then(|v| v.as_str()))
                    {
                        company_map.insert(id.to_string(), n.to_string());
                    }
                }
            }
        }
    }

    // Batch para Proveedores
    if !provider_ids.is_empty() {
        let q = "SELECT id, proveedor.empresa.nombre as nombre FROM ingreso_proveedor WHERE id IN $ids FETCH proveedor, proveedor.empresa";
        if let Ok(mut res) =
            db.query(q).bind(("ids", provider_ids.into_iter().collect::<Vec<_>>())).await
        {
            if let Ok(rows) = res.take::<Vec<serde_json::Value>>(0) {
                for row in rows {
                    if let (Some(id), Some(n)) =
                        (row.get("id"), row.get("nombre").and_then(|v| v.as_str()))
                    {
                        company_map.insert(id.to_string(), n.to_string());
                    }
                }
            }
        }
    }

    // Batch para Visitas
    if !visitor_ids.is_empty() {
        let q = "SELECT id, empresa_nombre as nombre FROM ingreso_visita WHERE id IN $ids";
        if let Ok(mut res) =
            db.query(q).bind(("ids", visitor_ids.into_iter().collect::<Vec<_>>())).await
        {
            if let Ok(rows) = res.take::<Vec<serde_json::Value>>(0) {
                for row in rows {
                    if let (Some(id), Some(n)) =
                        (row.get("id"), row.get("nombre").and_then(|v| v.as_str()))
                    {
                        company_map.insert(id.to_string(), n.to_string());
                    }
                }
            }
        }
    }

    // 4. Transformación final
    let response = alertas
        .into_iter()
        .map(|a| {
            let mut resp = AlertaGafeteResponse::from(a.clone());

            // Nombre reportador
            if let Some(name) = user_names.get(&a.reportado_por.to_string()) {
                resp.reportado_por_nombre = name.clone();
            } else {
                resp.reportado_por_nombre = "Sistema".to_string();
            }

            // Nombre resolutor
            if let Some(ref rid) = a.resuelto_por {
                if let Some(name) = user_names.get(&rid.to_string()) {
                    resp.resuelto_por_nombre = name.clone();
                } else {
                    resp.resuelto_por_nombre = "Desconocido".to_string();
                }
            }

            // Nombre empresa
            let possible_ids = [
                a.ingreso_contratista.as_ref().map(std::string::ToString::to_string),
                a.ingreso_proveedor.as_ref().map(std::string::ToString::to_string),
                a.ingreso_visita.as_ref().map(std::string::ToString::to_string),
            ];

            for id_opt in possible_ids.iter().flatten() {
                if let Some(c) = company_map.get(id_opt) {
                    resp.empresa_nombre = c.clone();
                    break;
                }
            }

            // Normalización de fechas para JS
            resp.fecha_reporte = clean_surreal_date(&resp.fecha_reporte);
            if let Some(ref fr) = resp.fecha_resolucion {
                resp.fecha_resolucion = Some(clean_surreal_date(fr));
            }

            resp
        })
        .collect();

    Ok(response)
}

/// Helper privado para limpiar fechas de `SurrealDB` d'YYYY-MM-DD...' -> YYYY-MM-DD...
fn clean_surreal_date(raw: &str) -> String {
    let mut cleaned = raw;
    if cleaned.starts_with('d') {
        cleaned = &cleaned[1..];
    }
    cleaned = cleaned.trim_matches(|c| c == '\'' || c == '"');
    cleaned.to_string()
}

/// Registra una nueva alerta en el sistema.
///
/// Se invoca automáticamente por el sistema de monitoreo o manualmente por el guardia.
///
/// # Arguments
///
/// * `input` - DTO con los datos completos de la alerta.
///
/// # Errors
///
/// * `AlertaError::Database`: Fallo de conexión o inserción.
pub async fn insert(input: crate::models::ingreso::CreateAlertaInput) -> Result<(), AlertaError> {
    let cedula = input.cedula.clone();
    let gafete = input.gafete_numero;

    db::insert(input).await.map_err(|e| {
        error!("Error de base de datos al insertar alerta para {cedula}: {e}");
        AlertaError::Database(e.to_string())
    })?;

    info!("Alerta crítica registrada para {cedula} (Gafete: {gafete})");
    Ok(())
}

/// Marca una alerta como gestionada/resuelta.
///
/// Registra qué usuario resolvió la incidencia y las notas correspondientes.
/// **IMPORTANTE**: También libera el gafete asociado para que vuelva al inventario.
///
/// # Arguments
///
/// * `input` - DTO con ID de alerta, usuario resolutor y notas.
///
/// # Errors
///
/// * `AlertaError::Database`: Fallo al actualizar el registro.
pub async fn resolver(
    input: crate::models::ingreso::ResolverAlertaInput,
) -> Result<(), AlertaError> {
    let id = input.alerta_id.clone();
    info!("Intentando resolver alerta con ID: '{id}'");

    // 1. Obtener la alerta antes de resolverla para conocer el gafete
    let alerta = find_by_id(&id).await?;
    let gafete_numero = alerta.gafete_numero;

    // 2. Determinar el tipo de gafete basado en qué tipo de ingreso tiene
    let tipo_gafete = if alerta.ingreso_contratista.is_some() {
        "contratista"
    } else if alerta.ingreso_proveedor.is_some() {
        "proveedor"
    } else if alerta.ingreso_visita.is_some() {
        "visita"
    } else {
        "contratista" // Default fallback
    };

    info!("Resolviendo alerta {id} (gafete: {gafete_numero}, tipo: {tipo_gafete})");

    // 3. Resolver la alerta en BD
    db::resolver(input).await.map_err(|e| {
        error!("Error al resolver alerta {id}: {e}");
        AlertaError::Database(e.to_string())
    })?;

    // 4. Liberar el gafete para que vuelva al inventario
    if gafete_numero != 0 {
        match gafete_service::liberar_gafete(gafete_numero, tipo_gafete).await {
            Ok(()) => {
                info!("Gafete {gafete_numero} liberado exitosamente tras resolver alerta {id}");
            }
            Err(e) => {
                // No fallamos la resolución por esto, solo advertimos
                warn!("No se pudo liberar gafete {gafete_numero} tras resolver alerta: {e:?}");
            }
        }
    }

    info!("Alerta {id} resuelta exitosamente");
    Ok(())
}

/// Elimina físicamente una alerta del sistema.
///
/// **Atención**: Esta operación es destructiva e irreversible. Solo para mantenimiento.
///
/// # Arguments
///
/// * `id` - Identificador único de la alerta a eliminar.
pub async fn delete(id: &str) -> Result<(), AlertaError> {
    db::delete(id).await.map_err(|e| AlertaError::Database(e.to_string()))
}
