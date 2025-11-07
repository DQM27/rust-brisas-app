
// ==========================================
// src/commands/user_commands.rs
// ==========================================
use crate::models::user::{User, CreateUserInput, UpdateUserInput};
use crate::services::auth::{hash_password, verify_password};
use sqlx::{SqlitePool, Row};
use tauri::State;
use uuid::Uuid;
use chrono::Utc;

#[tauri::command]
pub async fn create_user(
    pool: State<'_, SqlitePool>,
    input: CreateUserInput,
) -> Result<User, String> {
    let id = Uuid::new_v4().to_string();
    let hash = hash_password(&input.password)
        .map_err(|e| format!("Error al hashear contraseña: {}", e))?;
    
    let role = input.role.unwrap_or_else(|| "user".to_string());
    let now = Utc::now().to_rfc3339();
    
    sqlx::query(
        "INSERT INTO users (id, email, password_hash, nombre, apellido, role, created_at, updated_at) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&input.email)
    .bind(&hash)
    .bind(&input.nombre)
    .bind(&input.apellido)
    .bind(&role)
    .bind(&now)
    .bind(&now)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Error al crear usuario: {}", e))?;
    
    get_user_by_id(pool, id).await
}

#[tauri::command]
pub async fn get_user_by_id(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<User, String> {
    let row = sqlx::query(
        "SELECT id, email, nombre, apellido, role, is_active, created_at, updated_at 
         FROM users WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Usuario no encontrado: {}", e))?;
    
    Ok(User {
        id: row.get("id"),
        email: row.get("email"),
        nombre: row.get("nombre"),
        apellido: row.get("apellido"),
        role: row.get("role"),
        is_active: row.get::<i32, _>("is_active") != 0,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

#[tauri::command]
pub async fn get_all_users(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<User>, String> {
    let rows = sqlx::query(
        "SELECT id, email, nombre, apellido, role, is_active, created_at, updated_at 
         FROM users ORDER BY created_at DESC"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error al obtener usuarios: {}", e))?;
    
    let users = rows.into_iter().map(|row| {
        User {
            id: row.get("id"),
            email: row.get("email"),
            nombre: row.get("nombre"),
            apellido: row.get("apellido"),
            role: row.get("role"),
            is_active: row.get::<i32, _>("is_active") != 0,
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }).collect();
    
    Ok(users)
}

#[tauri::command]
pub async fn update_user(
    pool: State<'_, SqlitePool>,
    id: String,
    input: UpdateUserInput,
) -> Result<User, String> {
    let now = Utc::now().to_rfc3339();
    
    let password_hash = if let Some(ref pwd) = input.password {
        Some(hash_password(pwd)
            .map_err(|e| format!("Error al hashear contraseña: {}", e))?)
    } else {
        None
    };
    
    sqlx::query(
        r#"UPDATE users SET
            email = COALESCE(?, email),
            password_hash = COALESCE(?, password_hash),
            nombre = COALESCE(?, nombre),
            apellido = COALESCE(?, apellido),
            role = COALESCE(?, role),
            is_active = COALESCE(?, is_active),
            updated_at = ?
        WHERE id = ?"#
    )
    .bind(&input.email)
    .bind(&password_hash)
    .bind(&input.nombre)
    .bind(&input.apellido)
    .bind(&input.role)
    .bind(input.is_active.map(|b| if b { 1 } else { 0 }))
    .bind(&now)
    .bind(&id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Error al actualizar usuario: {}", e))?;
    
    get_user_by_id(pool, id).await
}

#[tauri::command]
pub async fn delete_user(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(&id)
        .execute(&*pool)
        .await
        .map_err(|e| format!("Error al eliminar usuario: {}", e))?;
    
    Ok(())
}

#[tauri::command]
pub async fn login(
    pool: State<'_, SqlitePool>,
    email: String,
    password: String,
) -> Result<User, String> {
    let row = sqlx::query(
        "SELECT id, email, nombre, apellido, role, is_active, created_at, updated_at, password_hash
         FROM users WHERE email = ?"
    )
    .bind(&email)
    .fetch_one(&*pool)
    .await
    .map_err(|_| "Credenciales inválidas".to_string())?;
    
    let password_hash: String = row.get("password_hash");
    let is_active: i32 = row.get("is_active");
    
    // Verificar contraseña
    let is_valid = verify_password(&password, &password_hash)
        .map_err(|e| format!("Error al verificar contraseña: {}", e))?;
    
    if !is_valid {
        return Err("Credenciales inválidas".to_string());
    }
    
    if is_active == 0 {
        return Err("Usuario inactivo".to_string());
    }
    
    Ok(User {
        id: row.get("id"),
        email: row.get("email"),
        nombre: row.get("nombre"),
        apellido: row.get("apellido"),
        role: row.get("role"),
        is_active: is_active != 0,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}
