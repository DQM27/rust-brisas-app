// src/services/ingreso_visita_service.rs
use crate::db::surrealdb_ingreso_visita_queries as db;
use crate::domain::errors::IngresoVisitaError;
// use crate::domain::ingreso_visita::{ // Legacy models
//     CreateIngresoVisitaFullInput, CreateIngresoVisitaInput, IngresoVisita, IngresoVisitaPopulated,
//     ValidacionIngresoVisitaResponse,
// };
use crate::domain::motor_validacion::{self as motor, ContextoIngreso};
use crate::models::ingreso::CreateIngresoVisitaInput;
use crate::models::visitante::CreateVisitanteInput;
use crate::services::{gafete_service, lista_negra_service, visitante_service};

// ==========================================
// FUNCIONES DE SERVICIO REALES
// ==========================================

pub async fn registrar_ingreso(
    input_domain: crate::domain::ingreso_visita::CreateIngresoVisitaInput,
) -> Result<crate::domain::ingreso_visita::IngresoVisita, IngresoVisitaError> {
    // 1. Obtener datos del visitante para completar input
    let v = visitante_service::get_visitante_by_id(&input_domain.visitante_id)
        .await
        .map_err(|e| IngresoVisitaError::Validation(e.to_string()))?
        .ok_or(IngresoVisitaError::NotFound)?; // Visitante debe existir

    // 2. Mapear input legacy a nuevo input
    // Ojo: models::ingreso::CreateIngresoVisitaInput requiere nombre, apellido, etc. que sacamos de visitante
    let input = CreateIngresoVisitaInput {
        cedula: v.cedula.clone(),
        nombre: v.nombre.clone(),
        apellido: v.apellido.clone(),
        anfitrion: input_domain.anfitrion,
        area_visitada: input_domain.area_visitada,
        motivo_visita: input_domain.motivo,
        tipo_autorizacion: "visita".to_string(), // Default, o ajustar si input legacy trae algo
        modo_ingreso: "caminando".to_string(), // Default, legacy no lo pide explícito aquí a veces
        vehiculo_placa: None,                  // Visita a pie por default en este flujo
        gafete_numero: input_domain.gafete.clone(),
        observaciones: input_domain.observaciones,
        usuario_ingreso_id: input_domain.usuario_ingreso_id,
    };

    // 3. Validaciones
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
        .map_err(|e| IngresoVisitaError::Database(sqlx::Error::Protocol(e.to_string())))?;
    if abierto.is_some() {
        return Err(IngresoVisitaError::Validation("Ya tiene ingreso activo".to_string()));
    }

    // 4. Crear Ingreso
    let nuevo_ingreso = db::insert(input)
        .await
        .map_err(|e| IngresoVisitaError::Database(sqlx::Error::Protocol(e.to_string())))?
        .ok_or(IngresoVisitaError::Database(sqlx::Error::Protocol(
            "Error creando ingreso".to_string(),
        )))?;

    // 5. Marcar Gafete
    if let Some(ref g) = nuevo_ingreso.gafete_numero {
        if g != "S/G" {
            let _ = gafete_service::marcar_en_uso(g, "visita").await;
        }
    }

    // 6. Retorno Legacy (Mapeo manual)
    let esta_adentro = nuevo_ingreso.fecha_hora_salida.is_none();
    Ok(crate::domain::ingreso_visita::IngresoVisita {
        id: nuevo_ingreso.id,
        visitante_id: v.id, // ID original del visitante
        anfitrion: nuevo_ingreso.anfitrion.unwrap_or_default(), // Recuperar de DB o input
        area_visitada: nuevo_ingreso.area_visitada.unwrap_or_default(),
        motivo: nuevo_ingreso.motivo.unwrap_or_default(),
        gafete: nuevo_ingreso.gafete_numero,
        fecha_ingreso: nuevo_ingreso.fecha_hora_ingreso,
        fecha_salida: nuevo_ingreso.fecha_hora_salida,
        estado: if esta_adentro { "activo".to_string() } else { "finalizado".to_string() },
        observaciones: nuevo_ingreso.observaciones,
        usuario_ingreso_id: nuevo_ingreso.usuario_ingreso_id,
        usuario_salida_id: nuevo_ingreso.usuario_salida_id,
        cita_id: None, // Cita ID se pierde si no está en modelo nuevo, TODO: agregar a Ingreso model si es crítico
        created_at: nuevo_ingreso.created_at,
        updated_at: nuevo_ingreso.updated_at,
    })
}

pub async fn registrar_ingreso_full(
    input: crate::domain::ingreso_visita::CreateIngresoVisitaFullInput,
) -> Result<crate::domain::ingreso_visita::IngresoVisita, IngresoVisitaError> {
    // Busca o crea el visitante y luego llama a registrar_ingreso
    let v_id = match visitante_service::get_visitante_by_cedula(&input.cedula)
        .await
        .map_err(|e| IngresoVisitaError::Validation(e.to_string()))?
    {
        Some(v) => v.id,
        None => {
            let c_i = CreateVisitanteInput {
                cedula: input.cedula.clone(), // Clone necesario si input se mueve
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
                .id // Option<Visitante> -> Visitante -> id
        }
    };
    registrar_ingreso(crate::domain::ingreso_visita::CreateIngresoVisitaInput {
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
    id: String,
    usuario_id: String,
    devolvio_gafete: bool,
    observaciones: Option<String>,
) -> Result<(), IngresoVisitaError> {
    let actualizado = db::update_salida(&id, &usuario_id, observaciones)
        .await
        .map_err(|e| IngresoVisitaError::Database(sqlx::Error::Protocol(e.to_string())))?
        .ok_or(IngresoVisitaError::NotFound)?;

    if devolvio_gafete {
        if let Some(ref g) = actualizado.gafete_numero {
            if g != "S/G" {
                let _ = gafete_service::liberar_gafete(g, "visita").await;
            }
        }
    }
    Ok(())
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
        .map_err(|e| IngresoVisitaError::Database(sqlx::Error::Protocol(e.to_string())))?;

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
