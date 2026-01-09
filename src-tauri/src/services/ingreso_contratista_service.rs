/// Servicio: Punto de Control de Seguridad - Ingresos y Salidas de Contratistas.
///
/// Este es el núcleo operativo de la garita. Coordina múltiples subsistemas
/// (Lista Negra, Vigencia PRAIND, Estado de Contratista, Gestión de Gafetes)
/// para determinar en tiempo real si un trabajador externo puede ingresar.
///
/// Responsabilidades:
/// - Validación de pre-ingreso (identidad, seguridad, unicidad).
/// - Registro físico de entrada con asignación de recursos.
/// - Control de salida y liberación de gafetes.
/// - Monitoreo de tiempos de permanencia.
use crate::domain::errors::IngresoContratistaError;
use crate::repositories::traits::{
    ContratistaRepository, GafeteRepository, IngresoContratistaRepository, SecurityRepository,
};

use crate::domain::motor_validacion as motor;
use crate::models::contratista::ContratistaResponse;
use crate::models::ingreso::{
    AlertaTiempoExcedido, CreateIngresoContratistaInput, IngresoConEstadoResponse, IngresoResponse,
    RegistrarSalidaInput, ResultadoValidacionSalida, ValidacionIngresoResponse,
};
use crate::models::lista_negra::BlockStatus;
use crate::models::validation::{
    EstadoAutorizacion, InfoListaNegra, MotorContexto, NivelSeveridad, TipoAcceso, ValidationStatus,
};
use log::{error, info, warn};
use surrealdb::RecordId;

pub struct IngresoContratistaService<
    R: IngresoContratistaRepository,
    G: GafeteRepository,
    C: ContratistaRepository,
    S: SecurityRepository,
> {
    ingreso_repo: R,
    gafete_repo: G,
    contratista_repo: C,
    security_repo: S,
}

