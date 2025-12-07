// ==========================================
// src/models/user.rs (REFACTORIZADO)
// ==========================================
// Solo modelos, DTOs y enums - SIN validaciones ni lógica

use serde::{Deserialize, Serialize};

// ==========================================
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

    // Nuevos campos obligatorios
    pub cedula: String,
    pub segundo_nombre: Option<String>,
    pub segundo_apellido: Option<String>,
    pub fecha_inicio_labores: Option<String>,
    pub numero_gafete: Option<String>,
    pub fecha_nacimiento: Option<String>,
    pub telefono: Option<String>,
    pub direccion: Option<String>,
    pub contacto_emergencia_nombre: Option<String>,
    pub contacto_emergencia_telefono: Option<String>,
    pub must_change_password: bool,
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
    pub password: Option<String>,
    pub nombre: String,
    pub apellido: String,
    pub role: Option<String>,

    // Obligatorio nuevo
    pub cedula: String,

    // Opcionales
    pub segundo_nombre: Option<String>,
    pub segundo_apellido: Option<String>,
    pub fecha_inicio_labores: Option<String>,
    pub numero_gafete: Option<String>,
    pub fecha_nacimiento: Option<String>,
    pub telefono: Option<String>,
    pub direccion: Option<String>,
    pub contacto_emergencia_nombre: Option<String>,
    pub contacto_emergencia_telefono: Option<String>,
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

    // Opcionales
    pub cedula: Option<String>,
    pub segundo_nombre: Option<String>,
    pub segundo_apellido: Option<String>,
    pub fecha_inicio_labores: Option<String>,
    pub numero_gafete: Option<String>,
    pub fecha_nacimiento: Option<String>,
    pub telefono: Option<String>,
    pub direccion: Option<String>,
    pub contacto_emergencia_nombre: Option<String>,
    pub contacto_emergencia_telefono: Option<String>,
}

// ==========================================
// DTOs DE SALIDA
// ==========================================

#[derive(Debug, Serialize, Clone)]
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

    // Nuevos campos
    pub cedula: String,
    pub segundo_nombre: Option<String>,
    pub segundo_apellido: Option<String>,
    pub fecha_inicio_labores: Option<String>,
    pub numero_gafete: Option<String>,
    pub fecha_nacimiento: Option<String>,
    pub telefono: Option<String>,
    pub direccion: Option<String>,
    pub contacto_emergencia_nombre: Option<String>,
    pub contacto_emergencia_telefono: Option<String>,
    pub must_change_password: bool,
    pub temporary_password: Option<String>,
}

impl From<User> for UserResponse {
    fn from(u: User) -> Self {
        let role_display = match u.role {
            UserRole::Admin => "Administrador",
            UserRole::Supervisor => "Supervisor",
            UserRole::Guardia => "Guardia",
        };

        let mut parts = Vec::new();
        parts.push(u.nombre.as_str());
        if let Some(ref sn) = u.segundo_nombre {
            parts.push(sn.as_str());
        }
        parts.push(u.apellido.as_str());
        if let Some(ref sa) = u.segundo_apellido {
            parts.push(sa.as_str());
        }
        let nombre_completo = parts.join(" ");

        Self {
            id: u.id,
            email: u.email.clone(),
            nombre: u.nombre.clone(),
            apellido: u.apellido.clone(),
            nombre_completo,
            role: u.role,
            role_display: role_display.to_string(),
            is_active: u.is_active,
            created_at: u.created_at,
            updated_at: u.updated_at,

            // Mapeo directo
            cedula: u.cedula,
            segundo_nombre: u.segundo_nombre,
            segundo_apellido: u.segundo_apellido,
            fecha_inicio_labores: u.fecha_inicio_labores,
            numero_gafete: u.numero_gafete,
            fecha_nacimiento: u.fecha_nacimiento,
            telefono: u.telefono,
            direccion: u.direccion,
            contacto_emergencia_nombre: u.contacto_emergencia_nombre,

            contacto_emergencia_telefono: u.contacto_emergencia_telefono,
            must_change_password: u.must_change_password,
            temporary_password: None, // Por defecto no se envía, solo al crear
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
