use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("Usuario no encontrado")]
    NotFound,

    #[error("Ya existe un usuario con este email")]
    EmailExists,

    #[error("Credenciales inválidas")]
    InvalidCredentials,

    #[error("La contraseña actual es incorrecta")]
    InvalidCurrentPassword,

    #[error("Usuario inactivo")]
    InactiveUser,

    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Error de búsqueda: {0}")]
    Search(String),

    #[error("Error de autenticación: {0}")]
    Auth(String),

    #[error("Rol desconocido: {0}")]
    InvalidRole(String),

    #[error("No se puede eliminar la empresa porque tiene {0} contratista(s) asociado(s)")]
    EmpresaHasContratistas(i64),

    #[error("Error inesperado: {0}")]
    Internal(String),

    // Validation errors (from domain) usually come as String currently
    #[error("Error de validación: {0}")]
    Validation(String),
}

// ==========================================
// CONTRATISTA ERRORS
// ==========================================

#[derive(Error, Debug)]
pub enum ContratistaError {
    #[error("Contratista no encontrado")]
    NotFound,

    #[error("Ya existe un contratista con esta cédula")]
    CedulaExists,

    #[error("Empresa no encontrada")]
    EmpresaNotFound,

    #[error("Estado inválido: {0}")]
    InvalidStatus(String),

    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Error de búsqueda: {0}")]
    Search(String),

    #[error("Error de validación: {0}")]
    Validation(String),
}

// ==========================================
// EMPRESA ERRORS
// ==========================================

#[derive(Error, Debug)]
pub enum EmpresaError {
    #[error("Empresa no encontrada")]
    NotFound,

    #[error("Ya existe una empresa con este nombre")]
    NameExists,

    #[error("No se puede eliminar la empresa porque tiene {0} contratista(s) asociado(s)")]
    HasContratistas(i64),

    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Error de validación: {0}")]
    Validation(String),
}

// ==========================================
// VEHICULO ERRORS
// ==========================================

#[derive(Error, Debug)]
pub enum VehiculoError {
    #[error("Vehículo no encontrado")]
    NotFound,

    #[error("Ya existe un vehículo con esta placa")]
    PlacaExists,

    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Error de validación: {0}")]
    Validation(String),
}

// ==========================================
// ALERTA ERRORS
// ==========================================

#[derive(Error, Debug)]
pub enum AlertaError {
    #[error("Alerta no encontrada")]
    NotFound,

    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Error de validación: {0}")]
    Validation(String),
}
