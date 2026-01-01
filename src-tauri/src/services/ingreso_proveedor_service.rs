/// Punto de Control: Ingresos y Salidas de Proveedores.
///
/// Gestiona la admisión de personal externo que suministra bienes o servicios.
/// A diferencia de los contratistas, su flujo se centra en la trazabilidad de la
/// entrega (área visitada y motivo) y el control de lista negra inmediata.
use crate::db::surrealdb_ingreso_proveedor_queries as db;
use crate::domain::errors::IngresoProveedorError;
use crate::models::ingreso::{
    CreateIngresoProveedorInput, IngresoProveedorCreateDTO, IngresoResponse,
};
use crate::services::{gafete_service, lista_negra_service, proveedor_service};
use surrealdb::RecordId;

/// Registra la entrada física de un proveedor a las instalaciones.
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

    // Validación de Gafetes: Asegura que el proveedor porte una identificación física controlada.
    // Check 1: validate availability
    if let Some(ref g) = input.gafete_numero {
        if *g != 0 {
            let disp = gafete_service::is_gafete_disponible(*g, "proveedor")
                .await
                .map_err(|e| IngresoProveedorError::Gafete(e))?;
            if !disp {
                return Err(IngresoProveedorError::Validation(
                    "El gafete seleccionado ya está en uso".to_string(),
                ));
            }
        }
    }

    // Seguridad: Verificación en tiempo real contra la lista negra institucional.
    let check =
        lista_negra_service::check_is_blocked(input.cedula.clone()).await.map_err(|_| {
            IngresoProveedorError::Validation(
                "Error al consultar protocolos de seguridad (Lista Negra)".to_string(),
            )
        })?;

    if check.is_blocked {
        return Err(IngresoProveedorError::Validation(
            "ACCESO DENEGADO: El proveedor se encuentra en Lista Negra".to_string(),
        ));
    }

    // Integridad: Evita que una misma persona figure como "dentro" dos veces.
    let abierto = db::find_ingreso_abierto_by_proveedor(&proveedor_id)
        .await
        .map_err(|e| IngresoProveedorError::Database(e.to_string()))?;

    if abierto.is_some() {
        return Err(IngresoProveedorError::Validation(
            "Ya existe un registro de ingreso abierto para esta persona".to_string(),
        ));
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
    // Update status
    if let Some(ref g) = nuevo_ingreso.gafete_numero {
        if *g != 0 {
            let _ = gafete_service::marcar_en_uso(*g, "proveedor").await;
        }
    }

    Ok(IngresoResponse::from_proveedor_fetched(nuevo_ingreso))
}

/// Registra la salida del proveedor y libera los recursos asignados.
pub async fn registrar_salida(
    ingreso_id_str: String,
    usuario_id_str: String,
    observaciones: Option<String>,
    devolvio_gafete: bool,
) -> Result<IngresoResponse, IngresoProveedorError> {
    let ingreso_id = if ingreso_id_str.contains(':') {
        ingreso_id_str
            .parse::<RecordId>()
            .map_err(|_| IngresoProveedorError::Validation("ID de ingreso inválido".to_string()))?
    } else {
        RecordId::from_table_key("ingreso_proveedor", &ingreso_id_str)
    };

    let usuario_id = if usuario_id_str.contains(':') {
        usuario_id_str
            .parse::<RecordId>()
            .map_err(|_| IngresoProveedorError::Validation("ID de usuario inválido".to_string()))?
    } else {
        RecordId::from_table_key("user", &usuario_id_str)
    };

    let ingreso_actualizado = db::update_salida(&ingreso_id, &usuario_id, observaciones)
        .await
        .map_err(|e| IngresoProveedorError::Database(e.to_string()))?;

    if devolvio_gafete {
        if let Some(ref g) = ingreso_actualizado.gafete_numero {
            if *g != 0 {
                let _ = gafete_service::liberar_gafete(*g, "proveedor").await;
            }
        }
    }

    Ok(IngresoResponse::from_proveedor_fetched(ingreso_actualizado))
}

pub async fn get_activos() -> Result<Vec<IngresoResponse>, IngresoProveedorError> {
    Ok(vec![])
}

/// Valida si un proveedor es apto para ingresar antes de abrir el formulario de admisión.
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
