// ==========================================
// src/models/role.rs
// ==========================================
// Solo modelos, DTOs y enums - SIN validaciones ni lógica

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// ==========================================
// ENUMS PARA MÓDULOS Y ACCIONES
// ==========================================

/// Módulos del sistema (recursos protegidos)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Module {
    Users,
    Roles,
    Contratistas,
    Empresas,
    Proveedores,
    Visitantes,
    Ingresos,
    Citas,
    Vehiculos,
    Gafetes,
    ListaNegra,
    Config,
    Backup,
    Export,
}

impl Module {
    pub fn all() -> Vec<Module> {
        vec![
            Module::Users,
            Module::Roles,
            Module::Contratistas,
            Module::Empresas,
            Module::Proveedores,
            Module::Visitantes,
            Module::Ingresos,
            Module::Citas,
            Module::Vehiculos,
            Module::Gafetes,
            Module::ListaNegra,
            Module::Config,
            Module::Backup,
            Module::Export,
        ]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Module::Users => "users",
            Module::Roles => "roles",
            Module::Contratistas => "contratistas",
            Module::Empresas => "empresas",
            Module::Proveedores => "proveedores",
            Module::Visitantes => "visitantes",
            Module::Ingresos => "ingresos",
            Module::Citas => "citas",
            Module::Vehiculos => "vehiculos",
            Module::Gafetes => "gafetes",
            Module::ListaNegra => "lista_negra",
            Module::Config => "config",
            Module::Backup => "backup",
            Module::Export => "export",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Module::Users => "Usuarios",
            Module::Roles => "Roles",
            Module::Contratistas => "Contratistas",
            Module::Empresas => "Empresas",
            Module::Proveedores => "Proveedores",
            Module::Visitantes => "Visitantes",
            Module::Ingresos => "Ingresos",
            Module::Citas => "Citas",
            Module::Vehiculos => "Vehículos",
            Module::Gafetes => "Gafetes",
            Module::ListaNegra => "Lista Negra",
            Module::Config => "Configuración",
            Module::Backup => "Respaldos",
            Module::Export => "Exportar",
        }
    }
}

impl std::str::FromStr for Module {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "users" => Ok(Module::Users),
            "roles" => Ok(Module::Roles),
            "contratistas" => Ok(Module::Contratistas),
            "empresas" => Ok(Module::Empresas),
            "proveedores" => Ok(Module::Proveedores),
            "visitantes" => Ok(Module::Visitantes),
            "ingresos" => Ok(Module::Ingresos),
            "citas" => Ok(Module::Citas),
            "vehiculos" => Ok(Module::Vehiculos),
            "gafetes" => Ok(Module::Gafetes),
            "lista_negra" => Ok(Module::ListaNegra),
            "config" => Ok(Module::Config),
            "backup" => Ok(Module::Backup),
            "export" => Ok(Module::Export),
            _ => Err(format!("Unknown module: {}", s)),
        }
    }
}

/// Acciones que se pueden realizar en un módulo
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    View,
    Create,
    Read,
    Update,
    Delete,
    Export,
}

impl Action {
    pub fn all() -> Vec<Action> {
        vec![
            Action::View,
            Action::Create,
            Action::Read,
            Action::Update,
            Action::Delete,
            Action::Export,
        ]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Action::View => "view",
            Action::Create => "create",
            Action::Read => "read",
            Action::Update => "update",
            Action::Delete => "delete",
            Action::Export => "export",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Action::View => "Ver",
            Action::Create => "Crear",
            Action::Read => "Leer",
            Action::Update => "Actualizar",
            Action::Delete => "Eliminar",
            Action::Export => "Exportar",
        }
    }
}

impl std::str::FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "view" => Ok(Action::View),
            "create" => Ok(Action::Create),
            "read" => Ok(Action::Read),
            "update" => Ok(Action::Update),
            "delete" => Ok(Action::Delete),
            "export" => Ok(Action::Export),
            _ => Err(format!("Unknown action: {}", s)),
        }
    }
}

// ==========================================
// MODELO DE DOMINIO (DB)
// ==========================================

/// Rol en la base de datos
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub is_system: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// Permiso en la base de datos
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Permission {
    pub id: String,
    pub module: String,
    pub action: String,
    pub description: Option<String>,
}

/// Relación role-permission
#[derive(Debug, Clone, FromRow)]
pub struct RolePermission {
    pub role_id: String,
    pub permission_id: String,
}

// ==========================================
// DTOs DE ENTRADA
// ==========================================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRoleInput {
    pub name: String,
    pub description: Option<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRoleInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub permissions: Option<Vec<String>>,
}

// ==========================================
// DTOs DE SALIDA
// ==========================================

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub is_system: bool,
    pub permissions: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl RoleResponse {
    pub fn from_role_with_permissions(role: Role, permissions: Vec<String>) -> Self {
        Self {
            id: role.id,
            name: role.name,
            description: role.description,
            is_system: role.is_system,
            permissions,
            created_at: role.created_at,
            updated_at: role.updated_at,
        }
    }
}

/// Módulo visible con sus permisos
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VisibleModule {
    pub module: String,
    pub display_name: String,
    pub can_create: bool,
    pub can_read: bool,
    pub can_update: bool,
    pub can_delete: bool,
    pub can_export: bool,
}

/// Lista de roles con estadísticas
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleListResponse {
    pub roles: Vec<RoleResponse>,
    pub total: usize,
    pub system_roles: usize,
    pub custom_roles: usize,
}
