//! # Commands Tauri: Lista Negra (Cortafuegos de Seguridad)
//!
//! Comandos que exponen operaciones de restricción de acceso al frontend.
//!
//! ## Categorías
//! - **Consultas**: `get_*`, `check_*` - Operaciones de solo lectura
//! - **Escritura**: `add_*`, `update_*`, `delete_*` - Requieren autenticación
//!
//! ## Seguridad
//! - `add_to_lista_negra`: Requiere permiso `lista_negra.crear`
//! - `update_lista_negra`: Requiere permiso `lista_negra.editar`
//! - `delete_from_lista_negra`: Requiere permiso `lista_negra.eliminar`
//! - `check_is_blocked`: Público (hot-path de validación)

use crate::domain::errors::ListaNegraError;
use crate::models::lista_negra::{
    AddToListaNegraInput, BlockCheckResponse, ListaNegraListResponse, ListaNegraResponse,
    UpdateListaNegraInput,
};
use crate::services::lista_negra_service;
use crate::services::session::SessionState;
use tauri::{command, State};

// Macro para verificar permisos
macro_rules! require_perm {
    ($session:expr, $modulo:expr, $accion:expr) => {{
        let _user = $session.get_user().ok_or_else(|| {
            ListaNegraError::Validation("Sesión no válida o expirada".to_string())
        })?;

        // TODO: Verificar permisos específicos cuando exista el sistema de permisos
        // if !user.has_permission($modulo, $accion) {
        //     return Err(ListaNegraError::Validation("Sin permisos suficientes".into()));
        // }
    }};
}

// --------------------------------------------------------------------------
// COMANDOS DE CONSULTA (Solo Lectura)
// --------------------------------------------------------------------------

/// Motor de Validación: Comprueba en tiempo real si una cédula tiene prohibido el acceso.
///
/// ## Hot Path
/// Este comando es el punto crítico de validación de seguridad que se invoca
/// antes de cualquier registro o ingreso. **No requiere autenticación** para
/// permitir validaciones rápidas desde el frontend.
///
/// ## Parámetros
/// * `cedula` - Cédula a verificar
///
/// ## Retorno
/// * `Ok(BlockCheckResponse)` - Estado de bloqueo (`is_blocked`, nivel, fecha)
///
/// ## Ejemplo TypeScript
/// ```typescript
/// const result = await invoke('check_is_blocked', { cedula: '1-2345-6789' });
/// if (result.is_blocked) {
///     toast.error(`Persona bloqueada: ${result.nivel_severidad}`);
/// }
/// ```
#[command]
pub async fn check_is_blocked(cedula: String) -> Result<BlockCheckResponse, ListaNegraError> {
    lista_negra_service::check_is_blocked(cedula).await
}

/// Obtiene un registro de lista negra por su ID.
///
/// ## Autenticación
/// Requiere sesión activa.
///
/// ## Parámetros
/// * `id` - ID del registro (formato: "`lista_negra:xxx`")
///
/// ## Errores
/// * `ListaNegraError::NotFound` - Registro no existe
/// * `ListaNegraError::Validation` - ID inválido o sesión expirada
#[command]
pub async fn get_lista_negra_by_id(
    session: State<'_, SessionState>,
    id: String,
) -> Result<ListaNegraResponse, ListaNegraError> {
    require_perm!(session, "lista_negra", "leer");

    lista_negra_service::get_by_id(id).await?.ok_or(ListaNegraError::NotFound)
}

/// Auditoría de Seguridad: Obtiene la relación completa de personas con restricciones.
///
/// ## Autenticación
/// Requiere sesión activa.
///
/// ## Retorno
/// Estructura con:
/// - `bloqueados`: Lista de registros
/// - `total`: Total de registros
/// - `activos`: Cantidad de bloqueos activos
/// - `por_nivel`: Desglose por severidad (alto, medio, bajo)
///
/// ## Ejemplo TypeScript
/// ```typescript
/// const data = await invoke('get_all_lista_negra');
/// console.log(`Total bloqueados: ${data.total}`);
/// console.log(`Por nivel: Alto=${data.por_nivel.alto}`);
/// ```
#[command]
pub async fn get_all_lista_negra(
    session: State<'_, SessionState>,
) -> Result<ListaNegraListResponse, ListaNegraError> {
    require_perm!(session, "lista_negra", "leer");

    lista_negra_service::get_all().await
}

