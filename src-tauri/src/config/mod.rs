// src-tauri/src/config/mod.rs

pub mod settings;
pub mod manager;

pub use settings::AppConfig;
pub use manager::{load_config, save_config, get_database_path};