// ==========================================
// src/models/user.rs
// ==========================================

use crate::domain::role::GOD_ID;
use crate::models::role::Role;
use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, RecordId};

// --------------------------------------------------------------------------
// ENUMS
// --------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String")]
pub enum Operacion {
    #[serde(rename = "Calle Blancos")]
    CalleBlancos,
    #[serde(rename = "Cartago")]
    Cartago,
    #[serde(rename = "Coronado")]
    Coronado,
    #[serde(rename = "Mega Brisas")]
    MegaBrisas,
    #[serde(rename = "Belen")]
    Belen,
}

impl std::str::FromStr for Operacion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Calle Blancos" => Ok(Self::CalleBlancos),
            "Cartago" => Ok(Self::Cartago),
            "Coronado" => Ok(Self::Coronado),
            "Mega Brisas" => Ok(Self::MegaBrisas),
            "Belen" => Ok(Self::Belen),
            _ => Err(format!("Operacion desconocida: {s}")),
        }
    }
}

impl TryFrom<String> for Operacion {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

/// Representa a un operador del sistema (Usuario).
///
/// Un usuario tiene credenciales de acceso, un rol asignado y datos personales
/// básicos para identificación y auditoría.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: RecordId,
    pub email: String,
    pub nombre: String,
    pub apellido: String,
    /// Referencia al rol asignado (record<role>).
    pub role: RecordId,
    pub operacion: Option<Operacion>,
    pub is_active: bool,
    pub created_at: Datetime,
    pub updated_at: Datetime,
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
    /// Vencimiento de portación de armas (DD/MM/AAAA)
    pub vencimiento_portacion: Option<String>,
    /// Indica si el usuario debe cambiar su contraseña en el próximo inicio de sesión.
    pub must_change_password: bool,
    pub deleted_at: Option<Datetime>,
    pub avatar_path: Option<String>,
}

/// Versión "poblada" del usuario con la estructura completa de su Rol (FETCH).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFetched {
    pub id: RecordId,
    pub email: String,
    pub nombre: String,
    pub apellido: String,
    pub role: Option<Role>, // Opcional para manejar inconsistencias de integridad
    pub operacion: Option<Operacion>,
    pub is_active: bool,
    pub created_at: Datetime,
    pub updated_at: Datetime,
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
    pub vencimiento_portacion: Option<String>,
    pub must_change_password: bool,
    pub deleted_at: Option<Datetime>,
    pub avatar_path: Option<String>,
}

// --------------------------------------------------------------------------
// DTOs DE ENTRADA (Commands)
// --------------------------------------------------------------------------

/// Datos necesarios para crear un nuevo usuario.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserInput {
    pub email: String,
    pub password: Option<String>,
    pub nombre: String,
    pub apellido: String,
    pub role_id: Option<String>,
    pub operacion: Operacion,
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
    pub vencimiento_portacion: String,
    pub must_change_password: Option<bool>,
    pub avatar_path: Option<String>,
}

/// Datos para actualizar un usuario existente.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserInput {
    pub email: Option<String>,
    pub password: Option<String>,
    pub nombre: Option<String>,
    pub apellido: Option<String>,
    pub role_id: Option<String>,
    pub operacion: Option<Operacion>,
    pub is_active: Option<bool>,
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
    pub vencimiento_portacion: Option<String>,
    pub must_change_password: Option<bool>,
    pub avatar_path: Option<String>,
}

/// Datos para cambio de contraseña.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordInput {
    pub current_password: Option<String>,
    pub new_password: String,
}

// --------------------------------------------------------------------------
// DTOs PARA PERSISTENCIA
// --------------------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct UserCreateDTO {
    pub email: String,
    pub password_hash: String,
    pub nombre: String,
    pub apellido: String,
    pub role: RecordId,
    pub operacion: Option<Operacion>,
    pub cedula: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_apellido: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fecha_inicio_labores: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numero_gafete: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fecha_nacimiento: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telefono: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direccion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacto_emergencia_nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacto_emergencia_telefono: Option<String>,
    pub vencimiento_portacion: String,
    pub must_change_password: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_path: Option<String>,
}

