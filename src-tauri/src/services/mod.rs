// Módulos de servicios que contienen la lógica de negocio de la aplicación.
pub mod alerta_service;
pub mod auth;
pub mod avatar_service;
pub mod backup;
pub mod session;

pub mod contratista_service;
pub mod empresa_service;

pub mod export_profile_service;
pub mod export_service;
pub mod gafete_service;
pub mod keyring_service;
pub mod role_service;

#[cfg(target_os = "linux")]
pub mod keyring_linux;

#[cfg(target_os = "windows")]
pub mod keyring_windows;

pub mod cita_service;
pub mod ingreso_contratista_service;
pub mod ingreso_general_service;
pub mod ingreso_proveedor_service;
pub mod ingreso_visita_service;
pub mod lista_negra_service;
pub mod proveedor_service;
pub mod search_service;

pub mod user_service;
pub mod vehiculo_service;
pub mod visitante_service;

// Servicios específicos para SurrealDB
pub mod surrealdb_authorization;
pub mod surrealdb_service;

// Servicios de rendimiento y optimización
pub mod cache_service;
