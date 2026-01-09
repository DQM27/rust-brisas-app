//! # Servicio de Lista Negra (Cortafuegos de Seguridad)
//!
//! Este servicio actúa como la **barrera de seguridad preventiva** del sistema.
//! Antes de registrar cualquier entidad (Contratista, Proveedor, Visitante) o permitir
//! un ingreso, el sistema consulta este servicio para verificar si la persona
//! tiene prohibido el acceso por motivos de seguridad o conducta.
//!
//! ## Responsabilidades
//! - Verificación de bloqueo por cédula (hot-path de seguridad)
//! - Gestión CRUD de registros de lista negra
//! - Validación de datos mediante capa de dominio
//! - Logging de operaciones críticas de seguridad
//!
//! ## Arquitectura
//! ```text
//! Commands → [Services] → Queries → SurrealDB
//!                ↓
//!            Domain (validaciones)
//! ```

use crate::db::surrealdb_lista_negra_queries as db;
use crate::domain::errors::ListaNegraError;
use crate::domain::lista_negra as domain;
use crate::models::lista_negra::{
    AddToListaNegraInput, BlockCheckResponse, ListaNegraListResponse, ListaNegraResponse,
    NivelStats, UpdateListaNegraInput,
};
use log::{debug, info, warn};
use surrealdb::RecordId;

// --------------------------------------------------------------------------
// VERIFICACIÓN DE BLOQUEO (HOT PATH)
// --------------------------------------------------------------------------

/// Verifica si una cédula tiene un bloqueo activo en el sistema.
///
/// Este es el **"Hot Path" de seguridad** que se invoca en cada intento de
/// registro o ingreso. Debe ser lo más eficiente posible.
///
/// ## Uso
/// ```rust
/// // Antes de registrar un contratista
/// // let bloqueado = check_is_blocked(cedula.clone()).await?;
/// // if bloqueado.is_blocked {
/// //     return Err("Persona bloqueada".into());
/// // }
/// ```
///
/// ## Logging
/// - `DEBUG`: Cédula verificada
/// - No se loguea resultado por privacidad (frecuencia alta)
pub async fn check_is_blocked(cedula: String) -> Result<BlockCheckResponse, ListaNegraError> {
    debug!("🔍 Verificando bloqueo: {cedula}");

    db::check_if_blocked_by_cedula(&cedula)
        .await
        .map_err(|e| ListaNegraError::Database(e.to_string()))
}

// --------------------------------------------------------------------------
// OPERACIONES DE LECTURA
// --------------------------------------------------------------------------

/// Obtiene un registro de lista negra por su ID.
///
/// ## Parámetros
/// * `id` - ID del registro (formato: "`lista_negra:xxx`")
///
/// ## Retorno
/// * `Ok(Some(ListaNegraResponse))` - Registro encontrado
/// * `Ok(None)` - No existe o está eliminado
pub async fn get_by_id(id: String) -> Result<Option<ListaNegraResponse>, ListaNegraError> {
    debug!("🔍 Obteniendo lista negra por ID: {id}");

    // Parsear el ID a RecordId
    let record_id: RecordId =
        id.parse().map_err(|_| ListaNegraError::Validation(format!("ID inválido: {id}")))?;

    let registro =
        db::find_by_id(&record_id).await.map_err(|e| ListaNegraError::Database(e.to_string()))?;

    Ok(registro.map(std::convert::Into::into))
}

/// Obtiene todos los registros de lista negra con estadísticas.
///
/// ## Retorno
/// Estructura con:
/// - Lista de registros (máximo 1000)
/// - Total de registros
/// - Cantidad de activos
/// - Desglose por nivel de severidad
///
/// ## Logging
/// - `DEBUG`: Cantidad de registros obtenidos
pub async fn get_all() -> Result<ListaNegraListResponse, ListaNegraError> {
    debug!("📋 Obteniendo todos los registros de lista negra");

    let registros = db::find_all().await.map_err(|e| ListaNegraError::Database(e.to_string()))?;

    // Calcular estadísticas
    let total = registros.len();
    let activos = registros.iter().filter(|r| r.is_active).count();

    // Contar por nivel de severidad
    let alto = registros.iter().filter(|r| r.nivel_severidad.to_uppercase() == "ALTO").count();
    let medio = registros.iter().filter(|r| r.nivel_severidad.to_uppercase() == "MEDIO").count();
    let bajo = registros.iter().filter(|r| r.nivel_severidad.to_uppercase() == "BAJO").count();

    // Convertir a responses
    let bloqueados: Vec<ListaNegraResponse> =
        registros.into_iter().map(std::convert::Into::into).collect();

    debug!(
        "📊 Lista negra: {total} total, {activos} activos, {alto} alto, {medio} medio, {bajo} bajo"
    );

    Ok(ListaNegraListResponse {
        bloqueados,
        total,
        activos,
        por_nivel: NivelStats { alto, medio, bajo },
    })
}

