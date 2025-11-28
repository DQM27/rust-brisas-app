// ==========================================
// src/services/contratista_service.rs
// ==========================================
// Capa de servicio: orquesta dominio y db
// Contiene la lógica de negocio completa

use crate::domain::contratista as domain;
use crate::db::contratista_queries as db;
use crate::db::empresa_queries;
use crate::db::lista_negra_queries;
use crate::models::contratista::{
    ContratistaResponse, ContratistaListResponse,
    CreateContratistaInput, UpdateContratistaInput, CambiarEstadoInput,
    EstadoContratista,
};
use crate::services::search_service::SearchService;
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use std::sync::Arc;

// ==========================================
// CREAR CONTRATISTA
// ==========================================

pub async fn create_contratista(
    pool: &SqlitePool,
    search_service: &Arc<SearchService>,
    input: CreateContratistaInput,
) -> Result<ContratistaResponse, String> {
    // 1. Validar input
    domain::validar_create_input(&input)?;
    
    // 2. Normalizar datos
    let cedula_normalizada = domain::normalizar_cedula(&input.cedula);
    let nombre_normalizado = domain::normalizar_nombre(&input.nombre);
    let segundo_nombre_normalizado = domain::normalizar_segundo_nombre(input.segundo_nombre.as_ref());
    let apellido_normalizado = domain::normalizar_apellido(&input.apellido);
    let segundo_apellido_normalizado = domain::normalizar_segundo_apellido(input.segundo_apellido.as_ref());
    
    // 3. Verificar que NO esté en lista negra
    let block_status = lista_negra_queries::check_if_blocked_by_cedula(pool, &cedula_normalizada).await?;
    if block_status.blocked {
        let motivo = block_status.motivo.unwrap_or_else(|| "Sin motivo especificado".to_string());
        return Err(format!(
            "No se puede registrar. La persona con cédula {} está en lista negra. Motivo: {}",
            cedula_normalizada, motivo
        ));
    }
    
    // 4. Verificar que la cédula no exista
    let count = db::count_by_cedula(pool, &cedula_normalizada).await?;
    if count > 0 {
        return Err("Ya existe un contratista con esta cédula".to_string());
    }
    
    // 5. Verificar que la empresa exista
    let empresa_existe = empresa_queries::exists(pool, &input.empresa_id).await?;
    if !empresa_existe {
        return Err("La empresa especificada no existe".to_string());
    }
    
    // 6. Generar ID y timestamps
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    // 7. Insertar en DB
    db::insert(
        pool,
        &id,
        &cedula_normalizada,
        &nombre_normalizado,
        segundo_nombre_normalizado.as_deref(),
        &apellido_normalizado,
        segundo_apellido_normalizado.as_deref(),
        &input.empresa_id,
        &input.fecha_vencimiento_praind,
        EstadoContratista::Activo.as_str(),
        &now,
        &now,
    ).await?;
    
    // 8. Obtener contratista creado
    let response = get_contratista_by_id(pool, &id).await?;
    
    // 9. Indexar en Tantivy (automático)
    if let Ok((contratista, empresa_nombre)) = db::find_by_id_with_empresa(pool, &id).await {
        if let Err(e) = search_service.add_contratista(&contratista, &empresa_nombre) {
            eprintln!("⚠️ Error al indexar contratista {}: {}", id, e);
            // No fallamos la operación, solo logueamos el error
        }
    }
    
    Ok(response)
}

// ==========================================
// OBTENER CONTRATISTA POR ID
// ==========================================

pub async fn get_contratista_by_id(
    pool: &SqlitePool,
    id: &str,
) -> Result<ContratistaResponse, String> {
    let (contratista, empresa_nombre) = db::find_by_id_with_empresa(pool, id).await?;
    
    let mut response = ContratistaResponse::from(contratista);
    response.empresa_nombre = empresa_nombre;
    
    Ok(response)
}

// ==========================================
// OBTENER CONTRATISTA POR CÉDULA
// ==========================================

pub async fn get_contratista_by_cedula(
    pool: &SqlitePool,
    cedula: &str,
) -> Result<ContratistaResponse, String> {
    let (contratista, empresa_nombre) = db::find_by_cedula_with_empresa(pool, cedula).await?;
    
    let mut response = ContratistaResponse::from(contratista);
    response.empresa_nombre = empresa_nombre;
    
    Ok(response)
}

// ==========================================
// OBTENER TODOS LOS CONTRATISTAS
// ==========================================

pub async fn get_all_contratistas(pool: &SqlitePool) -> Result<ContratistaListResponse, String> {
    let contratistas_with_empresa = db::find_all_with_empresa(pool).await?;
    
    // Convertir a ContratistaResponse
    let contratistas: Vec<ContratistaResponse> = contratistas_with_empresa
        .into_iter()
        .map(|(contratista, empresa_nombre)| {
            let mut response = ContratistaResponse::from(contratista);
            response.empresa_nombre = empresa_nombre;
            response
        })
        .collect();
    
    // Calcular estadísticas
    let total = contratistas.len();
    let activos = contratistas.iter().filter(|c| c.estado == EstadoContratista::Activo).count();
    let con_praind_vencido = contratistas.iter().filter(|c| c.praind_vencido).count();
    let requieren_atencion = contratistas.iter().filter(|c| c.requiere_atencion).count();
    
    Ok(ContratistaListResponse {
        contratistas,
        total,
        activos,
        con_praind_vencido,
        requieren_atencion,
    })
}

