// ==========================================
// src/services/live_service.rs
// SurrealDB LIVE Queries - Real-time Event Emitter
// ==========================================
//
// Este servicio gestiona suscripciones LIVE a tablas de SurrealDB
// y emite eventos Tauri cuando los datos cambian.
//
// Arquitectura:
// SurrealDB → LIVE Stream → live_service → Tauri Events → Frontend
//

use crate::models::lista_negra::ListaNegra;
use crate::models::user::User;
use crate::services::surrealdb_service::{get_surrealdb, SurrealDbError};
use futures::StreamExt;
use log::{error, info, warn};
use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};
use surrealdb::Notification;
use tauri::{AppHandle, Emitter};

// ==========================================
// TIPOS PÚBLICOS
// ==========================================

/// Acción que disparó la notificación LIVE
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LiveAction {
    Create,
    Update,
    Delete,
}

impl From<surrealdb::Action> for LiveAction {
    fn from(action: surrealdb::Action) -> Self {
        match action {
            surrealdb::Action::Create => Self::Create,
            surrealdb::Action::Update => Self::Update,
            surrealdb::Action::Delete => Self::Delete,
            // Handle any other variants that might exist
            _ => Self::Update,
        }
    }
}

/// Notificación enviada al frontend (tipada)
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveNotificationPayload<T: Serialize> {
    pub action: LiveAction,
    pub table: String,
    pub data: T,
}

// ==========================================
// NOMBRES DE EVENTOS
// ==========================================

pub mod events {
    pub const USER_CHANGED: &str = "user:changed";
    pub const LISTA_NEGRA_CHANGED: &str = "lista_negra:changed";
    // Futura fase:
    // pub const CONTRATISTA_CHANGED: &str = "contratista:changed";
}

// ==========================================
// ESTADO GLOBAL
// ==========================================

/// Flag para indicar si las suscripciones deben detenerse
static SHOULD_STOP: AtomicBool = AtomicBool::new(false);

// ==========================================
// FUNCIONES PÚBLICAS
// ==========================================

/// Inicia las suscripciones LIVE para todas las tablas prioritarias.
/// Debe llamarse después de que SurrealDB esté conectado.
pub async fn start_live_subscriptions(app_handle: AppHandle) -> Result<(), SurrealDbError> {
    info!("📡 Iniciando suscripciones LIVE...");

    // Resetear flag de parada
    SHOULD_STOP.store(false, Ordering::SeqCst);

    // Obtener el servicio de SurrealDB
    let service = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let db = service.get_client().await?;

    // ==========================================
    // PHASE 1: User subscription (tipada)
    // ==========================================
    {
        let app_handle_clone = app_handle.clone();
        let db_clone = db.clone();
        tokio::spawn(async move {
            if let Err(e) = subscribe_to_user(db_clone, app_handle_clone).await {
                error!("❌ Error en suscripción LIVE para usuario: {}", e);
            }
        });
        info!("✅ Suscripción LIVE iniciada para tabla: usuario");
    }

    // ==========================================
    // PHASE 2: Lista Negra subscription (tipada)
    // ==========================================
    {
        let app_handle_clone = app_handle.clone();
        let db_clone = db.clone();
        tokio::spawn(async move {
            if let Err(e) = subscribe_to_lista_negra(db_clone, app_handle_clone).await {
                error!("❌ Error en suscripción LIVE para lista_negra: {}", e);
            }
        });
        info!("✅ Suscripción LIVE iniciada para tabla: lista_negra");
    }

    // ==========================================
    // FUTURAS FASES (deshabilitadas temporalmente)
    // ==========================================
    // Phase 3: contratista

    info!("📡 Todas las suscripciones LIVE iniciadas correctamente");
    Ok(())
}

/// Detiene todas las suscripciones LIVE.
/// Útil para cleanup cuando la app se cierra.
pub fn stop_all_subscriptions() {
    info!("🛑 Deteniendo todas las suscripciones LIVE...");
    SHOULD_STOP.store(true, Ordering::SeqCst);
}

// ==========================================
// SUSCRIPCIONES TIPADAS POR TABLA
// ==========================================

