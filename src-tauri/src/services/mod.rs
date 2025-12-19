pub mod auth;
pub mod backup;

pub mod contratista_service;
pub mod empresa_service;
// pub mod entrada_service; // ELIMINADO

pub mod export_profile_service;
pub mod export_service;
pub mod gafete_service;
pub mod keyring_service;

#[cfg(target_os = "linux")]
pub mod keyring_linux;

#[cfg(target_os = "windows")]
pub mod keyring_windows;

pub mod cita_service;
pub mod ingreso_contratista_service; // NUEVO UNIFICADO
pub mod ingreso_proveedor_service;
pub mod ingreso_visita_service;
pub mod lista_negra_service;
// pub mod permanencia_service; // ELIMINADO
pub mod proveedor_service;
// pub mod salida_service; // ELIMINADO
pub mod search_service;

pub mod user_service;
pub mod vehiculo_service;
pub mod visitante_service;
