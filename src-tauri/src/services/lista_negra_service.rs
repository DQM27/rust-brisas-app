// ==========================================
// src/services/lista_negra_service.rs
// ==========================================
// Servicio de Lista Negra - Bloqueo universal por cédula

use crate::db::lista_negra_queries as db;
use crate::domain::errors::ListaNegraError;
use crate::domain::lista_negra as domain;
use crate::models::lista_negra::{
    AddToListaNegraInput, BlockCheckResponse, ListaNegraListResponse, ListaNegraResponse,
    NivelStats, UpdateListaNegraInput,
};
use crate::services::search_service::SearchService;
use chrono::Utc;
use log::warn;
use sqlx::SqlitePool;
use std::sync::Arc;
use uuid::Uuid;

// ==========================================
// AGREGAR A LISTA NEGRA
// ==========================================

pub async fn add_to_lista_negra(
    pool: &SqlitePool,
    search_service: &Arc<SearchService>,
    input: AddToListaNegraInput,
) -> Result<ListaNegraResponse, ListaNegraError> {
    // 1. Validar input
    domain::validar_add_input(&input)?;

    // 2. Verificar que no exista ya un bloqueo activo para esta cédula
    let count = db::count_active_by_cedula(pool, &input.cedula).await?;
    if count > 0 {
        return Err(ListaNegraError::AlreadyExists);
    }

    // 3. Normalizar datos
    let cedula_normalizada = domain::normalizar_texto(&input.cedula);
    let nombre_normalizado = domain::normalizar_texto(&input.nombre);
    let segundo_nombre_normalizado = input
        .segundo_nombre
        .as_ref()
        .map(|s| domain::normalizar_texto(s))
        .filter(|s| !s.is_empty());
    let apellido_normalizado = domain::normalizar_texto(&input.apellido);
    let segundo_apellido_normalizado = input
        .segundo_apellido
        .as_ref()
        .map(|s| domain::normalizar_texto(s))
        .filter(|s| !s.is_empty());

    let nivel_normalizado = input.nivel_severidad.to_uppercase();
    let motivo_normalizado = domain::normalizar_texto(&input.motivo_bloqueo);
    let bloqueado_por_normalizado = domain::normalizar_texto(&input.bloqueado_por);

    let observaciones_normalizadas =
        input.observaciones.as_ref().map(|o| domain::normalizar_texto(o)).filter(|o| !o.is_empty());

    // 4. Generar ID y timestamps
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    // 5. Insertar en DB
    db::insert(
        pool,
        &id,
        &cedula_normalizada,
        &nombre_normalizado,
        segundo_nombre_normalizado.as_deref(),
        &apellido_normalizado,
        segundo_apellido_normalizado.as_deref(),
        input.empresa_id.as_deref(),
        input.empresa_nombre.as_deref(),
        &nivel_normalizado,
        &motivo_normalizado,
        &bloqueado_por_normalizado,
        observaciones_normalizadas.as_deref(),
        &now,
        &now,
    )
    .await?;

    // 6. Indexar en Tantivy
    match db::find_by_id(pool, &id).await {
        Ok(lista_negra) => {
            if let Err(e) = search_service.add_lista_negra(&lista_negra).await {
                warn!("Error al indexar lista negra {}: {}", id, e);
            }
        }
        Err(e) => warn!("Error al obtener lista negra para indexar {}: {}", id, e),
    }

    // 7. Retornar bloqueo creado
    get_lista_negra_by_id(pool, &id).await
}

// ==========================================
// OBTENER POR ID
// ==========================================

pub async fn get_lista_negra_by_id(
    pool: &SqlitePool,
    id: &str,
) -> Result<ListaNegraResponse, ListaNegraError> {
    let lista_negra = db::find_by_id(pool, id).await.map_err(|e| match e {
        sqlx::Error::RowNotFound => ListaNegraError::NotFound,
        e => ListaNegraError::Database(e),
    })?;

    Ok(ListaNegraResponse::from(lista_negra))
}

