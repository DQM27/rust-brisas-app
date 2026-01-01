/// Capa de Dominio: Motor de Validación Unificado.
///
/// Este módulo implementa la lógica central para la validación de cualquier
/// tipo de acceso (Visitante, Contratista, Proveedor). Orquestra múltiples
/// reglas de negocio: listas negras, vigencia de documentos y alertas de seguridad.
use crate::models::validation::{
    EstadoAutorizacion, InfoIngresoActivoInt, InfoListaNegra, MotorContexto, NivelSeveridad,
    TipoAcceso, ValidationReason, ValidationResult, ValidationStatus,
};

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
        let info_gafete = if activo.gafete_numero == 0 {
            "Sin Gafete Asignado".to_string()
        } else {
            format!("Gafete #{}", activo.gafete_numero)
        };

        return ValidationResult {
            status: ValidationStatus::Denied,
            reason: ValidationReason::AlreadyInside,
            message: format!(
                "Ya cuenta con un ingreso activo desde {} ({})",
                activo.fecha_ingreso, info_gafete
            ),
        };
    }

    // 3. REGLA: ESTADO DE AUTORIZACIÓN (Vigencia de Documentos)
    match ctx.estado_autorizacion {
        EstadoAutorizacion::Vencido => ValidationResult {
            status: ValidationStatus::Denied,
            reason: ValidationReason::ExpiredDocuments,
            message: "Estado de autorización: VENCIDO. Debe actualizar documentos.".to_string(),
        },
        EstadoAutorizacion::Inactivo => ValidationResult {
            status: ValidationStatus::Denied,
            reason: ValidationReason::ExpiredDocuments,
            message: "Estado de autorización: INACTIVO. Acceso denegado.".to_string(),
        },
        EstadoAutorizacion::Suspendido => ValidationResult {
            status: ValidationStatus::Denied,
            reason: ValidationReason::ExpiredDocuments,
            message: "Estado de autorización: SUSPENDIDO. Contacte administración.".to_string(),
        },
        _ => {
            // 4. REGLA: ALERTAS DE GAFETE (Hardware/Pérdida)
            // Se ejecuta solo si la autorización base es válida (Activo o PorDefinir)
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
            tipo_acceso: TipoAcceso::Visitante,
            lista_negra: None,
            ingreso_activo: None,
            estado_autorizacion: EstadoAutorizacion::Activo,
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
        ctx.estado_autorizacion = EstadoAutorizacion::Vencido;
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

    #[test]
    fn test_estado_mapping() {
        assert_eq!(EstadoAutorizacion::from_str_lossy("vencido"), EstadoAutorizacion::Vencido);
        assert_eq!(EstadoAutorizacion::from_str_lossy("ACTIVE"), EstadoAutorizacion::Activo);
        assert_eq!(EstadoAutorizacion::from_str_lossy("random"), EstadoAutorizacion::PorDefinir);
    }
}
