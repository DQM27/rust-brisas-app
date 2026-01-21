/// Modelo: Preferencias de Atajos de Teclado por Usuario
///
/// Representa la personalización individual de atajos de teclado,
/// permitiendo que cada usuario configure sus propias combinaciones de teclas.
use serde::{Deserialize, Serialize};

/// Respuesta del atajo personalizado de usuario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserShortcutResponse {
    pub id: String,
    pub user_id: String,
    pub shortcut_id: String,
    pub custom_keys: String,
    pub enabled: bool,
}

/// Input para crear/actualizar un atajo personalizado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserShortcutInput {
    pub shortcut_id: String,
    pub custom_keys: String,
    pub enabled: bool,
}

/// Lista de atajos personalizados del usuario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserShortcutsListResponse {
    pub shortcuts: Vec<UserShortcutResponse>,
}
