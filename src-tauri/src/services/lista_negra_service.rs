// ==========================================
// src/services/lista_negra_service.rs
// ==========================================
// Capa de servicio: orquesta dominio, db
// Contiene la lógica de negocio completa

use crate::domain::lista_negra as domain;
use crate::db::lista_negra_queries as db;
use crate::models::lista_negra::{
    ListaNegraResponse, ListaNegraListResponse, BlockCheckResponse,
    AddToListaNegraInput, UpdateListaNegraInput,
};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;

// ==========================================
// AGREGAR A LISTA NEGRA
// ==========================================

pub async fn add_to_lista_negra(
    pool: &SqlitePool,
    input: AddToListaNegraInput,
) -> Result<ListaNegraResponse, String> {
    // 1. Validar input
    domain::validar_add_input(&input)?;
    
    // 2. Determinar datos según si tiene o no contratista_id
    let (contratista_id, cedula, nombre, apellido) = if let Some(ref cid) = input.contratista_id {
        // Caso 1: Tiene contratista_id - traer datos de la BD
        let (c, n, a) = db::get_contratista_data(pool, cid).await?;
        (Some(cid.clone()), c, n, a)
    } else {
        // Caso 2: Registro manual - usar datos proporcionados
        (
            None,
            input.cedula.clone().unwrap(),
            input.nombre.clone().unwrap(),
            input.apellido.clone().unwrap(),
        )
    };
    
    // 3. Verificar que no exista ya un bloqueo activo para esta cédula
    let count = db::count_active_by_cedula(pool, &cedula).await?;
    if count > 0 {
        return Err(format!("La persona con cédula {} ya está en la lista negra", cedula));
    }
    
    // 4. Normalizar datos
    let cedula_normalizada = domain::normalizar_texto(&cedula);
    let nombre_normalizado = domain::normalizar_texto(&nombre);
    let apellido_normalizado = domain::normalizar_texto(&apellido);
    let motivo_normalizado = domain::normalizar_texto(&input.motivo_bloqueo);
    let bloqueado_por_normalizado = domain::normalizar_texto(&input.bloqueado_por);
    
    let observaciones_normalizadas = input.observaciones
        .as_ref()
        .map(|o| domain::normalizar_texto(o))
        .filter(|o| !o.is_empty());
    
    // 5. Generar ID y timestamps
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    // 6. Insertar en DB
    db::insert(
        pool,
        &id,
        contratista_id.as_deref(),
        &cedula_normalizada,
        &nombre_normalizado,
        &apellido_normalizado,
        &motivo_normalizado,
        &now,
        input.fecha_fin_bloqueo.as_deref(),
        &bloqueado_por_normalizado,
        observaciones_normalizadas.as_deref(),
        &now,
        &now,
    ).await?;
    
    // 7. Retornar bloqueo creado con datos completos
    get_lista_negra_by_id(pool, &id).await
}

// ==========================================
// OBTENER POR ID
// ==========================================

