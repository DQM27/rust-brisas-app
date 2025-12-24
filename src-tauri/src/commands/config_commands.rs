use crate::config::manager::save_config;
use crate::config::settings::{AppConfig, AppConfigState, TerminalConfig};
use crate::domain::errors::ConfigError;
use log::info;
use tauri::{command, Manager, State};

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
    app: tauri::AppHandle,
    config: State<'_, AppConfigState>,
    enabled: bool,
) -> Result<bool, ConfigError> {
    info!("🔄 Cambiando modo demo a: {}", enabled);

    // 1. Actualizar configuración en memoria y archivo
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
    }; // El lock se libera aquí automáticamente al salir del scope

    // 2. Intercambio dinámico de conexiones (Hot-Swap)
    if enabled {
        info!("🧪 Iniciando entorno Demo aislado...");
        let db_path = crate::config::manager::get_demo_database_path();
        let search_path = crate::config::manager::get_demo_search_path();

        // Limpiar entorno demo previo para asegurar reseteo (según solicitud usuario)
        if db_path.exists() {
            if let Err(e) = std::fs::remove_file(&db_path) {
                log::error!("❌ Error eliminando DB demo antigua: {}", e);
                // No retornamos error fatal aquí, intentamos continuar o tal vez sea crítico
            }
        }
        if search_path.exists() {
            if let Err(e) = std::fs::remove_dir_all(&search_path) {
                log::error!("❌ Error eliminando índice búsqueda demo antiguo: {}", e);
            }
        }

        // Inicializar nuevo pool demo
        let demo_pool = match crate::db::init_pool_by_path(&db_path).await {
            Ok(p) => p,
            Err(e) => {
                log::error!("❌ Error fatal iniciando pool demo en {:?}: {}", db_path, e);
                return Err(ConfigError::Message(format!("Error iniciando DB demo: {}", e)));
            }
        };

        // Aplicar migraciones y seed
        if let Err(e) = crate::db::migrate::run_migrations(&demo_pool).await {
            log::error!("❌ Error ejecutando migraciones demo: {}", e);
            return Err(ConfigError::Message(format!("Error migraciones demo: {}", e)));
        }

        if let Err(e) = crate::config::seed::seed_db(&demo_pool).await {
            log::error!("❌ Error ejecutando seed base demo: {}", e);
            return Err(ConfigError::Message(format!("Error seed base demo: {}", e)));
        }

        if let Err(e) = crate::config::seed_demo::run_demo_seed(&demo_pool).await {
            log::error!("❌ Error ejecutando seed demo extendido: {}", e);
            return Err(ConfigError::Message(format!("Error seed demo extendido: {}", e)));
        }

        // Inicializar nuevo servicio de búsqueda demo
        let demo_search_service = match crate::search::init_search_service(&config_snapshot) {
            Ok(s) => s,
            Err(e) => {
                log::error!("❌ Error inicializando servicio búsqueda demo: {}", e);
                return Err(ConfigError::Message(format!("Error servicio búsqueda: {}", e)));
            }
        };

        // Reindexar demo
        if let Err(e) = demo_search_service.reindex_all(&demo_pool).await {
            log::error!("❌ Error reindexando demo: {}", e);
            return Err(ConfigError::Message(format!("Error reindexando demo: {}", e)));
        }

        // Sobrescribir estados globales de Tauri (Hot-Swap)
        app.manage(demo_pool);
        app.manage(demo_search_service);

        info!("✅ Entorno Demo listo y activo.");
    } else {
        info!("🏠 Volviendo a entorno de Producción...");

        // Re-inicializar pool de producción
        let prod_pool = match crate::db::init_pool(&config_snapshot).await {
            Ok(p) => p,
            Err(e) => {
                log::error!("❌ Error fatal reconectando a producción: {}", e);
                return Err(ConfigError::Message(format!("Error reconexión prod: {}", e)));
            }
        };

        // Re-inicializar servicio de búsqueda de producción
        let prod_search_service = match crate::search::init_search_service(&config_snapshot) {
            Ok(s) => s,
            Err(e) => {
                log::error!("❌ Error restaurando búsqueda producción: {}", e);
                return Err(ConfigError::Message(format!("Error búsqueda prod: {}", e)));
            }
        };

        // Sobrescribir estados globales (Hot-Swap back)
        app.manage(prod_pool);
        app.manage(prod_search_service);

        info!("✅ Entorno de Producción restaurado.");
    }

    Ok(enabled)
}
