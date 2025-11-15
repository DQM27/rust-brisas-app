// ==========================================
// src/commands/contratista_commands.rs
// ==========================================
use crate::models::contratista::{
    Contratista, ContratistaResponse, ContratistaListResponse,
    CreateContratistaInput, UpdateContratistaInput, CambiarEstadoInput,
    EstadoContratista, validaciones,
};
use sqlx::{SqlitePool, Row};
use tauri::State;
use uuid::Uuid;
use chrono::Utc;

#[tauri::command]
pub async fn create_contratista(
    pool: State<'_, SqlitePool>,
    input: CreateContratistaInput,
) -> Result<ContratistaResponse, String> {
    // Validar input
    validaciones::validar_create_input(&input)?;
    
    // Verificar que la cédula no exista
    let existe = sqlx::query("SELECT COUNT(*) as count FROM contratistas WHERE cedula = ?")
        .bind(&input.cedula)
        .fetch_one(&*pool)
        .await
        .map_err(|e| format!("Error al verificar cédula: {}", e))?;
    
    let count: i32 = existe.get("count");
    if count > 0 {
        return Err("Ya existe un contratista con esta cédula".to_string());
    }
    
    // Verificar que la empresa exista
    let empresa_existe = sqlx::query("SELECT COUNT(*) as count FROM empresas WHERE id = ?")
        .bind(&input.empresa_id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| format!("Error al verificar empresa: {}", e))?;
    
    let emp_count: i32 = empresa_existe.get("count");
    if emp_count == 0 {
        return Err("La empresa especificada no existe".to_string());
    }
    
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    sqlx::query(
        r#"INSERT INTO contratistas 
           (id, cedula, nombre, apellido, empresa_id, fecha_vencimiento_praind, estado, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#
    )
    .bind(&id)
    .bind(input.cedula.trim())
    .bind(input.nombre.trim())
    .bind(input.apellido.trim())
    .bind(&input.empresa_id)
    .bind(&input.fecha_vencimiento_praind)
    .bind(EstadoContratista::Activo.as_str())
    .bind(&now)
    .bind(&now)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Error al crear contratista: {}", e))?;
    
    get_contratista_by_id(pool, id).await
}

#[tauri::command]
pub async fn get_contratista_by_id(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<ContratistaResponse, String> {
    let row = sqlx::query(
        r#"SELECT 
            c.id, c.cedula, c.nombre, c.apellido, c.empresa_id,
            c.fecha_vencimiento_praind, c.estado, c.created_at, c.updated_at,
            e.nombre as empresa_nombre
           FROM contratistas c
           INNER JOIN empresas e ON c.empresa_id = e.id
           WHERE c.id = ?"#
    )
    .bind(&id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Contratista no encontrado: {}", e))?;
    
    let contratista = Contratista {
        id: row.get("id"),
        cedula: row.get("cedula"),
        nombre: row.get("nombre"),
        apellido: row.get("apellido"),
        empresa_id: row.get("empresa_id"),
        fecha_vencimiento_praind: row.get("fecha_vencimiento_praind"),
        estado: EstadoContratista::from_str(row.get("estado"))?,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    };
    
    let mut response = ContratistaResponse::from(contratista);
    response.empresa_nombre = row.get("empresa_nombre");
    
    Ok(response)
}

#[tauri::command]
pub async fn get_contratista_by_cedula(
    pool: State<'_, SqlitePool>,
    cedula: String,
) -> Result<ContratistaResponse, String> {
    let row = sqlx::query(
        r#"SELECT 
            c.id, c.cedula, c.nombre, c.apellido, c.empresa_id,
            c.fecha_vencimiento_praind, c.estado, c.created_at, c.updated_at,
            e.nombre as empresa_nombre
           FROM contratistas c
           INNER JOIN empresas e ON c.empresa_id = e.id
           WHERE c.cedula = ?"#
    )
    .bind(&cedula)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Contratista no encontrado: {}", e))?;
    
    let contratista = Contratista {
        id: row.get("id"),
        cedula: row.get("cedula"),
        nombre: row.get("nombre"),
        apellido: row.get("apellido"),
        empresa_id: row.get("empresa_id"),
        fecha_vencimiento_praind: row.get("fecha_vencimiento_praind"),
        estado: EstadoContratista::from_str(row.get("estado"))?,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    };
    
    let mut response = ContratistaResponse::from(contratista);
    response.empresa_nombre = row.get("empresa_nombre");
    
    Ok(response)
}

#[tauri::command]
pub async fn get_all_contratistas(
    pool: State<'_, SqlitePool>,
) -> Result<ContratistaListResponse, String> {
    let rows = sqlx::query(
        r#"SELECT 
            c.id, c.cedula, c.nombre, c.apellido, c.empresa_id,
            c.fecha_vencimiento_praind, c.estado, c.created_at, c.updated_at,
            e.nombre as empresa_nombre
           FROM contratistas c
           INNER JOIN empresas e ON c.empresa_id = e.id
           ORDER BY c.created_at DESC"#
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error al obtener contratistas: {}", e))?;
    
    let contratistas: Vec<ContratistaResponse> = rows.into_iter()
        .filter_map(|row| {
            let contratista = Contratista {
                id: row.get("id"),
                cedula: row.get("cedula"),
                nombre: row.get("nombre"),
                apellido: row.get("apellido"),
                empresa_id: row.get("empresa_id"),
                fecha_vencimiento_praind: row.get("fecha_vencimiento_praind"),
                estado: EstadoContratista::from_str(row.get("estado")).ok()?,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            
            let mut response = ContratistaResponse::from(contratista);
            response.empresa_nombre = row.get("empresa_nombre");
            Some(response)
        })
        .collect();
    
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

#[tauri::command]
pub async fn get_contratistas_activos(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<ContratistaResponse>, String> {
    let rows = sqlx::query(
        r#"SELECT 
            c.id, c.cedula, c.nombre, c.apellido, c.empresa_id,
            c.fecha_vencimiento_praind, c.estado, c.created_at, c.updated_at,
            e.nombre as empresa_nombre
           FROM contratistas c
           INNER JOIN empresas e ON c.empresa_id = e.id
           WHERE c.estado = ? 
           ORDER BY c.apellido, c.nombre"#
    )
    .bind(EstadoContratista::Activo.as_str())
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error al obtener contratistas activos: {}", e))?;
    
    let contratistas = rows.into_iter()
        .filter_map(|row| {
            let contratista = Contratista {
                id: row.get("id"),
                cedula: row.get("cedula"),
                nombre: row.get("nombre"),
                apellido: row.get("apellido"),
                empresa_id: row.get("empresa_id"),
                fecha_vencimiento_praind: row.get("fecha_vencimiento_praind"),
                estado: EstadoContratista::from_str(row.get("estado")).ok()?,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            
            let mut response = ContratistaResponse::from(contratista);
            response.empresa_nombre = row.get("empresa_nombre");
            Some(response)
        })
        .collect();
    
    Ok(contratistas)
}

#[tauri::command]
pub async fn update_contratista(
    pool: State<'_, SqlitePool>,
    id: String,
    input: UpdateContratistaInput,
) -> Result<ContratistaResponse, String> {
    // Validar campos opcionales
    if let Some(ref nombre) = input.nombre {
        validaciones::validar_nombre(nombre)?;
    }
    if let Some(ref apellido) = input.apellido {
        validaciones::validar_apellido(apellido)?;
    }
    if let Some(ref empresa_id) = input.empresa_id {
        validaciones::validar_empresa_id(empresa_id)?;
        
        // Verificar que la empresa exista
        let empresa_existe = sqlx::query("SELECT COUNT(*) as count FROM empresas WHERE id = ?")
            .bind(empresa_id)
            .fetch_one(&*pool)
            .await
            .map_err(|e| format!("Error al verificar empresa: {}", e))?;
        
        let count: i32 = empresa_existe.get("count");
        if count == 0 {
            return Err("La empresa especificada no existe".to_string());
        }
    }
    if let Some(ref fecha) = input.fecha_vencimiento_praind {
        validaciones::validar_fecha(fecha)?;
    }
    
    let now = Utc::now().to_rfc3339();
    
    sqlx::query(
        r#"UPDATE contratistas SET
            nombre = COALESCE(?, nombre),
            apellido = COALESCE(?, apellido),
            empresa_id = COALESCE(?, empresa_id),
            fecha_vencimiento_praind = COALESCE(?, fecha_vencimiento_praind),
            updated_at = ?
        WHERE id = ?"#
    )
    .bind(input.nombre.as_deref().map(|s| s.trim()))
    .bind(input.apellido.as_deref().map(|s| s.trim()))
    .bind(&input.empresa_id)
    .bind(&input.fecha_vencimiento_praind)
    .bind(&now)
    .bind(&id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Error al actualizar contratista: {}", e))?;
    
    get_contratista_by_id(pool, id).await
}

#[tauri::command]
pub async fn cambiar_estado_contratista(
    pool: State<'_, SqlitePool>,
    id: String,
    input: CambiarEstadoInput,
) -> Result<ContratistaResponse, String> {
    let estado = EstadoContratista::from_str(&input.estado)?;
    let now = Utc::now().to_rfc3339();
    
    sqlx::query(
        "UPDATE contratistas SET estado = ?, updated_at = ? WHERE id = ?"
    )
    .bind(estado.as_str())
    .bind(&now)
    .bind(&id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Error al cambiar estado: {}", e))?;
    
    get_contratista_by_id(pool, id).await
}

#[tauri::command]
pub async fn delete_contratista(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    sqlx::query("DELETE FROM contratistas WHERE id = ?")
        .bind(&id)
        .execute(&*pool)
        .await
        .map_err(|e| format!("Error al eliminar contratista: {}", e))?;
    
    Ok(())
}