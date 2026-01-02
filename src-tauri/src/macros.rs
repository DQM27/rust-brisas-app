/// Macro para asegurar que existe una sesión activa y obtener el usuario.
///
/// Uso:
/// ```rust
/// let user = require_session!(state);
/// ```
#[macro_export]
macro_rules! require_session {
    ($state:expr) => {{
        match $state.require_session() {
            Ok(user) => user,
            Err(e) => return Err(e.into()),
        }
    }};
}

/// Macro para verificar permisos de manera declarativa
///
/// Uso:
/// ```rust
/// require_perm!(state, "contratistas:create")?;
/// require_perm!(state, "users:delete", "Acción de eliminación de usuario")?;
/// ```
#[macro_export]
macro_rules! require_perm {
    ($state:expr, $perm:expr) => {{
        use $crate::services::surrealdb_authorization;
        use $crate::models::role::{Module, Action};

        // Parsear permiso
        let parts: Vec<&str> = $perm.split(':').collect();
        if parts.len() != 2 {
            return Err(surrealdb_authorization::AuthError::Database("Formato de permiso inválido".into()).into());
        }

        let module: Module = parts[0].parse()
            .map_err(|e: String| surrealdb_authorization::AuthError::Database(e))?;
        let action: Action = parts[1].parse()
            .map_err(|e: String| surrealdb_authorization::AuthError::Database(e))?;

        // Verificar permiso real (incluyendo súper usuarios y herencia) - re-uso lógica de SessionState
        match $state.require_permission(module, action).await {
            Ok(user) => Ok::<crate::models::user::SessionUser, crate::services::surrealdb_authorization::AuthError>(user),
            Err(e) => Err(e),
        }
    }};

    // Variante con mensaje de auditoría
    ($state:expr, $perm:expr, $audit_msg:expr) => {{
        let session_res = require_perm!($state, $perm);
        if let Ok(ref session) = session_res {
             log::info!(target: "audit", "[PERM] user={} email={} perm={} msg={}", session.id, session.email, $perm, $audit_msg);
        }
        session_res
    }};
}