/// Busca registros de lista negra por término.
///
/// ## Parámetros
/// * `query` - Término de búsqueda (nombre, apellido, cédula)
///
/// ## Retorno
/// Lista de registros que coinciden (máximo 50)
pub async fn search(query: &str) -> Result<Vec<ListaNegraResponse>, ListaNegraError> {
    debug!("🔍 Buscando en lista negra: '{query}'");

    let registros =
        db::search(query).await.map_err(|e| ListaNegraError::Database(e.to_string()))?;

    Ok(registros.into_iter().map(std::convert::Into::into).collect())
}

// --------------------------------------------------------------------------
// OPERACIONES DE ESCRITURA
// --------------------------------------------------------------------------

/// Agrega una persona a la lista negra.
///
/// ## Flujo de Operación
/// 1. Validar input (dominio)
/// 2. Verificar que no esté ya bloqueado
/// 3. Normalizar datos
/// 4. Crear registro en DB
/// 5. Loguear operación de seguridad
///
/// ## Parámetros
/// * `input` - Datos del bloqueo
///
/// ## Errores
/// * `ListaNegraError::Validation` - Datos inválidos
/// * `ListaNegraError::AlreadyBlocked` - Ya existe bloqueo activo
/// * `ListaNegraError::Database` - Error de persistencia
///
/// ## Logging
/// - `INFO`: Persona agregada a lista negra (operación crítica de seguridad)
/// - `WARN`: Intento de bloquear persona ya bloqueada
pub async fn add_to_lista_negra(
    input: AddToListaNegraInput,
) -> Result<ListaNegraResponse, ListaNegraError> {
    info!("🚫 Iniciando bloqueo: cédula={}", input.cedula);

    // 1. Validar input
    domain::validar_add_input(&input)?;

    // 2. Verificar que no esté ya bloqueado
    let existente = db::find_by_cedula(&input.cedula)
        .await
        .map_err(|e| ListaNegraError::Database(e.to_string()))?;

    if existente.is_some() {
        warn!("⚠️ Intento de bloquear persona ya bloqueada: cédula={}", input.cedula);
        return Err(ListaNegraError::AlreadyExists);
    }

    // 3. Normalizar datos
    let input_normalizado = AddToListaNegraInput {
        cedula: input.cedula.trim().to_string(),
        nombre: domain::normalizar_nombre_titulo(&input.nombre),
        segundo_nombre: input.segundo_nombre.map(|n| domain::normalizar_nombre_titulo(&n)),
        apellido: domain::normalizar_nombre_titulo(&input.apellido),
        segundo_apellido: input.segundo_apellido.map(|n| domain::normalizar_nombre_titulo(&n)),
        empresa_id: input.empresa_id,
        empresa_nombre: input.empresa_nombre,
        nivel_severidad: input.nivel_severidad.to_uppercase(),
        motivo_bloqueo: input.motivo_bloqueo.map(|m| domain::normalizar_texto(&m)),
        bloqueado_por: input.bloqueado_por.trim().to_string(),
    };

    // 4. Crear en DB
    let created = db::create(&input_normalizado)
        .await
        .map_err(|e| ListaNegraError::Database(e.to_string()))?;

    // 5. Log de operación crítica
    info!(
        "🚫 PERSONA BLOQUEADA: id={}, cédula={}, nombre={} {}, nivel={}, por={}",
        created.id,
        created.cedula,
        created.nombre,
        created.apellido,
        created.nivel_severidad,
        created.bloqueado_por
    );

    Ok(created.into())
}

