// ==========================================
// src/models/role.rs
// ==========================================

use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, RecordId};

// --------------------------------------------------------------------------
// ENUMS (Módulos y Acciones)
// --------------------------------------------------------------------------

/// Módulos del sistema sobre los cuales se pueden ejercer permisos.
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
    SettingsGeneral,
    SettingsVisual,
    SettingsSecurity,
    SettingsSessions,
    Backup,
    Export,
    Trash,
}

impl Module {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Users,
            Self::Roles,
            Self::Contratistas,
            Self::Empresas,
            Self::Proveedores,
            Self::Visitantes,
            Self::Ingresos,
            Self::Citas,
            Self::Vehiculos,
            Self::Gafetes,
            Self::ListaNegra,
            Self::Config,
            Self::SettingsGeneral,
            Self::SettingsVisual,
            Self::SettingsSecurity,
            Self::SettingsSessions,
            Self::Backup,
            Self::Export,
            Self::Trash,
        ]
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Users => "users",
            Self::Roles => "roles",
            Self::Contratistas => "contratistas",
            Self::Empresas => "empresas",
            Self::Proveedores => "proveedores",
            Self::Visitantes => "visitantes",
            Self::Ingresos => "ingresos",
            Self::Citas => "citas",
            Self::Vehiculos => "vehiculos",
            Self::Gafetes => "gafetes",
            Self::ListaNegra => "lista_negra",
            Self::Config => "config",
            Self::SettingsGeneral => "settings_general",
            Self::SettingsVisual => "settings_visual",
            Self::SettingsSecurity => "settings_security",
            Self::SettingsSessions => "settings_sessions",
            Self::Backup => "backup",
            Self::Export => "export",
            Self::Trash => "trash",
        }
    }

    pub const fn display_name(&self) -> &'static str {
        match self {
            Self::Users => "Usuarios",
            Self::Roles => "Roles",
            Self::Contratistas => "Contratistas",
            Self::Empresas => "Empresas",
            Self::Proveedores => "Proveedores",
            Self::Visitantes => "Visitantes",
            Self::Ingresos => "Ingresos",
            Self::Citas => "Citas",
            Self::Vehiculos => "Vehículos",
            Self::Gafetes => "Gafetes",
            Self::ListaNegra => "Lista Negra",
            Self::Config => "Configuración",
            Self::SettingsGeneral => "Ajustes Generales",
            Self::SettingsVisual => "Ajustes Visuales",
            Self::SettingsSecurity => "Seguridad",
            Self::SettingsSessions => "Sesiones",
            Self::Backup => "Respaldos",
            Self::Export => "Exportar",
            Self::Trash => "Papelera",
        }
    }
}

impl std::str::FromStr for Module {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "users" => Ok(Self::Users),
            "roles" => Ok(Self::Roles),
            "contratistas" => Ok(Self::Contratistas),
            "empresas" => Ok(Self::Empresas),
            "proveedores" => Ok(Self::Proveedores),
            "visitantes" => Ok(Self::Visitantes),
            "ingresos" => Ok(Self::Ingresos),
            "citas" => Ok(Self::Citas),
            "vehiculos" => Ok(Self::Vehiculos),
            "gafetes" => Ok(Self::Gafetes),
            "lista_negra" => Ok(Self::ListaNegra),
            "config" => Ok(Self::Config),
            "settings_general" => Ok(Self::SettingsGeneral),
            "settings_visual" => Ok(Self::SettingsVisual),
            "settings_security" => Ok(Self::SettingsSecurity),
            "settings_sessions" => Ok(Self::SettingsSessions),
            "backup" => Ok(Self::Backup),
            "export" => Ok(Self::Export),
            "trash" => Ok(Self::Trash),
            _ => Err(format!("Unknown module: {s}")),
        }
    }
}

/// Acciones posibles sobre un módulo (CRUD + Especiales).
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
    pub fn all() -> Vec<Self> {
        vec![Self::View, Self::Create, Self::Read, Self::Update, Self::Delete, Self::Export]
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::View => "view",
            Self::Create => "create",
            Self::Read => "read",
            Self::Update => "update",
            Self::Delete => "delete",
            Self::Export => "export",
        }
    }

    pub const fn display_name(&self) -> &'static str {
        match self {
            Self::View => "Ver",
            Self::Create => "Crear",
            Self::Read => "Leer",
            Self::Update => "Actualizar",
            Self::Delete => "Eliminar",
            Self::Export => "Exportar",
        }
    }
}

impl std::str::FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "view" => Ok(Self::View),
            "create" => Ok(Self::Create),
            "read" => Ok(Self::Read),
            "update" => Ok(Self::Update),
            "delete" => Ok(Self::Delete),
            "export" => Ok(Self::Export),
            _ => Err(format!("Unknown action: {s}")),
        }
    }
}

// --------------------------------------------------------------------------
// MODELO DE DOMINIO
// --------------------------------------------------------------------------

/// Representa un rol de usuario con permisos asociados.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub id: RecordId,
    pub name: String,
    pub description: Option<String>,
    #[serde(alias = "is_system")]
    pub is_system: bool,
    #[serde(alias = "inherits_from")]
    pub inherits_from: Option<RecordId>,
    /// Lista plana de permisos en formato string (ej. "users:create").
    pub permissions: Option<Vec<String>>,
    #[serde(alias = "created_at")]
    pub created_at: Datetime,
    #[serde(alias = "updated_at")]
    pub updated_at: Datetime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub id: String,
    pub module: String,
    pub action: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RolePermission {
    pub role_id: String,
    pub permission_id: String,
}

// --------------------------------------------------------------------------
// DTOs DE ENTRADA (Commands)
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRoleInput {
    pub name: String,
    pub description: Option<String>,
    pub inherits_from: Option<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRoleInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub inherits_from: Option<String>,
    pub permissions: Option<Vec<String>>,
}

// --------------------------------------------------------------------------
// DTOs PARA PERSISTENCIA
// --------------------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct RoleCreateDTO {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub is_system: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherits_from: Option<surrealdb::RecordId>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Serialize, Default)]
pub struct RoleUpdateDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherits_from: Option<surrealdb::RecordId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Datetime>,
}

// --------------------------------------------------------------------------
// DTOs DE SALIDA (Responses)
// --------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub is_system: bool,
    pub inherits_from: Option<String>,
    pub permissions: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl RoleResponse {
    pub fn from_role(role: Role) -> Self {
        Self {
            id: role.id.to_string(),
            name: role.name,
            description: role.description,
            is_system: role.is_system,
            inherits_from: role.inherits_from.map(|r| r.to_string()),
            permissions: role.permissions.unwrap_or_default(),
            created_at: role.created_at.to_string(),
            updated_at: role.updated_at.to_string(),
        }
    }

    pub fn from_role_with_permissions(role: Role, permissions: Vec<String>) -> Self {
        Self {
            id: role.id.to_string(),
            name: role.name,
            description: role.description,
            is_system: role.is_system,
            inherits_from: role.inherits_from.map(|r| r.to_string()),
            permissions,
            created_at: role.created_at.to_string(),
            updated_at: role.updated_at.to_string(),
        }
    }
}

/// Módulo visible con flags de acciones permitidas (para UI).
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleListResponse {
    pub roles: Vec<RoleResponse>,
    pub total: usize,
    pub system_roles: usize,
    pub custom_roles: usize,
}
