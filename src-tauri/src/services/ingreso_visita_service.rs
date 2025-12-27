// src/services/ingreso_visita_service.rs
use crate::db::surrealdb_ingreso_visita_queries as db;
use crate::domain::errors::IngresoVisitaError;
// use crate::domain::ingreso_visita::{ // Legacy models
//     CreateIngresoVisitaFullInput, CreateIngresoVisitaInput, IngresoVisita, IngresoVisitaPopulated,
//     ValidacionIngresoVisitaResponse,
// };
use crate::domain::motor_validacion::{self as motor, ContextoIngreso};
use crate::models::ingreso::{CreateIngresoVisitaInput, IngresoResponse};
use crate::models::visitante::CreateVisitanteInput;
use crate::services::{gafete_service, lista_negra_service, visitante_service};
use surrealdb::RecordId;

// ==========================================
// FUNCIONES DE SERVICIO REALES
// ==========================================
// Función principal
pub async fn registrar_ingreso(
    input: CreateIngresoVisitaInput,
    usuario_id_str: String,
) -> Result<IngresoResponse, IngresoVisitaError> {
    let usuario_id = if usuario_id_str.contains(':') {
        usuario_id_str
            .parse::<RecordId>()
            .map_err(|_| IngresoVisitaError::Validation("ID de usuario inválido".to_string()))?
    } else {
        RecordId::from_table_key("user", &usuario_id_str)
    };

    // 1. Validaciones
    // Gafete
    if let Some(ref g) = input.gafete_numero {
        if g != "S/G" {
            let disp = gafete_service::is_gafete_disponible(g, "visita")
                .await
                .map_err(|e| IngresoVisitaError::Validation(e.to_string()))?;
            if !disp {
                return Err(IngresoVisitaError::Validation("Gafete no disponible".to_string()));
            }
        }
    }

    // Lista Negra
    let check =
        lista_negra_service::check_is_blocked(input.cedula.clone()).await.unwrap_or_default();
    if check.is_blocked {
        return Err(IngresoVisitaError::Validation("Visitante bloqueado".to_string()));
    }

    // Ingreso Abierto
    let abierto = db::find_ingreso_abierto_by_cedula(&input.cedula)
        .await
        .map_err(|e| IngresoVisitaError::Database(e.to_string()))?;
    if abierto.is_some() {
        return Err(IngresoVisitaError::Validation("Ya tiene ingreso activo".to_string()));
    }

    // 4. Construct DTO
    let dto = crate::models::ingreso::IngresoCreateDTO {
        contratista: None,
        cedula: input.cedula,
        nombre: input.nombre,
        apellido: input.apellido,
        empresa_nombre: "".to_string(), // Default, or derive from visitor if needed
        tipo_ingreso: "visita".to_string(),
        tipo_autorizacion: input.tipo_autorizacion,
        modo_ingreso: input.modo_ingreso,
        vehiculo: None, // Logic for vehicle needed if any (visita pie default here?)
        placa_temporal: input.vehiculo_placa,
        gafete_numero: input.gafete_numero,
        gafete_tipo: Some("visita".to_string()),
        fecha_hora_ingreso: surrealdb::Datetime::from(chrono::Utc::now()),
        usuario_ingreso: usuario_id,
        praind_vigente_al_ingreso: None,
        estado_contratista_al_ingreso: None,
        observaciones: input.observaciones,
        anfitrion: Some(input.anfitrion),
        area_visitada: Some(input.area_visitada),
        motivo: Some(input.motivo_visita),
    };

    // 5. Crear Ingreso
    let nuevo_ingreso =
        db::insert(dto).await.map_err(|e| IngresoVisitaError::Database(e.to_string()))?;

    // 6. Marcar Gafete
    if let Some(ref g) = nuevo_ingreso.gafete_numero {
        if g != "S/G" {
            let _ = gafete_service::marcar_en_uso(g, "visita").await;
        }
    }

    IngresoResponse::from_fetched(nuevo_ingreso).map_err(|e| IngresoVisitaError::Validation(e))
}