// ==========================================
// OBTENER TODOS
// ==========================================

pub async fn get_all_lista_negra(
    pool: &SqlitePool,
) -> Result<ListaNegraListResponse, ListaNegraError> {
    let bloqueados_db = db::find_activos(pool).await?;

    let bloqueados: Vec<ListaNegraResponse> =
        bloqueados_db.into_iter().map(ListaNegraResponse::from).collect();

    let total = bloqueados.len();
    let activos = bloqueados.iter().filter(|b| b.is_active).count();

    let alto = bloqueados.iter().filter(|b| b.nivel_severidad == "ALTO").count();
    let medio = bloqueados.iter().filter(|b| b.nivel_severidad == "MEDIO").count();
    let bajo = bloqueados.iter().filter(|b| b.nivel_severidad == "BAJO").count();

    Ok(ListaNegraListResponse {
        bloqueados,
        total,
        activos,
        por_nivel: NivelStats { alto, medio, bajo },
    })
}

// ==========================================
// OBTENER ACTIVOS
// ==========================================

pub async fn get_lista_negra_activos(
    pool: &SqlitePool,
) -> Result<Vec<ListaNegraResponse>, ListaNegraError> {
    let bloqueados_db = db::find_activos(pool).await?;

    let bloqueados: Vec<ListaNegraResponse> =
        bloqueados_db.into_iter().map(ListaNegraResponse::from).collect();

    Ok(bloqueados)
}

// ==========================================
// VERIFICAR SI ESTÁ BLOQUEADO (PARA GUARDIAS)
// ==========================================

/// Verifica si una cédula está bloqueada
/// Retorna info mínima: solo nivel de severidad (para color) y fecha
/// NO retorna motivo (privacidad)
pub async fn check_is_blocked(
    pool: &SqlitePool,
    cedula: String,
) -> Result<BlockCheckResponse, ListaNegraError> {
    let status = db::check_if_blocked_by_cedula(pool, &cedula).await?;

    Ok(BlockCheckResponse {
        is_blocked: status.blocked,
        nivel_severidad: status.nivel_severidad,
        bloqueado_desde: status.bloqueado_desde,
    })
}

// ==========================================
// OBTENER BLOQUEO POR CÉDULA (PARA ADMINS)
// ==========================================

pub async fn get_blocked_by_cedula(
    pool: &SqlitePool,
    cedula: String,
) -> Result<Option<ListaNegraResponse>, ListaNegraError> {
    let lista_negra_opt = db::find_active_by_cedula(pool, &cedula).await?;

    Ok(lista_negra_opt.map(ListaNegraResponse::from))
}

// ==========================================
// DESACTIVAR BLOQUEO
// ==========================================

pub async fn remove_from_lista_negra(
    pool: &SqlitePool,
    search_service: &Arc<SearchService>,
    id: String,
) -> Result<ListaNegraResponse, ListaNegraError> {
    // 1. Verificar que existe
    let _ = db::find_by_id(pool, &id).await.map_err(|e| match e {
        sqlx::Error::RowNotFound => ListaNegraError::NotFound,
        e => ListaNegraError::Database(e),
    })?;

    // 2. Desactivar (no borrar, para mantener historial implícito)
    let now = Utc::now().to_rfc3339();
    db::deactivate(pool, &id, &now).await?;

    // 3. Actualizar índice
    match db::find_by_id(pool, &id).await {
        Ok(lista_negra) => {
            if let Err(e) = search_service.update_lista_negra(&lista_negra).await {
                warn!("Error al actualizar índice lista negra {}: {}", id, e);
            }
        }
        Err(e) => warn!("Error al obtener lista negra para actualizar índice {}: {}", id, e),
    }

    get_lista_negra_by_id(pool, &id).await
}

// ==========================================
// REACTIVAR BLOQUEO
// ==========================================

