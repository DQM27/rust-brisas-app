// ==========================================
// src/domain/errors.rs
// ==========================================
// Errores tipados para cada módulo del dominio
// Uso de thiserror para derivación automática

use serde::{Serialize, Serializer};
use thiserror::Error;

/// Helper function to serialize errors as strings
fn serialize_error_as_string<S>(e: &sqlx::Error, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&e.to_string())
}

// ==========================================
// USER ERRORS
// ==========================================

#[derive(Error, Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum UserError {
    #[error("Usuario no encontrado")]
    NotFound,

    #[error("Ya existe un usuario con este email")]
    EmailExists,

    #[error("Ya existe un usuario con esta cédula")]
    CedulaExists,

    #[error("Credenciales inválidas")]
    InvalidCredentials,

    #[error("La contraseña actual es incorrecta")]
    InvalidCurrentPassword,

    #[error("Usuario inactivo")]
    InactiveUser,

    #[error("Rol desconocido: {0}")]
    InvalidRole(String),

    #[error("No se puede eliminar la empresa porque tiene {0} contratista(s) asociado(s)")]
    EmpresaHasContratistas(i64),

    #[error("Error de base de datos: {0}")]
    Database(
        #[from]
        #[serde(serialize_with = "serialize_error_as_string")]
        sqlx::Error,
    ),

    #[error("Error de búsqueda: {0}")]
    Search(String),

    #[error("Error de autenticación: {0}")]
    Auth(String),

    #[error("Error de validación: {0}")]
    Validation(String),

    #[error("Error inesperado: {0}")]
    Internal(String),

    #[error("Error de I/O: {0}")]
    IO(String),
}

// ==========================================
// CONTRATISTA ERRORS
// ==========================================

#[derive(Error, Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum ContratistaError {
    #[error("Contratista no encontrado")]
    NotFound,

    #[error("Ya existe un contratista con esta cédula")]
    CedulaExists,

    #[error("Empresa no encontrada")]
    EmpresaNotFound,

    #[error("Estado inválido: {0}")]
    InvalidStatus(String),

    #[error("El contratista está en lista negra: {0}")]
    Blacklisted(String),

    #[error("PRAIND vencido desde {0}")]
    PraindExpired(String),

    #[error("El contratista ya tiene un ingreso activo")]
    AlreadyInside,

    #[error("Error de base de datos: {0}")]
    Database(
        #[from]
        #[serde(serialize_with = "serialize_error_as_string")]
        sqlx::Error,
    ),

    #[error("Error de búsqueda: {0}")]
    Search(String),

    #[error("Error de validación: {0}")]
    Validation(String),
}

// ==========================================
// EMPRESA ERRORS
// ==========================================

#[derive(Error, Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum EmpresaError {
    #[error("Empresa no encontrada")]
    NotFound,

    #[error("Ya existe una empresa con este nombre")]
    NameExists,

    #[error("Ya existe una empresa con este RIF/NIT")]
    RifExists,

    #[error("No se puede eliminar: tiene {0} contratista(s) asociado(s)")]
    HasContratistas(i64),

    #[error("No se puede eliminar: tiene {0} proveedor(es) asociado(s)")]
    HasProveedores(i64),

    #[error("Error de base de datos: {0}")]
    Database(
        #[from]
        #[serde(serialize_with = "serialize_error_as_string")]
        sqlx::Error,
    ),

    #[error("Error de validación: {0}")]
    Validation(String),
}

// ==========================================
// VEHICULO ERRORS
// ==========================================

#[derive(Error, Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum VehiculoError {
    #[error("Vehículo no encontrado")]
    NotFound,

    #[error("Ya existe un vehículo con esta placa")]
    PlacaExists,

    #[error("Tipo de vehículo inválido: {0}")]
    InvalidType(String),

    #[error("Error de base de datos: {0}")]
    Database(
        #[from]
        #[serde(serialize_with = "serialize_error_as_string")]
        sqlx::Error,
    ),

    #[error("Error de validación: {0}")]
    Validation(String),
}

