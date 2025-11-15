// ==========================================
// src/commands/user_commands.rs
// ==========================================
use crate::models::user::{
    User, UserResponse, UserListResponse, RoleStats,
    CreateUserInput, UpdateUserInput, UserRole, validaciones,
};
use crate::services::auth::{hash_password, verify_password};
use sqlx::{SqlitePool, Row};
use tauri::State;
use uuid::Uuid;
use chrono::Utc;

#[tauri::command]
pub async fn create_user(
    pool: State<'_, SqlitePool>,
    input: CreateUserInput,
) -> Result<UserResponse, String> {
    // Validar input
    validaciones::validar_create_input(&input)?;
    
    // Verificar que el email no exista
    let existe = sqlx::query("SELECT COUNT(*) as count FROM users WHERE email = ?")
        .bind(&input.email)
        .fetch_one(&*pool)
        .await
        .map_err(|e| format!("Error al verificar email: {}", e))?;
    
    let count: i32 = existe.get("count");
    if count > 0 {
        return Err("Ya existe un usuario con este email".to_string());
    }
    
    let id = Uuid::new_v4().to_string();
    let hash = hash_password(&input.password)
        .map_err(|e| format!("Error al hashear contraseña: {}", e))?;
    
    let role = if let Some(ref r) = input.role {
        UserRole::from_str(r)?
    } else {
        UserRole::Guardia
    };
    
    let now = Utc::now().to_rfc3339();
    
    sqlx::query(
        "INSERT INTO users (id, email, password_hash, nombre, apellido, role, created_at, updated_at) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(input.email.trim())
    .bind(&hash)
    .bind(input.nombre.trim())
    .bind(input.apellido.trim())
    .bind(role.as_str())
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
) -> Result<UserResponse, String> {
    let row = sqlx::query(
        "SELECT id, email, nombre, apellido, role, is_active, created_at, updated_at 
         FROM users WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Usuario no encontrado: {}", e))?;
    
    let user = User {
        id: row.get("id"),
        email: row.get("email"),
        nombre: row.get("nombre"),
        apellido: row.get("apellido"),
        role: UserRole::from_str(row.get("role"))?,
        is_active: row.get::<i32, _>("is_active") != 0,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    };
    
    Ok(UserResponse::from(user))
}

#[tauri::command]
pub async fn get_all_users(
    pool: State<'_, SqlitePool>,
) -> Result<UserListResponse, String> {
    let rows = sqlx::query(
        "SELECT id, email, nombre, apellido, role, is_active, created_at, updated_at 
         FROM users ORDER BY created_at DESC"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error al obtener usuarios: {}", e))?;
    
    let users: Vec<UserResponse> = rows.into_iter()
        .filter_map(|row| {
            let user = User {
                id: row.get("id"),
                email: row.get("email"),
                nombre: row.get("nombre"),
                apellido: row.get("apellido"),
                role: UserRole::from_str(row.get("role")).ok()?,
                is_active: row.get::<i32, _>("is_active") != 0,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            Some(UserResponse::from(user))
        })
        .collect();
    
    let total = users.len();
    let activos = users.iter().filter(|u| u.is_active).count();
    let admins = users.iter().filter(|u| u.role == UserRole::Admin).count();
    let supervisores = users.iter().filter(|u| u.role == UserRole::Supervisor).count();
    let guardias = users.iter().filter(|u| u.role == UserRole::Guardia).count();
    
    Ok(UserListResponse {
        users,
        total,
        activos,
        por_rol: RoleStats {
            admins,
            supervisores,
            guardias,
        },
    })
}

#[tauri::command]
pub async fn update_user(
    pool: State<'_, SqlitePool>,
    id: String,
    input: UpdateUserInput,
) -> Result<UserResponse, String> {
    // Validaciones
    if let Some(ref email) = input.email {
        validaciones::validar_email(email)?;
        
        // Verificar email único
        let existe = sqlx::query(
            "SELECT COUNT(*) as count FROM users WHERE email = ? AND id != ?"
        )
        .bind(email)
        .bind(&id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| format!("Error al verificar email: {}", e))?;
        
        let count: i32 = existe.get("count");
        if count > 0 {
            return Err("Ya existe otro usuario con este email".to_string());
        }
    }
    
    if let Some(ref password) = input.password {
        validaciones::validar_password(password)?;
    }
    
    if let Some(ref nombre) = input.nombre {
        validaciones::validar_nombre(nombre)?;
    }
    
    if let Some(ref apellido) = input.apellido {
        validaciones::validar_apellido(apellido)?;
    }
    
    let role_str = if let Some(ref r) = input.role {
        Some(UserRole::from_str(r)?.as_str().to_string())
    } else {
        None
    };
    
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
    .bind(input.email.as_deref().map(|s| s.trim()))
    .bind(&password_hash)
    .bind(input.nombre.as_deref().map(|s| s.trim()))
    .bind(input.apellido.as_deref().map(|s| s.trim()))
    .bind(role_str.as_deref())
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
) -> Result<UserResponse, String> {
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
    
    let is_valid = verify_password(&password, &password_hash)
        .map_err(|e| format!("Error al verificar contraseña: {}", e))?;
    
    if !is_valid {
        return Err("Credenciales inválidas".to_string());
    }
    
    if is_active == 0 {
        return Err("Usuario inactivo".to_string());
    }
    
    let user = User {
        id: row.get("id"),
        email: row.get("email"),
        nombre: row.get("nombre"),
        apellido: row.get("apellido"),
        role: UserRole::from_str(row.get("role"))?,
        is_active: is_active != 0,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    };
    
    Ok(UserResponse::from(user))
}