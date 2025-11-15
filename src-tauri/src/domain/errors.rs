// ==========================================
// src/domain/errors.rs
// ==========================================

use thiserror::Error;

// ==========================================
// CONTRATISTA
// ==========================================

#[derive(Error, Debug)]
pub enum ContratistaError {
    #[error("Cédula duplicada: {0}")]
    CedulaDuplicada(String),
    
    #[error("Empresa no existe: {0}")]
    EmpresaNoExiste(String),
    
    #[error("En lista negra - Cédula: {cedula}, Motivo: {motivo}, Bloqueado por: {bloqueado_por}")]
    EnListaNegra {
        cedula: String,
        motivo: String,
        bloqueado_por: String,
    },
    
    #[error("Error de validación: {0}")]
    ValidationError(String),
    
    #[error("Error al parsear: {0}")]
    ParseError(String),
    
    #[error("Contratista no encontrado")]
    NoEncontrado,
    
    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),
}

// ==========================================
// INGRESO
// ==========================================

#[derive(Error, Debug)]
pub enum IngresoError {
    #[error("Ingreso no encontrado")]
    NoEncontrado,
    
    #[error("Contratista no encontrado")]
    ContratistaNoEncontrado,
    
    #[error("Ya tiene un ingreso abierto")]
    IngresoAbierto,
    
    #[error("En lista negra - Cédula: {cedula}, Motivo: {motivo}")]
    EnListaNegra {
        cedula: String,
        motivo: String,
    },
    
    #[error("Error de validación: {0}")]
    ValidationError(String),
    
    #[error("Error al parsear: {0}")]
    ParseError(String),
    
    #[error("Error de gafete: {0}")]
    GafeteError(String),
    
    #[error("Error de gafete perdido: {0}")]
    GafetePerdidoError(String),
    
    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),
}

// ==========================================
// EMPRESA
// ==========================================

#[derive(Error, Debug)]
pub enum EmpresaError {
    #[error("Empresa no encontrada")]
    NoEncontrada,
    
    #[error("Nombre de empresa duplicado: {0}")]
    NombreDuplicado(String),
    
    #[error("Error de validación: {0}")]
    ValidationError(String),
    
    #[error("Error al parsear: {0}")]
    ParseError(String),
    
    #[error("No se puede eliminar - Tiene contratistas asociados")]
    TieneContratistasAsociados,
    
    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),
}

// ==========================================
// GAFETE
// ==========================================

#[derive(Error, Debug)]
pub enum GafeteError {
    #[error("Gafete no encontrado")]
    NoEncontrado,
    
    #[error("Número de gafete duplicado: {0}")]
    NumeroDuplicado(String),
    
    #[error("Gafete ya está asignado")]
    YaAsignado,
    
    #[error("Gafete no está asignado")]
    NoAsignado,
    
    #[error("Gafete perdido - No se puede asignar")]
    Perdido,
    
    #[error("Error de validación: {0}")]
    ValidationError(String),
    
    #[error("Error al parsear: {0}")]
    ParseError(String),
    
    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),
}

// ==========================================
// GAFETE PERDIDO
// ==========================================

#[derive(Error, Debug)]
pub enum GafetePerdidoError {
    #[error("Reporte de gafete perdido no encontrado")]
    NoEncontrado,
    
    #[error("Gafete no encontrado")]
    GafeteNoEncontrado,
    
    #[error("Contratista no encontrado")]
    ContratistaNoEncontrado,
    
    #[error("Deuda ya fue pagada")]
    DeudaYaPagada,
    
    #[error("Monto de pago inválido: esperado {esperado}, recibido {recibido}")]
    MontoPagoInvalido {
        esperado: f64,
        recibido: f64,
    },
    
    #[error("Error de validación: {0}")]
    ValidationError(String),
    
    #[error("Error al parsear: {0}")]
    ParseError(String),
    
    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),
}

// ==========================================
// VEHÍCULO
// ==========================================

#[derive(Error, Debug)]
pub enum VehiculoError {
    #[error("Vehículo no encontrado")]
    NoEncontrado,
    
    #[error("Placa duplicada: {0}")]
    PlacaDuplicada(String),
    
    #[error("Contratista no encontrado")]
    ContratistaNoEncontrado,
    
    #[error("Error de validación: {0}")]
    ValidationError(String),
    
    #[error("Error al parsear: {0}")]
    ParseError(String),
    
    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),
}

// ==========================================
// LISTA NEGRA
// ==========================================

#[derive(Error, Debug)]
pub enum ListaNegraError {
    #[error("Registro de lista negra no encontrado")]
    NoEncontrado,
    
    #[error("La persona ya está en lista negra")]
    YaEnListaNegra,
    
    #[error("Error de validación: {0}")]
    ValidationError(String),
    
    #[error("Error al parsear: {0}")]
    ParseError(String),
    
    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),
}

// ==========================================
// USER (AUTH)
// ==========================================

#[derive(Error, Debug)]
pub enum UserError {
    #[error("Usuario no encontrado")]
    NoEncontrado,
    
    #[error("Email duplicado: {0}")]
    EmailDuplicado(String),
    
    #[error("Credenciales inválidas")]
    CredencialesInvalidas,
    
    #[error("Usuario inactivo")]
    UsuarioInactivo,
    
    #[error("Error de validación: {0}")]
    ValidationError(String),
    
    #[error("Error al hashear contraseña: {0}")]
    HashError(String),
    
    #[error("Error al parsear: {0}")]
    ParseError(String),
    
    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),
}

// ==========================================
// CONVERSIONES ENTRE ERRORES
// ==========================================

// Permite convertir GafeteError en IngresoError cuando sea necesario
impl From<GafeteError> for IngresoError {
    fn from(err: GafeteError) -> Self {
        IngresoError::GafeteError(err.to_string())
    }
}

// Permite convertir GafetePerdidoError en IngresoError
impl From<GafetePerdidoError> for IngresoError {
    fn from(err: GafetePerdidoError) -> Self {
        IngresoError::GafetePerdidoError(err.to_string())
    }
}

// Permite convertir ContratistaError en IngresoError cuando sea necesario
impl From<ContratistaError> for IngresoError {
    fn from(err: ContratistaError) -> Self {
        match err {
            ContratistaError::NoEncontrado => IngresoError::ContratistaNoEncontrado,
            ContratistaError::EnListaNegra { cedula, motivo, .. } => {
                IngresoError::EnListaNegra { cedula, motivo }
            }
            ContratistaError::ValidationError(msg) => IngresoError::ValidationError(msg),
            ContratistaError::ParseError(msg) => IngresoError::ParseError(msg),
            ContratistaError::Database(e) => IngresoError::Database(e),
            _ => IngresoError::ValidationError(err.to_string()),
        }
    }
}

// ==========================================
// TESTS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_contratista_error_display() {
        let error = ContratistaError::CedulaDuplicada("123456789".to_string());
        assert_eq!(error.to_string(), "Cédula duplicada: 123456789");
    }
    
    #[test]
    fn test_ingreso_error_display() {
        let error = IngresoError::IngresoAbierto;
        assert_eq!(error.to_string(), "Ya tiene un ingreso abierto");
    }
    
    #[test]
    fn test_gafete_error_conversion() {
        let gafete_err = GafeteError::NoEncontrado;
        let ingreso_err: IngresoError = gafete_err.into();
        assert!(matches!(ingreso_err, IngresoError::GafeteError(_)));
    }
}