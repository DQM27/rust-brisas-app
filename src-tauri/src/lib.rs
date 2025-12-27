// src/lib.rs

pub mod commands;
pub mod config;
pub mod db;
pub mod domain;
pub mod export;
pub mod models;
pub mod search;
pub mod services;

use crate::services::surrealdb_service::{setup_embedded_surrealdb, SurrealDbConfig};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_app| {
            // Inicializar SurrealDB embebido
            // Por defecto usa la ruta de datos de la aplicación
            let config = SurrealDbConfig::default();

            // En el futuro, esto podría leerse de una configuración persistente
            // para decidir si usar modo demo o no.

            tauri::async_runtime::block_on(async {
                setup_embedded_surrealdb(config).await.expect("No se pudo inicializar SurrealDB");
            });

            Ok(())
        })
        .invoke_handler(register_handlers!())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
