// src/services/lista_negra_service.rs
use crate::db::surrealdb_lista_negra_queries as db;
use crate::models::lista_negra::{
    BlockCheckResponse, ListaNegra, ListaNegraResponse, PersonaSearchResult,
};

pub async fn check_is_blocked(cedula: String) -> Result<BlockCheckResponse, String> {
    db::check_if_blocked_by_cedula(&cedula).await.map_err(|e| e.to_string())
}
pub async fn add_to_lista_negra(
    _c: String,
    _n: String,
    _a: String,
    _ns: String,
    _m: String,
    _u: String,
) -> Result<ListaNegra, String> {
    Err("Stubbed".to_string())
}
pub async fn get_lista_negra_by_id(_id: String) -> Result<Option<ListaNegraResponse>, String> {
    Ok(None)
}
pub async fn get_all_lista_negra() -> Result<Vec<ListaNegraResponse>, String> {
    Ok(vec![])
}
pub async fn get_lista_negra_activos() -> Result<Vec<ListaNegraResponse>, String> {
    Ok(vec![])
}
pub async fn get_blocked_by_cedula(_c: &str) -> Result<Option<ListaNegraResponse>, String> {
    Ok(None)
}
pub async fn remove_from_lista_negra(_id: String, _m: String, _u: String) -> Result<(), String> {
    Err("Stubbed".to_string())
}
pub async fn reactivate_lista_negra(_id: String, _m: String, _u: String) -> Result<(), String> {
    Err("Stubbed".to_string())
}
pub async fn update_lista_negra(
    _id: String,
    _m: String,
    _ns: String,
    _u: String,
) -> Result<ListaNegraResponse, String> {
    Err("Stubbed".to_string())
}
pub async fn delete_lista_negra(_id: String) -> Result<(), String> {
    Err("Stubbed".to_string())
}
pub async fn search_personas_for_block(_q: &str) -> Result<Vec<PersonaSearchResult>, String> {
    Ok(vec![])
}