// ==========================================
// PROVEEDOR ERRORS
// ==========================================

#[derive(Error, Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum ProveedorError {
    #[error("Proveedor no encontrado")]
    NotFound,

    #[error("Ya existe un proveedor con esta cédula")]
    CedulaExists,

    #[error("Empresa no encontrada")]
    EmpresaNotFound,

    #[error("Estado inválido: {0}")]
    InvalidStatus(String),

    #[error("El proveedor está en lista negra: {0}")]
    Blacklisted(String),

    #[error("El proveedor ya tiene un ingreso activo")]
    AlreadyInside,

    #[error("Error de base de datos: {0}")]
    Database(
        #[from]
        #[serde(serialize_with = "serialize_error_as_string")]
        sqlx::Error,
    ),

    #[error("Error de búsqueda: {0}")]
    Search(String),

    #[error("Error de validación: {0}")]
    Validation(String),
}

// ==========================================
// GAFETE ERRORS
// ==========================================

#[derive(Error, Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum GafeteError {
    #[error("Gafete no encontrado")]
    NotFound,

    #[error("Gafete ya existe con ese número")]
    AlreadyExists,

    #[error("El gafete está actualmente en uso")]
    InUse,

    #[error("El gafete no está disponible")]
    NotAvailable,

    #[error("Tipo de gafete inválido: {0}")]
    InvalidType(String),

    #[error("Estado de gafete inválido: {0}")]
    InvalidState(String),

    #[error("El gafete está deshabilitado")]
    Disabled,

    #[error("Error de base de datos: {0}")]
    Database(
        #[from]
        #[serde(serialize_with = "serialize_error_as_string")]
        sqlx::Error,
    ),

    #[error("Error de validación: {0}")]
    Validation(String),
}

// ==========================================
// ALERTA ERRORS
// ==========================================

#[derive(Error, Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum AlertaError {
    #[error("Alerta no encontrada")]
    NotFound,

    #[error("Tipo de alerta inválido: {0}")]
    InvalidType(String),

    #[error("La alerta ya fue atendida")]
    AlreadyResolved,

    #[error("Error de base de datos: {0}")]
    Database(
        #[from]
        #[serde(serialize_with = "serialize_error_as_string")]
        sqlx::Error,
    ),

    #[error("Error de validación: {0}")]
    Validation(String),
}

// ==========================================
// LISTA NEGRA ERRORS
// ==========================================

#[derive(Error, Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum ListaNegraError {
    #[error("Registro no encontrado")]
    NotFound,

    #[error("La persona ya está en la lista negra")]
    AlreadyExists,

    #[error("La cédula no puede estar vacía")]
    EmptyCedula,

    #[error("Motivo de registro requerido")]
    MotivoRequired,

    #[error("Error de base de datos: {0}")]
    Database(
        #[from]
        #[serde(serialize_with = "serialize_error_as_string")]
        sqlx::Error,
    ),

    #[error("Error de validación: {0}")]
    Validation(String),

    #[error("Error al parsear fecha: {0}")]
    DateParse(String),
}

// ==========================================
// VISITANTE ERRORS
// ==========================================

#[derive(Error, Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum VisitanteError {
    #[error("Visitante no encontrado")]
    NotFound,

    #[error("Ya existe un visitante con esta cédula")]
    CedulaExists,

    #[error("El visitante está en lista negra: {0}")]
    Blacklisted(String),

    #[error("Error de base de datos: {0}")]
    Database(
        #[from]
        #[serde(serialize_with = "serialize_error_as_string")]
        sqlx::Error,
    ),

    #[error("Error de validación: {0}")]
    Validation(String),
}

// ==========================================
// INGRESO VISITA ERRORS
// ==========================================

