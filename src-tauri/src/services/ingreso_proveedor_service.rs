// src/services/ingreso_proveedor_service.rs
use crate::db::surrealdb_ingreso_proveedor_queries as db;
use crate::domain::errors::IngresoProveedorError;
use crate::models::ingreso::{
    CreateIngresoProveedorInput, IngresoProveedorCreateDTO, IngresoResponse,
};
use crate::services::{gafete_service, lista_negra_service, proveedor_service};
use surrealdb::RecordId;

pub async fn registrar_ingreso(
    input: CreateIngresoProveedorInput,
    usuario_id_str: String,
) -> Result<IngresoResponse, IngresoProveedorError> {
    let proveedor_id = if input.proveedor_id.contains(':') {
        input.proveedor_id.parse::<RecordId>().map_err(|_| {
            IngresoProveedorError::Validation("ID de proveedor inválido".to_string())
        })?
    } else {
        RecordId::from_table_key("proveedor", &input.proveedor_id)
    };

    let usuario_id = if usuario_id_str.contains(':') {
        usuario_id_str
            .parse::<RecordId>()
            .map_err(|_| IngresoProveedorError::Validation("ID de usuario inválido".to_string()))?
    } else {
        RecordId::from_table_key("user", &usuario_id_str)
    };

    if let Some(ref g) = input.gafete_numero {
        if g != "S/G" && !g.is_empty() {
            let disp = gafete_service::is_gafete_disponible(g, "proveedor")
                .await
                .map_err(|e| IngresoProveedorError::Validation(e))?;
            if !disp {
                return Err(IngresoProveedorError::Validation("Gafete no disponible".to_string()));
            }
        }
    }

    let check =
        lista_negra_service::check_is_blocked(input.cedula.clone()).await.map_err(|_| {
            IngresoProveedorError::Validation("Error verificando lista negra".to_string())
        })?;

    if check.is_blocked {
        return Err(IngresoProveedorError::Validation("Proveedor en lista negra".to_string()));
    }

    let abierto = db::find_ingreso_abierto_by_proveedor(&proveedor_id)
        .await
        .map_err(|e| IngresoProveedorError::Database(e.to_string()))?;

    if abierto.is_some() {
        return Err(IngresoProveedorError::Validation("Ya tiene un ingreso activo".to_string()));
    }

    let dto = IngresoProveedorCreateDTO {
        proveedor: proveedor_id,
        nombre: input.nombre,
        apellido: input.apellido,
        cedula: input.cedula,
        area_visitada: input.area_visitada,
        motivo: input.motivo,
        modo_ingreso: input.modo_ingreso,
        placa_vehiculo: input.placa_vehiculo,
        gafete_numero: input.gafete_numero,
        usuario_ingreso: usuario_id,
        observaciones: input.observaciones,
    };

    let nuevo_ingreso =
        db::insert(dto).await.map_err(|e| IngresoProveedorError::Database(e.to_string()))?;

    if let Some(ref g) = nuevo_ingreso.gafete_numero {
        if g != "S/G" && !g.is_empty() {
            let _ = gafete_service::marcar_en_uso(g, "proveedor").await;
        }
    }

    Ok(IngresoResponse::from_proveedor_fetched(nuevo_ingreso))
}

pub async fn registrar_salida(
    ingreso_id_str: String,
    usuario_id_str: String,
    observaciones: Option<String>,
    devolvio_gafete: bool,
) -> Result<IngresoResponse, IngresoProveedorError> {
    let ingreso_id = if ingreso_id_str.contains(':') {
        ingreso_id_str
            .parse::<RecordId>()
            .map_err(|_| IngresoProveedorError::Validation("ID inválido".to_string()))?
    } else {
        RecordId::from_table_key("ingreso_proveedor", &ingreso_id_str)
    };

    let usuario_id = if usuario_id_str.contains(':') {
        usuario_id_str
            .parse::<RecordId>()
            .map_err(|_| IngresoProveedorError::Validation("ID inválido".to_string()))?
    } else {
        RecordId::from_table_key("user", &usuario_id_str)
    };

    let actualizado = db::update_salida(&ingreso_id, &usuario_id, observaciones)
        .await
        .map_err(|e| IngresoProveedorError::Database(e.to_string()))?;

    if devolvio_gafete {
        if let Some(g) = &actualizado.gafete_numero {
            if g != "S/G" && !g.is_empty() {
                let _ = gafete_service::liberar_gafete(g, "proveedor").await;
            }
        }
    }

    Ok(IngresoResponse::from_proveedor_fetched(actualizado))
}

pub async fn get_activos() -> Result<Vec<IngresoResponse>, IngresoProveedorError> {
    Ok(vec![])
}

pub async fn get_historial() -> Result<Vec<IngresoResponse>, IngresoProveedorError> {
    Ok(vec![])
}

pub async fn search_proveedores(_q: &str) -> Result<Vec<serde_json::Value>, IngresoProveedorError> {
    Ok(vec![])
}

pub async fn validar_ingreso(
    proveedor_id_str: String,
) -> Result<serde_json::Value, IngresoProveedorError> {
    let p_id = if proveedor_id_str.contains(':') {
        proveedor_id_str.parse::<RecordId>().unwrap()
    } else {
        RecordId::from_table_key("proveedor", &proveedor_id_str)
    };

    let p = proveedor_service::get_proveedor_by_id(&p_id.to_string())
        .await
        .map_err(|e| IngresoProveedorError::Validation(e.to_string()))?;

    Ok(serde_json::json!({
        "puedeIngresar": true,
        "proveedor": p
    }))
}
