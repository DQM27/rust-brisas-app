// src/services/gafete_service.rs
use crate::db::surrealdb_gafete_queries as db;
use crate::models::gafete::{Gafete, GafeteEstado};

pub async fn is_gafete_disponible(numero: &str, tipo: &str) -> Result<bool, String> {
    match db::get_gafete(numero, tipo).await {
        Ok(Some(g)) => Ok(g.estado == GafeteEstado::Activo), // TODO: Verificar status "en_uso" si agregamos ese campo
        Ok(None) => Ok(false),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn marcar_en_uso(numero: &str, tipo: &str) -> Result<(), String> {
    // Por ahora solo verificamos que exista y sea activo.
    // En una implementación más completa, actualizaríamos un flag `en_uso` = true.
    // SurrealDB permite campos flexibles, podríamos agregarlo.
    // Por simplicidad en este paso, asumimos que si el ingreso se crea, el gafete se está usando.
    // Idealmente: db::set_gafete_usage(numero, tipo, true).await...

    // De momento, delegamos a la DB si decidimos implementar el flag.
    db::set_gafete_uso(numero, tipo, true).await.map_err(|e| e.to_string())
}

pub async fn liberar_gafete(numero: &str, tipo: &str) -> Result<(), String> {
    db::set_gafete_uso(numero, tipo, false).await.map_err(|e| e.to_string())
}

pub async fn create_gafete(_n: &str, _t: &str) -> Result<Gafete, String> {
    Err("Stubbed".to_string())
}
pub async fn create_gafete_range(_p: &str, _s: i32, _e: i32, _t: &str) -> Result<i32, String> {
    Err("Stubbed".to_string())
}
pub async fn get_gafete(n: &str, t: &str) -> Result<Option<Gafete>, String> {
    db::get_gafete(n, t).await.map_err(|e| e.to_string())
}
pub async fn get_all_gafetes() -> Result<Vec<Gafete>, String> {
    db::get_all_gafetes().await.map_err(|e| e.to_string())
}
pub async fn get_gafetes_disponibles(_t: &str) -> Result<Vec<Gafete>, String> {
    Ok(vec![])
}
pub async fn update_gafete(_n: &str, _t: &str, _nt: &str) -> Result<Gafete, String> {
    Err("Stubbed".to_string())
}
pub async fn update_gafete_status(_n: &str, _t: &str, _st: String) -> Result<Gafete, String> {
    Err("Stubbed".to_string())
}
pub async fn delete_gafete(_n: &str, _t: &str) -> Result<(), String> {
    Err("Stubbed".to_string())
}