#[derive(Error, Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum IngresoVisitaError {
    #[error("Ingreso de visita no encontrado")]
    NotFound,

    #[error("El visitante ya tiene un ingreso activo")]
    AlreadyInside,

    #[error("No hay ingreso activo para registrar salida")]
    NoActiveIngreso,

    #[error("Gafete no disponible")]
    GafeteNotAvailable,

    #[error("Error de gafete: {0}")]
    Gafete(String),

    #[error("Error de base de datos: {0}")]
    Database(
        #[from]
        #[serde(serialize_with = "serialize_error_as_string")]
        sqlx::Error,
    ),

    #[error("Error de validación: {0}")]
    Validation(String),
}

impl From<crate::domain::common::CommonError> for IngresoVisitaError {
    fn from(err: crate::domain::common::CommonError) -> Self {
        IngresoVisitaError::Validation(err.to_string())
    }
}

// ==========================================
// INGRESO PROVEEDOR ERRORS
// ==========================================

#[derive(Error, Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum IngresoProveedorError {
    #[error("Ingreso de proveedor no encontrado")]
    NotFound,

    #[error("El proveedor ya tiene un ingreso activo")]
    AlreadyInside,

    #[error("No hay ingreso activo para registrar salida")]
    NoActiveIngreso,

    #[error("Gafete no disponible")]
    GafeteNotAvailable,

    #[error("Error de gafete: {0}")]
    Gafete(String),

    #[error("Error de base de datos: {0}")]
    Database(
        #[from]
        #[serde(serialize_with = "serialize_error_as_string")]
        sqlx::Error,
    ),

    #[error("Error de validación: {0}")]
    Validation(String),
}

impl From<crate::domain::common::CommonError> for IngresoProveedorError {
    fn from(err: crate::domain::common::CommonError) -> Self {
        IngresoProveedorError::Validation(err.to_string())
    }
}

// ==========================================
// INGRESO CONTRATISTA ERRORS
// ==========================================

#[derive(Error, Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum IngresoContratistaError {
    #[error("Ingreso de contratista no encontrado")]
    NotFound,

    #[error("El contratista ya tiene un ingreso activo")]
    AlreadyInside,

    #[error("No hay ingreso activo para registrar salida")]
    NoActiveIngreso,

    #[error("Contratista no encontrado")]
    ContratistaNotFound,

    #[error("El contratista está en lista negra: {0}")]
    Blacklisted(String),

    #[error("PRAIND vencido: {0}")]
    PraindExpired(String),

    #[error("El contratista no está activo")]
    ContratistaInactive,

    #[error("Gafete no disponible")]
    GafeteNotAvailable,

    #[error("Error de gafete: {0}")]
    Gafete(String),

    #[error("Error de base de datos: {0}")]
    Database(
        #[from]
        #[serde(serialize_with = "serialize_error_as_string")]
        sqlx::Error,
    ),

    #[error("Error de validación: {0}")]
    Validation(String),

    #[error("Error interno: {0}")]
    Internal(String),
}

impl From<crate::domain::common::CommonError> for IngresoContratistaError {
    fn from(err: crate::domain::common::CommonError) -> Self {
        IngresoContratistaError::Validation(err.to_string())
    }
}

// ==========================================
// INGRESO GENERAL ERRORS (Multi-tipo)
// ==========================================

#[derive(Error, Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum IngresoError {
    #[error("Ingreso no encontrado")]
    NotFound,

    #[error("Tipo de ingreso inválido: {0}")]
    InvalidType(String),

    #[error("La persona ya tiene un ingreso activo")]
    AlreadyInside,

    #[error("No hay ingreso activo para registrar salida")]
    NoActiveIngreso,

    #[error("La persona está en lista negra: {0}")]
    Blacklisted(String),

    #[error("Gafete no disponible")]
    GafeteNotAvailable,

    #[error("Error de gafete: {0}")]
    Gafete(String),

    #[error("Error de base de datos: {0}")]
    Database(
        #[from]
        #[serde(serialize_with = "serialize_error_as_string")]
        sqlx::Error,
    ),

    #[error("Error de validación: {0}")]
    Validation(String),
}

// ==========================================
// CITA ERRORS
// ==========================================

