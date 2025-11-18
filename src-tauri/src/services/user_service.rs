// ==========================================
// src/services/user_service.rs
// ==========================================
// Capa de servicio: orquesta dominio, db y auth
// Contiene la lógica de negocio completa

use crate::domain::user as domain;
use crate::db::user_queries as db;
use crate::models::user::{
    UserResponse, UserListResponse, RoleStats, UserRole,
    CreateUserInput, UpdateUserInput,
};
use crate::services::auth;
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;

// ==========================================
// CREAR USUARIO
// ==========================================

pub async fn create_user(
    pool: &SqlitePool,
    input: CreateUserInput,
) -> Result<UserResponse, String> {
    // 1. Validar input
    domain::validar_create_input(&input)?;
    
    // 2. Normalizar datos
    let email_normalizado = domain::normalizar_email(&input.email);
    let nombre_normalizado = domain::normalizar_nombre(&input.nombre);
    let apellido_normalizado = domain::normalizar_nombre(&input.apellido);
    
    // 3. Verificar que el email no exista
    let count = db::count_by_email(pool, &email_normalizado).await?;
    if count > 0 {
        return Err("Ya existe un usuario con este email".to_string());
    }
    
    // 4. Determinar rol (default: Guardia)
    let role = if let Some(ref r) = input.role {
        UserRole::from_str(r)?
    } else {
        UserRole::Guardia
    };
    
    // 5. Hashear contraseña
    let password_hash = auth::hash_password(&input.password)?;
    
    // 6. Generar ID y timestamps
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    // 7. Insertar en DB
    db::insert(
        pool,
        &id,
        &email_normalizado,
        &password_hash,
        &nombre_normalizado,
        &apellido_normalizado,
        role.as_str(),
        &now,
        &now,
    ).await?;
    
    // 8. Retornar usuario creado
    get_user_by_id(pool, &id).await
}

// ==========================================
// OBTENER USUARIO POR ID
// ==========================================

pub async fn get_user_by_id(
    pool: &SqlitePool,
    id: &str,
) -> Result<UserResponse, String> {
    let user = db::find_by_id(pool, id).await?;
    Ok(UserResponse::from(user))
}

// ==========================================
// OBTENER TODOS LOS USUARIOS
// ==========================================

pub async fn get_all_users(pool: &SqlitePool) -> Result<UserListResponse, String> {
    let users = db::find_all(pool).await?;
    
    // Convertir a UserResponse
    let user_responses: Vec<UserResponse> = users
        .into_iter()
        .map(UserResponse::from)
        .collect();
    
    // Calcular estadísticas
    let total = user_responses.len();
    let activos = user_responses.iter().filter(|u| u.is_active).count();
    let admins = user_responses.iter().filter(|u| u.role == UserRole::Admin).count();
    let supervisores = user_responses.iter().filter(|u| u.role == UserRole::Supervisor).count();
    let guardias = user_responses.iter().filter(|u| u.role == UserRole::Guardia).count();
    
    Ok(UserListResponse {
        users: user_responses,
        total,
        activos,
        por_rol: RoleStats {
            admins,
            supervisores,
            guardias,
        },
    })
}

// ==========================================
// ACTUALIZAR USUARIO
// ==========================================

pub async fn update_user(
    pool: &SqlitePool,
    id: String,
    input: UpdateUserInput,
) -> Result<UserResponse, String> {
    // 1. Validar input
    domain::validar_update_input(&input)?;
    
    // 2. Verificar que el usuario existe
    let _ = db::find_by_id(pool, &id).await?;
    
    // 3. Normalizar y verificar email si viene
    let email_normalizado = if let Some(ref email) = input.email {
        let normalizado = domain::normalizar_email(email);
        
        // Verificar email único
        let count = db::count_by_email_excluding_id(pool, &normalizado, &id).await?;
        if count > 0 {
            return Err("Ya existe otro usuario con este email".to_string());
        }
        
        Some(normalizado)
    } else {
        None
    };
    
    // 4. Normalizar nombres si vienen
    let nombre_normalizado = input.nombre
        .as_ref()
        .map(|n| domain::normalizar_nombre(n));
    
    let apellido_normalizado = input.apellido
        .as_ref()
        .map(|a| domain::normalizar_nombre(a));
    
    // 5. Convertir rol si viene
    let role_str = if let Some(ref r) = input.role {
        Some(UserRole::from_str(r)?.as_str().to_string())
    } else {
        None
    };
    
    // 6. Hashear contraseña si viene
    let password_hash = if let Some(ref pwd) = input.password {
        Some(auth::hash_password(pwd)?)
    } else {
        None
    };
    
    // 7. Timestamp de actualización
    let now = Utc::now().to_rfc3339();
    
    // 8. Convertir is_active a i32 si viene
    let is_active_int = input.is_active.map(|b| if b { 1 } else { 0 });
    
    // 9. Actualizar en DB
    db::update(
        pool,
        &id,
        email_normalizado.as_deref(),
        password_hash.as_deref(),
        nombre_normalizado.as_deref(),
        apellido_normalizado.as_deref(),
        role_str.as_deref(),
        is_active_int,
        &now,
    ).await?;
    
    // 10. Retornar usuario actualizado
    get_user_by_id(pool, &id).await
}

// ==========================================
// ELIMINAR USUARIO
// ==========================================

pub async fn delete_user(pool: &SqlitePool, id: String) -> Result<(), String> {
    // Verificar que existe antes de eliminar
    let _ = db::find_by_id(pool, &id).await?;
    
    // Eliminar
    db::delete(pool, &id).await
}

// ==========================================
// LOGIN
// ==========================================

pub async fn login(
    pool: &SqlitePool,
    email: String,
    password: String,
) -> Result<UserResponse, String> {
    // 1. Normalizar email
    let email_normalizado = domain::normalizar_email(&email);
    
    // 2. Buscar usuario con password_hash
    let (user, password_hash) = db::find_by_email_with_password(pool, &email_normalizado).await?;
    
    // 3. Verificar contraseña
    let is_valid = auth::verify_password(&password, &password_hash)?;
    if !is_valid {
        return Err("Credenciales inválidas".to_string());
    }
    
    // 4. Verificar que esté activo
    if !user.is_active {
        return Err("Usuario inactivo".to_string());
    }
    
    // 5. Retornar usuario
    Ok(UserResponse::from(user))
}

// ==========================================
// HELPERS INTERNOS
// ==========================================

/// Verifica si un email ya está en uso
pub async fn email_exists(pool: &SqlitePool, email: &str) -> Result<bool, String> {
    let count = db::count_by_email(pool, email).await?;
    Ok(count > 0)
}

/// Verifica si un email está en uso por otro usuario (útil para updates)
pub async fn email_exists_for_other_user(
    pool: &SqlitePool,
    email: &str,
    exclude_id: &str,
) -> Result<bool, String> {
    let count = db::count_by_email_excluding_id(pool, email, exclude_id).await?;
    Ok(count > 0)
}