/// Actualiza un registro de lista negra existente.
///
/// ## Campos Actualizables
/// - `nivel_severidad`: Puede cambiar la gravedad
/// - `motivo_bloqueo`: Agregar más información (opcional)
///
/// ## Logging
/// - `INFO`: Registro actualizado
pub async fn update(
    id: String,
    input: UpdateListaNegraInput,
) -> Result<ListaNegraResponse, ListaNegraError> {
    info!("✏️ Actualizando lista negra: id={id}");

    // Validar input
    domain::validar_update_input(&input)?;

    // Parsear ID
    let record_id: RecordId =
        id.parse().map_err(|_| ListaNegraError::Validation(format!("ID inválido: {id}")))?;

    // Actualizar
    let updated = db::update(&record_id, &input).await.map_err(|e| {
        let msg = e.to_string();
        if msg.contains("no encontrado") || msg.contains("not found") {
            ListaNegraError::NotFound
        } else {
            ListaNegraError::Database(msg)
        }
    })?;

    info!("✏️ Lista negra actualizada: id={}, cédula={}", updated.id, updated.cedula);

    Ok(updated.into())
}

/// Elimina (desactiva) un registro de lista negra.
///
/// ## Soft Delete
/// La persona NO es eliminada físicamente, solo se marca como inactiva.
/// Esto permite:
/// - Auditoría histórica
/// - Restauración si fue un error
///
/// ## Logging
/// - `WARN`: Persona removida de lista negra (operación notable)
pub async fn delete(id: String) -> Result<(), ListaNegraError> {
    warn!("🗑️ Removiendo de lista negra: id={id}");

    // Parsear ID
    let record_id: RecordId =
        id.parse().map_err(|_| ListaNegraError::Validation(format!("ID inválido: {id}")))?;

    // Verificar que existe (para log)
    let existente =
        db::find_by_id(&record_id).await.map_err(|e| ListaNegraError::Database(e.to_string()))?;

    let registro = existente.ok_or(ListaNegraError::NotFound)?;

    // Eliminar (soft delete)
    db::delete(&record_id).await.map_err(|e| ListaNegraError::Database(e.to_string()))?;

    warn!(
        "🗑️ PERSONA REMOVIDA DE LISTA NEGRA: id={}, cédula={}, nombre={} {}",
        registro.id, registro.cedula, registro.nombre, registro.apellido
    );

    Ok(())
}

/// Restaura un registro previamente eliminado.
///
/// ## Logging
/// - `WARN`: Persona restaurada a lista negra
pub async fn restore(id: String) -> Result<ListaNegraResponse, ListaNegraError> {
    warn!("♻️ Restaurando a lista negra: id={id}");

    // Parsear ID
    let record_id: RecordId =
        id.parse().map_err(|_| ListaNegraError::Validation(format!("ID inválido: {id}")))?;

    // Restaurar
    let restored = db::restore(&record_id).await.map_err(|e| {
        let msg = e.to_string();
        if msg.contains("no encontrado") || msg.contains("not found") {
            ListaNegraError::NotFound
        } else {
            ListaNegraError::Database(msg)
        }
    })?;

    warn!(
        "♻️ PERSONA RESTAURADA A LISTA NEGRA: id={}, cédula={}, nombre={} {}",
        restored.id, restored.cedula, restored.nombre, restored.apellido
    );

    Ok(restored.into())
}

// --------------------------------------------------------------------------
// FUNCIONES DEPRECADAS (Compatibilidad temporal)
// --------------------------------------------------------------------------

/// @deprecated Use `get_by_id` en su lugar
pub async fn get_lista_negra_by_id(id: String) -> Result<Option<ListaNegraResponse>, String> {
    get_by_id(id).await.map_err(|e| e.to_string())
}

/// @deprecated Use `get_all` en su lugar
pub async fn get_all_lista_negra() -> Result<Vec<ListaNegraResponse>, String> {
    let response = get_all().await.map_err(|e| e.to_string())?;
    Ok(response.bloqueados)
}

/// @deprecated Use `search` en su lugar
pub async fn search_personas_for_block(
    query: &str,
) -> Result<Vec<crate::models::lista_negra::PersonaSearchResult>, String> {
    // TODO: Implementar búsqueda cross-module (contratistas, proveedores, visitantes)
    // Por ahora retorna vacío
    debug!("🔍 search_personas_for_block: '{query}' (pendiente implementación cross-module)");
    Ok(vec![])
}