pub async fn get_lista_negra_by_id(
    pool: &SqlitePool,
    id: &str,
) -> Result<ListaNegraResponse, String> {
    // Obtener bloqueo de DB
    let lista_negra = db::find_by_id(pool, id).await?;
    
    // Construir response
    let mut response = ListaNegraResponse::from(lista_negra.clone());
    
    // Si tiene contratista_id, obtener nombre de empresa
    if let Some(ref contratista_id) = lista_negra.contratista_id {
        let row = sqlx::query(
            r#"SELECT e.nombre as empresa_nombre
               FROM contratistas c
               INNER JOIN empresas e ON c.empresa_id = e.id
               WHERE c.id = ?"#
        )
        .bind(contratista_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Error al obtener datos de empresa: {}", e))?;
        
        if let Some(row) = row {
            use sqlx::Row;
            response.empresa_nombre = row.get("empresa_nombre");
        }
    }
    
    Ok(response)
}

// ==========================================
// OBTENER TODOS
// ==========================================

pub async fn get_all_lista_negra(pool: &SqlitePool) -> Result<ListaNegraListResponse, String> {
    let bloqueados_db = db::find_all(pool).await?;
    
    // Obtener datos de empresa para cada uno que tenga contratista_id
    let mut bloqueados = Vec::new();
    
    for lista_negra in bloqueados_db {
        let mut response = ListaNegraResponse::from(lista_negra.clone());
        
        if let Some(ref contratista_id) = lista_negra.contratista_id {
            let row = sqlx::query(
                r#"SELECT e.nombre as empresa_nombre
                   FROM contratistas c
                   INNER JOIN empresas e ON c.empresa_id = e.id
                   WHERE c.id = ?"#
            )
            .bind(contratista_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| format!("Error al obtener datos de empresa: {}", e))?;
            
            if let Some(row) = row {
                use sqlx::Row;
                response.empresa_nombre = row.get("empresa_nombre");
            }
        }
        
        bloqueados.push(response);
    }
    
    // Calcular estadísticas
    let total = bloqueados.len();
    let activos = bloqueados.iter().filter(|b| b.is_active).count();
    let permanentes = bloqueados.iter()
        .filter(|b| b.is_active && b.es_bloqueo_permanente)
        .count();
    let temporales = bloqueados.iter()
        .filter(|b| b.is_active && !b.es_bloqueo_permanente)
        .count();
    
    Ok(ListaNegraListResponse {
        bloqueados,
        total,
        activos,
        permanentes,
        temporales,
    })
}

// ==========================================
// OBTENER ACTIVOS
// ==========================================

pub async fn get_lista_negra_activos(pool: &SqlitePool) -> Result<Vec<ListaNegraResponse>, String> {
    let bloqueados_db = db::find_activos(pool).await?;
    
    let mut bloqueados = Vec::new();
    
    for lista_negra in bloqueados_db {
        let mut response = ListaNegraResponse::from(lista_negra.clone());
        
        if let Some(ref contratista_id) = lista_negra.contratista_id {
            let row = sqlx::query(
                r#"SELECT e.nombre as empresa_nombre
                   FROM contratistas c
                   INNER JOIN empresas e ON c.empresa_id = e.id
                   WHERE c.id = ?"#
            )
            .bind(contratista_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| format!("Error al obtener datos de empresa: {}", e))?;
            
            if let Some(row) = row {
                use sqlx::Row;
                response.empresa_nombre = row.get("empresa_nombre");
            }
        }
        
        bloqueados.push(response);
    }
    
    Ok(bloqueados)
}

// ==========================================
// VERIFICAR SI ESTÁ BLOQUEADO (CRÍTICO)
// ==========================================

pub async fn check_is_blocked(
    pool: &SqlitePool,
    cedula: String,
) -> Result<BlockCheckResponse, String> {
    let lista_negra_opt = db::find_active_by_cedula(pool, &cedula).await?;
    
    if let Some(lista_negra) = lista_negra_opt {
        Ok(BlockCheckResponse {
            is_blocked: true,
            motivo: Some(lista_negra.motivo_bloqueo),
            bloqueado_desde: Some(lista_negra.fecha_inicio_bloqueo),
            bloqueado_hasta: lista_negra.fecha_fin_bloqueo,
            bloqueado_por: Some(lista_negra.bloqueado_por),
        })
    } else {
        Ok(BlockCheckResponse {
            is_blocked: false,
            motivo: None,
            bloqueado_desde: None,
            bloqueado_hasta: None,
            bloqueado_por: None,
        })
    }
}

// ==========================================
// OBTENER POR CÉDULA
// ==========================================

pub async fn get_blocked_by_cedula(
    pool: &SqlitePool,
    cedula: String,
) -> Result<Option<ListaNegraResponse>, String> {
    let lista_negra_opt = db::find_active_by_cedula(pool, &cedula).await?;
    
    if let Some(lista_negra) = lista_negra_opt {
        let mut response = ListaNegraResponse::from(lista_negra.clone());
        
        if let Some(ref contratista_id) = lista_negra.contratista_id {
            let row = sqlx::query(
                r#"SELECT e.nombre as empresa_nombre
                   FROM contratistas c
                   INNER JOIN empresas e ON c.empresa_id = e.id
                   WHERE c.id = ?"#
            )
            .bind(contratista_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| format!("Error al obtener datos de empresa: {}", e))?;
            
            if let Some(row) = row {
                use sqlx::Row;
                response.empresa_nombre = row.get("empresa_nombre");
            }
        }
        
        Ok(Some(response))
    } else {
        Ok(None)
    }
}



// ==========================================
// DESACTIVAR BLOQUEO
// ==========================================

pub async fn remove_from_lista_negra(
    pool: &SqlitePool,
    id: String,
    motivo: String,             // <--- Nuevo parámetro
    observacion: Option<String> // <--- Nuevo parámetro
) -> Result<ListaNegraResponse, String> {
    // 1. Verificar que existe antes de desactivar
    let _ = db::find_by_id(pool, &id).await?;
    
    // 2. Normalizar datos (Igual que haces en el Add/Update)
    let motivo_normalizado = domain::normalizar_texto(&motivo);
    
    let observacion_normalizada = observacion
        .map(|o| domain::normalizar_texto(&o))
        .filter(|o| !o.is_empty());

    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    // 3. Desactivar y actualizar motivo/observaciones
    // NOTA: Aquí asumimos que actualizaste la firma de db::deactivate
    db::deactivate(
        pool, 
        &id, 
        &motivo_normalizado, 
        observacion_normalizada.as_deref(), 
        &now
    ).await?;
    
    // 4. Retornar actualizado
    get_lista_negra_by_id(pool, &id).await
}
// ==========================================
// ACTUALIZAR BLOQUEO
// ==========================================

pub async fn update_lista_negra(
    pool: &SqlitePool,
    id: String,
    input: UpdateListaNegraInput,
) -> Result<ListaNegraResponse, String> {
    // 1. Validar input
    domain::validar_update_input(&input)?;
    
    // 2. Verificar que existe
    let _ = db::find_by_id(pool, &id).await?;
    
    // 3. Normalizar datos si vienen
    let motivo_normalizado = input.motivo_bloqueo
        .as_ref()
        .map(|m| domain::normalizar_texto(m))
        .filter(|m| !m.is_empty());
    
    let observaciones_normalizadas = input.observaciones
        .as_ref()
        .map(|o| domain::normalizar_texto(o))
        .filter(|o| !o.is_empty());
    
    // 4. Timestamp de actualización
    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    // 5. Actualizar en DB
    db::update(
        pool,
        &id,
        motivo_normalizado.as_deref(),
        input.fecha_fin_bloqueo.as_deref(),
        observaciones_normalizadas.as_deref(),
        &now,
    ).await?;
    
    // 6. Retornar actualizado
    get_lista_negra_by_id(pool, &id).await
}

// ==========================================
// ELIMINAR PERMANENTEMENTE
// ==========================================

pub async fn delete_lista_negra(pool: &SqlitePool, id: String) -> Result<(), String> {
    // Verificar que existe antes de eliminar
    let _ = db::find_by_id(pool, &id).await?;
    
    // Eliminar
    db::delete(pool, &id).await
}