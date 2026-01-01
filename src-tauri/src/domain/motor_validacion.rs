/// Capa de Dominio: Motor de Validación Unificado.
///
/// Este módulo implementa la lógica central para la validación de cualquier
/// tipo de acceso (Visitante, Contratista, Proveedor). Orquestra múltiples
/// reglas de negocio: listas negras, vigencia de documentos y alertas de seguridad.
use crate::models::ingreso::{
    CommonValidationContext, ValidationReason, ValidationResult, ValidationStatus,
};
use crate::models::lista_negra::NivelSeveridad;

// --------------------------------------------------------------------------
// DEFINICIONES DE CONTEXTO Y RESULTADOS
// --------------------------------------------------------------------------

/// Representa el conjunto de datos necesarios para evaluar la legitimidad de un acceso.
#[derive(Debug, Clone)]
pub struct MotorContexto {
    pub ident_cedula: String,
    pub ident_nombre: String,
    pub tipo_acceso: String,
    pub lista_negra: Option<InfoListaNegra>,
    pub ingreso_activo: Option<InfoIngresoActivoInt>,
    pub estado_autorizacion: String,
    pub alerta_gafete: Option<String>,
}

/// Información simplificada de la lista negra para el motor.
#[derive(Debug, Clone)]
pub struct InfoListaNegra {
    pub motivo: String,
    pub severidad: NivelSeveridad,
}

/// Información simplificada sobre un ingreso que no ha registrado salida.
#[derive(Debug, Clone)]
pub struct InfoIngresoActivoInt {
    pub id: String,
    pub fecha_ingreso: String,
    pub gafete_numero: i32,
}

// --------------------------------------------------------------------------
// LÓGICA PRINCIPAL DEL MOTOR
// --------------------------------------------------------------------------

/// Ejecuta todas las reglas de negocio sobre un contexto y retorna una decisión de acceso.
pub fn ejecutar_validacion_motor(ctx: &MotorContexto) -> ValidationResult {
    // 1. REGLA: LISTA NEGRA (Prioridad Crítica)
    if let Some(ref ln) = ctx.lista_negra {
        return ValidationResult {
            status: ValidationStatus::Denied,
            reason: ValidationReason::Blacklisted,
            message: format!(
                "Persona en LISTA NEGRA. Severidad: {:?}. Motivo: {}",
                ln.severidad, ln.motivo
            ),
        };
    }

    // 2. REGLA: INGRESO DUPLICADO
    if let Some(ref activo) = ctx.ingreso_activo {
        return ValidationResult {
            status: ValidationStatus::Denied,
            reason: ValidationReason::AlreadyInside,
            message: format!(
                "Ya cuenta con un ingreso activo desde {} (Gafete #{})",
                activo.fecha_ingreso, activo.gafete_numero
            ),
        };
    }

    // 3. REGLA: ESTADO DE AUTORIZACIÓN (Vigencia de Documentos)
    let status_lower = ctx.estado_autorizacion.to_lowercase();
    if status_lower == "vencido" || status_lower == "inactivo" || status_lower == "suspendido" {
        return ValidationResult {
            status: ValidationStatus::Denied,
            reason: ValidationReason::ExpiredDocuments,
            message: format!(
                "Estado de autorización: {}. Debe actualizar documentos.",
                ctx.estado_autorizacion
            ),
        };
    }

    // 4. REGLA: ALERTAS DE GAFETE (Hardware/Pérdida)
    if let Some(ref alerta) = ctx.alerta_gafete {
        return ValidationResult {
            status: ValidationStatus::Warning,
            reason: ValidationReason::GafeteAlert,
            message: format!("Alerta de Gafete detectada: {}", alerta),
        };
    }

    // SI PASA TODO -> ACCESO PERMITIDO
    ValidationResult {
        status: ValidationStatus::Allowed,
        reason: ValidationReason::None,
        message: "Acceso validado correctamente".to_string(),
    }
}

// --------------------------------------------------------------------------
// MAPEO Y ADAPTACIÓN DE DATOS
// --------------------------------------------------------------------------

/// Convierte el contexto genérico de modelos al contexto específico de negocio del motor.
pub fn map_to_motor_context(common: &CommonValidationContext) -> MotorContexto {
    MotorContexto {
        ident_cedula: common.cedula.clone(),
        ident_nombre: common.nombre_completo.clone(),
        tipo_acceso: common.tipo_identidad.clone(),
        lista_negra: common.lista_negra_info.as_ref().map(|ln| InfoListaNegra {
            motivo: ln.motivo.clone(),
            severidad: ln.severidad.clone(),
        }),
        ingreso_activo: common.ingreso_activo_id.as_ref().map(|id| InfoIngresoActivoInt {
            id: id.clone(),
            fecha_ingreso: common.fecha_ingreso_activo.clone().unwrap_or_default(),
            gafete_numero: common.gafete_activo_numero.clone().unwrap_or_default(),
        }),
        estado_autorizacion: common.estado_autorizacion.clone(),
        alerta_gafete: common.alerta_gafete.clone(),
    }
}

// --------------------------------------------------------------------------
// PRUEBAS UNITARIAS
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn create_base_context() -> MotorContexto {
        MotorContexto {
            ident_cedula: "123".to_string(),
            ident_nombre: "Test".to_string(),
            tipo_acceso: "VISITANTE".to_string(),
            lista_negra: None,
            ingreso_activo: None,
            estado_autorizacion: "ACTIVO".to_string(),
            alerta_gafete: None,
        }
    }

    #[test]
    fn test_motor_allow_success() {
        let ctx = create_base_context();
        let res = ejecutar_validacion_motor(&ctx);
        assert_eq!(res.status, ValidationStatus::Allowed);
    }

    #[test]
    fn test_motor_deny_blacklist() {
        let mut ctx = create_base_context();
        ctx.lista_negra =
            Some(InfoListaNegra { motivo: "Robo".to_string(), severidad: NivelSeveridad::Alto });
        let res = ejecutar_validacion_motor(&ctx);
        assert_eq!(res.status, ValidationStatus::Denied);
        assert_eq!(res.reason, ValidationReason::Blacklisted);
    }

    #[test]
    fn test_motor_deny_duplicate() {
        let mut ctx = create_base_context();
        ctx.ingreso_activo = Some(InfoIngresoActivoInt {
            id: "id".to_string(),
            fecha_ingreso: "2023-01-01".to_string(),
            gafete_numero: 10,
        });
        let res = ejecutar_validacion_motor(&ctx);
        assert_eq!(res.status, ValidationStatus::Denied);
        assert_eq!(res.reason, ValidationReason::AlreadyInside);
    }

    #[test]
    fn test_motor_deny_expired() {
        let mut ctx = create_base_context();
        ctx.estado_autorizacion = "VENCIDO".to_string();
        let res = ejecutar_validacion_motor(&ctx);
        assert_eq!(res.status, ValidationStatus::Denied);
        assert_eq!(res.reason, ValidationReason::ExpiredDocuments);
    }

    #[test]
    fn test_motor_warning_gafete() {
        let mut ctx = create_base_context();
        ctx.alerta_gafete = Some("Batería baja".to_string());
        let res = ejecutar_validacion_motor(&ctx);
        assert_eq!(res.status, ValidationStatus::Warning);
        assert_eq!(res.reason, ValidationReason::GafeteAlert);
    }
}
