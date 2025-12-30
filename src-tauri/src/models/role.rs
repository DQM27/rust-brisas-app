// ==========================================
// src/models/role.rs
// ==========================================

use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, RecordId};

// ==========================================
// ENUMS PARA MÓDULOS Y ACCIONES
// ==========================================

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
            Module::SettingsGeneral,
            Module::SettingsVisual,
            Module::SettingsSecurity,
            Module::SettingsSessions,
            Module::Backup,
            Module::Export,
            Module::Trash,
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
            Module::SettingsGeneral => "settings_general",
            Module::SettingsVisual => "settings_visual",
            Module::SettingsSecurity => "settings_security",
            Module::SettingsSessions => "settings_sessions",
            Module::Backup => "backup",
            Module::Export => "export",
            Module::Trash => "trash",
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
            Module::SettingsGeneral => "Ajustes Generales",
            Module::SettingsVisual => "Ajustes Visuales",
            Module::SettingsSecurity => "Seguridad",
            Module::SettingsSessions => "Sesiones",
            Module::Backup => "Respaldos",
            Module::Export => "Exportar",
            Module::Trash => "Papelera",
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
            "settings_general" => Ok(Module::SettingsGeneral),
            "settings_visual" => Ok(Module::SettingsVisual),
            "settings_security" => Ok(Module::SettingsSecurity),
            "settings_sessions" => Ok(Module::SettingsSessions),
            "backup" => Ok(Module::Backup),
            "export" => Ok(Module::Export),
            "trash" => Ok(Module::Trash),
            _ => Err(format!("Unknown module: {}", s)),
        }
    }
}

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

// ==========================================
// DTOs DE ENTRADA (Frontend -> Command)
// ==========================================

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

// ==========================================
// DTOs PARA PERSISTENCIA (Service -> DB)
// ==========================================

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

// ==========================================
// DTOs DE SALIDA (Service -> Frontend)
// ==========================================

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
