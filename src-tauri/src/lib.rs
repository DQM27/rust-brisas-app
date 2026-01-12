#[macro_use]
mod macros;
pub mod commands;
pub mod config;
pub mod db;
pub mod domain;
pub mod export;
pub mod models;
pub mod repositories;
pub mod search;
pub mod services;
#[cfg(test)]
pub mod tests;

use crate::config::manager as config_manager;
use crate::config::seed;
use crate::config::settings::{AppConfig, AppConfigState};
use crate::services::search_service::SearchService;
use crate::services::session::SessionState;
use crate::services::surrealdb_service::{setup_embedded_surrealdb, SurrealDbConfig};
use log::{error, info};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, RwLock};
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, TrayIconBuilder, TrayIconEvent};
use tauri::{Manager, WindowEvent};

/// Estado global de la aplicación.
pub struct AppState {
    pub backend_ready: AtomicBool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    // Configuración del sistema de logs.
    builder = builder.plugin(
        tauri_plugin_log::Builder::new()
            .level(log::LevelFilter::Info)
            .level_for("zbus", log::LevelFilter::Warn)
            .level_for("tantivy", log::LevelFilter::Warn)
            .level_for("tracing", log::LevelFilter::Warn)
            .build(),
    );

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_dialog::init());
        builder = builder.plugin(tauri_plugin_opener::init());
        builder = builder.plugin(tauri_plugin_store::Builder::new().build());
        builder = builder.plugin(tauri_plugin_updater::Builder::new().build());
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("Ventana principal no encontrada")
                .set_focus();
        }));
    }

    builder
        .setup(|app| {
            // Se carga la configuración global de la aplicación. Este paso es fundamental 
            // porque define identificadores de terminal y preferencias que afectan a todo el sistema.
            let app_config: AppConfig = match config_manager::load_config() {
                Ok(config) => {
                    info!("✅ Configuración cargada: terminal_id = {}", config.terminal.id);
                    config
                }
                Err(e) => {
                    // Si falla la carga (ej. archivo corrupto), usamos defaults para no bloquear
                    // el arranque, pero notificamos el error para su revisión.
                    error!("⚠️ Error al cargar configuración (usando valores por defecto): {e}");
                    AppConfig::default()
                }
            };

            // Protocolo de Resiliencia: Verificamos restauración pendiente ANTES de abrir la DB.
            // Se hace aquí para asegurar que app_config no haya sido movido aún.
            let mut db_config = SurrealDbConfig::default();
            db_config.data_path = config_manager::get_database_path(&app_config);

            if let Err(e) = crate::services::backup::check_and_restore_database(&app_config) {
                error!("❌ Error en el protocolo de resiliencia: {e}");
            }

            let is_configured = app_config.setup.is_configured;
            let config_state: AppConfigState = Arc::new(RwLock::new(app_config));
            app.manage(config_state.clone());

            // Verificamos y creamos el directorio de datos de la aplicación si no existe.
            let app_data_dir = app.path().app_data_dir()?;
            if !app_data_dir.exists() {
                std::fs::create_dir_all(&app_data_dir)?;
            }

            tauri::async_runtime::block_on(async {
                setup_embedded_surrealdb(db_config)
                    .await
                    .expect("❌ Error fatal: La aplicación no puede iniciar sin la base de datos");
            });
            info!("✅ Motor de base de datos listo.");

            // Si el asistente de configuración ya terminó, ejecutamos el proceso de "seeding".
            // Esto asegura que tengamos los roles y usuarios administrativos básicos necesarios para operar.
            tauri::async_runtime::block_on(async {
                if is_configured {
                    info!("🌱 Sistema configurado previamente. Verificando integridad de datos...");
                    // Pasamos config_state para controlar si ya se hizo el seed
                    if let Err(e) = seed::seed_db(config_state.clone()).await {
                        error!("❌ Error durante la verificación de datos iniciales: {e}");
                    }
                } else {
                    info!("⚠️ El sistema está en modo de espera hasta que el asistente de configuración se complete.");
                }
            });

            let search_path = app_data_dir.join("search_index");
            let search_path_str = search_path.to_string_lossy().to_string();

            // Registramos los estados globales para que los comandos de Tauri puedan acceder a ellos mediante inyección de dependencias.
            app.manage(AppState { backend_ready: AtomicBool::new(true) });
            app.manage(SessionState::new());

            // Inicialización del servicio de búsqueda basado en Tantivy.
            // Se usa el mismo directorio de datos para facilitar respaldos unificados.
            match SearchService::new(&search_path_str) {
                Ok(s) => {
                    let search_service = Arc::new(s);
                    app.manage(search_service.clone());

                    // Si el índice de búsqueda está vacío, programamos una reconstrucción en segundo plano.
                    // Se hace en un hilo separado (spawn) para no retrasar la carga de la interfaz de usuario.
                    if search_service.is_empty() {
                        let search_service_clone = search_service;

                        tauri::async_runtime::spawn(async move {
                            // Breve pausa para dar prioridad a la carga de la ventana principal.
                            tokio::time::sleep(std::time::Duration::from_secs(1)).await;

                            info!("🔄 Iniciando reconstrucción del índice de búsqueda en segundo plano...");
                            if let Err(e) = search_service_clone.reindex_all().await {
                                eprintln!("❌ Fallo en la reindexación asíncrona: {e}");
                            } else {
                                info!(
                                    "✅ Índice reconstruido exitosamente con {} registros.",
                                    search_service_clone.doc_count()
                                );
                            }
                        });
                    }
                }
                Err(e) => {
                    error!("❌ Fallo crítico al inicializar el índice de búsqueda: {e}");
                    return Err(Box::new(e));
                }
            }

            // =========================================================================
            // SYSTEM TRAY SETUP
            // =========================================================================
            #[cfg(desktop)]
            {
                let show_i = MenuItem::with_id(app, "show", "Abrir Mega Brisas", true, None::<&str>)?;
                let quit_i = MenuItem::with_id(app, "quit", "Salir Totalmente", true, None::<&str>)?;
                let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

                let _tray = TrayIconBuilder::with_id("tray")
                    .tooltip("Mega Brisas ERP")
                    .icon(app.default_window_icon().unwrap().clone())
                    .menu(&menu)
                    .on_menu_event(|app, event| {
                        match event.id.as_ref() {
                            "show" => {
                                if let Some(window) = app.get_webview_window("main") {
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                }
                            }
                            "quit" => {
                                app.exit(0);
                            }
                            _ => {}
                        }
                    })
                    .on_tray_icon_event(|tray, event| {
                        if let TrayIconEvent::Click {
                            button: MouseButton::Left,
                            ..
                        } = event
                        {
                            let app = tray.app_handle();
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    })
                    .build(app)?;
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                // En vez de cerrar, ocultamos la ventana
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .invoke_handler(register_handlers!())
        .run(tauri::generate_context!())
        .expect("Error al ejecutar la aplicación");
}
