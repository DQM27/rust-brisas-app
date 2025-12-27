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

use crate::services::search_service::SearchService;
use crate::services::surrealdb_service::{setup_embedded_surrealdb, SurrealDbConfig};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tauri::Manager;

pub struct AppState {
    pub backend_ready: AtomicBool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_dialog::init());
        builder = builder.plugin(tauri_plugin_opener::init());
        builder = builder.plugin(tauri_plugin_updater::Builder::new().build());
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app.get_webview_window("main").expect("no main window").set_focus();
        }));
    }

    builder
        .setup(|app| {
            // Inicializar SurrealDB embebido
            let config = SurrealDbConfig::default();

            // Inicializar SearchService (Tantivy)
            let app_data_dir = app.path().app_data_dir().unwrap_or(std::path::PathBuf::from("."));
            let search_path = app_data_dir.to_string_lossy().to_string();

            // Estado de la aplicación
            let app_state = AppState { backend_ready: AtomicBool::new(true) };
            app.manage(app_state);

            match SearchService::new(&search_path) {
                Ok(s) => {
                    let search_service = Arc::new(s);
                    app.manage(search_service.clone());

                    // Reindexado en segundo plano si está vacío
                    if search_service.is_empty() {
                        println!(
                            "📇 Índice vacío, detectado. Iniciando reindexado en segundo plano..."
                        );
                        let search_service_clone = search_service.clone();
                        tauri::async_runtime::spawn(async move {
                            println!("🔄 Iniciando reindexado background task...");
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
                    eprintln!("Error fatal inicializando SearchService: {}", e);
                }
            }

            // Inicializar DB (esto bloquea el inicio hasta que DB esté lista,
            // lo cual es correcto para garantizar que los comandos funcionen)
            tauri::async_runtime::block_on(async {
                setup_embedded_surrealdb(config).await.expect("No se pudo inicializar SurrealDB");
            });

            Ok(())
        })
        .invoke_handler(register_handlers!())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
