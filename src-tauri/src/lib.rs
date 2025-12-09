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
pub mod services; // ✅ AGREGAR ESTO

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(not(mobile))]
    {
        #[tokio::main]
        async fn main_inner() -> Result<(), Box<dyn std::error::Error>> {
            dotenvy::dotenv().ok();

            let app_config = config::load_config()?;
            println!(
                "🏢 Terminal: {} (ID: {})",
                app_config.terminal.nombre, app_config.terminal.id
            );

            // Verificar si hay restauración pendiente ANTES de conectar a la DB
            if let Err(e) = services::backup::check_and_restore_database(&app_config) {
                eprintln!("❌ Error crítico al restaurar base de datos: {}", e);
            }

            let pool = db::init_pool(&app_config).await?;
            db::migrate::run_migrations(&pool).await?;
            db::seed::seed_db(&pool).await?;
            let search_service = search::init_search_service(&app_config)?;

            // Reindexar todo al inicio para asegurar consistencia con la DB
            println!("🔄 Reindexando base de datos completa...");
            if let Err(e) = search_service.reindex_all(&pool).await {
                eprintln!("❌ Error al reindexar al inicio: {}", e);
            } else {
                println!("✅ Reindexado completado con éxito");
            }

            tauri::Builder::default()
                .manage(pool)
                .manage(app_config)
                .manage(search_service)
                .plugin(tauri_plugin_dialog::init())
                .plugin(tauri_plugin_opener::init())
                .plugin(tauri_plugin_updater::Builder::new().build())
                .invoke_handler(register_handlers!())
                .run(tauri::generate_context!())?;
            Ok(())
        }
        main_inner().expect("error");
    }
}
