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

use crate::services::surrealdb_service::{get_surrealdb, SurrealDbError};
use futures::StreamExt;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use surrealdb::Notification;
use tauri::{AppHandle, Emitter};

// ==========================================
// TIPOS PÚBLICOS
// ==========================================

/// Acción que disparó la notificación LIVE
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Notificación enviada al frontend
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveNotificationPayload {
    pub action: LiveAction,
    pub table: String,
    /// Los datos se envían como JSON Value para flexibilidad
    pub data: serde_json::Value,
}

// ==========================================
// NOMBRES DE EVENTOS
// ==========================================

pub mod events {
    pub const INGRESO_CHANGED: &str = "ingreso:changed";
    pub const ALERTA_GAFETE_CHANGED: &str = "alerta_gafete:changed";
    pub const GAFETE_CHANGED: &str = "gafete:changed";
    pub const CONTRATISTA_CHANGED: &str = "contratista:changed";
    pub const PROVEEDOR_CHANGED: &str = "proveedor:changed";
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

    // Definir tablas y sus eventos correspondientes
    let subscriptions: Vec<(&str, &str)> = vec![
        ("ingreso", events::INGRESO_CHANGED),
        ("alerta_gafete", events::ALERTA_GAFETE_CHANGED),
        ("gafete", events::GAFETE_CHANGED),
        ("contratista", events::CONTRATISTA_CHANGED),
        ("proveedor", events::PROVEEDOR_CHANGED),
    ];

    for (table, event_name) in subscriptions {
        let app_handle_clone = app_handle.clone();
        let table_name = table.to_string();
        let event_name_owned = event_name.to_string();
        let db_clone = db.clone();

        // Spawn una tarea async para cada suscripción
        tokio::spawn(async move {
            if let Err(e) =
                subscribe_to_table(db_clone, &table_name, &event_name_owned, app_handle_clone).await
            {
                error!("❌ Error en suscripción LIVE para {}: {}", table_name, e);
            }
        });

        info!("✅ Suscripción LIVE iniciada para tabla: {}", table);
    }

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
// FUNCIONES INTERNAS
// ==========================================

/// Suscribe a una tabla específica y emite eventos cuando hay cambios
async fn subscribe_to_table(
    db: surrealdb::Surreal<surrealdb::engine::local::Db>,
    table: &str,
    event_name: &str,
    app_handle: AppHandle,
) -> Result<(), SurrealDbError> {
    info!("🔄 Iniciando LIVE query para tabla: {}", table);

    // Crear la suscripción LIVE usando el método del SDK
    // Usamos serde_json::Value para deserializar cualquier estructura
    let mut stream = db
        .select(table)
        .live()
        .await
        .map_err(|e| SurrealDbError::Query(format!("Error creando LIVE query: {}", e)))?;

    info!("📡 Stream LIVE activo para tabla: {}", table);

    // Procesar notificaciones del stream
    while let Some(result) = stream.next().await {
        // Verificar si debemos detenernos
        if SHOULD_STOP.load(Ordering::SeqCst) {
            info!("🛑 Deteniendo LIVE query para tabla: {}", table);
            break;
        }

        match result {
            Ok(notification) => {
                if let Err(e) =
                    process_notification(notification, table, event_name, &app_handle).await
                {
                    error!("❌ Error procesando notificación para {}: {}", table, e);
                }
            }
            Err(e) => {
                error!("❌ Error en stream LIVE para {}: {}", table, e);
                // Continuar intentando recibir notificaciones
            }
        }
    }

    warn!(
        "⚠️ Stream LIVE terminado para tabla: {} (esto no debería pasar en operación normal)",
        table
    );
    Ok(())
}

/// Procesa una notificación individual y la emite como evento Tauri
async fn process_notification(
    notification: Notification<serde_json::Value>,
    table: &str,
    event_name: &str,
    app_handle: &AppHandle,
) -> Result<(), SurrealDbError> {
    let action = LiveAction::from(notification.action);

    info!("📨 Notificación LIVE recibida: tabla={}, acción={:?}", table, action);

    let payload =
        LiveNotificationPayload { action, table: table.to_string(), data: notification.data };

    // Emitir evento a todos los listeners del frontend
    app_handle
        .emit(event_name, &payload)
        .map_err(|e| SurrealDbError::Query(format!("Error emitiendo evento Tauri: {}", e)))?;

    info!("✅ Evento '{}' emitido al frontend", event_name);
    Ok(())
}

// ==========================================
// UTILIDADES PARA AGREGAR TABLAS DINÁMICAMENTE
// ==========================================

/// Agrega una nueva suscripción LIVE para una tabla específica.
/// Útil para agregar tablas después de la inicialización.
pub async fn add_table_subscription(
    app_handle: AppHandle,
    table: &str,
    event_name: &str,
) -> Result<(), SurrealDbError> {
    let service = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let db = service.get_client().await?;

    let table_owned = table.to_string();
    let event_owned = event_name.to_string();

    tokio::spawn(async move {
        if let Err(e) = subscribe_to_table(db, &table_owned, &event_owned, app_handle).await {
            error!("❌ Error agregando suscripción LIVE para {}: {}", table_owned, e);
        }
    });

    info!("✅ Nueva suscripción LIVE agregada para tabla: {}", table);
    Ok(())
}
