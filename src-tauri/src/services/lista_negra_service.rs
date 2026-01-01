/// Cortafuegos de Seguridad: Gestión de Lista Negra.
///
/// Este servicio actúa como la barrera de seguridad preventiva del sistema.
/// Antes de registrar cualquier entidad (Contratista, Proveedor, Visitante) o permitir
/// un ingreso, el sistema consulta este servicio para verificar si la persona
/// tiene prohibido el acceso por motivos de seguridad o conducta.
use crate::db::surrealdb_lista_negra_queries as db;
use crate::models::lista_negra::{BlockCheckResponse, ListaNegraResponse, PersonaSearchResult};

/// Verifica si una cédula tiene un bloqueo activo en el sistema.
///
/// Este es el "Hot Path" de seguridad que se invoca en cada intento de registro o ingreso.
pub async fn check_is_blocked(cedula: String) -> Result<BlockCheckResponse, String> {
    db::check_if_blocked_by_cedula(&cedula).await.map_err(|e| e.to_string())
}

// Nota: Las siguientes funciones están en fase de desarrollo (Stubbed).
// Representan la lógica de gestión administrativa de los bloqueos.

/// Registra un nuevo bloqueo de seguridad.
pub async fn add_to_lista_negra(
    _c: String,
    _n: String,
    _a: String,
    _ns: String,
    _m: String,
    _u: String,
) -> Result<(), String> {
    Err("Módulo administrativo de lista negra en construcción".to_string())
}

pub async fn get_lista_negra_by_id(_id: String) -> Result<Option<ListaNegraResponse>, String> {
    Ok(None)
}

pub async fn get_all_lista_negra() -> Result<Vec<ListaNegraResponse>, String> {
    Ok(vec![])
}

/// Busca personas candidatas a ser bloqueadas (integración con otros servicios).
pub async fn search_personas_for_block(_q: &str) -> Result<Vec<PersonaSearchResult>, String> {
    Ok(vec![])
}
