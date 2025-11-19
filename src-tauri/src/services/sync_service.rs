// src/services/sync_service.rs

use crate::models::user::UserResponse;
use crate::SupabaseState;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;

// ==========================================
// HELPERS
// ==========================================

fn log_sync_error(entity: &str, id: &str, error: &str) {
    eprintln!("❌ Error sincronizando {} [{}]: {}", entity, id, error);
}

fn log_sync_warning(entity: &str, message: &str) {
    println!("⚠️  {} - {}", entity, message);
}

// ==========================================
// SYNC TO SUPABASE (Upload)
// ==========================================

/// Datos completos para sincronizar un usuario (incluye password_hash)
pub struct UserSyncData {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub nombre: String,
    pub apellido: String,
    pub role: String,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// Sincroniza un usuario a Supabase (INSERT o UPDATE)
pub async fn sync_user_to_supabase(
    supabase_state: Arc<RwLock<SupabaseState>>,
    user_data: UserSyncData,
) -> Result<(), String> {
    let state = supabase_state.read().await;
    
    let client = match &state.client {
        Some(c) => c,
        None => {
            log_sync_warning("Usuario", "Cliente de Supabase no disponible");
            return Ok(());
        }
    };
    
    match upsert_user_to_pg(client.pool(), &user_data).await {
        Ok(_) => {
            println!("✅ Usuario {} sincronizado a Supabase", user_data.id);
            Ok(())
        }
        Err(e) => {
            log_sync_error("Usuario", &user_data.id, &e);
            Ok(())
        }
    }
}

/// Elimina un usuario de Supabase
pub async fn delete_user_from_supabase(
    supabase_state: Arc<RwLock<SupabaseState>>,
    user_id: &str,
) -> Result<(), String> {
    let state = supabase_state.read().await;
    
    let client = match &state.client {
        Some(c) => c,
        None => {
            log_sync_warning("Usuario", "Cliente de Supabase no disponible");
            return Ok(());
        }
    };
    
    match delete_user_from_pg(client.pool(), user_id).await {
        Ok(_) => {
            println!("✅ Usuario {} eliminado de Supabase", user_id);
            Ok(())
        }
        Err(e) => {
            log_sync_error("Usuario (delete)", user_id, &e);
            Ok(())
        }
    }
}

// ==========================================
// QUERIES DE SUPABASE (PostgreSQL)
// ==========================================

/// INSERT o UPDATE usuario en PostgreSQL
async fn upsert_user_to_pg(
    pool: &PgPool,
    user: &UserSyncData,
) -> Result<(), String> {
    sqlx::query(
        r#"INSERT INTO usuarios (id, email, password_hash, nombre, apellido, role, is_active, created_at, updated_at)
           VALUES ($1::uuid, $2, $3, $4, $5, $6, $7, $8::timestamptz, $9::timestamptz)
           ON CONFLICT (id) DO UPDATE SET
               email = EXCLUDED.email,
               password_hash = EXCLUDED.password_hash,
               nombre = EXCLUDED.nombre,
               apellido = EXCLUDED.apellido,
               role = EXCLUDED.role,
               is_active = EXCLUDED.is_active,
               updated_at = EXCLUDED.updated_at"#
    )
    .bind(&user.id)
    .bind(&user.email)
    .bind(&user.password_hash)
    .bind(&user.nombre)
    .bind(&user.apellido)
    .bind(&user.role)
    .bind(user.is_active)
    .bind(&user.created_at)
    .bind(&user.updated_at)
    .execute(pool)
    .await
    .map_err(|e| format!("Error en upsert a Supabase: {}", e))?;
    
    Ok(())
}

/// DELETE usuario de PostgreSQL
async fn delete_user_from_pg(
    pool: &PgPool,
    user_id: &str,
) -> Result<(), String> {
    sqlx::query("DELETE FROM usuarios WHERE id = $1::uuid")
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Error al eliminar de Supabase: {}", e))?;
    
    Ok(())
}

// ==========================================
// PULL FROM SUPABASE (Download)
// ==========================================

/// Obtiene todos los usuarios de Supabase
pub async fn get_all_users_from_supabase(
    supabase_state: Arc<RwLock<SupabaseState>>,
) -> Result<Vec<UserResponse>, String> {
    let state = supabase_state.read().await;
    
    let client = match &state.client {
        Some(c) => c,
        None => return Err("Cliente de Supabase no disponible".to_string()),
    };
    
    let rows = sqlx::query(
        "SELECT id, email, nombre, apellido, role, is_active, created_at, updated_at 
         FROM usuarios ORDER BY created_at DESC"
    )
    .fetch_all(client.pool())
    .await
    .map_err(|e| format!("Error obteniendo usuarios de Supabase: {}", e))?;
    
    let users: Vec<UserResponse> = rows
        .into_iter()
        .filter_map(|row| {
            use sqlx::Row;
            use crate::models::user::{User, UserRole};
            
            let user = User {
                id: row.get("id"),
                email: row.get("email"),
                nombre: row.get("nombre"),
                apellido: row.get("apellido"),
                role: UserRole::from_str(row.get("role")).ok()?,
                is_active: row.get("is_active"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            
            Some(UserResponse::from(user))
        })
        .collect();
    
    Ok(users)
}