// ==========================================
// OBTENER CONTRATISTAS ACTIVOS
// ==========================================

pub async fn get_contratistas_activos(pool: &SqlitePool) -> Result<Vec<ContratistaResponse>, String> {
    let contratistas_with_empresa = db::find_activos_with_empresa(pool).await?;
    
    let contratistas = contratistas_with_empresa
        .into_iter()
        .map(|(contratista, empresa_nombre)| {
            let mut response = ContratistaResponse::from(contratista);
            response.empresa_nombre = empresa_nombre;
            response
        })
        .collect();
    
    Ok(contratistas)
}

// ==========================================
// ACTUALIZAR CONTRATISTA
// ==========================================

pub async fn update_contratista(
    pool: &SqlitePool,
    search_service: &Arc<SearchService>,
    id: String,
    input: UpdateContratistaInput,
) -> Result<ContratistaResponse, String> {
    // 1. Validar input
    domain::validar_update_input(&input)?;
    
    // 2. Verificar que el contratista existe
    let _ = db::find_by_id_with_empresa(pool, &id).await?;
    
    // 3. Normalizar datos si vienen
    let nombre_normalizado = input.nombre
        .as_ref()
        .map(|n| domain::normalizar_nombre(n));
    
    let segundo_nombre_normalizado = input.segundo_nombre
        .as_ref()
        .map(|sn| domain::normalizar_segundo_nombre(Some(sn)))
        .flatten();
    
    let apellido_normalizado = input.apellido
        .as_ref()
        .map(|a| domain::normalizar_apellido(a));
    
    let segundo_apellido_normalizado = input.segundo_apellido
        .as_ref()
        .map(|sa| domain::normalizar_segundo_apellido(Some(sa)))
        .flatten();
    
    // 4. Verificar que la empresa exista si viene
    if let Some(ref empresa_id) = input.empresa_id {
        let empresa_existe = empresa_queries::exists(pool, empresa_id).await?;
        if !empresa_existe {
            return Err("La empresa especificada no existe".to_string());
        }
    }
    
    // 5. Timestamp de actualización
    let now = Utc::now().to_rfc3339();
    
    // 6. Actualizar en DB
    db::update(
        pool,
        &id,
        nombre_normalizado.as_deref(),
        segundo_nombre_normalizado.as_deref(),
        apellido_normalizado.as_deref(),
        segundo_apellido_normalizado.as_deref(),
        input.empresa_id.as_deref(),
        input.fecha_vencimiento_praind.as_deref(),
        &now,
    ).await?;
    
    // 7. Obtener contratista actualizado
    let response = get_contratista_by_id(pool, &id).await?;
    
    // 8. Actualizar índice de Tantivy (automático)
    if let Ok((contratista, empresa_nombre)) = db::find_by_id_with_empresa(pool, &id).await {
        if let Err(e) = search_service.update_contratista(&contratista, &empresa_nombre) {
            eprintln!("⚠️ Error al actualizar índice del contratista {}: {}", id, e);
        }
    }
    
    Ok(response)
}

// ==========================================
// CAMBIAR ESTADO
// ==========================================

pub async fn cambiar_estado_contratista(
    pool: &SqlitePool,
    search_service: &Arc<SearchService>,
    id: String,
    input: CambiarEstadoInput,
) -> Result<ContratistaResponse, String> {
    // 1. Validar estado
    let estado = domain::validar_estado(&input.estado)?;
    
    // 2. Verificar que el contratista existe
    let _ = db::find_by_id_with_empresa(pool, &id).await?;
    
    // 3. Timestamp de actualización
    let now = Utc::now().to_rfc3339();
    
    // 4. Actualizar estado en DB
    db::update_estado(pool, &id, estado.as_str(), &now).await?;
    
    // 5. Obtener contratista actualizado
    let response = get_contratista_by_id(pool, &id).await?;
    
    // 6. Actualizar índice de Tantivy (automático)
    if let Ok((contratista, empresa_nombre)) = db::find_by_id_with_empresa(pool, &id).await {
        if let Err(e) = search_service.update_contratista(&contratista, &empresa_nombre) {
            eprintln!("⚠️ Error al actualizar índice del contratista {}: {}", id, e);
        }
    }
    
    Ok(response)
}

// ==========================================
// ELIMINAR CONTRATISTA
// ==========================================

pub async fn delete_contratista(
    pool: &SqlitePool,
    search_service: &Arc<SearchService>,
    id: String,
) -> Result<(), String> {
    // Verificar que existe antes de eliminar
    let _ = db::find_by_id_with_empresa(pool, &id).await?;
    
    // Eliminar de DB
    db::delete(pool, &id).await?;
    
    // Eliminar del índice de Tantivy (automático)
    if let Err(e) = search_service.delete_contratista(&id) {
        eprintln!("⚠️ Error al eliminar del índice el contratista {}: {}", id, e);
    }
    
    Ok(())
}