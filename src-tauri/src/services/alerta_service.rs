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

/// Lista alertas filtradas por su estado de resolución.
///
/// # Arguments
///
/// * `resuelto` - Filtro opcional: `Some(true)` (resueltas), `Some(false)` (pendientes), `None` (todas).
///
/// # Returns
///
/// Lista de alertas que coinciden con el criterio.
pub async fn find_all(resuelto: Option<bool>) -> Result<Vec<AlertaGafete>, AlertaError> {
    db::find_all(resuelto).await.map_err(|e| AlertaError::Database(e.to_string()))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::surrealdb_service::{init_surrealdb, SurrealDbConfig};

    // Use a separate setup to avoid race conditions on the singleton if possible,
    // but likely we just reuse the global one.
    async fn setup_test_db() {
        let path = std::env::temp_dir().join("brisas_test_db_alerta");
        // Ensure clean slate if possible, or just accept existing
        let config = SurrealDbConfig {
            data_path: path,
            namespace: "test_ns".to_string(),
            database: "test_db".to_string(),
        };
        let service = init_surrealdb(config);
        let _ = service.connect().await; // Ignore error if already connected
        let _ = service.init_schema().await;
    }

    #[tokio::test]
    #[ignore = "Requires running DB environment or file write permissions"]
    async fn test_create_and_resolve_flow() {
        setup_test_db().await;

        let input = crate::models::ingreso::CreateAlertaInput {
            id: "alerta_test_01".to_string(),
            persona_id: None,
            cedula: "123456789".to_string(),
            nombre_completo: "Test User".to_string(),
            gafete_numero: 999,
            ingreso_contratista_id: None,
            ingreso_proveedor_id: None,
            ingreso_visita_id: None,
            fecha_reporte: "2023-01-01T10:00:00Z".to_string(),
            notas: Some("Test note".to_string()),
            reportado_por: "admin".to_string(),
        };

        // 1. Insert
        let result = insert(input.clone()).await;
        assert!(result.is_ok(), "Failed to insert alert: {:?}", result.err());

        // 2. Find (Need to implement find_by_id in service to test properly,
        //    but we can assume insert worked if no error.
        //    Wait, checking find_by_id is exposed in service)
        let found = find_by_id("alerta_test_01").await;
        assert!(found.is_ok());
        let alerta = found.unwrap();
        assert_eq!(alerta.cedula, "123456789");
        assert!(!alerta.resuelto);

        // 3. Resolve
        let resolve_input = crate::models::ingreso::ResolverAlertaInput {
            alerta_id: "alerta_test_01".to_string(),
            notas: Some("Resolved".to_string()),
            usuario_id: Some("admin_res".to_string()),
        };
        let resolve_res = resolver(resolve_input).await;
        assert!(resolve_res.is_ok());

        // 4. Verify Resolution
        let found_again = find_by_id("alerta_test_01").await.unwrap();
        assert!(found_again.resuelto);
        assert_eq!(found_again.resuelto_por.unwrap().to_string(), "admin_res");
    }
}