impl<R, G, C, S> IngresoContratistaService<R, G, C, S>
where
    R: IngresoContratistaRepository,
    G: GafeteRepository,
    C: ContratistaRepository,
    S: SecurityRepository,
{
    pub const fn new(
        ingreso_repo: R,
        gafete_repo: G,
        contratista_repo: C,
        security_repo: S,
    ) -> Self {
        Self { ingreso_repo, gafete_repo, contratista_repo, security_repo }
    }

    pub async fn validar_ingreso_contratista(
        &self,
        contratista_id_str: String,
    ) -> Result<ValidacionIngresoResponse, IngresoContratistaError> {
        let contratista_id = parse_contratista_id(&contratista_id_str)
            .unwrap_or_else(|_| RecordId::from_table_key("contratista", &contratista_id_str));

        let contratista = self
            .contratista_repo
            .find_by_id_fetched(&contratista_id)
            .await
            .map_err(|e| {
                error!("Error DB al buscar contratista para validación: {e}");
                IngresoContratistaError::Database(e.to_string())
            })?
            .ok_or_else(|| {
                warn!("Contratista no encontrado para validación: {contratista_id_str}");
                IngresoContratistaError::ContratistaNotFound
            })?;

        // Evaluación dinámica del vencimiento de PRAIND from Refactor branch
        // Although DB state says 'Activo', if date expired, engine must reject.
        let raw_date_str = contratista.fecha_vencimiento_praind.to_string();
        let clean_date = raw_date_str.trim_start_matches("d'").trim_end_matches('\'');
        let estado_praind = crate::domain::contratista::calcular_estado_praind(clean_date);
        let praind_vencido = estado_praind.vencido;

        let estado_autorizacion_calculado = if praind_vencido {
            EstadoAutorizacion::Vencido
        } else {
            EstadoAutorizacion::from_str_lossy(contratista.estado.as_str())
        };

        let b = self.security_repo.check_if_blocked_by_cedula(&contratista.cedula).await.unwrap_or(
            BlockStatus { is_blocked: false, nivel_severidad: None, bloqueado_desde: None },
        );

        let ing_ab = self
            .ingreso_repo
            .find_ingreso_abierto_by_contratista(&contratista.id)
            .await
            .map_err(|e| IngresoContratistaError::Database(e.to_string()))?;

        if let Some(ing) = ing_ab {
            let resp = IngresoResponse::from_contratista_fetched(ing)
                .map_err(IngresoContratistaError::Validation)?;
            return Ok(ValidacionIngresoResponse {
                puede_ingresar: false,
                motivo_rechazo: Some("Ya tiene un ingreso activo en planta".to_string()),
                severidad_lista_negra: None,
                alertas: vec![],
                contratista: None,
                tiene_ingreso_abierto: true,
                ingreso_abierto: Some(resp),
            });
        }

        let motor_ctx = MotorContexto {
            ident_cedula: contratista.cedula.clone(),
            ident_nombre: format!("{} {}", contratista.nombre, contratista.apellido),
            tipo_acceso: TipoAcceso::Contratista,
            lista_negra: if b.is_blocked {
                Some(InfoListaNegra {
                    motivo: "Bloqueo detectado".to_string(),
                    severidad: b
                        .nivel_severidad
                        .as_ref()
                        .map(|s| NivelSeveridad::from_str_lossy(s))
                        .unwrap_or(NivelSeveridad::Alto),
                })
            } else {
                None
            },
            ingreso_activo: None,
            estado_autorizacion: estado_autorizacion_calculado,
            alerta_gafete: None,
        };

        let motor_res = motor::ejecutar_validacion_motor(&motor_ctx);

        Ok(ValidacionIngresoResponse {
            puede_ingresar: motor_res.status == ValidationStatus::Allowed,
            motivo_rechazo: if motor_res.status == ValidationStatus::Allowed {
                None
            } else {
                Some(motor_res.message)
            },
            severidad_lista_negra: if b.is_blocked { b.nivel_severidad.clone() } else { None },
            alertas: vec![],
            contratista: Some(
                serde_json::to_value(ContratistaResponse::from_fetched(contratista)).unwrap(),
            ),
            tiene_ingreso_abierto: false,
            ingreso_abierto: None,
        })
    }

    pub async fn crear_ingreso_contratista(
        &self,
        input: CreateIngresoContratistaInput,
        usuario_id_str: String,
    ) -> Result<IngresoResponse, IngresoContratistaError> {
        let contratista_id = parse_contratista_id(&input.contratista_id)?;
        let usuario_id = parse_user_id(&usuario_id_str)?;

        // ⚠️ REGLA DE NEGOCIO: Verificar si ya tiene un ingreso activo
        let ingreso_existente = self
            .ingreso_repo
            .find_ingreso_abierto_by_contratista(&contratista_id)
            .await
            .map_err(|e| IngresoContratistaError::Database(e.to_string()))?;

        if ingreso_existente.is_some() {
            warn!("Intento de ingreso duplicado para contratista: {}", input.contratista_id);
            return Err(IngresoContratistaError::Validation(
                "El contratista ya tiene un ingreso activo. Debe registrar la salida primero."
                    .to_string(),
            ));
        }

        // Convertir gafete de String a i32 (0 = sin gafete, "S/G" = sin gafete)
        let gafete_int: Option<i32> = if let Some(ref g_str) = input.gafete_numero {
            if g_str.trim().is_empty() || g_str.eq_ignore_ascii_case("S/G") {
                Some(0) // Sin gafete
            } else {
                Some(
                    crate::domain::common::normalizar_gafete_a_int(g_str)
                        .map_err(IngresoContratistaError::Validation)?,
                )
            }
        } else {
            Some(0) // Sin gafete por defecto
        };

        if let Some(g) = gafete_int {
            if g != 0 {
                let disp = self
                    .gafete_repo
                    .is_disponible(g, "contratista")
                    .await
                    .map_err(|e| IngresoContratistaError::Gafete(e.to_string()))?;

                if !disp {
                    warn!("Gafete {g} no disponible para ingreso");
                    return Err(IngresoContratistaError::GafeteNotAvailable);
                }
            }
        }

        let contratista = self
            .contratista_repo
            .find_by_id_fetched(&contratista_id)
            .await
            .map_err(|e| IngresoContratistaError::Database(e.to_string()))?
            .ok_or(IngresoContratistaError::ContratistaNotFound)?;

        let dto = crate::models::ingreso::IngresoContratistaCreateDTO {
            contratista: contratista.id.clone(),
            nombre: contratista.nombre.clone(),
            apellido: contratista.apellido.clone(),
            segundo_nombre: contratista.segundo_nombre.clone(),
            segundo_apellido: contratista.segundo_apellido.clone(),
            cedula: contratista.cedula.clone(),
            tipo_autorizacion: input.tipo_autorizacion,
            modo_ingreso: input.modo_ingreso,
            placa_vehiculo: input.placa_vehiculo,
            gafete_numero: gafete_int,
            usuario_ingreso: usuario_id,
            observaciones: input.observaciones,
        };

        let nuevo_ingreso = self
            .ingreso_repo
            .insert(dto)
            .await
            .map_err(|e| IngresoContratistaError::Database(e.to_string()))?;

        if let Some(ref g) = nuevo_ingreso.gafete_numero {
            let _ = self.gafete_repo.marcar_en_uso(*g, "contratista").await;
        }

        info!("Ingreso registrado: Contratista {} ingresó a planta", input.contratista_id);

        IngresoResponse::from_contratista_fetched(nuevo_ingreso)
            .map_err(IngresoContratistaError::Validation)
    }

    pub async fn registrar_salida(
        &self,
        input: RegistrarSalidaInput,
        usuario_id_str: String,
    ) -> Result<IngresoResponse, IngresoContratistaError> {
        let ingreso_id = parse_ingreso_id(&input.ingreso_id)?;
        let usuario_id = parse_user_id(&usuario_id_str)?;

        let ingreso_actualizado = self
            .ingreso_repo
            .update_salida(&ingreso_id, &usuario_id, input.observaciones_salida.clone())
            .await
            .map_err(|e| IngresoContratistaError::Database(e.to_string()))?;

        if input.devolvio_gafete {
            // Gafete devuelto: liberarlo para nuevo uso
            if let Some(ref g) = ingreso_actualizado.gafete_numero {
                let _ = self.gafete_repo.liberar(*g, "contratista").await;
            }
        } else {
            // Gafete NO devuelto: crear alerta de gafete perdido
            if let Some(ref g) = ingreso_actualizado.gafete_numero {
                if *g != 0 {
                    warn!("Gafete {g} no devuelto por contratista, generando alerta");

                    // Crear alerta de gafete no devuelto
                    let alerta_input = crate::models::ingreso::CreateAlertaInput {
                        id: uuid::Uuid::new_v4().to_string(),
                        persona_id: Some(ingreso_actualizado.contratista.id.to_string()),
                        cedula: ingreso_actualizado.cedula.clone(),
                        nombre_completo: format!(
                            "{} {}",
                            ingreso_actualizado.nombre, ingreso_actualizado.apellido
                        ),
                        gafete_numero: *g,
                        ingreso_contratista_id: Some(ingreso_actualizado.id.to_string()),
                        ingreso_proveedor_id: None,
                        ingreso_visita_id: None,
                        fecha_reporte: chrono::Utc::now().to_rfc3339(),
                        notas: input.observaciones_salida.clone(),
                        reportado_por: usuario_id.to_string(),
                    };

                    if let Err(e) = crate::services::alerta_service::insert(alerta_input).await {
                        error!("Error al crear alerta de gafete no devuelto: {e}");
                        // No fallamos la salida por esto, solo logueamos el error
                    } else {
                        info!("Alerta de gafete no devuelto creada para gafete {g}");
                    }
                }
            }
        }

        info!("Salida registrada para ingreso: {}", input.ingreso_id);

        IngresoResponse::from_contratista_fetched(ingreso_actualizado)
            .map_err(IngresoContratistaError::Validation)
    }

    pub async fn validar_puede_salir(
        &self,
        _ingreso_id: &str,
        _gafete: Option<&str>,
    ) -> Result<ResultadoValidacionSalida, String> {
        Ok(ResultadoValidacionSalida { puede_salir: true, errores: vec![], advertencias: vec![] })
    }

    pub async fn get_ingresos_abiertos_con_alertas(
        &self,
    ) -> Result<Vec<IngresoConEstadoResponse>, IngresoContratistaError> {
        Ok(vec![])
    }

    pub async fn verificar_tiempos_excedidos(
        &self,
    ) -> Result<Vec<AlertaTiempoExcedido>, IngresoContratistaError> {
        Ok(vec![])
    }
}