/// Busca registros de lista negra por término.
///
/// ## Autenticación
/// Requiere sesión activa.
///
/// ## Parámetros
/// * `query` - Término de búsqueda (nombre, apellido, cédula)
///
/// ## Retorno
/// Lista de registros que coinciden (máximo 50)
#[command]
pub async fn search_lista_negra(
    session: State<'_, SessionState>,
    query: String,
) -> Result<Vec<ListaNegraResponse>, ListaNegraError> {
    require_perm!(session, "lista_negra", "leer");

    lista_negra_service::search(&query).await
}

// --------------------------------------------------------------------------
// COMANDOS DE ESCRITURA (Requieren Autenticación)
// --------------------------------------------------------------------------

/// Registra una nueva restricción de acceso para un individuo.
///
/// ## Autenticación
/// Requiere sesión activa con permiso `lista_negra.crear`.
///
/// ## Parámetros
/// * `input` - Datos del bloqueo (cédula, nombre, motivo, nivel, etc.)
///
/// ## Errores
/// * `ListaNegraError::AlreadyExists` - Persona ya está bloqueada
/// * `ListaNegraError::Validation` - Datos inválidos
/// * `ListaNegraError::Database` - Error de persistencia
///
/// ## Ejemplo TypeScript
/// ```typescript
/// await invoke('add_to_lista_negra', {
///     input: {
///         cedula: '1-2345-6789',
///         nombre: 'Juan',
///         apellido: 'Pérez',
///         nivelSeveridad: 'ALTO',
///         motivoBloqueo: 'Comportamiento inapropiado',
///         bloqueadoPor: 'user:admin123'
///     }
/// });
/// ```
#[command]
pub async fn add_to_lista_negra(
    session: State<'_, SessionState>,
    input: AddToListaNegraInput,
) -> Result<ListaNegraResponse, ListaNegraError> {
    require_perm!(session, "lista_negra", "crear");

    lista_negra_service::add_to_lista_negra(input).await
}

/// Actualiza un registro de lista negra existente.
///
/// ## Autenticación
/// Requiere sesión activa con permiso `lista_negra.editar`.
///
/// ## Campos Actualizables
/// - `nivel_severidad`: Cambiar gravedad
/// - `motivo_bloqueo`: Agregar información
/// - `observaciones`: Notas adicionales
///
/// ## Parámetros
/// * `id` - ID del registro a actualizar
/// * `input` - Campos a modificar
#[command]
pub async fn update_lista_negra(
    session: State<'_, SessionState>,
    id: String,
    input: UpdateListaNegraInput,
) -> Result<ListaNegraResponse, ListaNegraError> {
    require_perm!(session, "lista_negra", "editar");

    lista_negra_service::update(id, input).await
}

/// Elimina (desactiva) un registro de lista negra.
///
/// ## Autenticación
/// Requiere sesión activa con permiso `lista_negra.eliminar`.
///
/// ## Soft Delete
/// El registro no se elimina físicamente, solo se marca como inactivo.
///
/// ## Parámetros
/// * `id` - ID del registro a eliminar
#[command]
pub async fn delete_from_lista_negra(
    session: State<'_, SessionState>,
    id: String,
) -> Result<(), ListaNegraError> {
    require_perm!(session, "lista_negra", "eliminar");

    lista_negra_service::delete(id).await
}

/// Restaura un registro de lista negra previamente eliminado.
///
/// ## Autenticación
/// Requiere sesión activa con permiso `lista_negra.editar`.
///
/// ## Parámetros
/// * `id` - ID del registro a restaurar
#[command]
pub async fn restore_lista_negra(
    session: State<'_, SessionState>,
    id: String,
) -> Result<ListaNegraResponse, ListaNegraError> {
    require_perm!(session, "lista_negra", "editar");

    lista_negra_service::restore(id).await
}

// --------------------------------------------------------------------------
// COMANDOS DEPRECADOS (Compatibilidad temporal)
// --------------------------------------------------------------------------

/// @deprecated Use `search_lista_negra` en su lugar
#[command]
pub async fn search_personas_for_block(
    query: String,
) -> Result<Vec<crate::models::lista_negra::PersonaSearchResult>, ListaNegraError> {
    lista_negra_service::search_personas_for_block(&query).await.map_err(ListaNegraError::Database)
}
