// src/lib.rs

#[macro_use]
pub mod commands;
pub mod config;
pub mod db;
pub mod domain;
pub mod export;
pub mod models;
pub mod search;
pub mod services;

use crate::config::manager as config_manager;
use crate::config::seed;
use crate::config::settings::{AppConfig, AppConfigState};
use crate::services::search_service::SearchService;
use crate::services::session::SessionState;
use crate::services::surrealdb_service::{setup_embedded_surrealdb, SurrealDbConfig};
use log::{error, info};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, RwLock};
use tauri::Manager;

pub struct AppState {
    pub backend_ready: AtomicBool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    // 1. SETUP LOGGING
    // Inicializar logger una sola vez
    builder = builder.plugin(
        tauri_plugin_log::Builder::new()
            .level(log::LevelFilter::Info)
            .level_for("zbus", log::LevelFilter::Warn)
            .level_for("tantivy", log::LevelFilter::Warn)
            .level_for("tracing", log::LevelFilter::Warn)
            .build(),
    );

    // DEBUG BANNER
    println!("\n\n***************************************************");
    println!("***       DEBUG MODE: SERIALIZATION FIX ACTIVE    ***");
    println!("***************************************************\n\n");

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_dialog::init());
        builder = builder.plugin(tauri_plugin_opener::init());
        builder = builder.plugin(tauri_plugin_store::Builder::new().build());
        builder = builder.plugin(tauri_plugin_updater::Builder::new().build());
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app.get_webview_window("main").expect("no main window").set_focus();
        }));
    }

    builder
        .setup(|app| {
            // 0. CARGAR CONFIGURACIÓN (crea archivos si no existen)
            let app_config: AppConfig = match config_manager::load_config() {
                Ok(config) => {
                    info!("✅ Configuración cargada: terminal_id = {}", config.terminal.id);
                    config
                }
                Err(e) => {
                    error!("⚠️ Error cargando configuración (usando defaults): {}", e);
                    AppConfig::default()
                }
            };

            // Guardar estado de configuración para seed condicional
            let is_configured = app_config.setup.is_configured;

            // Gestionar AppConfigState para comandos de setup
            let config_state: AppConfigState = Arc::new(RwLock::new(app_config));
            app.manage(config_state);

            // 1. CONFIGURACIÓN DE RUTAS
            // Es vital asegurar que el directorio de datos existe
            let app_data_dir = app.path().app_data_dir()?;
            if !app_data_dir.exists() {
                std::fs::create_dir_all(&app_data_dir)?;
            }

            // 2. INICIALIZAR BASE DE DATOS (CRÍTICO: HACER ESTO PRIMERO)
            // Bloqueamos el hilo principal aquí intencionalmente para garantizar
            // que la DB esté lista antes de que cualquier comando o el indexador intenten usarla.
            let db_config = SurrealDbConfig::default();
            tauri::async_runtime::block_on(async {
                setup_embedded_surrealdb(db_config)
                    .await
                    .expect("❌ Error fatal: No se pudo inicializar SurrealDB");
            });
            info!("✅ SurrealDB inicializado correctamente.");

            // 2.5. SEED DATABASE (condicional basado en is_configured)
            // Solo sembrar si ya está configurado para evitar desincronización con Argon2
            tauri::async_runtime::block_on(async {
                if is_configured {
                    info!("🌱 App configurada, ejecutando seed...");
                    if let Err(e) = seed::seed_db().await {
                        error!("❌ Error en seed: {}", e);
                    }
                } else {
                    info!("⚠️ App NO configurada, saltando seed hasta completar Wizard.");
                }
            });

            // 2.6. INICIALIZAR LIVE SUBSCRIPTIONS (Tiempo Real)
            // Solo iniciar si la app está configurada
            if is_configured {
                let app_handle = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                    // Pequeña pausa para asegurar que todo esté inicializado
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    if let Err(e) =
                        services::live_service::start_live_subscriptions(app_handle).await
                    {
                        error!("❌ Error iniciando suscripciones LIVE: {}", e);
                    }
                });
            }

            // 3. INICIALIZAR SEARCH SERVICE (TANTIVY)
            // Usamos un subdirectorio para no ensuciar la raíz de app_data
            let search_path = app_data_dir.join("search_index");
            let search_path_str = search_path.to_string_lossy().to_string();

            // Gestionar Estado Global
            let app_state = AppState { backend_ready: AtomicBool::new(true) };
            app.manage(app_state);

            // Gestionar SessionState para autenticación
            let session_state = SessionState::new();
            app.manage(session_state);

            match SearchService::new(&search_path_str) {
                Ok(s) => {
                    let search_service = Arc::new(s);
                    app.manage(search_service.clone());

                    // 4. LÓGICA DE REINDEXADO (Solo ahora que la DB es segura)
                    if search_service.is_empty() {
                        println!("📇 Índice vacío detectado. Programando reindexado...");

                        let search_service_clone = search_service.clone();

                        // Spawn en background
                        tauri::async_runtime::spawn(async move {
                            // Pequeña pausa de seguridad para dejar que la UI respire al inicio (opcional)
                            tokio::time::sleep(std::time::Duration::from_secs(1)).await;

                            println!("🔄 Iniciando tarea de reindexado...");
                            // Ahora es seguro llamar a reindex_all porque la DB ya pasó el block_on
                            if let Err(e) = search_service_clone.reindex_all().await {
                                eprintln!("❌ Error al reindexar en background: {}", e);
                            } else {
                                println!(
                                    "✅ Reindexado background completado: {} documentos",
                                    search_service_clone.doc_count()
                                );
                            }
                        });
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error fatal inicializando SearchService: {}", e);
                    // Aquí podrías decidir si quieres hacer panic! o dejar que la app corra sin búsqueda
                    return Err(Box::new(e));
                }
            }

            Ok(())
        })
        .invoke_handler(register_handlers!())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
