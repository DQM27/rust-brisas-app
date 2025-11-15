// ==========================================
// src/commands/lista_negra_commands.rs
// ==========================================
use crate::models::lista_negra::{
    ListaNegra, ListaNegraResponse, ListaNegraListResponse, BlockCheckResponse,
    AddToListaNegraInput, UpdateListaNegraInput, validaciones,
};
use sqlx::{SqlitePool, Row};
use tauri::State;
use uuid::Uuid;
use chrono::Utc;

/// Agrega una persona a la lista negra
#[tauri::command]
pub async fn add_to_lista_negra(
    pool: State<'_, SqlitePool>,
    input: AddToListaNegraInput,
) -> Result<ListaNegraResponse, String> {
    // Validar input
    validaciones::validar_add_input(&input)?;
    
    let (contratista_id, cedula, nombre, apellido) = if let Some(ref cid) = input.contratista_id {
        // Caso 1: Tiene contratista_id - traer datos de la BD
        let row = sqlx::query(
            "SELECT cedula, nombre, apellido FROM contratistas WHERE id = ?"
        )
        .bind(cid)
        .fetch_one(&*pool)
        .await
        .map_err(|_| "El contratista especificado no existe".to_string())?;
        
        (
            Some(cid.clone()),
            row.get::<String, _>("cedula"),
            row.get::<String, _>("nombre"),
            row.get::<String, _>("apellido"),
        )
    } else {
        // Caso 2: Registro manual - usar datos proporcionados
        (
            None,
            input.cedula.clone().unwrap(),
            input.nombre.clone().unwrap(),
            input.apellido.clone().unwrap(),
        )
    };
    
    // Verificar que no exista ya un bloqueo activo para esta cédula
    let existe = sqlx::query(
        "SELECT COUNT(*) as count FROM lista_negra WHERE cedula = ? AND is_active = 1"
    )
    .bind(&cedula)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Error al verificar bloqueo existente: {}", e))?;
    
    let count: i32 = existe.get("count");
    if count > 0 {
        return Err(format!("La persona con cédula {} ya está en la lista negra", cedula));
    }
    
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    sqlx::query(
        r#"INSERT INTO lista_negra 
           (id, contratista_id, cedula, nombre, apellido, motivo_bloqueo, 
            fecha_inicio_bloqueo, fecha_fin_bloqueo, bloqueado_por, observaciones, 
            is_active, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 1, ?, ?)"#
    )
    .bind(&id)
    .bind(&contratista_id)
    .bind(&cedula)
    .bind(&nombre)
    .bind(&apellido)
    .bind(input.motivo_bloqueo.trim())
    .bind(&now)
    .bind(&input.fecha_fin_bloqueo)
    .bind(input.bloqueado_por.trim())
    .bind(input.observaciones.as_deref().map(|s| s.trim()))
    .bind(&now)
    .bind(&now)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Error al agregar a lista negra: {}", e))?;
    
    get_lista_negra_by_id(pool, id).await
}

