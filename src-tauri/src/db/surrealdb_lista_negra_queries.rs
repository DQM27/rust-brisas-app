//! # Queries SurrealDB: Lista Negra
//!
//! Operaciones de base de datos para el módulo de restricciones de acceso.
//!
//! ## Responsabilidades
//! - CRUD completo de registros de lista negra
//! - Verificación de bloqueo por cédula (hot-path de seguridad)
//! - Búsqueda y filtrado de registros
//!
//! ## Tabla: `lista_negra`
//! Almacena personas con acceso denegado a las instalaciones.
//!
//! ## Soft Delete
//! Los registros usan `is_active = false` para borrado lógico.
//! Queries de lectura filtran por `is_active = true` por defecto.

use crate::models::lista_negra::{
    AddToListaNegraInput, BlockCheckResponse, ListaNegra, UpdateListaNegraInput,
};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use log::{debug, info, warn};
use serde::Deserialize;
use surrealdb::{Datetime, RecordId};

// --------------------------------------------------------------------------
// CONSTANTES
// --------------------------------------------------------------------------

/// Límite máximo de registros por consulta para protección de memoria.
const MAX_QUERY_RESULTS: usize = 1000;

// --------------------------------------------------------------------------
// VERIFICACIÓN DE BLOQUEO (HOT PATH)
// --------------------------------------------------------------------------

/// Verifica si una cédula tiene un bloqueo activo.
///
/// Este es el **hot-path de seguridad** que se invoca en cada intento de
/// registro o ingreso. Debe ser lo más eficiente posible.
///
/// ## Query Ejecutado
/// ```sql
/// SELECT nivel_severidad, created_at
/// FROM lista_negra
/// WHERE cedula = $cedula AND is_active = true
/// ```
///
/// ## Optimizaciones
/// - Selecciona solo campos necesarios (no `SELECT *`)
/// - Usa índice en `cedula` para búsqueda O(log n)
///
/// ## Parámetros
/// * `cedula` - Cédula normalizada a verificar
///
/// ## Retorno
/// * `Ok(BlockCheckResponse)` - Estado de bloqueo (is_blocked, nivel, fecha)
///
/// ## Errores
/// * `SurrealDbError::Connection` - Fallo de conexión
/// * `SurrealDbError::Query` - Error en query
pub async fn check_if_blocked_by_cedula(
    cedula: &str,
) -> Result<BlockCheckResponse, SurrealDbError> {
    debug!("🔍 Verificando bloqueo para cédula: {}", cedula);

    let db = get_db().await?;

    #[derive(Deserialize)]
    struct LN {
        nivel_severidad: Option<String>,
        created_at: Option<Datetime>,
    }

    let mut result = db
        .query("SELECT nivel_severidad, created_at FROM lista_negra WHERE cedula = $cedula AND is_active = true")
        .bind(("cedula", cedula.to_string()))
        .await
        .map_err(|e| SurrealDbError::Query(format!(
            "Error al verificar bloqueo para cédula '{}': {}",
            cedula, e
        )))?;

    let res: Option<LN> = result.take(0)?;

    match res {
        Some(ln) => {
            debug!("🚫 Cédula {} BLOQUEADA - nivel: {:?}", cedula, ln.nivel_severidad);
            Ok(BlockCheckResponse {
                is_blocked: true,
                nivel_severidad: ln.nivel_severidad,
                bloqueado_desde: ln.created_at.map(|d| d.to_string()),
            })
        }
        None => {
            debug!("✅ Cédula {} no bloqueada", cedula);
            Ok(BlockCheckResponse {
                is_blocked: false,
                nivel_severidad: None,
                bloqueado_desde: None,
            })
        }
    }
}

// --------------------------------------------------------------------------
// OPERACIONES DE LECTURA
// --------------------------------------------------------------------------

