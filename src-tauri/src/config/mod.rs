// src-tauri/src/config/mod.rs

pub mod manager;
pub mod seed;
pub mod seed_demo;
pub mod settings;

#[cfg(feature = "surrealdb-backend")]
pub mod surrealdb_seed;

pub use manager::{get_database_path, load_config, save_config};
pub use settings::{AppConfig, SetupState};
