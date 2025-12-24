use crate::config::manager::save_config;
use crate::config::settings::{AppConfig, AppConfigState, TerminalConfig};
use crate::domain::errors::ConfigError;
use log::info;
use tauri::{command, State};

/// Obtiene la configuración completa actual
#[command]
pub async fn get_app_config(config: State<'_, AppConfigState>) -> Result<AppConfig, ConfigError> {
    let config_guard = config
        .read()
        .map_err(|e| ConfigError::Message(format!("Error al leer configuración: {}", e)))?;

    log::info!("📖 get_app_config llamado: show_demo_mode = {}", config_guard.setup.show_demo_mode);

    Ok(config_guard.clone())
}

/// Actualiza la configuración de la terminal (nombre y ubicación)
#[command]
pub async fn update_terminal_config(
    config: State<'_, AppConfigState>,
    nombre: String,
    ubicacion: String,
) -> Result<TerminalConfig, ConfigError> {
    info!("Actualizando configuración de terminal: {} - {}", nombre, ubicacion);

    // Obtener lock de escritura y modificar
    let mut config_guard = config
        .write()
        .map_err(|e| ConfigError::Message(format!("Error al escribir configuración: {}", e)))?;

    config_guard.terminal.nombre = nombre;
    config_guard.terminal.ubicacion = ubicacion;

    // Guardar en archivo
    let config_path = if let Some(data_dir) = dirs::data_local_dir() {
        data_dir.join("Brisas").join("brisas.toml")
    } else {
        std::path::PathBuf::from("./config/brisas.toml")
    };

    save_config(&config_guard, &config_path)
        .map_err(|e| ConfigError::Message(format!("Error al guardar configuración: {}", e)))?;

    info!("Configuración guardada en: {}", config_path.display());

    Ok(config_guard.terminal.clone())
}

/// Habilita o deshabilita el modo demo en la pantalla de login con intercambio dinámico de DB
#[command]
pub async fn toggle_demo_mode(
    pool_state: State<'_, crate::db::DbPool>,
    search_state: State<'_, crate::services::search_service::SearchState>,
    config: State<'_, AppConfigState>,
    enabled: bool,
) -> Result<bool, ConfigError> {
    info!("🔄 Cambiando modo demo a: {}", enabled);

    // 1. Actualizar configuración
    let config_snapshot = {
        let mut config_guard = config
            .write()
            .map_err(|e| ConfigError::Message(format!("Error al escribir configuración: {}", e)))?;
        config_guard.setup.show_demo_mode = enabled;

        let config_path = if let Some(data_dir) = dirs::data_local_dir() {
            data_dir.join("Brisas").join("brisas.toml")
        } else {
            std::path::PathBuf::from("./config/brisas.toml")
        };
        save_config(&config_guard, &config_path)
            .map_err(|e| ConfigError::Message(format!("Error al guardar configuración: {}", e)))?;
        config_guard.clone()
    };

    // 2. Intercambio atómico (Hot-Swap via RwLock)
    if enabled {
        info!("🧪 Iniciando entorno Demo aislado...");
        let db_path = crate::config::manager::get_demo_database_path();
        let search_path = crate::config::manager::get_demo_search_path();

        // Limpiar previo
        if db_path.exists() {
            let _ = std::fs::remove_file(&db_path);
        }
        if search_path.exists() {
            let _ = std::fs::remove_dir_all(&search_path);
        }

        // Nuevo Pool Demo
        let demo_pool = match crate::db::init_pool_by_path(&db_path).await {
            Ok(p) => p,
            Err(e) => return Err(ConfigError::Message(format!("Error DB demo: {}", e))),
        };

        // Migraciones y Seeds
        if let Err(e) = crate::db::migrate::run_migrations(&demo_pool).await {
            return Err(ConfigError::Message(format!("Error migraciones demo: {}", e)));
        }
        if let Err(e) = crate::config::seed::seed_db(&demo_pool).await {
            return Err(ConfigError::Message(format!("Error seed base demo: {}", e)));
        }
        if let Err(e) = crate::config::seed_demo::run_demo_seed(&demo_pool).await {
            return Err(ConfigError::Message(format!("Error seed demo: {}", e)));
        }

        // Buscar Demo
        let demo_search = match crate::search::init_search_service(&config_snapshot) {
            Ok(s) => s,
            Err(e) => return Err(ConfigError::Message(format!("Error search demo: {}", e))),
        };
        if let Err(e) = demo_search.reindex_all(&demo_pool).await {
            return Err(ConfigError::Message(format!("Error reindex demo: {}", e)));
        }

        // ⚡ ATOMIC SWAP ⚡
        {
            let mut pool_guard = pool_state.0.write().await;
            *pool_guard = demo_pool;
        }
        {
            let mut search_guard = search_state.0.write().await;
            *search_guard = demo_search;
        }

        info!("✅ Entorno Demo SWAPPED y activo.");
    } else {
        info!("🏠 Volviendo a Producción...");

        let prod_pool = match crate::db::init_pool(&config_snapshot).await {
            Ok(p) => p,
            Err(e) => return Err(ConfigError::Message(format!("Error DB prod: {}", e))),
        };

        let prod_search = match crate::search::init_search_service(&config_snapshot) {
            Ok(s) => s,
            Err(e) => return Err(ConfigError::Message(format!("Error search prod: {}", e))),
        };

        // ⚡ ATOMIC SWAP ⚡
        {
            let mut pool_guard = pool_state.0.write().await;
            *pool_guard = prod_pool;
        }
        {
            let mut search_guard = search_state.0.write().await;
            *search_guard = prod_search;
        }

        info!("✅ Entorno Producción SWAPPED y activo.");
    }

    Ok(enabled)
}