/// Obtiene todos los registros activos de lista negra.
///
/// ## Query Ejecutado
/// ```sql
/// SELECT * FROM lista_negra
/// WHERE is_active = true
/// ORDER BY created_at DESC
/// LIMIT 1000
/// ```
///
/// ## Optimizaciones
/// - Filtra por `is_active = true` (excluye soft-deleted)
/// - Ordenado por fecha de creación (más recientes primero)
/// - Límite de 1000 registros para protección de memoria
///
/// ## Retorno
/// * `Ok(Vec<ListaNegra>)` - Lista de registros activos
pub async fn find_all() -> Result<Vec<ListaNegra>, SurrealDbError> {
    debug!("📋 Obteniendo todos los registros de lista negra");

    let db = get_db().await?;

    let mut result = db
        .query(
            "SELECT * FROM lista_negra 
             WHERE is_active = true 
             ORDER BY created_at DESC 
             LIMIT $limit",
        )
        .bind(("limit", MAX_QUERY_RESULTS))
        .await
        .map_err(|e| SurrealDbError::Query(format!("Error al obtener lista negra: {}", e)))?;

    let registros: Vec<ListaNegra> = result.take(0)?;

    debug!("📊 Encontrados {} registros de lista negra", registros.len());
    Ok(registros)
}

/// Busca un registro de lista negra por su ID.
///
/// ## Query Ejecutado
/// ```sql
/// SELECT * FROM $id WHERE is_active = true
/// ```
///
/// ## Parámetros
/// * `id` - RecordId del registro (ej: "lista_negra:abc123")
///
/// ## Retorno
/// * `Ok(Some(ListaNegra))` - Registro encontrado
/// * `Ok(None)` - No existe o está eliminado (soft delete)
pub async fn find_by_id(id: &RecordId) -> Result<Option<ListaNegra>, SurrealDbError> {
    debug!("🔍 Buscando lista negra por ID: {}", id);

    let db = get_db().await?;

    let mut result = db
        .query("SELECT * FROM $id WHERE is_active = true")
        .bind(("id", id.clone()))
        .await
        .map_err(|e| {
            SurrealDbError::Query(format!("Error al buscar lista negra por ID '{}': {}", id, e))
        })?;

    let registro: Option<ListaNegra> = result.take(0)?;
    Ok(registro)
}

/// Busca un registro de lista negra por cédula.
///
/// ## Query Ejecutado
/// ```sql
/// SELECT * FROM lista_negra
/// WHERE cedula = $cedula AND is_active = true
/// ```
///
/// ## Uso
/// Esta función se usa para verificar si una persona ya está bloqueada
/// antes de intentar agregarla nuevamente.
///
/// ## Parámetros
/// * `cedula` - Cédula a buscar
///
/// ## Retorno
/// * `Ok(Some(ListaNegra))` - Registro encontrado
/// * `Ok(None)` - No existe bloqueo activo para esa cédula
pub async fn find_by_cedula(cedula: &str) -> Result<Option<ListaNegra>, SurrealDbError> {
    debug!("🔍 Buscando lista negra por cédula: {}", cedula);

    let db = get_db().await?;

    let mut result = db
        .query("SELECT * FROM lista_negra WHERE cedula = $cedula AND is_active = true")
        .bind(("cedula", cedula.to_string()))
        .await
        .map_err(|e| {
            SurrealDbError::Query(format!(
                "Error al buscar lista negra por cédula '{}': {}",
                cedula, e
            ))
        })?;

    let registro: Option<ListaNegra> = result.take(0)?;
    Ok(registro)
}

// --------------------------------------------------------------------------
// OPERACIONES DE ESCRITURA
// --------------------------------------------------------------------------

