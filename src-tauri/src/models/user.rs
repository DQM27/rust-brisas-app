// ==========================================
// src/models/user.rs
// ==========================================
use serde::{Deserialize, Serialize};

/// Modelo de dominio - Representa un usuario del sistema
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub email: String,
    pub nombre: String,
    pub apellido: String,
    pub role: UserRole,  // CAMBIÓ: ahora es enum
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

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
// DTOs de entrada
// ==========================================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserInput {
    pub email: String,
    pub password: String,
    pub nombre: String,
    pub apellido: String,
    pub role: Option<String>,  // String desde frontend, se convierte a enum
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
// DTOs de salida
// ==========================================

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub nombre: String,
    pub apellido: String,
    pub nombre_completo: String,  // Nuevo
    pub role: UserRole,
    pub role_display: String,      // Nuevo - "Administrador", "Supervisor", "Guardia"
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

// ==========================================
// Validaciones
// ==========================================

pub mod validaciones {
    use super::UserRole;
    
    pub fn validar_email(email: &str) -> Result<(), String> {
        let limpio = email.trim();
        
        if limpio.is_empty() {
            return Err("El email no puede estar vacío".to_string());
        }
        
        if !limpio.contains('@') {
            return Err("Email inválido".to_string());
        }
        
        if limpio.len() > 100 {
            return Err("El email no puede exceder 100 caracteres".to_string());
        }
        
        Ok(())
    }
    
    pub fn validar_password(password: &str) -> Result<(), String> {
        if password.len() < 6 {
            return Err("La contraseña debe tener al menos 6 caracteres".to_string());
        }
        
        if password.len() > 100 {
            return Err("La contraseña no puede exceder 100 caracteres".to_string());
        }
        
        Ok(())
    }
    
    pub fn validar_nombre(nombre: &str) -> Result<(), String> {
        let limpio = nombre.trim();
        
        if limpio.is_empty() {
            return Err("El nombre no puede estar vacío".to_string());
        }
        
        if limpio.len() > 50 {
            return Err("El nombre no puede exceder 50 caracteres".to_string());
        }
        
        Ok(())
    }
    
    pub fn validar_apellido(apellido: &str) -> Result<(), String> {
        let limpio = apellido.trim();
        
        if limpio.is_empty() {
            return Err("El apellido no puede estar vacío".to_string());
        }
        
        if limpio.len() > 50 {
            return Err("El apellido no puede exceder 50 caracteres".to_string());
        }
        
        Ok(())
    }
    
    pub fn validar_role(role_str: &str) -> Result<UserRole, String> {
        UserRole::from_str(role_str)
    }
    
    pub fn validar_create_input(input: &super::CreateUserInput) -> Result<(), String> {
        validar_email(&input.email)?;
        validar_password(&input.password)?;
        validar_nombre(&input.nombre)?;
        validar_apellido(&input.apellido)?;
        
        if let Some(ref role) = input.role {
            validar_role(role)?;
        }
        
        Ok(())
    }
}