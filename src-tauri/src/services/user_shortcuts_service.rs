/// Servicio: Gestión de Atajos de Teclado Personalizados
///
/// Este servicio maneja la persistencia de atajos de teclado personalizados
/// por usuario en SurrealDB.
use crate::models::user_shortcuts::{
    UserShortcutInput, UserShortcutResponse, UserShortcutsListResponse,
};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use log::info;
use serde::{Deserialize, Serialize};

/// Estructura interna para deserializar desde SurrealDB
#[derive(Debug, Serialize, Deserialize)]
struct DbUserShortcut {
    id: surrealdb::sql::Thing,
    user: surrealdb::sql::Thing,
    shortcut_id: String,
    custom_keys: String,
    enabled: bool,
}

impl From<DbUserShortcut> for UserShortcutResponse {
    fn from(db: DbUserShortcut) -> Self {
        Self {
            id: db.id.to_string(),
            user_id: db.user.to_string(),
            shortcut_id: db.shortcut_id,
            custom_keys: db.custom_keys,
            enabled: db.enabled,
        }
    }
}

/// Extrae el ID limpio de un user_id que puede venir como "user:⟨uuid⟩" o solo "uuid"
fn extract_user_id(user_id: &str) -> String {
    // Si viene como "user:⟨uuid⟩" extraemos solo el uuid
    if user_id.starts_with("user:") {
        let id = user_id.trim_start_matches("user:");
        // Remover los caracteres unicode ⟨ y ⟩
        id.trim_start_matches('⟨').trim_end_matches('⟩').to_string()
    } else {
        user_id.to_string()
    }
}

pub struct UserShortcutService;

impl UserShortcutService {
    /// Obtiene todos los atajos personalizados de un usuario
    pub async fn get_user_shortcuts(
        user_id: &str,
    ) -> Result<UserShortcutsListResponse, SurrealDbError> {
        let db = get_db().await?;
        let clean_id = extract_user_id(user_id);

        let results: Vec<DbUserShortcut> = db
            .query("SELECT * FROM user_shortcuts WHERE user = type::thing('user', $user_id)")
            .bind(("user_id", clean_id))
            .await?
            .take(0)?;

        let shortcuts: Vec<UserShortcutResponse> = results.into_iter().map(|r| r.into()).collect();

        Ok(UserShortcutsListResponse { shortcuts })
    }

    /// Guarda o actualiza un atajo personalizado
    pub async fn upsert_shortcut(
        user_id: &str,
        input: UserShortcutInput,
    ) -> Result<UserShortcutResponse, SurrealDbError> {
        let db = get_db().await?;
        let clean_id = extract_user_id(user_id);

        // Verificar si ya existe
        let existing: Option<DbUserShortcut> = db
            .query("SELECT * FROM user_shortcuts WHERE user = type::thing('user', $user_id) AND shortcut_id = $shortcut_id LIMIT 1")
            .bind(("user_id", clean_id.clone()))
            .bind(("shortcut_id", input.shortcut_id.clone()))
            .await?
            .take(0)?;

        let result: Option<DbUserShortcut> = if let Some(existing) = existing {
            // Actualizar
            info!("🔄 Actualizando atajo '{}' para usuario '{}'", input.shortcut_id, user_id);
            let mut response = db.query(
                "UPDATE $record_id SET custom_keys = $custom_keys, enabled = $enabled, updated_at = time::now() RETURN AFTER"
            )
            .bind(("record_id", existing.id.clone()))
            .bind(("custom_keys", input.custom_keys.clone()))
            .bind(("enabled", input.enabled))
            .await?;

            let results: Vec<DbUserShortcut> = response.take(0)?;
            results.into_iter().next()
        } else {
            // Crear nuevo
            info!("➕ Creando atajo '{}' para usuario '{}'", input.shortcut_id, user_id);
            let mut response = db
                .query(
                    r#"
                CREATE user_shortcuts SET 
                    user = type::thing('user', $user_id),
                    shortcut_id = $shortcut_id,
                    custom_keys = $custom_keys,
                    enabled = $enabled
                RETURN AFTER
                "#,
                )
                .bind(("user_id", clean_id))
                .bind(("shortcut_id", input.shortcut_id.clone()))
                .bind(("custom_keys", input.custom_keys.clone()))
                .bind(("enabled", input.enabled))
                .await?;

            let results: Vec<DbUserShortcut> = response.take(0)?;
            results.into_iter().next()
        };

        result
            .map(|r| r.into())
            .ok_or_else(|| SurrealDbError::Query("No se pudo guardar el atajo".to_string()))
    }

    /// Elimina un atajo personalizado (vuelve al default)
    pub async fn delete_shortcut(user_id: &str, shortcut_id: &str) -> Result<(), SurrealDbError> {
        let db = get_db().await?;
        let clean_id = extract_user_id(user_id);

        info!("🗑️ Eliminando atajo personalizado '{}' para usuario '{}'", shortcut_id, user_id);

        db.query("DELETE FROM user_shortcuts WHERE user = type::thing('user', $user_id) AND shortcut_id = $shortcut_id")
            .bind(("user_id", clean_id))
            .bind(("shortcut_id", shortcut_id.to_string()))
            .await?;

        Ok(())
    }

    /// Elimina todas las personalizaciones de un usuario (reset total)
    pub async fn reset_all_shortcuts(user_id: &str) -> Result<(), SurrealDbError> {
        let db = get_db().await?;
        let clean_id = extract_user_id(user_id);

        info!("🔄 Reseteando todos los atajos personalizados para usuario '{}'", user_id);

        db.query("DELETE FROM user_shortcuts WHERE user = type::thing('user', $user_id)")
            .bind(("user_id", clean_id))
            .await?;

        Ok(())
    }
}
