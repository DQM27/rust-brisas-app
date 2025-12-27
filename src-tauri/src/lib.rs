// src/lib.rs

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
use std::sync::Arc;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_app| {
            // Inicializar SurrealDB embebido
            // Por defecto usa la ruta de datos de la aplicación
            let config = SurrealDbConfig::default();

            // En el futuro, esto podría leerse de una configuración persistente
            // para decidir si usar modo demo o no.

            // Inicializar SearchService (Tantivy)
            let app_data_dir = _app.path().app_data_dir().unwrap_or(std::path::PathBuf::from("."));
            let search_path = app_data_dir.to_string_lossy().to_string();

            match SearchService::new(&search_path) {
                Ok(s) => {
                    _app.manage(Arc::new(s));
                }
                Err(e) => {
                    eprintln!("Error fatal inicializando SearchService: {}", e);
                }
            }

            tauri::async_runtime::block_on(async {
                setup_embedded_surrealdb(config).await.expect("No se pudo inicializar SurrealDB");
            });

            Ok(())
        })
        .invoke_handler(register_handlers!())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
