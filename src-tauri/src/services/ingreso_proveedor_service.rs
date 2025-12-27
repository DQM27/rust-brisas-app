// src/services/ingreso_proveedor_service.rs
use crate::db::surrealdb_ingreso_proveedor_queries as db;
use crate::domain::errors::IngresoProveedorError;
use crate::models::ingreso::{CreateIngresoProveedorInput, IngresoResponse};
use crate::services::{gafete_service, lista_negra_service};
use surrealdb::sql::Thing;

// Función principal
pub async fn registrar_ingreso(
    input: CreateIngresoProveedorInput,
    usuario_id_str: String,
) -> Result<IngresoResponse, IngresoProveedorError> {
    let _empresa_id = Thing::try_from(input.empresa_id.clone())
        .map_err(|_| IngresoProveedorError::Validation("ID de empresa inválido".to_string()))?;

    let usuario_id = Thing::try_from(usuario_id_str)
        .map_err(|_| IngresoProveedorError::Validation("ID de usuario inválido".to_string()))?;

    // 1. Validar Gafete
    if let Some(ref g) = input.gafete_numero {
        let disp = gafete_service::is_gafete_disponible(g, "proveedor")
            .await
            .map_err(|e| IngresoProveedorError::Validation(e))?;
        if !disp {
            return Err(IngresoProveedorError::Validation("Gafete no disponible".to_string()));
        }
    }

    // 2. Verificar Lista Negra
    let check =
        lista_negra_service::check_is_blocked(input.cedula.clone()).await.map_err(|_| {
            IngresoProveedorError::Validation("Error verificando lista negra".to_string())
        })?;

    if check.is_blocked {
        return Err(IngresoProveedorError::Validation("Proveedor en lista negra".to_string()));
    }

    // 3. Verificar Ingreso Abierto
    let abierto = db::find_ingreso_abierto_by_cedula(&input.cedula)
        .await
        .map_err(|e| IngresoProveedorError::Database(e.to_string()))?;

    if abierto.is_some() {
        return Err(IngresoProveedorError::Validation("Ya tiene un ingreso activo".to_string()));
    }

    // 4. Construct DTO
    let dto = crate::models::ingreso::IngresoCreateDTO {
        contratista: None,
        cedula: input.cedula,
        nombre: input.nombre,
        apellido: input.apellido,
        empresa_nombre: "".to_string(), // In reality fetch from empresa_id
        tipo_ingreso: "proveedor".to_string(),
        tipo_autorizacion: input.tipo_autorizacion,
        modo_ingreso: input.modo_ingreso,
        vehiculo: None, // Logic for vehicle needed if any
        placa_temporal: input.vehiculo_placa,
        gafete_numero: input.gafete_numero,
        gafete_tipo: Some("proveedor".to_string()),
        fecha_hora_ingreso: chrono::Utc::now(),
        usuario_ingreso: usuario_id,
        praind_vigente_al_ingreso: None,
        estado_contratista_al_ingreso: None,
        observaciones: input.observaciones,
        anfitrion: None,
        area_visitada: Some(input.area_visitada),
        motivo: Some(input.motivo),
    };

    // 5. Crear
    let nuevo_ingreso =
        db::insert(dto).await.map_err(|e| IngresoProveedorError::Database(e.to_string()))?;

    // 6. Marcar gafete
    if let Some(ref g) = nuevo_ingreso.gafete_numero {
        let _ = gafete_service::marcar_en_uso(g, "proveedor").await;
    }

    IngresoResponse::try_from(nuevo_ingreso).map_err(|e| IngresoProveedorError::Validation(e))
}

pub async fn registrar_salida(
    id_str: String,
    usuario_id_str: String,
    observaciones: Option<String>,
    devolvio_gafete: bool,
) -> Result<IngresoResponse, IngresoProveedorError> {
    let ingreso_id = Thing::try_from(id_str)
        .map_err(|_| IngresoProveedorError::Validation("ID de ingreso inválido".to_string()))?;

    let usuario_id = Thing::try_from(usuario_id_str)
        .map_err(|_| IngresoProveedorError::Validation("ID de usuario inválido".to_string()))?;

    let actualizado = db::update_salida(&ingreso_id, &usuario_id, observaciones)
        .await
        .map_err(|e| IngresoProveedorError::Database(e.to_string()))?;

    if devolvio_gafete {
        if let Some(g) = &actualizado.gafete_numero {
            let _ = gafete_service::liberar_gafete(g, "proveedor").await;
        }
    }

    IngresoResponse::try_from(actualizado).map_err(|e| IngresoProveedorError::Validation(e))
}

pub async fn get_activos() -> Result<Vec<IngresoResponse>, IngresoProveedorError> {
    let ingresos =
        db::find_activos().await.map_err(|e| IngresoProveedorError::Database(e.to_string()))?;

    let mut responses = Vec::new();
    for i in ingresos {
        if let Ok(res) = IngresoResponse::try_from(i) {
            responses.push(res);
        }
    }
    Ok(responses)
}

pub async fn get_historial() -> Result<Vec<IngresoResponse>, IngresoProveedorError> {
    let ingresos =
        db::find_historial().await.map_err(|e| IngresoProveedorError::Database(e.to_string()))?;

    let mut responses = Vec::new();
    for i in ingresos {
        if let Ok(res) = IngresoResponse::try_from(i) {
            responses.push(res);
        }
    }
    Ok(responses)
}

pub async fn search_proveedores(_q: &str) -> Result<Vec<serde_json::Value>, IngresoProveedorError> {
    Ok(vec![])
}

pub async fn validar_ingreso(
    _proveedor_id: String,
) -> Result<serde_json::Value, IngresoProveedorError> {
    Ok(serde_json::json!({"puedeIngresar":true}))
}
