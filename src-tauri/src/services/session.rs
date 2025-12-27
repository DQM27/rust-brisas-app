// ==========================================
// src/services/session.rs
// ==========================================
// Estado de sesión del usuario actual

use crate::models::role::{Action, Module};
use crate::services::surrealdb_authorization::{self as authorization, AuthError};
use serde::{Deserialize, Serialize};
use std::sync::RwLock;

// ==========================================
// USUARIO DE SESIÓN (simplificado)
// ==========================================

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

// ==========================================
// ESTADO DE SESIÓN
// ==========================================

pub struct SessionState {
    current_user: RwLock<Option<SessionUser>>,
}

impl SessionState {
    pub fn new() -> Self {
        Self { current_user: RwLock::new(None) }
    }

    /// Establece el usuario de la sesión actual
    pub fn set_user(&self, user: SessionUser) {
        let mut guard = self.current_user.write().expect("Session lock poisoned");
        *guard = Some(user);
    }

    /// Obtiene el usuario de la sesión actual
    pub fn get_user(&self) -> Option<SessionUser> {
        let guard = self.current_user.read().expect("Session lock poisoned");
        guard.clone()
    }

    /// Limpia la sesión (logout)
    pub fn clear(&self) {
        let mut guard = self.current_user.write().expect("Session lock poisoned");
        *guard = None;
    }

    /// Verifica si hay sesión activa
    pub fn is_authenticated(&self) -> bool {
        let guard = self.current_user.read().expect("Session lock poisoned");
        guard.is_some()
    }

    /// Requiere sesión activa, retorna error si no hay
    pub fn require_session(&self) -> Result<SessionUser, AuthError> {
        self.get_user().ok_or(AuthError::SessionRequired)
    }

    /// Verifica permiso y retorna el usuario si tiene acceso
    pub async fn require_permission(
        &self,
        module: Module,
        action: Action,
    ) -> Result<SessionUser, AuthError> {
        let user = self.require_session()?;

        authorization::check_permission(&user.id, &user.role_id, module, action).await?;

        Ok(user)
    }
}

impl Default for SessionState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_state_workflow() {
        let state = SessionState::new();
        assert!(!state.is_authenticated());
        assert!(state.get_user().is_none());

        let user = SessionUser {
            id: "u-1".into(),
            email: "test@example.com".into(),
            nombre: "Test".into(),
            apellido: "User".into(),
            role_id: "admin".into(),
            role_name: "Administrator".into(),
        };

        // Login
        state.set_user(user.clone());
        assert!(state.is_authenticated());
        assert_eq!(state.get_user().unwrap().id, "u-1");

        // Required check
        let req_user = state.require_session().unwrap();
        assert_eq!(req_user.email, "test@example.com");

        // Logout
        state.clear();
        assert!(!state.is_authenticated());
        assert!(state.require_session().is_err());
    }
}