/// Suscripción tipada para tabla `usuario`
async fn subscribe_to_user(
    db: surrealdb::Surreal<surrealdb::engine::local::Db>,
    app_handle: AppHandle,
) -> Result<(), SurrealDbError> {
    info!("🔄 Iniciando LIVE query para tabla: usuario");

    // Crear la suscripción LIVE con tipo específico
    let mut stream = db
        .select("usuario")
        .live()
        .await
        .map_err(|e| SurrealDbError::Query(format!("Error creando LIVE query: {}", e)))?;

    info!("📡 Stream LIVE activo para tabla: usuario");

    // Procesar notificaciones del stream
    while let Some(result) = stream.next().await {
        // Verificar si debemos detenernos
        if SHOULD_STOP.load(Ordering::SeqCst) {
            info!("🛑 Deteniendo LIVE query para tabla: usuario");
            break;
        }

        match result {
            Ok(notification) => {
                if let Err(e) = process_user_notification(notification, &app_handle).await {
                    error!("❌ Error procesando notificación para usuario: {}", e);
                }
            }
            Err(e) => {
                error!("❌ Error en stream LIVE para usuario: {}", e);
                // Continuar intentando recibir notificaciones
            }
        }
    }

    warn!(
        "⚠️ Stream LIVE terminado para tabla: usuario (esto no debería pasar en operación normal)"
    );
    Ok(())
}

/// Procesa una notificación de usuario y la emite como evento Tauri
async fn process_user_notification(
    notification: Notification<User>,
    app_handle: &AppHandle,
) -> Result<(), SurrealDbError> {
    let action = LiveAction::from(notification.action);

    info!("📨 Notificación LIVE recibida: tabla=usuario, acción={:?}", action);

    // Convertir User a JSON Value para el frontend
    let data = serde_json::to_value(&notification.data)
        .map_err(|e| SurrealDbError::Query(format!("Error serializando user data: {}", e)))?;

    let payload = LiveNotificationPayload { action, table: "usuario".to_string(), data };

    // Emitir evento a todos los listeners del frontend
    app_handle
        .emit(events::USER_CHANGED, &payload)
        .map_err(|e| SurrealDbError::Query(format!("Error emitiendo evento Tauri: {}", e)))?;

    info!("✅ Evento '{}' emitido al frontend", events::USER_CHANGED);
    Ok(())
}

/// Suscripción tipada para tabla `lista_negra`
async fn subscribe_to_lista_negra(
    db: surrealdb::Surreal<surrealdb::engine::local::Db>,
    app_handle: AppHandle,
) -> Result<(), SurrealDbError> {
    info!("🔄 Iniciando LIVE query para tabla: lista_negra");

    // Crear la suscripción LIVE con tipo específico
    let mut stream = db
        .select("lista_negra")
        .live()
        .await
        .map_err(|e| SurrealDbError::Query(format!("Error creando LIVE query: {}", e)))?;

    info!("📡 Stream LIVE activo para tabla: lista_negra");

    // Procesar notificaciones del stream
    while let Some(result) = stream.next().await {
        // Verificar si debemos detenernos
        if SHOULD_STOP.load(Ordering::SeqCst) {
            info!("🛑 Deteniendo LIVE query para tabla: lista_negra");
            break;
        }

        match result {
            Ok(notification) => {
                if let Err(e) = process_lista_negra_notification(notification, &app_handle).await {
                    error!("❌ Error procesando notificación para lista_negra: {}", e);
                }
            }
            Err(e) => {
                error!("❌ Error en stream LIVE para lista_negra: {}", e);
                // Continuar intentando recibir notificaciones
            }
        }
    }

    warn!("⚠️ Stream LIVE terminado para tabla: lista_negra (esto no debería pasar en operación normal)");
    Ok(())
}

/// Procesa una notificación de lista_negra y la emite como evento Tauri
async fn process_lista_negra_notification(
    notification: Notification<ListaNegra>,
    app_handle: &AppHandle,
) -> Result<(), SurrealDbError> {
    let action = LiveAction::from(notification.action);

    info!("📨 Notificación LIVE recibida: tabla=lista_negra, acción={:?}", action);

    // Convertir ListaNegra a JSON Value para el frontend
    let data = serde_json::to_value(&notification.data).map_err(|e| {
        SurrealDbError::Query(format!("Error serializando lista_negra data: {}", e))
    })?;

    let payload = LiveNotificationPayload { action, table: "lista_negra".to_string(), data };

    // Emitir evento a todos los listeners del frontend
    app_handle
        .emit(events::LISTA_NEGRA_CHANGED, &payload)
        .map_err(|e| SurrealDbError::Query(format!("Error emitiendo evento Tauri: {}", e)))?;

    info!("✅ Evento '{}' emitido al frontend", events::LISTA_NEGRA_CHANGED);
    Ok(())
}
