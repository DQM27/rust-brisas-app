pub mod app_commands;
pub mod backup;

pub mod cita_commands;
pub mod config_commands;
pub mod contratista_commands;
pub mod debug_commands;
pub mod system_commands;

pub mod empresa_commands;
// REEMPLAZADO: pub mod entrada_commands;
pub mod export_commands;
pub mod export_profiles;
pub mod gafete_commands;

// NUEVO UNIFICADO:
pub mod ingreso_contratista_commands;

pub mod ingreso_commands;
pub mod ingreso_proveedor_commands;
pub mod ingreso_visita_commands;
pub mod keyring_commands;
pub mod lista_negra_commands;
// REEMPLAZADO: pub mod permanencia_commands;
pub mod proveedor_commands;
pub mod role_commands;

// REEMPLAZADO: pub mod salida_commands;
pub mod search_commands;

pub mod user_commands;
pub mod vehiculo_commands;
pub mod window_commands;
#[macro_use]
pub mod handlers;
