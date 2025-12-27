// ==========================================
// src/services/gafete_service.rs
// ==========================================

use crate::db::surrealdb_gafete_queries as db;
use crate::models::gafete::{
    CreateGafeteInput, CreateGafeteRangeInput, GafeteCreateDTO, GafeteEstado, GafeteResponse,
    TipoGafete,
};
use surrealdb::RecordId;

/// Helper para parsear ID de gafete (acepta con o sin prefijo)
fn parse_gafete_id(id_str: &str) -> RecordId {
    if id_str.contains(':') {
        let parts: Vec<&str> = id_str.split(':').collect();
        RecordId::from_table_key(parts[0], parts[1])
    } else {
        RecordId::from_table_key("gafete", id_str)
    }
}

pub async fn is_gafete_disponible(numero: &str, tipo: &str) -> Result<bool, String> {
    match db::get_gafete(numero, tipo).await {
        Ok(Some(g)) => Ok(g.estado == GafeteEstado::Activo), // Logic can be more complex
        Ok(None) => Ok(false),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn marcar_en_uso(numero: &str, tipo: &str) -> Result<(), String> {
    match db::get_gafete(numero, tipo).await {
        Ok(Some(g)) => db::set_gafete_uso(&g.id, true).await.map_err(|e| e.to_string()),
        Ok(None) => Err("Gafete no encontrado".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn liberar_gafete(numero: &str, tipo: &str) -> Result<(), String> {
    match db::get_gafete(numero, tipo).await {
        Ok(Some(g)) => db::set_gafete_uso(&g.id, false).await.map_err(|e| e.to_string()),
        Ok(None) => Err("Gafete no encontrado".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn create_gafete(input: CreateGafeteInput) -> Result<GafeteResponse, String> {
    let tipo = input.tipo.parse::<TipoGafete>().map_err(|e| e.to_string())?;

    let dto = GafeteCreateDTO { numero: input.numero, tipo, estado: GafeteEstado::Activo };

    let gafete = db::create_gafete(dto).await.map_err(|e| e.to_string())?;
    Ok(GafeteResponse::from(gafete))
}

pub async fn create_gafete_range(input: CreateGafeteRangeInput) -> Result<i32, String> {
    let tipo = input.tipo.parse::<TipoGafete>().map_err(|e| e.to_string())?;
    let mut created = 0;

    for i in input.start..=input.end {
        let numero = if let Some(ref p) = input.prefix {
            if let Some(pad) = input.padding {
                format!("{}{:0width$}", p, i, width = pad)
            } else {
                format!("{}{}", p, i)
            }
        } else {
            i.to_string()
        };

        let dto = GafeteCreateDTO { numero, tipo: tipo.clone(), estado: GafeteEstado::Activo };

        if db::create_gafete(dto).await.is_ok() {
            created += 1;
        }
    }

    Ok(created)
}

pub async fn get_gafete_by_id(id_str: &str) -> Result<Option<GafeteResponse>, String> {
    let id = parse_gafete_id(id_str);
    db::find_by_id(&id).await.map(|opt| opt.map(GafeteResponse::from)).map_err(|e| e.to_string())
}

pub async fn get_all_gafetes() -> Result<Vec<GafeteResponse>, String> {
    let gafetes = db::get_all_gafetes().await.map_err(|e| e.to_string())?;
    Ok(gafetes.into_iter().map(GafeteResponse::from).collect())
}

pub async fn get_gafetes_disponibles(tipo_str: &str) -> Result<Vec<GafeteResponse>, String> {
    let gafetes = db::get_gafetes_disponibles(tipo_str).await.map_err(|e| e.to_string())?;
    Ok(gafetes.into_iter().map(GafeteResponse::from).collect())
}

pub async fn update_gafete_status(
    id_str: &str,
    estado: GafeteEstado,
) -> Result<GafeteResponse, String> {
    let id = parse_gafete_id(id_str);
    let gafete = db::update_estado(&id, estado.as_str()).await.map_err(|e| e.to_string())?;
    Ok(GafeteResponse::from(gafete))
}

pub async fn delete_gafete(id_str: &str) -> Result<(), String> {
    let id = parse_gafete_id(id_str);
    db::find_by_id(&id).await.map_err(|e| e.to_string())?.ok_or("Gafete no encontrado")?;
    db::delete_gafete_by_id(&id).await.map_err(|e| e.to_string())
}