pub async fn registrar_ingreso_full(
    input: crate::domain::ingreso_visita::CreateIngresoVisitaFullInput,
    usuario_id: String,
) -> Result<IngresoResponse, IngresoVisitaError> {
    // Busca o crea el visitante y luego llama a registrar_ingreso
    let v = match visitante_service::get_visitante_by_cedula(&input.cedula)
        .await
        .map_err(|e| IngresoVisitaError::Validation(e.to_string()))?
    {
        Some(v) => v,
        None => {
            let c_i = CreateVisitanteInput {
                cedula: input.cedula.clone(),
                nombre: input.nombre.clone(),
                apellido: input.apellido.clone(),
                segundo_nombre: None,
                segundo_apellido: None,
                empresa: input.empresa.clone(),
                empresa_id: None,
                has_vehicle: false,
            };
            visitante_service::create_visitante(c_i)
                .await
                .map_err(|e| IngresoVisitaError::Validation(e.to_string()))?
        }
    };

    let ingreso_input = CreateIngresoVisitaInput {
        cedula: v.cedula,
        nombre: v.nombre,
        apellido: v.apellido,
        anfitrion: input.anfitrion,
        area_visitada: input.area_visitada,
        motivo_visita: input.motivo,
        tipo_autorizacion: "visita".to_string(),
        modo_ingreso: "caminando".to_string(),
        vehiculo_placa: None,
        gafete_numero: input.gafete,
        observaciones: input.observaciones,
        usuario_ingreso_id: usuario_id.clone(),
    };

    registrar_ingreso(ingreso_input, usuario_id).await
}

pub async fn registrar_salida(
    id_str: String,
    usuario_id_str: String,
    devolvio_gafete: bool,
    observaciones: Option<String>,
) -> Result<IngresoResponse, IngresoVisitaError> {
    let ingreso_id = if id_str.contains(':') {
        id_str
            .parse::<RecordId>()
            .map_err(|_| IngresoVisitaError::Validation("ID de ingreso inválido".to_string()))?
    } else {
        RecordId::from_table_key("ingreso", &id_str)
    };

    let usuario_id = if usuario_id_str.contains(':') {
        usuario_id_str
            .parse::<RecordId>()
            .map_err(|_| IngresoVisitaError::Validation("ID de usuario inválido".to_string()))?
    } else {
        RecordId::from_table_key("user", &usuario_id_str)
    };

    let actualizado = db::update_salida(&ingreso_id, &usuario_id, observaciones)
        .await
        .map_err(|e| IngresoVisitaError::Database(e.to_string()))?;

    if devolvio_gafete {
        if let Some(ref g) = actualizado.gafete_numero {
            if g != "S/G" {
                let _ = gafete_service::liberar_gafete(g, "visita").await;
            }
        }
    }

    IngresoResponse::from_fetched(actualizado).map_err(|e| IngresoVisitaError::Validation(e))
}

pub async fn get_activos(
) -> Result<Vec<crate::domain::ingreso_visita::IngresoVisitaPopulated>, IngresoVisitaError> {
    Ok(vec![])
}
pub async fn get_historial(
) -> Result<Vec<crate::domain::ingreso_visita::IngresoVisitaPopulated>, IngresoVisitaError> {
    Ok(vec![])
}

pub async fn validar_ingreso(
    visitante_id: &str,
) -> Result<crate::domain::ingreso_visita::ValidacionIngresoVisitaResponse, IngresoVisitaError> {
    let v = visitante_service::get_visitante_by_id(visitante_id)
        .await
        .map_err(|e| IngresoVisitaError::Validation(e.to_string()))?
        .ok_or(IngresoVisitaError::NotFound)?;

    let b = lista_negra_service::check_is_blocked(v.cedula.clone()).await.unwrap_or_default();

    // Check ingreso activo real
    let abierto = db::find_ingreso_abierto_by_cedula(&v.cedula)
        .await
        .map_err(|e| IngresoVisitaError::Database(e.to_string()))?;

    if let Some(_) = abierto {
        return Ok(crate::domain::ingreso_visita::ValidacionIngresoVisitaResponse {
            puede_ingresar: false,
            motivo_rechazo: Some("Ya tiene ingreso activo".to_string()),
            alertas: vec![],
            visitante: None, // TODO llenar datos
            tiene_ingreso_abierto: true,
            ingreso_abierto: None, // TODO llenar si necesario
        });
    }

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
    Ok(crate::domain::ingreso_visita::ValidacionIngresoVisitaResponse {
        puede_ingresar: res.puede_ingresar,
        motivo_rechazo: res.mensaje_bloqueo(),
        alertas: res.alertas,
        visitante: Some(serde_json::json!(v)),
        tiene_ingreso_abierto: false,
        ingreso_abierto: None,
    })
}
