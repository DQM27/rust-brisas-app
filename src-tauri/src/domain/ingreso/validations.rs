// ==========================================
// src/domain/ingreso/validations.rs
// ==========================================

use crate::models::ingreso::*;
use crate::domain::errors::IngresoError;
use crate::db::{contratista as contratista_db, lista_negra as lista_negra_db, ingreso as ingreso_db};
use sqlx::SqlitePool;
use chrono::NaiveDateTime;

// ==========================================
// ERRORES DE VALIDACIÓN
// ==========================================

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("La cédula no puede estar vacía")]
    CedulaVacia,
    
    #[error("El nombre no puede estar vacío")]
    NombreVacio,
    
    #[error("El nombre no puede exceder 50 caracteres")]
    NombreMuyLargo,
    
    #[error("El apellido no puede estar vacío")]
    ApellidoVacio,
    
    #[error("El apellido no puede exceder 50 caracteres")]
    ApellidoMuyLargo,
    
    #[error("El nombre de empresa no puede estar vacío")]
    EmpresaVacia,
    
    #[error("El gafete no puede estar vacío")]
    GafeteVacio,
    
    #[error("La placa temporal no puede estar vacía")]
    PlacaTemporalVacia,
}

// ==========================================
// CONSTANTES
// ==========================================

const NOMBRE_MAX_LENGTH: usize = 50;
const APELLIDO_MAX_LENGTH: usize = 50;

// ==========================================
// VALIDACIONES DE FORMATO
// ==========================================

pub fn validar_cedula(cedula: &str) -> Result<String, ValidationError> {
    let limpia = cedula.trim();
    
    if limpia.is_empty() {
        return Err(ValidationError::CedulaVacia);
    }
    
    Ok(limpia.to_string())
}

pub fn validar_nombre(nombre: &str) -> Result<String, ValidationError> {
    let limpio = nombre.trim();
    
    if limpio.is_empty() {
        return Err(ValidationError::NombreVacio);
    }
    
    if limpio.len() > NOMBRE_MAX_LENGTH {
        return Err(ValidationError::NombreMuyLargo);
    }
    
    Ok(limpio.to_string())
}

pub fn validar_apellido(apellido: &str) -> Result<String, ValidationError> {
    let limpio = apellido.trim();
    
    if limpio.is_empty() {
        return Err(ValidationError::ApellidoVacio);
    }
    
    if limpio.len() > APELLIDO_MAX_LENGTH {
        return Err(ValidationError::ApellidoMuyLargo);
    }
    
    Ok(limpio.to_string())
}

pub fn validar_empresa_nombre(empresa: &str) -> Result<String, ValidationError> {
    let limpia = empresa.trim();
    
    if limpia.is_empty() {
        return Err(ValidationError::EmpresaVacia);
    }
    
    Ok(limpia.to_string())
}

// ==========================================
// VALIDADOR PRINCIPAL
// ==========================================

pub struct IngresoValidator;

impl IngresoValidator {
    /// Valida si un contratista puede ingresar
    pub async fn validar_puede_ingresar(
        pool: &SqlitePool,
        cedula: &str,
    ) -> Result<ValidacionIngresoResponse, IngresoError> {
        let mut alertas = Vec::new();
        let mut puede_ingresar = true;
        let mut motivo_rechazo = None;
        
        // 1. Validar formato de cédula
        let cedula_limpia = validar_cedula(cedula)
            .map_err(|e| IngresoError::ValidationError(e.to_string()))?;
        
        // 2. Verificar lista negra
        if let Some(bloqueo) = lista_negra_db::find_activo_by_cedula(pool, &cedula_limpia).await? {
            puede_ingresar = false;
            motivo_rechazo = Some(format!(
                "BLOQUEADO - En lista negra. Motivo: {}",
                bloqueo.motivo_bloqueo
            ));
            
            return Ok(ValidacionIngresoResponse {
                puede_ingresar,
                motivo_rechazo,
                alertas,
                contratista: None,
                tiene_ingreso_abierto: false,
                ingreso_abierto: None,
            });
        }
        
        // 3. Buscar contratista
        match contratista_db::find_by_cedula(pool, &cedula_limpia).await {
            Ok(contratista) => {
                // 3.1 Validar estado activo
                if contratista.estado != crate::models::contratista::EstadoContratista::Activo {
                    puede_ingresar = false;
                    motivo_rechazo = Some(format!(
                        "DENEGADO - Contratista inactivo (estado: {:?})",
                        contratista.estado
                    ));
                }
                
                // 3.2 Validar PRAIND vigente
                if contratista.praind_vencido {
                    puede_ingresar = false;
                    motivo_rechazo = Some(format!(
                        "DENEGADO - PRAIND vencido (venció hace {} días)",
                        contratista.dias_hasta_vencimiento.abs()
                    ));
                } else if contratista.requiere_atencion {
                    alertas.push(format!(
                        "⚠️ PRAIND vence pronto ({} días restantes)",
                        contratista.dias_hasta_vencimiento
                    ));
                }
                
                // 3.3 Verificar ingreso abierto
                let ingreso_abierto = ingreso_db::find_abierto_by_cedula(pool, &cedula_limpia)
                    .await
                    .ok();
                
                let tiene_ingreso_abierto = ingreso_abierto.is_some();
                
                // 3.4 Verificar deudas de gafetes
                let deudas = crate::db::gafete_perdido::find_deudas_by_contratista(
                    pool,
                    &contratista.id
                ).await?;
                
                for deuda in deudas {
                    alertas.push(format!(
                        "⚠️ Debe gafete #{} - Monto: ${:.2}",
                        deuda.gafete_numero, deuda.monto_cobro
                    ));
                }
                
                // Construir objeto contratista
                let contratista_json = serde_json::json!({
                    "id": contratista.id,
                    "cedula": contratista.cedula,
                    "nombre": contratista.nombre,
                    "apellido": contratista.apellido,
                    "empresa_nombre": contratista.empresa_nombre,
                    "fecha_vencimiento_praind": contratista.fecha_vencimiento_praind,
                    "estado": contratista.estado,
                });
                
                Ok(ValidacionIngresoResponse {
                    puede_ingresar,
                    motivo_rechazo,
                    alertas,
                    contratista: Some(contratista_json),
                    tiene_ingreso_abierto,
                    ingreso_abierto,
                })
            }
            Err(_) => {
                // Contratista no existe
                Ok(ValidacionIngresoResponse {
                    puede_ingresar: true,
                    motivo_rechazo: None,
                    alertas: vec![
                        "ℹ️ Contratista no registrado. Puede crear registro o ingreso temporal.".to_string()
                    ],
                    contratista: None,
                    tiene_ingreso_abierto: false,
                    ingreso_abierto: None,
                })
            }
        }
    }
    
