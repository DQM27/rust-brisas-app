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

use std::time::SystemTime;

/// Gestor centralizado de la sesión activa del usuario.
/// Utiliza `RwLock` para permitir lecturas concurrentes rápidas
/// y escrituras atómicas durante el login/logout.
pub struct SessionState {
    current_user: RwLock<Option<SessionUser>>,
    session_start_time: RwLock<Option<SystemTime>>,
    ip_address: RwLock<Option<String>>,
}

impl SessionState {
    pub const fn new() -> Self {
        Self {
            current_user: RwLock::new(None),
            session_start_time: RwLock::new(None),
            ip_address: RwLock::new(None),
        }
    }

    /// Inicia la sesión vinculando un usuario autenticado.
    pub fn set_user(&self, user: SessionUser) {
        info!("🔐 Sesión iniciada para el usuario: {} ({})", user.email, user.role_name);
        {
            let mut guard =
                self.current_user.write().expect("Fallo crítico: Bloqueo de sesión corrompido");
            *guard = Some(user);
        }
        {
            let mut time_guard = self.session_start_time.write().expect("Lock fail");
            *time_guard = Some(SystemTime::now());
        }
    }

    pub fn set_ip(&self, ip: String) {
        let mut guard = self.ip_address.write().expect("Lock fail");
        *guard = Some(ip);
    }

    pub fn get_ip(&self) -> Option<String> {
        let guard = self.ip_address.read().expect("Lock fail");
        guard.clone()
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
        {
            let mut guard =
                self.current_user.write().expect("Fallo crítico: Bloqueo de sesión corrompido");
            *guard = None;
        }
        {
            let mut time_guard = self.session_start_time.write().expect("Lock fail");
            *time_guard = None;
        }
        {
            let mut ip_guard = self.ip_address.write().expect("Lock fail");
            *ip_guard = None;
        }
    }

    pub fn is_authenticated(&self) -> bool {
        let guard = self.current_user.read().expect("Fallo crítico: Bloqueo de sesión corrompido");
        guard.is_some()
    }

    /// Calcula la duración de la sesión actual
    pub fn get_duration_string(&self) -> String {
        let guard = self.session_start_time.read().expect("Lock fail");
        if let Some(start) = *guard {
            if let Ok(duration) = start.elapsed() {
                let seconds = duration.as_secs();
                let h = seconds / 3600;
                let m = (seconds % 3600) / 60;
                let s = seconds % 60;
                if h > 0 {
                    return format!("{h}h {m}m");
                }
                if m > 0 {
                    return format!("{m}m {s}s");
                }
                return format!("{s}s");
            }
        }
        "0s".to_string()
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