#[derive(Debug, Serialize, Default)]
pub struct UserUpdateDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apellido: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<RecordId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operacion: Option<Operacion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cedula: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_apellido: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fecha_inicio_labores: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numero_gafete: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fecha_nacimiento: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telefono: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direccion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacto_emergencia_nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacto_emergencia_telefono: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vencimiento_portacion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub must_change_password: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Datetime>,
}

// --------------------------------------------------------------------------
// DTOs DE SALIDA (Responses)
// --------------------------------------------------------------------------

/// Respuesta estándar de Usuario para el Frontend.
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub nombre: String,
    pub apellido: String,
    pub nombre_completo: String,
    pub role_id: String,
    pub role_name: String,
    pub operacion: Option<Operacion>,
    pub is_superuser: bool,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
    pub permissions: Vec<String>,
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
    pub vencimiento_portacion: Option<String>,
    pub must_change_password: bool,
    pub temporary_password: Option<String>,
}

impl UserResponse {
    pub fn from_user_with_role(u: User, role: Role, permissions: Vec<String>) -> Self {
        let role_name = role.name;

        // Construir nombre completo
        let mut parts = vec![u.nombre.as_str()];
        if let Some(ref sn) = u.segundo_nombre {
            parts.push(sn.as_str());
        }
        parts.push(u.apellido.as_str());
        if let Some(ref sa) = u.segundo_apellido {
            parts.push(sa.as_str());
        }
        let nombre_completo = parts.join(" ");

        Self {
            id: u.id.to_string(),
            email: u.email,
            nombre: u.nombre,
            apellido: u.apellido,
            nombre_completo,
            role_id: u.role.to_string(),
            role_name,
            operacion: u.operacion,
            is_superuser: u.id.to_string() == format!("user:{GOD_ID}")
                || u.id.to_string() == format!("user:⟨{GOD_ID}⟩"), // Fallback robusto
            permissions, // Now included
            is_active: u.is_active,
            created_at: u.created_at.to_string(),
            updated_at: u.updated_at.to_string(),
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
            vencimiento_portacion: u.vencimiento_portacion,
            must_change_password: u.must_change_password,
            temporary_password: None,
        }
    }

    pub fn from_fetched(u: UserFetched, permissions: Vec<String>) -> Self {
        // Handle optional role with defaults
        let (role_id, role_name) = match &u.role {
            Some(role) => (role.id.to_string(), role.name.clone()),
            None => ("unknown".to_string(), "Sin Rol".to_string()),
        };

        // Construir nombre completo
        let mut parts = vec![u.nombre.as_str()];
        if let Some(ref sn) = u.segundo_nombre {
            parts.push(sn.as_str());
        }
        parts.push(u.apellido.as_str());
        if let Some(ref sa) = u.segundo_apellido {
            parts.push(sa.as_str());
        }
        let nombre_completo = parts.join(" ");

        Self {
            id: u.id.to_string(),
            email: u.email,
            nombre: u.nombre,
            apellido: u.apellido,
            nombre_completo,
            role_id,
            role_name,
            operacion: u.operacion,
            is_superuser: u.id.to_string() == format!("user:{GOD_ID}")
                || u.id.to_string() == format!("user:⟨{GOD_ID}⟩"),
            permissions,
            is_active: u.is_active,
            created_at: u.created_at.to_string(),
            updated_at: u.updated_at.to_string(),
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
            vencimiento_portacion: u.vencimiento_portacion,
            must_change_password: u.must_change_password,
            temporary_password: None,
        }
    }
}

// --------------------------------------------------------------------------
// DTOs PARA SESIÓN
// --------------------------------------------------------------------------

/// Representa el subconjunto de datos del usuario que se mantiene en RAM
/// para la gestión de la sesión activa y control de permisos.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionUser {
    pub id: String,
    pub email: String,
    pub nombre: String,
    pub apellido: String,
    pub role_id: String,
    pub role_name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserListResponse {
    pub users: Vec<UserResponse>,
    pub total: usize,
    pub activos: usize,
}