pub async fn reactivate_lista_negra(
    pool: &SqlitePool,
    search_service: &Arc<SearchService>,
    id: String,
    nivel_severidad: String,
    motivo_bloqueo: String,
    bloqueado_por: String,
) -> Result<ListaNegraResponse, ListaNegraError> {
    // 1. Validar nivel
    domain::validar_nivel_severidad(&nivel_severidad)?;
    domain::validar_motivo(&motivo_bloqueo)?;
    domain::validar_bloqueado_por(&bloqueado_por)?;

    // 2. Verificar que existe
    let existing = db::find_by_id(pool, &id).await.map_err(|e| match e {
        sqlx::Error::RowNotFound => ListaNegraError::NotFound,
        e => ListaNegraError::Database(e),
    })?;

    // 3. Verificar que no haya otro bloqueo activo para esa cédula
    let count = db::count_active_by_cedula(pool, &existing.cedula).await?;
    if count > 0 && existing.is_active {
        return Err(ListaNegraError::AlreadyExists);
    }

    // 4. Reactivar
    let now = Utc::now().to_rfc3339();
    db::reactivate(
        pool,
        &id,
        &nivel_severidad.to_uppercase(),
        &motivo_bloqueo,
        &bloqueado_por,
        &now,
    )
    .await?;

    // 5. Actualizar índice
    match db::find_by_id(pool, &id).await {
        Ok(lista_negra) => {
            if let Err(e) = search_service.update_lista_negra(&lista_negra).await {
                warn!("Error al actualizar índice lista negra {}: {}", id, e);
            }
        }
        Err(e) => warn!("Error al obtener lista negra para actualizar índice {}: {}", id, e),
    }

    get_lista_negra_by_id(pool, &id).await
}

// ==========================================
// ACTUALIZAR BLOQUEO
// ==========================================

pub async fn update_lista_negra(
    pool: &SqlitePool,
    search_service: &Arc<SearchService>,
    id: String,
    input: UpdateListaNegraInput,
) -> Result<ListaNegraResponse, ListaNegraError> {
    // 1. Validar input
    domain::validar_update_input(&input)?;

    // 2. Verificar que existe
    let _ = db::find_by_id(pool, &id).await.map_err(|e| match e {
        sqlx::Error::RowNotFound => ListaNegraError::NotFound,
        e => ListaNegraError::Database(e),
    })?;

    // 3. Actualizar
    let now = Utc::now().to_rfc3339();
    db::update(
        pool,
        &id,
        input.nivel_severidad.as_deref(),
        input.motivo_bloqueo.as_deref(),
        input.observaciones.as_deref(),
        &now,
    )
    .await?;

    // 4. Actualizar índice
    match db::find_by_id(pool, &id).await {
        Ok(lista_negra) => {
            if let Err(e) = search_service.update_lista_negra(&lista_negra).await {
                warn!("Error al actualizar índice lista negra {}: {}", id, e);
            }
        }
        Err(e) => warn!("Error al obtener lista negra para actualizar índice {}: {}", id, e),
    }

    get_lista_negra_by_id(pool, &id).await
}

// ==========================================
// ELIMINAR PERMANENTEMENTE
// ==========================================

pub async fn delete_lista_negra(
    pool: &SqlitePool,
    search_service: &Arc<SearchService>,
    id: String,
) -> Result<(), ListaNegraError> {
    // 1. Verificar que existe
    let _ = db::find_by_id(pool, &id).await.map_err(|e| match e {
        sqlx::Error::RowNotFound => ListaNegraError::NotFound,
        e => ListaNegraError::Database(e),
    })?;

    // 2. Eliminar de índice
    if let Err(e) = search_service.delete_lista_negra(&id).await {
        warn!("Error al eliminar de índice lista negra {}: {}", id, e);
    }

    // 3. Eliminar de BD
    db::delete(pool, &id).await?;

    Ok(())
}
