// ==========================================
// src/db/user_queries.rs
// ==========================================
// Capa de data access: queries SQL puras
// Sin lógica de negocio, solo interacción con la base de datos

use crate::models::user::{User, UserRole};
use sqlx::{SqlitePool, Row};

// ==========================================
// QUERIES DE LECTURA
// ==========================================

/// Busca un usuario por ID
pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<User, String> {
    let row = sqlx::query(
        "SELECT id, email, nombre, apellido, role, is_active, created_at, updated_at 
         FROM users WHERE id = ?"
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Usuario no encontrado: {}", e))?;
    
    Ok(User {
        id: row.get("id"),
        email: row.get("email"),
        nombre: row.get("nombre"),
        apellido: row.get("apellido"),
        role: UserRole::from_str(row.get("role"))?,
        is_active: row.get::<i32, _>("is_active") != 0,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

/// Busca un usuario por email (incluyendo password_hash para login)
pub async fn find_by_email_with_password(
    pool: &SqlitePool,
    email: &str,
) -> Result<(User, String), String> {
    let row = sqlx::query(
        "SELECT id, email, nombre, apellido, role, is_active, created_at, updated_at, password_hash
         FROM users WHERE email = ?"
    )
    .bind(email)
    .fetch_one(pool)
    .await
    .map_err(|_| "Credenciales inválidas".to_string())?;
    
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
    
    let password_hash: String = row.get("password_hash");
    
    Ok((user, password_hash))
}

/// Obtiene todos los usuarios ordenados por fecha de creación
pub async fn find_all(pool: &SqlitePool) -> Result<Vec<User>, String> {
    let rows = sqlx::query(
        "SELECT id, email, nombre, apellido, role, is_active, created_at, updated_at 
         FROM users ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error al obtener usuarios: {}", e))?;
    
    let users: Vec<User> = rows
        .into_iter()
        .filter_map(|row| {
            Some(User {
                id: row.get("id"),
                email: row.get("email"),
                nombre: row.get("nombre"),
                apellido: row.get("apellido"),
                role: UserRole::from_str(row.get("role")).ok()?,
                is_active: row.get::<i32, _>("is_active") != 0,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
        })
        .collect();
    
    Ok(users)
}

/// Cuenta cuántos usuarios tienen un email específico (para verificar unicidad)
pub async fn count_by_email(pool: &SqlitePool, email: &str) -> Result<i32, String> {
    let row = sqlx::query("SELECT COUNT(*) as count FROM users WHERE email = ?")
        .bind(email)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Error al verificar email: {}", e))?;
    
    Ok(row.get("count"))
}

/// Cuenta cuántos usuarios tienen un email específico excluyendo un ID
/// (útil para updates)
pub async fn count_by_email_excluding_id(
    pool: &SqlitePool,
    email: &str,
    exclude_id: &str,
) -> Result<i32, String> {
    let row = sqlx::query(
        "SELECT COUNT(*) as count FROM users WHERE email = ? AND id != ?"
    )
    .bind(email)
    .bind(exclude_id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error al verificar email: {}", e))?;
    
    Ok(row.get("count"))
}

// ==========================================
// QUERIES DE ESCRITURA
// ==========================================

/// Inserta un nuevo usuario en la base de datos
pub async fn insert(
    pool: &SqlitePool,
    id: &str,
    email: &str,
    password_hash: &str,
    nombre: &str,
    apellido: &str,
    role: &str,
    created_at: &str,
    updated_at: &str,
) -> Result<(), String> {
    sqlx::query(
        "INSERT INTO users (id, email, password_hash, nombre, apellido, role, created_at, updated_at) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(id)
    .bind(email)
    .bind(password_hash)
    .bind(nombre)
    .bind(apellido)
    .bind(role)
    .bind(created_at)
    .bind(updated_at)
    .execute(pool)
    .await
    .map_err(|e| format!("Error al crear usuario: {}", e))?;
    
    Ok(())
}

/// Actualiza un usuario existente (campos opcionales)
pub async fn update(
    pool: &SqlitePool,
    id: &str,
    email: Option<&str>,
    password_hash: Option<&str>,
    nombre: Option<&str>,
    apellido: Option<&str>,
    role: Option<&str>,
    is_active: Option<i32>,
    updated_at: &str,
) -> Result<(), String> {
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
    .bind(email)
    .bind(password_hash)
    .bind(nombre)
    .bind(apellido)
    .bind(role)
    .bind(is_active)
    .bind(updated_at)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| format!("Error al actualizar usuario: {}", e))?;
    
    Ok(())
}

/// Elimina un usuario por ID
pub async fn delete(pool: &SqlitePool, id: &str) -> Result<(), String> {
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| format!("Error al eliminar usuario: {}", e))?;
    
    Ok(())
}