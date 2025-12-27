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

use std::sync::atomic::AtomicBool;
use tauri::Manager;

pub struct AppState {
    pub backend_ready: AtomicBool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(not(mobile))]
    {
        #[tokio::main]
        async fn main_inner() -> Result<(), Box<dyn std::error::Error>> {
            dotenvy::dotenv().ok();

            let app_config = config::load_config()?;

            // Verificar si hay restauración pendiente ANTES de conectar a la DB
            if let Err(e) = services::backup::check_and_restore_database(&app_config) {
                eprintln!("❌ Error crítico al restaurar base de datos: {}", e);
            }

            // Inicializar pool y servicio de búsqueda en paralelo
            let pool = db::init_pool(&app_config).await?;
            let search_service = search::init_search_service(&app_config)?;

            // Migraciones y seed (secuenciales, dependen del pool)
            db::migrate::run_migrations(&pool).await?;
            db::seed::seed_db(&pool).await?;

            // Solo reindexar si el índice está vacío (primera vez o después de restauración)
            if search_service.is_empty() {
                println!("📇 Índice vacío, detectado. Iniciando reindexado en segundo plano...");
                let pool_clone = pool.clone();
                let search_service_clone = search_service.clone();

                tokio::spawn(async move {
                    println!("🔄 Iniciando reindexado background task...");
                    if let Err(e) = search_service_clone.reindex_all(&pool_clone).await {
                        eprintln!("❌ Error al reindexar en background: {}", e);
                    } else {
                        println!(
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

            tauri::Builder::default()
                .manage(pool)
                .manage(app_config)
                .manage(search_service)
                .manage(app_state)
                .plugin(tauri_plugin_dialog::init())
                .plugin(tauri_plugin_opener::init())
                .plugin(tauri_plugin_updater::Builder::new().build())
                .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
                    let _ = app
                        .get_webview_window("main")
                        .expect("no main window")
                        .set_focus();
                }))
                .invoke_handler(register_handlers!())
                .run(tauri::generate_context!())?;
            Ok(())
        }
        main_inner().expect("error");
    }
}
