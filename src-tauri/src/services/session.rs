/// Gestión de Persistencia y Ciclo de Vida de la Interacción (Sesión).
///
/// Este servicio mantiene el estado del usuario actualmente autenticado en la
/// memoria ram de la aplicación. Es la "Brújula de Identidad" que permite a
/// otros servicios saber quién está operando y qué permisos tiene concedidos.
use crate::models::role::{Action, Module};
use crate::services::surrealdb_authorization::{self as authorization, AuthError};
use serde::{Deserialize, Serialize};
use std::sync::RwLock;

// ==========================================
// MODELO DE IDENTIDAD EN SESIÓN
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
// ESTADO DE SESIÓN (Contenedor de Seguridad)
// ==========================================

pub struct SessionState {
    /// Uso de RwLock para permitir lecturas concurrentes rápidas
    /// y escrituras (login/logout) seguras.
    current_user: RwLock<Option<SessionUser>>,
}

impl SessionState {
    pub fn new() -> Self {
        Self { current_user: RwLock::new(None) }
    }

    /// Inicia la sesión vinculando un usuario autenticado.
    pub fn set_user(&self, user: SessionUser) {
        let mut guard =
            self.current_user.write().expect("Fallo crítico: Bloqueo de sesión corrompido");
        *guard = Some(user);
    }

    pub fn get_user(&self) -> Option<SessionUser> {
        let guard = self.current_user.read().expect("Fallo crítico: Bloqueo de sesión corrompido");
        guard.clone()
    }

    pub fn clear(&self) {
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
        self.get_user().ok_or(AuthError::SessionRequired)
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

        authorization::check_permission(&user.id, &user.role_id, module, action).await?;

        Ok(user)
    }
}

impl Default for SessionState {
    fn default() -> Self {
        Self::new()
    }
}
