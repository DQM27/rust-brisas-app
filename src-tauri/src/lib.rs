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

fn setup_plugins(mut builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
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
        builder = builder.plugin(tauri_plugin_fs::init());
        builder = builder.plugin(tauri_plugin_opener::init());
        builder = builder.plugin(tauri_plugin_store::Builder::new().build());
        builder = builder.plugin(tauri_plugin_updater::Builder::new().build());
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show(); // Restaurar si estaba oculta
                let _ = window.set_focus();
            }
        }));
    }
    builder
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[allow(clippy::large_stack_frames, clippy::too_many_lines)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    builder = setup_plugins(builder);

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

                    // AUDIT: Detect Unexpected Closure
                    let terminal_id = {
                        if let Ok(config) = config_state.read() {
                            config.terminal.id.clone()
                        } else {
                            "UNKNOWN".to_string()
                        }
                    };

                    if let Err(e) = detect_unexpected_closure(&terminal_id).await {
                        error!("⚠️ Error al verificar cierre inesperado: {e}");
                    }

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
                                let session_state = app.state::<SessionState>();
                                log_exit_event(app.app_handle(), &session_state, "Application Exit (Tray)");
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
                 // Log exit event locally before hiding/preventing close (or usually here we just hide)
                 // But user wants to capture "Close from X" as logout.
                 // NOTE: Since we just HIDE the window on close, we should NOT log logout here
                 // unless the user intends to "Quit" via X.
                 // Standard behavior for Tray apps is X -> Hide. 
                 // If user wants X -> Logout & Quit, we change logic.
                 // ASSUMPTION: User wants to track "Session End" or actual App Termination.
                 // If X hides, session persists. So we only log on "Quit" or "Logout".
                  // BUT user said "la app no se cierra desde la x... sino desde el tray pero no capturo nada".
                  // So the issue is specifically TRAY QUIT.

                  // However, "Close Manual" implies X. If X only hides, then it's NOT a logout.
                  // I will ONLY Fix the Tray Quit logging for now as requested.
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .invoke_handler(register_handlers!())
        .run(tauri::generate_context!())
        .expect("Error al ejecutar la aplicación");
}

/// Helper to log exit event synchronously block
fn log_exit_event(app: &tauri::AppHandle, session: &SessionState, reason: &str) {
    if let Some(user) = session.get_user() {
        let duration = session.get_duration_string();
        let ip = session.get_ip();
        let config_state = app.state::<AppConfigState>();

        // We need to run async code in blocking context
        tauri::async_runtime::block_on(async {
            // 1. Get Terminal Info
            let (terminal_id, terminal_name) = {
                if let Ok(config) = config_state.read() {
                    (config.terminal.id.clone(), config.terminal.nombre.clone())
                } else {
                    ("UNKNOWN".to_string(), "UNKNOWN".to_string())
                }
            };

            // 2. Insert Log
            use crate::db::surrealdb_audit_queries::insert_sys_log;
            let _ = insert_sys_log(
                terminal_id,
                terminal_name,
                user.full_name(), // Make sure full_name exists or use proper field
                "LOGOUT".to_string(),
                Some(duration),
                ip,
                Some(reason.to_string()),
            )
            .await;
        });

        // Clear session
        session.clear();
    }
}

/// Detects if the previous session ended unexpectedly (e.g. crash, power loss, Ctrl+C)
async fn detect_unexpected_closure(terminal_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    use crate::db::surrealdb_audit_queries::{get_last_terminal_log, insert_sys_log};

    // 1. Get last log for this terminal
    if let Ok(Some(last_log)) = get_last_terminal_log(terminal_id).await {
        // 2. If it was a LOGIN, and it happened more than 10 seconds ago (to avoid race during fast restart)
        // or just if it was a LOGIN, it means no LOGOUT followed.
        if last_log.event_type == "LOGIN" {
            info!(
                "🚨 Cierre inesperado detectado para la terminal: {terminal_id}. Registrando en auditoría..."
            );

            // 3. Log the anomaly
            insert_sys_log(
                last_log.terminal_id,
                last_log.terminal_name,
                last_log.user_name,
                "LOGOUT".to_string(), // We categorize it as a LOGOUT event (end of session)
                None, // We can't easily calculate duration without knowing exact crash time
                last_log.ip_address,
                Some("Unexpected Closure (Crash/Force Quit detected)".to_string()),
            )
            .await?;
        }
    }

    Ok(())
}
