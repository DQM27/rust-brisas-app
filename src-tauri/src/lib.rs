// ==========================================
// src-tauri/src/lib.rs
// ==========================================

#[macro_use]
pub mod commands;
pub mod config;
pub mod db;
pub mod domain;
pub mod export;
pub mod models;
pub mod search;
pub mod services;

use log::error;
use std::sync::{atomic::AtomicBool, Arc, RwLock};
use tauri::Manager; // Import log macros

pub struct AppState {
    pub backend_ready: AtomicBool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(not(mobile))]
    {
        #[tokio::main]
        async fn main_inner() -> Result<(), Box<dyn std::error::Error>> {
            // Environment logger is initialized by tauri-plugin-log if attached,
            // but dotenv needs to be loaded first for config
            dotenvy::dotenv().ok();

            let app_config = config::load_config()?;

            // Verificar si hay restauración pendiente ANTES de conectar a la DB
            if let Err(e) = services::backup::check_and_restore_database(&app_config) {
                // eprintln -> log::error
                error!("❌ Error crítico al restaurar base de datos: {}", e);
            }

            // Inicializar servicio de búsqueda (placeholder por ahora)
            // let search_service = search::init_search_service(&app_config)?;

            // ==========================================
            // SURREALDB CORE
            // ==========================================
            println!("🚀 [SURREALDB] Inicializando SurrealDB embebido...");
            let surreal_config = if app_config.setup.show_demo_mode {
                services::surrealdb_service::SurrealDbConfig::demo()
            } else {
                services::surrealdb_service::SurrealDbConfig::default()
            };

            match services::surrealdb_service::setup_embedded_surrealdb(surreal_config).await {
                Ok(_) => {
                    println!("✅ [SURREALDB] SurrealDB embebido inicializado correctamente");
                    // Ejecutar seeds de SurrealDB
                    if let Err(e) = config::surrealdb_seed::seed_surrealdb().await {
                        println!("❌ [SURREALDB] Error en seeds: {}", e);
                    }
                }
                Err(e) => {
                    error!("❌ [SURREALDB] Error inicializando: {}", e);
                    // Convertir SurrealDbError a un error genérico para tauri
                    return Err(format!("SurrealDB Init Error: {}", e).into());
                }
            }

            // Estado de la aplicación
            let app_state = AppState { backend_ready: AtomicBool::new(true) };

            // Estado de sesión del usuario
            let session_state = services::session::SessionState::new();

            tauri::Builder::default()
                .plugin(
                    tauri_plugin_log::Builder::new()
                        .level(log::LevelFilter::Info)
                        .level_for("zbus", log::LevelFilter::Warn)
                        .level_for("tantivy", log::LevelFilter::Warn)
                        .level_for("tracing", log::LevelFilter::Warn)
                        .build(),
                )
                .manage(Arc::new(RwLock::new(app_config)))
                // .manage(services::search_service::SearchState(tokio::sync::RwLock::new(
                //     search_service,
                // )))
                .manage(app_state)
                .manage(session_state)
                .plugin(tauri_plugin_dialog::init())
                .plugin(tauri_plugin_opener::init())
                .plugin(tauri_plugin_updater::Builder::new().build())
                .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
                    let _ = app.get_webview_window("main").expect("no main window").set_focus();
                }))
                .invoke_handler(register_handlers!())
                .run(tauri::generate_context!())?;
            Ok(())
        }
        main_inner().expect("error");
    }
}
