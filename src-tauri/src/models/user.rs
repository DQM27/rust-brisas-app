use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, RecordId};

// ==========================================
// MODELO DE BASE DE DATOS (SurrealDB Native)
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: RecordId,
    pub email: String,
    pub nombre: String,
    pub apellido: String,
    pub role: RecordId, // Según esquema: record<role>
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
    pub must_change_password: bool,
    pub deleted_at: Option<Datetime>,
    pub avatar_path: Option<String>,
}

// ==========================================
// DTOs DE ENTRADA (Frontend Friendly)
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
// DTOs DE BASE DE DATOS (SurrealDB Friendly)
// ==========================================

#[derive(Debug, Serialize)]
pub struct UserCreateDTO {
    pub email: String,
    pub password_hash: String,
    pub nombre: String,
    pub apellido: String,
    pub role: RecordId,
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
    pub must_change_password: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_path: Option<String>,
}

// ==========================================
// DTOs DE SALIDA (JSON Stable)
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
            id: u.id.to_string(),
            email: u.email,
            nombre: u.nombre,
            apellido: u.apellido,
            nombre_completo,
            role_id: u.role.to_string(),
            role_name,
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
