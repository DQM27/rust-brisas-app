// src-tauri/src/config/manager.rs

use super::settings::AppConfig;
use std::fs;
use std::path::PathBuf;

/// Ubicaciones donde buscar el archivo de configuración (en orden de prioridad)
fn get_config_search_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    // 1. Carpeta actual (para desarrollo)
    paths.push(PathBuf::from("./config/brisas.toml"));

    // 2. Carpeta de datos de la app
    if let Some(data_dir) = dirs::data_local_dir() {
        paths.push(data_dir.join("Brisas").join("brisas.toml"));
    }

    // 3. AppData (Windows) o equivalente
    if let Some(config_dir) = dirs::config_dir() {
        paths.push(config_dir.join("Brisas").join("brisas.toml"));
    }

    // 4. ProgramData (Windows)
    paths.push(PathBuf::from("C:/ProgramData/Brisas/brisas.toml"));

    paths
}

/// Obtiene la ruta donde se guardará la configuración por defecto
fn get_default_config_path() -> PathBuf {
    if let Some(data_dir) = dirs::data_local_dir() {
        data_dir.join("Brisas").join("brisas.toml")
    } else {
        PathBuf::from("./config/brisas.toml")
    }
}

/// Carga la configuración desde el archivo TOML
pub fn load_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    // Buscar archivo existente
    for path in get_config_search_paths() {
        if path.exists() {
            log::info!("📁 Cargando config desde: {}", path.display());
            let content = fs::read_to_string(&path)?;
            let mut config: AppConfig = toml::from_str(&content)?;

            log::info!("🔧 Config cargada: show_demo_mode = {}", config.setup.show_demo_mode);

            // Generar ID si está vacío
            if config.terminal.id.is_empty() {
                config.terminal.id = generate_hardware_id()?;
                save_config(&config, &path)?;
            }

            return Ok(config);
        }
    }

    // No se encontró, crear nueva configuración
    log::info!("📁 No se encontró config, creando nueva...");
    create_default_config()
}

/// Crea y guarda una configuración por defecto
fn create_default_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let mut config = AppConfig::default();

    // Generar ID único
    config.terminal.id = generate_hardware_id()?;

    // Configurar ruta por defecto de la DB
    if let Some(data_dir) = dirs::data_local_dir() {
        config.database.default_path =
            data_dir.join("Brisas").join("brisas.db").to_string_lossy().to_string();
    } else {
        config.database.default_path = "./data/brisas.db".to_string();
    }

    // Guardar configuración
    let config_path = get_default_config_path();
    save_config(&config, &config_path)?;

    Ok(config)
}

/// Guarda la configuración en un archivo TOML
pub fn save_config(config: &AppConfig, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // Crear directorio si no existe
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let toml_string = toml::to_string_pretty(config)?;
    fs::write(path, toml_string)?;

    Ok(())
}

/// Genera un ID único basado en el hardware de la máquina
fn generate_hardware_id() -> Result<String, Box<dyn std::error::Error>> {
    // Intentar obtener MAC address
    if let Ok(mac) = mac_address::get_mac_address() {
        if let Some(mac) = mac {
            let mac_str = mac.to_string().replace(":", "");
            return Ok(format!("HW-{}", mac_str));
        }
    }

    // Fallback: UUID aleatorio
    Ok(format!("HW-{}", uuid::Uuid::new_v4().to_string().replace("-", "")))
}

/// Obtiene la ruta de la base de datos según la configuración
pub fn get_database_path(config: &AppConfig) -> PathBuf {
    // Intentar rutas en orden de prioridad
    for path_str in &config.database.paths {
        let path = expand_path(path_str);
        if path.exists() {
            return path;
        }
    }

    // No existe ninguna, usar la ruta por defecto
    let default_path = expand_path(&config.database.default_path);

    // Crear directorio si no existe
    if let Some(parent) = default_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    default_path
}

/// Obtiene la ruta de la base de datos demo
pub fn get_demo_database_path() -> PathBuf {
    if let Some(data_dir) = dirs::data_local_dir() {
        let dir = data_dir.join("Brisas");
        let _ = fs::create_dir_all(&dir);
        dir.join("brisas_demo.db")
    } else {
        PathBuf::from("./data/brisas_demo.db")
    }
}

/// Obtiene el directorio de búsqueda para el modo demo
pub fn get_demo_search_path() -> PathBuf {
    if let Some(data_dir) = dirs::data_local_dir() {
        let dir = data_dir.join("Brisas").join("search_index_demo");
        let _ = fs::create_dir_all(&dir);
        dir
    } else {
        PathBuf::from("./data/search_index_demo")
    }
}

/// Obtiene el directorio de búsqueda para producción
pub fn get_search_index_path() -> PathBuf {
    if let Some(data_dir) = dirs::data_local_dir() {
        let dir = data_dir.join("Brisas").join("search_index");
        let _ = fs::create_dir_all(&dir);
        dir
    } else {
        PathBuf::from("./data/search_index")
    }
}

/// Expande variables de entorno en rutas
fn expand_path(path: &str) -> PathBuf {
    let expanded = shellexpand::env(path).unwrap_or(std::borrow::Cow::Borrowed(path));
    PathBuf::from(expanded.as_ref())
}
