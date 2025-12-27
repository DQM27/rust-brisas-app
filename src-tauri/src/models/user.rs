// ==========================================
// src/models/user.rs
// ==========================================

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// ==========================================
// MODELO DE DOMINIO
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub email: String,
    pub nombre: String,
    pub apellido: String,
    pub role_id: String,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
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
    pub deleted_at: Option<String>,
    pub avatar_path: Option<String>,
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
    pub role_id: Option<String>,
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
    pub must_change_password: Option<bool>,
    pub avatar_path: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserInput {
    pub email: Option<String>,
    pub password: Option<String>,
    pub nombre: Option<String>,
    pub apellido: Option<String>,
    pub role_id: Option<String>,
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
    pub must_change_password: Option<bool>,
    pub avatar_path: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordInput {
    pub current_password: Option<String>,
    pub new_password: String,
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
    pub role_id: String,
    pub role_name: String,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
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

impl UserResponse {
    pub fn from_user_with_role(u: User, role_name: String) -> Self {
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
            email: u.email,
            nombre: u.nombre,
            apellido: u.apellido,
            nombre_completo,
            role_id: u.role_id,
            role_name,
            is_active: u.is_active,
            created_at: u.created_at,
            updated_at: u.updated_at,
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
            temporary_password: None,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserListResponse {
    pub users: Vec<UserResponse>,
    pub total: usize,
    pub activos: usize,
}
