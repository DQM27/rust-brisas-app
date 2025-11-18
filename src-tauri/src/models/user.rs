// ==========================================
// src/models/user.rs (REFACTORIZADO)
// ==========================================
// Solo modelos, DTOs y enums - SIN validaciones ni lógica

use serde::{Deserialize, Serialize};

// ==========================================
// MODELO DE DOMINIO
// ==========================================

/// Representa un usuario del sistema
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub email: String,
    pub nombre: String,
    pub apellido: String,
    pub role: UserRole,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

// ==========================================
// ENUM DE ROLES
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Supervisor,
    Guardia,
}

impl UserRole {
    pub fn as_str(&self) -> &str {
        match self {
            UserRole::Admin => "admin",
            UserRole::Supervisor => "supervisor",
            UserRole::Guardia => "guardia",
        }
    }
    
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "admin" => Ok(UserRole::Admin),
            "supervisor" => Ok(UserRole::Supervisor),
            "guardia" => Ok(UserRole::Guardia),
            _ => Err(format!("Rol desconocido: {}", s)),
        }
    }
}

// ==========================================
// DTOs DE ENTRADA
// ==========================================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserInput {
    pub email: String,
    pub password: String,
    pub nombre: String,
    pub apellido: String,
    pub role: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserInput {
    pub email: Option<String>,
    pub password: Option<String>,
    pub nombre: Option<String>,
    pub apellido: Option<String>,
    pub role: Option<String>,
    pub is_active: Option<bool>,
}

// ==========================================
// DTOs DE SALIDA
// ==========================================

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub nombre: String,
    pub apellido: String,
    pub nombre_completo: String,
    pub role: UserRole,
    pub role_display: String,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<User> for UserResponse {
    fn from(u: User) -> Self {
        let role_display = match u.role {
            UserRole::Admin => "Administrador",
            UserRole::Supervisor => "Supervisor",
            UserRole::Guardia => "Guardia",
        };
        
        Self {
            id: u.id,
            email: u.email.clone(),
            nombre: u.nombre.clone(),
            apellido: u.apellido.clone(),
            nombre_completo: format!("{} {}", u.nombre, u.apellido),
            role: u.role,
            role_display: role_display.to_string(),
            is_active: u.is_active,
            created_at: u.created_at,
            updated_at: u.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserListResponse {
    pub users: Vec<UserResponse>,
    pub total: usize,
    pub activos: usize,
    pub por_rol: RoleStats,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleStats {
    pub admins: usize,
    pub supervisores: usize,
    pub guardias: usize,
}