/// Crea un nuevo registro de lista negra.
///
/// ## Query Ejecutado
/// ```sql
/// CREATE lista_negra CONTENT {
///     cedula: $cedula,
///     nombre: $nombre,
///     ...
///     is_active: true,
///     created_at: time::now(),
///     updated_at: time::now()
/// }
/// ```
///
/// ## Precondiciones
/// - Los datos deben estar validados previamente en `domain::lista_negra`
/// - Verificar que no exista bloqueo activo con `find_by_cedula`
///
/// ## Parámetros
/// * `input` - Datos del nuevo bloqueo
///
/// ## Retorno
/// * `Ok(ListaNegra)` - Registro creado con ID asignado
///
/// ## Errores
/// * `SurrealDbError::Query` - Error al crear (ej: constraint violation)
/// * `SurrealDbError::NotFound` - CREATE no retornó registro (muy raro)
pub async fn create(input: &AddToListaNegraInput) -> Result<ListaNegra, SurrealDbError> {
    debug!("➕ Creando registro de lista negra para cédula: {}", input.cedula);

    let db = get_db().await?;

    let mut result = db
        .query(
            "CREATE lista_negra CONTENT {
                cedula: $cedula,
                nombre: $nombre,
                segundo_nombre: $segundo_nombre,
                apellido: $apellido,
                segundo_apellido: $segundo_apellido,
                empresa_id: $empresa_id,
                empresa_nombre: $empresa_nombre,
                nivel_severidad: $nivel_severidad,
                motivo_bloqueo: $motivo_bloqueo,
                bloqueado_por: $bloqueado_por,
                observaciones: $observaciones,
                is_active: true,
                created_at: time::now(),
                updated_at: time::now()
            }",
        )
        .bind(("cedula", input.cedula.clone()))
        .bind(("nombre", input.nombre.clone()))
        .bind(("segundo_nombre", input.segundo_nombre.clone()))
        .bind(("apellido", input.apellido.clone()))
        .bind(("segundo_apellido", input.segundo_apellido.clone()))
        .bind(("empresa_id", input.empresa_id.clone()))
        .bind(("empresa_nombre", input.empresa_nombre.clone()))
        .bind(("nivel_severidad", input.nivel_severidad.clone()))
        .bind(("motivo_bloqueo", input.motivo_bloqueo.clone()))
        .bind(("bloqueado_por", input.bloqueado_por.clone()))
        .bind(("observaciones", input.observaciones.clone()))
        .await
        .map_err(|e| {
            SurrealDbError::Query(format!(
                "Error al crear registro de lista negra para cédula '{}': {}",
                input.cedula, e
            ))
        })?;

    let created: Option<ListaNegra> = result.take(0)?;

    match created {
        Some(registro) => {
            info!(
                "🚫 Persona agregada a lista negra: id={}, cédula={}, nivel={}",
                registro.id, registro.cedula, registro.nivel_severidad
            );
            Ok(registro)
        }
        None => Err(SurrealDbError::Query("CREATE lista_negra no retornó registro".to_string())),
    }
}

/// Actualiza un registro existente de lista negra.
///
/// ## Query Ejecutado
/// ```sql
/// UPDATE $id MERGE {
///     nivel_severidad: $nivel_severidad,
///     motivo_bloqueo: $motivo_bloqueo,
///     observaciones: $observaciones,
///     updated_at: time::now()
/// } WHERE is_active = true
/// ```
///
/// ## Campos Actualizables
/// Solo se pueden modificar: nivel_severidad, motivo_bloqueo, observaciones.
/// Campos como cedula, nombre, etc. son inmutables.
///
/// ## Parámetros
/// * `id` - ID del registro a actualizar
/// * `input` - Campos a actualizar (solo los presentes se modifican)
///
/// ## Retorno
/// * `Ok(ListaNegra)` - Registro actualizado
///
/// ## Errores
/// * `SurrealDbError::Query` - Registro no existe o está eliminado
pub async fn update(
    id: &RecordId,
    input: &UpdateListaNegraInput,
) -> Result<ListaNegra, SurrealDbError> {
    debug!("✏️ Actualizando registro de lista negra: {}", id);

    let db = get_db().await?;

    // Construir query dinámico solo con campos presentes
    let mut set_clauses = vec!["updated_at = time::now()".to_string()];

    if input.nivel_severidad.is_some() {
        set_clauses.push("nivel_severidad = $nivel_severidad".to_string());
    }
    if input.motivo_bloqueo.is_some() {
        set_clauses.push("motivo_bloqueo = $motivo_bloqueo".to_string());
    }
    if input.observaciones.is_some() {
        set_clauses.push("observaciones = $observaciones".to_string());
    }

    let query = format!("UPDATE $id SET {} WHERE is_active = true", set_clauses.join(", "));

    let mut result = db
        .query(&query)
        .bind(("id", id.clone()))
        .bind(("nivel_severidad", input.nivel_severidad.clone()))
        .bind(("motivo_bloqueo", input.motivo_bloqueo.clone()))
        .bind(("observaciones", input.observaciones.clone()))
        .await
        .map_err(|e| {
            SurrealDbError::Query(format!("Error al actualizar lista negra '{}': {}", id, e))
        })?;

    let updated: Option<ListaNegra> = result.take(0)?;

    match updated {
        Some(registro) => {
            info!("✏️ Lista negra actualizada: id={}", registro.id);
            Ok(registro)
        }
        None => Err(SurrealDbError::Query(format!(
            "Registro de lista negra no encontrado o eliminado: {}",
            id
        ))),
    }
}

