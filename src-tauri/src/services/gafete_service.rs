// ==========================================
// src/services/gafete_service.rs
// ==========================================
// Orquesta dominio y DB - Lógica de negocio completa

use crate::db::gafete_queries as db;
use crate::domain::gafete as domain;
use crate::models::gafete::{
    CreateGafeteInput, GafeteListResponse, GafeteResponse, StatsGafetes, StatsPorTipo, TipoGafete,
    UpdateGafeteInput,
};
use chrono::Utc;
use sqlx::SqlitePool;

// ==========================================
// CREAR GAFETE
// ==========================================

pub async fn create_gafete(
    pool: &SqlitePool,
    input: CreateGafeteInput,
) -> Result<GafeteResponse, String> {
    // 1. Validar input
    domain::validar_create_input(&input)?;

    // 2. Normalizar número
    let numero_normalizado = domain::normalizar_numero(&input.numero);

    // 3. Verificar que no exista
    // 3. Verificar que no exista con este número + tipo
let tipo = TipoGafete::from_str(&input.tipo)?;
let exists = db::exists_by_numero_and_tipo(pool, &numero_normalizado, tipo.as_str()).await?;
if exists {
    return Err(format!(
        "Ya existe un gafete con el número {} de tipo {}",
        numero_normalizado, input.tipo
    ));
}

    // 4. Parsear tipo
    let tipo = TipoGafete::from_str(&input.tipo)?;

    // 5. Timestamps
    let now = Utc::now().to_rfc3339();

    // 6. Insertar
    db::insert(pool, &numero_normalizado, tipo.as_str(), &now, &now).await?;

    // 7. Retornar
    get_gafete(pool, &numero_normalizado).await
}

// ==========================================
// OBTENER GAFETE
// ==========================================

pub async fn get_gafete(pool: &SqlitePool, numero: &str) -> Result<GafeteResponse, String> {
    let gafete = db::find_by_numero(pool, numero).await?;
    let en_uso = db::is_en_uso(pool, numero).await?;

    let mut response = GafeteResponse::from(gafete);
    response.esta_disponible = !en_uso;

    Ok(response)
}

// ==========================================
// OBTENER TODOS
// ==========================================

pub async fn get_all_gafetes(pool: &SqlitePool) -> Result<GafeteListResponse, String> {
    let gafetes = db::find_all(pool).await?;

    // Calcular disponibilidad para cada uno
    let mut responses = Vec::new();
    for gafete in gafetes {
        let en_uso = db::is_en_uso(pool, &gafete.numero).await?;
        let mut response = GafeteResponse::from(gafete);
        response.esta_disponible = !en_uso;
        responses.push(response);
    }

    // Stats
    let total = responses.len();
    let disponibles = responses.iter().filter(|g| g.esta_disponible).count();
    let en_uso = total - disponibles;

    let contratistas = responses
        .iter()
        .filter(|g| g.tipo == TipoGafete::Contratista)
        .count();
    let proveedores = responses
        .iter()
        .filter(|g| g.tipo == TipoGafete::Proveedor)
        .count();
    let visitas = responses
        .iter()
        .filter(|g| g.tipo == TipoGafete::Visita)
        .count();
    let otros = responses
        .iter()
        .filter(|g| g.tipo == TipoGafete::Otro)
        .count();

    Ok(GafeteListResponse {
        gafetes: responses,
        total,
        stats: StatsGafetes {
            total,
            disponibles,
            en_uso,
            por_tipo: StatsPorTipo {
                contratistas,
                proveedores,
                visitas,
                otros,
            },
        },
    })
}

// ==========================================
// OBTENER DISPONIBLES POR TIPO
// ==========================================

pub async fn get_gafetes_disponibles(
    pool: &SqlitePool,
    tipo: TipoGafete,
) -> Result<Vec<GafeteResponse>, String> {
    let numeros = db::find_disponibles_by_tipo(pool, tipo.as_str()).await?;

    let mut responses = Vec::new();
    for numero in numeros {
        let gafete = db::find_by_numero(pool, &numero).await?;
        let mut response = GafeteResponse::from(gafete);
        response.esta_disponible = true; // Ya lo filtramos
        responses.push(response);
    }

    Ok(responses)
}

// ==========================================
// VERIFICAR DISPONIBILIDAD
// ==========================================

pub async fn is_gafete_disponible(pool: &SqlitePool, numero: &str) -> Result<bool, String> {
    let en_uso = db::is_en_uso(pool, numero).await?;
    Ok(!en_uso)
}

// ==========================================
// ACTUALIZAR
// ==========================================

pub async fn update_gafete(
    pool: &SqlitePool,
    numero: String,
    input: UpdateGafeteInput,
) -> Result<GafeteResponse, String> {
    // 1. Validar input
    domain::validar_update_input(&input)?;

    // 2. Verificar que existe
    let _ = db::find_by_numero(pool, &numero).await?;

    // 3. Parsear tipo si viene
    let tipo_str = if let Some(ref t) = input.tipo {
        Some(TipoGafete::from_str(t)?.as_str().to_string())
    } else {
        None
    };

    // 4. Timestamp
    let now = Utc::now().to_rfc3339();

    // 5. Actualizar
    db::update(pool, &numero, tipo_str.as_deref(), &now).await?;

    // 6. Retornar
    get_gafete(pool, &numero).await
}

// ==========================================
// ELIMINAR
// ==========================================

pub async fn delete_gafete(pool: &SqlitePool, numero: String) -> Result<(), String> {
    // 1. Verificar que existe
    let _ = db::find_by_numero(pool, &numero).await?;

    // 2. Verificar que no esté en uso
    let en_uso = db::is_en_uso(pool, &numero).await?;
    if en_uso {
        return Err("No se puede eliminar un gafete en uso".to_string());
    }

    // 3. Eliminar
    db::delete(pool, &numero).await
}