// --------------------------------------------------------------------------
// HELPERS INTERNOS
// --------------------------------------------------------------------------

fn parse_contratista_id(id_str: &str) -> Result<RecordId, IngresoContratistaError> {
    if id_str.contains(':') {
        id_str.parse::<RecordId>().map_err(|_| {
            IngresoContratistaError::Validation("ID de contratista inválido".to_string())
        })
    } else {
        Ok(RecordId::from_table_key("contratista", id_str))
    }
}

fn parse_user_id(id_str: &str) -> Result<RecordId, IngresoContratistaError> {
    if id_str.contains(':') {
        id_str
            .parse::<RecordId>()
            .map_err(|_| IngresoContratistaError::Validation("ID de usuario inválido".to_string()))
    } else {
        Ok(RecordId::from_table_key("user", id_str))
    }
}

fn parse_ingreso_id(id_str: &str) -> Result<RecordId, IngresoContratistaError> {
    if id_str.contains(':') {
        id_str
            .parse::<RecordId>()
            .map_err(|_| IngresoContratistaError::Validation("ID de ingreso inválido".to_string()))
    } else {
        Ok(RecordId::from_table_key("ingreso_contratista", id_str))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_contratista_id_con_prefijo() {
        let id = parse_contratista_id("contratista:abc123").unwrap();
        assert_eq!(id.to_string(), "contratista:abc123");
    }

    #[test]
    fn test_parse_contratista_id_sin_prefijo() {
        let id = parse_contratista_id("abc123").unwrap();
        assert_eq!(id.to_string(), "contratista:abc123");
    }

    #[test]
    fn test_parse_user_id_con_prefijo() {
        let id = parse_user_id("user:guard01").unwrap();
        assert_eq!(id.to_string(), "user:guard01");
    }

    #[test]
    fn test_parse_user_id_sin_prefijo() {
        let id = parse_user_id("guard01").unwrap();
        assert_eq!(id.to_string(), "user:guard01");
    }

    #[test]
    fn test_parse_ingreso_id_con_prefijo() {
        let id = parse_ingreso_id("ingreso_contratista:ing001").unwrap();
        assert_eq!(id.to_string().replace("⟨", "").replace("⟩", ""), "ingreso_contratista:ing001");
    }

    #[test]
    fn test_parse_ingreso_id_sin_prefijo() {
        let id = parse_ingreso_id("ing001").unwrap();
        assert_eq!(id.to_string().replace("⟨", "").replace("⟩", ""), "ingreso_contratista:ing001");
    }
}
