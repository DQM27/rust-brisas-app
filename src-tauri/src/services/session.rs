//! # Servicio: Gestión de Sesión y RBAC
//!
//! Este servicio mantiene el estado del usuario actualmente autenticado en la
//! memoria RAM de la aplicación. Actúa como el "Gatekeeper" de seguridad,
//! proporcionando el contexto de identidad necesario para la validación de permisos.
//!
//! ## Responsabilidades
//! - Persistencia temporal de la identidad del usuario (`SessionState`).
//! - Orquestación de la validación de permisos (RBAC).
//! - Control de acceso fail-fast mediante `require_session`.

use crate::models::role::{Action, Module};
use crate::models::user::SessionUser;
use crate::services::surrealdb_authorization::{self as authorization, AuthError};
use log::{debug, info, warn};
use std::sync::RwLock;

// ==========================================
// ESTADO DE SESIÓN (Contenedor de Seguridad)
// ==========================================

/// Gestor centralizado de la sesión activa del usuario.
/// Utiliza `RwLock` para permitir lecturas concurrentes rápidas
/// y escrituras atómicas durante el login/logout.
pub struct SessionState {
    current_user: RwLock<Option<SessionUser>>,
}

impl SessionState {
    pub const fn new() -> Self {
        Self { current_user: RwLock::new(None) }
    }

    /// Inicia la sesión vinculando un usuario autenticado.
    pub fn set_user(&self, user: SessionUser) {
        info!("🔐 Sesión iniciada para el usuario: {} ({})", user.email, user.role_name);
        let mut guard =
            self.current_user.write().expect("Fallo crítico: Bloqueo de sesión corrompido");
        *guard = Some(user);
    }

    /// Recupera los datos del usuario actual si existe una sesión activa.
    pub fn get_user(&self) -> Option<SessionUser> {
        let guard = self.current_user.read().expect("Fallo crítico: Bloqueo de sesión corrompido");
        guard.clone()
    }

    pub fn clear(&self) {
        if let Some(user) = self.get_user() {
            info!("🔓 Sesión finalizada para el usuario: {}", user.email);
        }
        let mut guard =
            self.current_user.write().expect("Fallo crítico: Bloqueo de sesión corrompido");
        *guard = None;
    }

    pub fn is_authenticated(&self) -> bool {
        let guard = self.current_user.read().expect("Fallo crítico: Bloqueo de sesión corrompido");
        guard.is_some()
    }

    /// Control de Flujo: Asegura que el usuario esté presente antes de continuar.
    pub fn require_session(&self) -> Result<SessionUser, AuthError> {
        self.get_user().ok_or_else(|| {
            warn!("🛑 Intento de acceso denegado: Sesión requerida");
            AuthError::SessionRequired
        })
    }

    /// Verificación de Privilegios: El "Gatekeeper" de la lógica de negocio.
    ///
    /// Verifica dinámicamente si el usuario actual tiene el permiso (Módulo + Acción)
    /// necesario para ejecutar una operación, consultando el motor RBAC.
    pub async fn require_permission(
        &self,
        module: Module,
        action: Action,
    ) -> Result<SessionUser, AuthError> {
        let user = self.require_session()?;

        debug!(
            "🕵️ Verificando permisos para {}:{} -> User: {}",
            module.as_str(),
            action.as_str(),
            user.email
        );

        authorization::check_permission(&user.id, &user.role_id, module, action).await?;

        Ok(user)
    }
}

impl Default for SessionState {
    fn default() -> Self {
        Self::new()
    }
}

// --------------------------------------------------------------------------
// PRUEBAS UNITARIAS
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_user() -> SessionUser {
        SessionUser {
            id: "user:123".to_string(),
            email: "test@brisas.local".to_string(),
            nombre: "Test".to_string(),
            apellido: "User".to_string(),
            role_id: "role:admin".to_string(),
            role_name: "Administrador".to_string(),
        }
    }

    #[test]
    fn test_session_lifecycle() {
        let state = SessionState::new();
        assert!(!state.is_authenticated());

        let user = mock_user();
        state.set_user(user.clone());

        assert!(state.is_authenticated());
        assert_eq!(state.get_user().unwrap().id, "user:123");

        state.clear();
        assert!(!state.is_authenticated());
    }

    #[test]
    fn test_require_session() {
        let state = SessionState::new();
        let res = state.require_session();
        assert!(matches!(res, Err(AuthError::SessionRequired)));

        state.set_user(mock_user());
        assert!(state.require_session().is_ok());
    }
}