    /// Valida creación de ingreso contratista
    pub async fn validar_creacion_contratista(
        pool: &SqlitePool,
        input: &CreateIngresoContratistaInput,
    ) -> Result<ValidatedCreateContratistaInput, IngresoError> {
        // Verificar que contratista existe y obtener sus datos
        let contratista = contratista_db::find_by_id(pool, &input.contratista_id)
            .await
            .map_err(|_| IngresoError::ContratistaNoEncontrado)?;
        
        // Validar que no tenga ingreso abierto
        if ingreso_db::tiene_ingreso_abierto(pool, &contratista.cedula).await? {
            return Err(IngresoError::IngresoAbierto);
        }
        
        Ok(ValidatedCreateContratistaInput {
            contratista_id: input.contratista_id.clone(),
            cedula: contratista.cedula,
            nombre: contratista.nombre,
            apellido: contratista.apellido,
            empresa_nombre: contratista.empresa_nombre,
            estado: contratista.estado.as_str().to_string(),
            praind_vigente: !contratista.praind_vencido,
            vehiculo_id: input.vehiculo_id.clone(),
            gafete_id: input.gafete_id.clone(),
            usuario_ingreso_id: input.usuario_ingreso_id.clone(),
            observaciones: input.observaciones.clone(),
        })
    }
    
    /// Valida creación de ingreso temporal
    pub async fn validar_creacion_temporal(
        pool: &SqlitePool,
        input: &CreateIngresoTemporalInput,
    ) -> Result<ValidatedCreateTemporalInput, IngresoError> {
        // Validar formato
        let cedula = validar_cedula(&input.cedula)
            .map_err(|e| IngresoError::ValidationError(e.to_string()))?;
        let nombre = validar_nombre(&input.nombre)
            .map_err(|e| IngresoError::ValidationError(e.to_string()))?;
        let apellido = validar_apellido(&input.apellido)
            .map_err(|e| IngresoError::ValidationError(e.to_string()))?;
        let empresa_nombre = validar_empresa_nombre(&input.empresa_nombre)
            .map_err(|e| IngresoError::ValidationError(e.to_string()))?;
        
        // Verificar que no esté en lista negra
        if let Some(bloqueo) = lista_negra_db::find_activo_by_cedula(pool, &cedula).await? {
            return Err(IngresoError::EnListaNegra {
                cedula: cedula.clone(),
                motivo: bloqueo.motivo_bloqueo,
            });
        }
        
        // Verificar que no tenga ingreso abierto
        if ingreso_db::tiene_ingreso_abierto(pool, &cedula).await? {
            return Err(IngresoError::IngresoAbierto);
        }
        
        Ok(ValidatedCreateTemporalInput {
            cedula,
            nombre,
            apellido,
            empresa_nombre,
            placa_temporal: input.placa_temporal.clone(),
            gafete_id: input.gafete_id.clone(),
            usuario_ingreso_id: input.usuario_ingreso_id.clone(),
            observaciones: input.observaciones.clone(),
        })
    }
}

// ==========================================
// INPUTS VALIDADOS
// ==========================================

#[derive(Debug, Clone)]
pub struct ValidatedCreateContratistaInput {
    pub contratista_id: String,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub empresa_nombre: String,
    pub estado: String,
    pub praind_vigente: bool,
    pub vehiculo_id: Option<String>,
    pub gafete_id: String,
    pub usuario_ingreso_id: String,
    pub observaciones: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ValidatedCreateTemporalInput {
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub empresa_nombre: String,
    pub placa_temporal: Option<String>,
    pub gafete_id: String,
    pub usuario_ingreso_id: String,
    pub observaciones: Option<String>,
}

// ==========================================
// TESTS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validar_cedula() {
        assert!(validar_cedula("123456789").is_ok());
        assert!(validar_cedula("  123  ").is_ok());
        assert!(validar_cedula("").is_err());
    }
    
    #[test]
    fn test_validar_nombre() {
        assert!(validar_nombre("Juan").is_ok());
        assert!(validar_nombre("").is_err());
        assert!(validar_nombre(&"a".repeat(51)).is_err());
    }
}