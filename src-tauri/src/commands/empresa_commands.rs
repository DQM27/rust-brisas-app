// src/commands/empresa_commands.rs

use crate::models::empresa::{
    EmpresaResponse, EmpresaListResponse,
    CreateEmpresaInput, UpdateEmpresaInput, validaciones,
};
use sqlx::{SqlitePool, Row};
use tauri::State;
use uuid::Uuid;
use chrono::Utc;

#[tauri::command]
pub async fn create_empresa(
    pool: State<'_, SqlitePool>,
    input: CreateEmpresaInput,
) -> Result<EmpresaResponse, String> {
    // Validar input
    validaciones::validar_create_input(&input)?;
    
    // Verificar que el nombre no exista
    let existe = sqlx::query("SELECT COUNT(*) as count FROM empresas WHERE nombre = ?")
        .bind(input.nombre.trim())
        .fetch_one(&*pool)
        .await
        .map_err(|e| format!("Error al verificar nombre: {}", e))?;
    
    let count: i32 = existe.get("count");
    if count > 0 {
        return Err("Ya existe una empresa con este nombre".to_string());
    }
    
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    sqlx::query(
        r#"INSERT INTO empresas 
           (id, nombre, is_active, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?)"#
    )
    .bind(&id)
    .bind(input.nombre.trim())
    .bind(1) // is_active = true por defecto
    .bind(&now)
    .bind(&now)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Error al crear empresa: {}", e))?;
    
    get_empresa_by_id(pool, id).await
}

#[tauri::command]
pub async fn get_empresa_by_id(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<EmpresaResponse, String> {
    let row = sqlx::query(
        r#"SELECT id, nombre, is_active, created_at, updated_at
           FROM empresas WHERE id = ?"#
    )
    .bind(&id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Empresa no encontrada: {}", e))?;
    
    // Contar contratistas de esta empresa
    let count_row = sqlx::query(
        "SELECT COUNT(*) as count FROM contratistas WHERE empresa_id = ?"
    )
    .bind(&id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Error al contar contratistas: {}", e))?;
    
    let total_contratistas: i32 = count_row.get("count");
    
    Ok(EmpresaResponse {
        id: row.get("id"),
        nombre: row.get("nombre"),
        is_active: row.get::<i32, _>("is_active") != 0,
        total_contratistas: total_contratistas as usize,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

#[tauri::command]
pub async fn get_all_empresas(
    pool: State<'_, SqlitePool>,
) -> Result<EmpresaListResponse, String> {
    let rows = sqlx::query(
        r#"SELECT id, nombre, is_active, created_at, updated_at
           FROM empresas ORDER BY nombre ASC"#
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error al obtener empresas: {}", e))?;
    
    let mut empresas = Vec::new();
    
    for row in rows {
        let empresa_id: String = row.get("id");
        
        // Contar contratistas de cada empresa
        let count_row = sqlx::query(
            "SELECT COUNT(*) as count FROM contratistas WHERE empresa_id = ?"
        )
        .bind(&empresa_id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| format!("Error al contar contratistas: {}", e))?;
        
        let total_contratistas: i32 = count_row.get("count");
        
        empresas.push(EmpresaResponse {
            id: empresa_id,
            nombre: row.get("nombre"),
            is_active: row.get::<i32, _>("is_active") != 0,
            total_contratistas: total_contratistas as usize,
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        });
    }
    
    let total = empresas.len();
    let activas = empresas.iter().filter(|e| e.is_active).count();
    
    Ok(EmpresaListResponse {
        empresas,
        total,
        activas,
    })
}

#[tauri::command]
pub async fn get_empresas_activas(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<EmpresaResponse>, String> {
    let rows = sqlx::query(
        r#"SELECT id, nombre, is_active, created_at, updated_at
           FROM empresas WHERE is_active = 1 ORDER BY nombre ASC"#
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error al obtener empresas activas: {}", e))?;
    
    let mut empresas = Vec::new();
    
    for row in rows {
        let empresa_id: String = row.get("id");
        
        // Contar contratistas de cada empresa
        let count_row = sqlx::query(
            "SELECT COUNT(*) as count FROM contratistas WHERE empresa_id = ?"
        )
        .bind(&empresa_id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| format!("Error al contar contratistas: {}", e))?;
        
        let total_contratistas: i32 = count_row.get("count");
        
        empresas.push(EmpresaResponse {
            id: empresa_id,
            nombre: row.get("nombre"),
            is_active: row.get::<i32, _>("is_active") != 0,
            total_contratistas: total_contratistas as usize,
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        });
    }
    
    Ok(empresas)
}

#[tauri::command]
pub async fn update_empresa(
    pool: State<'_, SqlitePool>,
    id: String,
    input: UpdateEmpresaInput,
) -> Result<EmpresaResponse, String> {
    // Validar nombre si viene
    if let Some(ref nombre) = input.nombre {
        validaciones::validar_nombre(nombre)?;
        
        // Verificar que el nombre no exista en otra empresa
        let existe = sqlx::query(
            "SELECT COUNT(*) as count FROM empresas WHERE nombre = ? AND id != ?"
        )
        .bind(nombre.trim())
        .bind(&id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| format!("Error al verificar nombre: {}", e))?;
        
        let count: i32 = existe.get("count");
        if count > 0 {
            return Err("Ya existe otra empresa con este nombre".to_string());
        }
    }
    
    let now = Utc::now().to_rfc3339();
    
    sqlx::query(
        r#"UPDATE empresas SET
            nombre = COALESCE(?, nombre),
            is_active = COALESCE(?, is_active),
            updated_at = ?
        WHERE id = ?"#
    )
    .bind(input.nombre.as_deref().map(|s| s.trim()))
    .bind(input.is_active.map(|b| if b { 1 } else { 0 }))
    .bind(&now)
    .bind(&id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Error al actualizar empresa: {}", e))?;
    
    get_empresa_by_id(pool, id).await
}

#[tauri::command]
pub async fn delete_empresa(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    // Verificar que no tenga contratistas asociados
    let count_row = sqlx::query(
        "SELECT COUNT(*) as count FROM contratistas WHERE empresa_id = ?"
    )
    .bind(&id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Error al verificar contratistas: {}", e))?;
    
    let count: i32 = count_row.get("count");
    if count > 0 {
        return Err(format!(
            "No se puede eliminar la empresa porque tiene {} contratista(s) asociado(s)",
            count
        ));
    }
    
    sqlx::query("DELETE FROM empresas WHERE id = ?")
        .bind(&id)
        .execute(&*pool)
        .await
        .map_err(|e| format!("Error al eliminar empresa: {}", e))?;
    
    Ok(())
}