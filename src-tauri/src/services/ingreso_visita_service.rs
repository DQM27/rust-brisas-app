// src/services/ingreso_visita_service.rs
use crate::domain::errors::IngresoVisitaError;
use crate::domain::ingreso_visita::{
    CreateIngresoVisitaFullInput, CreateIngresoVisitaInput, IngresoVisita, IngresoVisitaPopulated,
    ValidacionIngresoVisitaResponse,
};
use crate::domain::motor_validacion::{self as motor, ContextoIngreso};
use crate::models::visitante::CreateVisitanteInput;
use crate::services::{gafete_service, lista_negra_service, visitante_service};

pub async fn registrar_ingreso(
    input: CreateIngresoVisitaInput,
) -> Result<IngresoVisita, IngresoVisitaError> {
    let val = validar_ingreso(&input.visitante_id).await?;
    if !val.puede_ingresar {
        return Err(IngresoVisitaError::Validation(val.motivo_rechazo.unwrap_or_default()));
    }
    if let Some(ref g) = input.gafete {
        if g != "S/G" {
            let disp = gafete_service::is_gafete_disponible(g, "visita")
                .await
                .map_err(|e| IngresoVisitaError::Validation(e.to_string()))?;
            if !disp {
                return Err(IngresoVisitaError::Validation("Gafete no disponible".to_string()));
            }
        }
    }
    Err(IngresoVisitaError::Database(sqlx::Error::Protocol("no implementado".to_string())))
}

pub async fn registrar_ingreso_full(
    input: CreateIngresoVisitaFullInput,
) -> Result<IngresoVisita, IngresoVisitaError> {
    let v_id = match visitante_service::get_visitante_by_cedula(&input.cedula)
        .await
        .map_err(|e| IngresoVisitaError::Validation(e.to_string()))?
    {
        Some(v) => v.id,
        None => {
            let c_i = CreateVisitanteInput {
                cedula: input.cedula,
                nombre: input.nombre,
                apellido: input.apellido,
                segundo_nombre: None,
                segundo_apellido: None,
                empresa: input.empresa,
                empresa_id: None,
                has_vehicle: false,
            };
            visitante_service::create_visitante(c_i)
                .await
                .map_err(|e| IngresoVisitaError::Validation(e.to_string()))?
                .id
        }
    };
    registrar_ingreso(CreateIngresoVisitaInput {
        visitante_id: v_id,
        cita_id: input.cita_id,
        anfitrion: input.anfitrion,
        area_visitada: input.area_visitada,
        motivo: input.motivo,
        gafete: input.gafete,
        observaciones: input.observaciones,
        usuario_ingreso_id: input.usuario_ingreso_id,
    })
    .await
}

pub async fn registrar_salida(
    _id: String,
    _u: String,
    _d: bool,
    _o: Option<String>,
) -> Result<(), IngresoVisitaError> {
    Err(IngresoVisitaError::Database(sqlx::Error::Protocol("no implementado".to_string())))
}

pub async fn get_activos() -> Result<Vec<IngresoVisitaPopulated>, IngresoVisitaError> {
    Ok(vec![])
}
pub async fn get_historial() -> Result<Vec<IngresoVisitaPopulated>, IngresoVisitaError> {
    Ok(vec![])
}

pub async fn validar_ingreso(
    visitante_id: &str,
) -> Result<ValidacionIngresoVisitaResponse, IngresoVisitaError> {
    let v = visitante_service::get_visitante_by_id(visitante_id)
        .await
        .map_err(|e| IngresoVisitaError::Validation(e.to_string()))?
        .ok_or(IngresoVisitaError::NotFound)?;
    let b = lista_negra_service::check_is_blocked(v.cedula.clone()).await.unwrap_or_default();
    let ctx = ContextoIngreso::new_visita(
        v.cedula.clone(),
        format!("{} {}", v.nombre, v.apellido),
        None,
        b.is_blocked,
        b.nivel_severidad.clone(),
        false,
        0,
    );
    let res = motor::validar_ingreso(&ctx);
    Ok(ValidacionIngresoVisitaResponse {
        puede_ingresar: res.puede_ingresar,
        motivo_rechazo: res.mensaje_bloqueo(),
        alertas: res.alertas,
        visitante: None,
        tiene_ingreso_abierto: false,
        ingreso_abierto: None,
    })
}