#[derive(Error, Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum CitaError {
    #[error("Cita no encontrada")]
    NotFound,

    #[error("La cita ya fue procesada")]
    AlreadyProcessed,

    #[error("La cita fue cancelada")]
    Cancelled,

    #[error("Visitante no encontrado")]
    VisitanteNotFound,

    #[error("Fecha de cita inválida")]
    InvalidDate,

    #[error("Error de base de datos: {0}")]
    Database(
        #[from]
        #[serde(serialize_with = "serialize_error_as_string")]
        sqlx::Error,
    ),

    #[error("Error de visitante: {0}")]
    Visitante(#[from] VisitanteError),

    #[error("Error de ingreso de visita: {0}")]
    IngresoVisita(#[from] IngresoVisitaError),

    #[error("Error de validación: {0}")]
    Validation(String),
}

// ==========================================
// SEARCH/INDEX ERRORS
// ==========================================

#[derive(Error, Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum SearchError {
    #[error("Error de índice: {0}")]
    Index(String),

    #[error("Error de búsqueda: {0}")]
    Query(String),

    #[error("Índice no inicializado")]
    NotInitialized,

    #[error("Error de base de datos: {0}")]
    Database(
        #[from]
        #[serde(serialize_with = "serialize_error_as_string")]
        sqlx::Error,
    ),

    #[error("Error del motor de búsqueda: {0}")]
    Engine(#[from] crate::search::errors::SearchError),
}

// ==========================================
// EXPORT ERRORS
// ==========================================

#[derive(Error, Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum ExportError {
    #[error("Error al generar exportación: {0}")]
    Generation(String),

    #[error("Formato no soportado: {0}")]
    UnsupportedFormat(String),

    #[error("Error de I/O: {0}")]
    Io(String),

    #[error("Error de base de datos: {0}")]
    Database(
        #[from]
        #[serde(serialize_with = "serialize_error_as_string")]
        sqlx::Error,
    ),

    #[error("Error del motor de exportación: {0}")]
    Engine(#[from] crate::export::errors::ExportError),
}

// ==========================================
// CONFIG ERRORS
// ==========================================

#[derive(Error, Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum ConfigError {
    #[error("Error de configuración: {0}")]
    Message(String),

    #[error("Error de I/O: {0}")]
    Io(String),

    #[error("Error de base de datos: {0}")]
    Database(
        #[from]
        #[serde(serialize_with = "serialize_error_as_string")]
        sqlx::Error,
    ),
}

// ==========================================
// KEYRING ERRORS
// ==========================================

#[derive(Error, Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum KeyringError {
    #[error("Error en almacén de claves: {0}")]
    Message(String),

    #[error("Error de guardado: {0}")]
    StoreError(String),

    #[error("Error de recuperación: {0}")]
    RetrieveError(String),

    #[error("Error de eliminación: {0}")]
    DeleteError(String),
}

// ==========================================
// SYSTEM ERRORS
// ==========================================

#[derive(Error, Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum SystemError {
    #[error("Error de sistema: {0}")]
    Message(String),

    #[error("Error de ventana: {0}")]
    Window(String),

    #[error("Error de proceso: {0}")]
    Process(String),
}
// ==========================================
// ROLE ERRORS
// ==========================================

#[derive(Error, Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum RoleError {
    #[error("Rol no encontrado")]
    NotFound,

    #[error("Ya existe un rol con este nombre")]
    NameExists,

    #[error("No se puede eliminar un rol del sistema")]
    CannotDeleteSystemRole,

    #[error("Solo el superusuario puede modificar roles del sistema")]
    CannotModifySystemRole,

    #[error("Rol del sistema")]
    SystemRole,

    #[error("Error de base de datos: {0}")]
    Database(
        #[from]
        #[serde(serialize_with = "serialize_error_as_string")]
        sqlx::Error,
    ),

    #[error("Error de validación: {0}")]
    Validation(String),

    #[error("No autorizado: {0}")]
    Unauthorized(String),
}
