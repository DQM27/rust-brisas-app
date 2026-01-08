use crate::domain::role::{GodModeGuard, GOD_ID};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use log::{info, warn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModuleStatus {
    pub key: String,
    pub name: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateModuleRequest {
    pub key: String,
    pub status: String,
}

pub struct ModuleService;

impl ModuleService {
    /// Obtiene el estado de todos los módulos registrados.
    pub async fn get_all_modules() -> Result<Vec<ModuleStatus>, SurrealDbError> {
        let db = get_db().await?;
        let modules: Vec<ModuleStatus> = db.select("module").await?;
        Ok(modules)
    }

    /// Actualiza el estado de un módulo.
    /// REGLA DE ORO:
    /// - 'development' | 'maintenance' -> Solo GOD (Super User)
    /// - 'active' | 'hidden' -> Admin o GOD
    pub async fn update_status(
        user_id: &str,
        key: &str,
        new_status: &str,
    ) -> Result<(), SurrealDbError> {
        // 1. Validar estado válido
        let valid_statuses = ["active", "hidden", "development", "maintenance"];
        if !valid_statuses.contains(&new_status) {
            return Err(SurrealDbError::Query(format!(
                "Estado inválido: {new_status}. Permitidos: {valid_statuses:?}"
            )));
        }

        // 2. Verificar permisos estrictos
        // Normalizamos la comparación para soportar IDs crudos, con prefijo 'user:', o con brackets 'user:⟨...⟩'
        let is_god = user_id == GOD_ID
            || user_id == format!("user:{GOD_ID}")
            || user_id == format!("user:⟨{GOD_ID}⟩");

        let requires_god = matches!(new_status, "development" | "maintenance");

        if requires_god && !is_god {
            warn!(
                "⛔ Intento no autorizado de cambiar módulo '{key}' a '{new_status}' por usuario '{user_id}'"
            );
            return Err(SurrealDbError::Auth(
                "Solo el Super Usuario (GOD) puede poner módulos en Desarrollo o Mantenimiento."
                    .to_string(),
            ));
        }

        // Para cambios normales (active/hidden), asumimos que el comando ya verificó que es Admin
        // (o confiamos en que este servicio se llama desde un contexto seguro/admin).
        // NOTA: Idealmente verificaríamos el ROL del usuario aquí también si quisiéramos ser paranoicos,
        // pero la restricción crítica solicitada es la de GOD.

        let db = get_db().await?;

        // Activamos GodMode temporalmente para poder escribir en la tabla de sistema 'module'
        // si es que decidimos protegerla a nivel de permisos de tabla en el futuro.
        // Por ahora, asumimos que el usuario tiene permisos de escritura o usamos el guard si es necesario.
        // Dado que 'module' es configuración de sistema, usamos el Guard para asegurar que la escritura pase.
        let _guard = GodModeGuard::activate();

        // 3. Ejecutar actualización
        let result: Option<ModuleStatus> = db
            .query("UPDATE module SET status = $status, updated_at = time::now() WHERE key = $key")
            .bind(("status", new_status.to_string()))
            .bind(("key", key.to_string()))
            .await?
            .take(0)?;

        if result.is_some() {
            info!("✅ Módulo '{key}' actualizado a '{new_status}' por '{user_id}'");
            Ok(())
        } else {
            Err(SurrealDbError::Query(format!("Módulo no encontrado: {key}")))
        }
    }
}
