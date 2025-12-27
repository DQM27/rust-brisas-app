// src/db/mod.rs

pub mod migrate;

pub mod alerta_gafete_queries;
pub mod cita_queries;
pub mod contratista_queries;
pub mod empresa_queries;
pub mod gafete_queries;

pub mod ingreso_contratista_queries; // Contratistas
pub mod ingreso_general_queries; // General (Logs, Historial completo)
pub mod ingreso_proveedor_queries; // Proveedores
pub mod ingreso_visita_queries; // Visitas

pub mod lista_negra_queries;
pub mod proveedor_queries;
pub mod user_queries;
pub mod vehiculo_queries;
pub mod visitante_queries;

pub mod audit_queries; // Tablas de auditoría

// SurrealDB
pub mod surrealdb_audit_queries;
pub mod surrealdb_cita_queries;
pub mod surrealdb_contratista_queries;
pub mod surrealdb_empresa_queries;
pub mod surrealdb_gafete_queries;
pub mod surrealdb_ingreso_queries;
pub mod surrealdb_lista_negra_queries;
pub mod surrealdb_proveedor_queries;
pub mod surrealdb_user_queries;
pub mod surrealdb_vehiculo_queries;
pub mod surrealdb_visitante_queries;
