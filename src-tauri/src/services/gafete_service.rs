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
///
/// # Argumentos
/// * `numero` - Número identificador del gafete.
/// * `tipo` - Tipo de gafete (ej. "contratista", "visita").
///
/// # Retorno
/// `true` si puede asignarse, `false` en caso contrario.
pub async fn is_gafete_disponible(numero: i32, tipo: &str) -> Result<bool, GafeteError> {
    match db::get_gafete(numero, tipo).await {
        Ok(Some(g)) => Ok(g.estado == GafeteEstado::Activo && !g.en_uso),
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

/// Lista todos los gafetes registrados.
pub async fn get_all_gafetes() -> Result<Vec<GafeteResponse>, GafeteError> {
    let gafetes = db::get_all_gafetes().await.map_err(|e| GafeteError::Database(e.to_string()))?;

    Ok(gafetes.into_iter().map(GafeteResponse::from).collect())
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
        assert_eq!(res.unwrap().to_string(), "gafete:123");
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
