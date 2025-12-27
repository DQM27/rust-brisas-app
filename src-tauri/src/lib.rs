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

use log::{error, info};
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

            // Inicializar pool y servicio de búsqueda en paralelo
            let pool = db::init_pool(&app_config).await?;
            let search_service = search::init_search_service(&app_config)?;

            // Migraciones (siempre necesarias)
            db::migrate::run_migrations(&pool).await?;

            // Solo sembrar DB si ya está configurado (para evitar desincronización de Argon2)
            if app_config.setup.is_configured {
                info!("🌱 App configurada, verificando integridad de datos...");
                config::seed::seed_db(&pool).await?;
            } else {
                info!("⚠️ App NO configurada, saltando seed hasta que se complete el Wizard.");
            }

            // ==========================================
            // SURREALDB (EXPERIMENTAL)
            // ==========================================
            #[cfg(feature = "surrealdb-backend")]
            {
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
                    Err(e) => println!("❌ [SURREALDB] Error inicializando: {}", e),
                }
            }

            // Solo reindexar si el índice está vacío (primera vez o después de restauración)
            if search_service.is_empty() {
                info!("📇 Índice vacío, detectado. Iniciando reindexado en segundo plano...");
                let pool_clone = pool.clone();
                let search_service_clone = search_service.clone();

                tokio::spawn(async move {
                    info!("🔄 Iniciando reindexado background task...");
                    if let Err(e) = search_service_clone.reindex_all(&pool_clone).await {
                        error!("❌ Error al reindexar en background: {}", e);
                    } else {
                        info!(
                            "✅ Reindexado background completado: {} documentos",
                            search_service_clone.doc_count()
                        );
                    }
                });
            }

            // Estado de la aplicación
            let app_state = AppState {
                backend_ready: AtomicBool::new(true), // Backend listo tras inicialización
            };

            // Estado de sesión del usuario
            let session_state = services::session::SessionState::new();

            tauri::Builder::default()
                .plugin(
                    tauri_plugin_log::Builder::new()
                        .level(log::LevelFilter::Info) // Solo INFO, WARN, ERROR
                        .level_for("zbus", log::LevelFilter::Warn) // Silenciar D-Bus spam
                        .level_for("tantivy", log::LevelFilter::Warn) // Silenciar Tantivy spam
                        .level_for("tracing", log::LevelFilter::Warn) // Silenciar tracing spans
                        .build(),
                ) // Logging Plugin
                .manage(db::DbPool(tokio::sync::RwLock::new(pool)))
                .manage(Arc::new(RwLock::new(app_config)))
                .manage(services::search_service::SearchState(tokio::sync::RwLock::new(
                    search_service,
                )))
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