/// Elimina (soft delete) un registro de lista negra.
///
/// ## Soft Delete
/// Esta función NO elimina físicamente el registro. En su lugar, marca
/// `is_active = false` para que sea excluido de queries normales.
///
/// ## Query Ejecutado
/// ```sql
/// UPDATE $id SET is_active = false, updated_at = time::now()
/// ```
///
/// ## Parámetros
/// * `id` - ID del registro a eliminar
///
/// ## Retorno
/// * `Ok(())` - Eliminado exitosamente
///
/// ## Errores
/// * `SurrealDbError::Query` - Registro no existe o ya está eliminado
///
/// ## Restauración
/// Para restaurar un registro, usar `restore()`.
pub async fn delete(id: &RecordId) -> Result<(), SurrealDbError> {
    warn!("🗑️ Eliminando (soft delete) registro de lista negra: {}", id);

    let db = get_db().await?;

    let mut result = db
        .query("UPDATE $id SET is_active = false, updated_at = time::now() WHERE is_active = true")
        .bind(("id", id.clone()))
        .await
        .map_err(|e| {
            SurrealDbError::Query(format!("Error al eliminar lista negra '{}': {}", id, e))
        })?;

    let updated: Option<ListaNegra> = result.take(0)?;

    match updated {
        Some(_) => {
            warn!("🗑️ Lista negra eliminada (desactivada): id={}", id);
            Ok(())
        }
        None => Err(SurrealDbError::Query(format!(
            "Registro de lista negra no encontrado o ya eliminado: {}",
            id
        ))),
    }
}

/// Restaura un registro previamente eliminado.
///
/// ## Query Ejecutado
/// ```sql
/// UPDATE $id SET is_active = true, updated_at = time::now()
/// WHERE is_active = false
/// ```
///
/// ## Parámetros
/// * `id` - ID del registro a restaurar
///
/// ## Retorno
/// * `Ok(ListaNegra)` - Registro restaurado
///
/// ## Errores
/// * `SurrealDbError::Query` - Registro no existe o ya está activo
pub async fn restore(id: &RecordId) -> Result<ListaNegra, SurrealDbError> {
    warn!("♻️ Restaurando registro de lista negra: {}", id);

    let db = get_db().await?;

    let mut result = db
        .query("UPDATE $id SET is_active = true, updated_at = time::now() WHERE is_active = false")
        .bind(("id", id.clone()))
        .await
        .map_err(|e| {
            SurrealDbError::Query(format!("Error al restaurar lista negra '{}': {}", id, e))
        })?;

    let restored: Option<ListaNegra> = result.take(0)?;

    match restored {
        Some(registro) => {
            warn!("♻️ Lista negra restaurada: id={}, cédula={}", registro.id, registro.cedula);
            Ok(registro)
        }
        None => Err(SurrealDbError::Query(format!(
            "Registro de lista negra no encontrado o ya activo: {}",
            id
        ))),
    }
}

// --------------------------------------------------------------------------
// BÚSQUEDA AVANZADA
// --------------------------------------------------------------------------

/// Busca registros por término en nombre, apellido o cédula.
///
/// ## Query Ejecutado
/// ```sql
/// SELECT * FROM lista_negra
/// WHERE is_active = true AND (
///     cedula CONTAINS $query OR
///     nombre CONTAINS $query OR
///     apellido CONTAINS $query
/// )
/// ORDER BY created_at DESC
/// LIMIT 50
/// ```
///
/// ## Parámetros
/// * `query` - Término de búsqueda (mínimo 2 caracteres)
///
/// ## Retorno
/// * `Ok(Vec<ListaNegra>)` - Registros que coinciden
pub async fn search(query: &str) -> Result<Vec<ListaNegra>, SurrealDbError> {
    let query_trimmed = query.trim();

    if query_trimmed.len() < 2 {
        return Ok(vec![]);
    }

    debug!("🔍 Buscando en lista negra: '{}'", query_trimmed);

    let db = get_db().await?;

    let mut result = db
        .query(
            "SELECT * FROM lista_negra 
             WHERE is_active = true AND (
                 cedula CONTAINS $query OR
                 nombre CONTAINS $query OR
                 apellido CONTAINS $query
             )
             ORDER BY created_at DESC
             LIMIT 50",
        )
        .bind(("query", query_trimmed.to_string()))
        .await
        .map_err(|e| SurrealDbError::Query(format!("Error en búsqueda de lista negra: {}", e)))?;

    let registros: Vec<ListaNegra> = result.take(0)?;

    debug!("🔍 Búsqueda '{}' encontró {} resultados", query_trimmed, registros.len());
    Ok(registros)
}