/// Obtiene un registro de lista negra por ID
#[tauri::command]
pub async fn get_lista_negra_by_id(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<ListaNegraResponse, String> {
    let row = sqlx::query(
        r#"SELECT 
            ln.id, ln.contratista_id, ln.cedula, ln.nombre, ln.apellido,
            ln.motivo_bloqueo, ln.fecha_inicio_bloqueo, ln.fecha_fin_bloqueo,
            ln.bloqueado_por, ln.observaciones, ln.is_active, 
            ln.created_at, ln.updated_at,
            e.nombre as empresa_nombre
           FROM lista_negra ln
           LEFT JOIN contratistas c ON ln.contratista_id = c.id
           LEFT JOIN empresas e ON c.empresa_id = e.id
           WHERE ln.id = ?"#
    )
    .bind(&id)
    .fetch_one(&*pool)
    .await
    .map_err(|_| "Registro no encontrado".to_string())?;
    
    let lista_negra = ListaNegra {
        id: row.get("id"),
        contratista_id: row.get("contratista_id"),
        cedula: row.get("cedula"),
        nombre: row.get("nombre"),
        apellido: row.get("apellido"),
        motivo_bloqueo: row.get("motivo_bloqueo"),
        fecha_inicio_bloqueo: row.get("fecha_inicio_bloqueo"),
        fecha_fin_bloqueo: row.get("fecha_fin_bloqueo"),
        bloqueado_por: row.get("bloqueado_por"),
        observaciones: row.get("observaciones"),
        is_active: row.get("is_active"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    };
    
    let mut response = ListaNegraResponse::from(lista_negra);
    response.empresa_nombre = row.get("empresa_nombre");
    
    Ok(response)
}

/// Obtiene todos los registros de lista negra
#[tauri::command]
pub async fn get_all_lista_negra(
    pool: State<'_, SqlitePool>,
) -> Result<ListaNegraListResponse, String> {
    let rows = sqlx::query(
        r#"SELECT 
            ln.id, ln.contratista_id, ln.cedula, ln.nombre, ln.apellido,
            ln.motivo_bloqueo, ln.fecha_inicio_bloqueo, ln.fecha_fin_bloqueo,
            ln.bloqueado_por, ln.observaciones, ln.is_active, 
            ln.created_at, ln.updated_at,
            e.nombre as empresa_nombre
           FROM lista_negra ln
           LEFT JOIN contratistas c ON ln.contratista_id = c.id
           LEFT JOIN empresas e ON c.empresa_id = e.id
           ORDER BY ln.created_at DESC"#
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error al obtener lista negra: {}", e))?;
    
    let bloqueados: Vec<ListaNegraResponse> = rows.into_iter()
        .map(|row| {
            let lista_negra = ListaNegra {
                id: row.get("id"),
                contratista_id: row.get("contratista_id"),
                cedula: row.get("cedula"),
                nombre: row.get("nombre"),
                apellido: row.get("apellido"),
                motivo_bloqueo: row.get("motivo_bloqueo"),
                fecha_inicio_bloqueo: row.get("fecha_inicio_bloqueo"),
                fecha_fin_bloqueo: row.get("fecha_fin_bloqueo"),
                bloqueado_por: row.get("bloqueado_por"),
                observaciones: row.get("observaciones"),
                is_active: row.get("is_active"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            
            let mut response = ListaNegraResponse::from(lista_negra);
            response.empresa_nombre = row.get("empresa_nombre");
            response
        })
        .collect();
    
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

/// Obtiene solo los registros activos de lista negra
#[tauri::command]
pub async fn get_lista_negra_activos(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<ListaNegraResponse>, String> {
    let rows = sqlx::query(
        r#"SELECT 
            ln.id, ln.contratista_id, ln.cedula, ln.nombre, ln.apellido,
            ln.motivo_bloqueo, ln.fecha_inicio_bloqueo, ln.fecha_fin_bloqueo,
            ln.bloqueado_por, ln.observaciones, ln.is_active, 
            ln.created_at, ln.updated_at,
            e.nombre as empresa_nombre
           FROM lista_negra ln
           LEFT JOIN contratistas c ON ln.contratista_id = c.id
           LEFT JOIN empresas e ON c.empresa_id = e.id
           WHERE ln.is_active = 1
           ORDER BY ln.fecha_inicio_bloqueo DESC"#
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error al obtener bloqueados activos: {}", e))?;
    
    let bloqueados = rows.into_iter()
        .map(|row| {
            let lista_negra = ListaNegra {
                id: row.get("id"),
                contratista_id: row.get("contratista_id"),
                cedula: row.get("cedula"),
                nombre: row.get("nombre"),
                apellido: row.get("apellido"),
                motivo_bloqueo: row.get("motivo_bloqueo"),
                fecha_inicio_bloqueo: row.get("fecha_inicio_bloqueo"),
                fecha_fin_bloqueo: row.get("fecha_fin_bloqueo"),
                bloqueado_por: row.get("bloqueado_por"),
                observaciones: row.get("observaciones"),
                is_active: row.get("is_active"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            
            let mut response = ListaNegraResponse::from(lista_negra);
            response.empresa_nombre = row.get("empresa_nombre");
            response
        })
        .collect();
    
    Ok(bloqueados)
}

/// Verifica si una cédula está bloqueada (CRÍTICO para validaciones)
#[tauri::command]
pub async fn check_is_blocked(
    pool: State<'_, SqlitePool>,
    cedula: String,
) -> Result<BlockCheckResponse, String> {
    let row = sqlx::query(
        r#"SELECT motivo_bloqueo, fecha_inicio_bloqueo, fecha_fin_bloqueo, bloqueado_por
           FROM lista_negra 
           WHERE cedula = ? AND is_active = 1
           LIMIT 1"#
    )
    .bind(&cedula)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Error al verificar bloqueo: {}", e))?;
    
    if let Some(row) = row {
        Ok(BlockCheckResponse {
            is_blocked: true,
            motivo: Some(row.get("motivo_bloqueo")),
            bloqueado_desde: Some(row.get("fecha_inicio_bloqueo")),
            bloqueado_hasta: row.get("fecha_fin_bloqueo"),
            bloqueado_por: Some(row.get("bloqueado_por")),
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

/// Obtiene información de bloqueo por cédula
#[tauri::command]
pub async fn get_blocked_by_cedula(
    pool: State<'_, SqlitePool>,
    cedula: String,
) -> Result<Option<ListaNegraResponse>, String> {
    let row = sqlx::query(
        r#"SELECT 
            ln.id, ln.contratista_id, ln.cedula, ln.nombre, ln.apellido,
            ln.motivo_bloqueo, ln.fecha_inicio_bloqueo, ln.fecha_fin_bloqueo,
            ln.bloqueado_por, ln.observaciones, ln.is_active, 
            ln.created_at, ln.updated_at,
            e.nombre as empresa_nombre
           FROM lista_negra ln
           LEFT JOIN contratistas c ON ln.contratista_id = c.id
           LEFT JOIN empresas e ON c.empresa_id = e.id
           WHERE ln.cedula = ? AND ln.is_active = 1
           LIMIT 1"#
    )
    .bind(&cedula)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Error al buscar bloqueo: {}", e))?;
    
    if let Some(row) = row {
        let lista_negra = ListaNegra {
            id: row.get("id"),
            contratista_id: row.get("contratista_id"),
            cedula: row.get("cedula"),
            nombre: row.get("nombre"),
            apellido: row.get("apellido"),
            motivo_bloqueo: row.get("motivo_bloqueo"),
            fecha_inicio_bloqueo: row.get("fecha_inicio_bloqueo"),
            fecha_fin_bloqueo: row.get("fecha_fin_bloqueo"),
            bloqueado_por: row.get("bloqueado_por"),
            observaciones: row.get("observaciones"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };
        
        let mut response = ListaNegraResponse::from(lista_negra);
        response.empresa_nombre = row.get("empresa_nombre");
        
        Ok(Some(response))
    } else {
        Ok(None)
    }
}

/// Desactiva un bloqueo (quita de lista negra)
#[tauri::command]
pub async fn remove_from_lista_negra(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<ListaNegraResponse, String> {
    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    sqlx::query(
        "UPDATE lista_negra SET is_active = 0, updated_at = ? WHERE id = ?"
    )
    .bind(&now)
    .bind(&id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Error al desactivar bloqueo: {}", e))?;
    
    get_lista_negra_by_id(pool, id).await
}

/// Actualiza información de un bloqueo
#[tauri::command]
pub async fn update_lista_negra(
    pool: State<'_, SqlitePool>,
    id: String,
    input: UpdateListaNegraInput,
) -> Result<ListaNegraResponse, String> {
    // Validar motivo si se proporciona
    if let Some(ref motivo) = input.motivo_bloqueo {
        validaciones::validar_motivo(motivo)?;
    }
    
    // Validar fecha_fin si se proporciona
    if let Some(ref fecha) = input.fecha_fin_bloqueo {
        validaciones::validar_fecha_fin(fecha)?;
    }
    
    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    sqlx::query(
        r#"UPDATE lista_negra SET
            motivo_bloqueo = COALESCE(?, motivo_bloqueo),
            fecha_fin_bloqueo = COALESCE(?, fecha_fin_bloqueo),
            observaciones = COALESCE(?, observaciones),
            updated_at = ?
        WHERE id = ?"#
    )
    .bind(input.motivo_bloqueo.as_deref().map(|s| s.trim()))
    .bind(&input.fecha_fin_bloqueo)
    .bind(input.observaciones.as_deref().map(|s| s.trim()))
    .bind(&now)
    .bind(&id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Error al actualizar bloqueo: {}", e))?;
    
    get_lista_negra_by_id(pool, id).await
}

/// Elimina permanentemente un registro de lista negra
#[tauri::command]
pub async fn delete_lista_negra(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    sqlx::query("DELETE FROM lista_negra WHERE id = ?")
        .bind(&id)
        .execute(&*pool)
        .await
        .map_err(|e| format!("Error al eliminar registro: {}", e))?;
    
    Ok(())
}