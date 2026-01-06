/// Capa de Comandos (Tauri Bridge): Punto de Entrada de la Interfaz de Usuario.
///
/// Este módulo centraliza todos los puentes asíncronos que conectan el frontend
/// (Svelte/TypeScript) con la lógica de negocio del backend (Rust).
/// Cada sub-módulo expone funciones decoradas con `#[tauri::command]` para su
/// invocación remota, manejando la serialización JSON y el mapeo de errores.
pub mod app_commands;
pub mod backup;

pub mod audio_commands;
pub mod cita_commands;
pub mod config_commands;
pub mod contratista_commands;
pub mod module_commands;
pub mod system_commands;

pub mod empresa_commands;
pub mod export_commands;
pub mod export_profiles;
pub mod gafete_commands;

// Servicios de Ingreso Unificados:
pub mod ingreso_commands;
pub mod ingreso_contratista_commands;
pub mod ingreso_proveedor_commands;
pub mod ingreso_visita_commands;

pub mod keyring_commands;
pub mod lista_negra_commands;
pub mod proveedor_commands;
pub mod role_commands;

pub mod search_commands;
pub mod security_commands;

pub mod user_commands;
pub mod validation_commands;
pub mod vehiculo_commands;
pub mod visitante_commands;
pub mod window_commands;

#[macro_use]
pub mod handlers;
