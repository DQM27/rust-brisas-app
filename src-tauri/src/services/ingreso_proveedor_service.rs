// src/services/ingreso_proveedor_service.rs
use crate::db::surrealdb_ingreso_proveedor_queries as db;
use crate::domain::errors::IngresoProveedorError;
use crate::models::ingreso::CreateIngresoProveedorInput;
use crate::services::{gafete_service, lista_negra_service};
use serde_json::json;

// Helper para convertir Ingreso (Surreal) -> IngresoProveedor (Legacy Domain)
// Esto es temporal mientras se migran los comandos y el frontend.
#[derive(serde::Serialize)]
pub struct IngresoProveedorShim {
    pub id: String,
    pub nombre: String,
    pub apellido: String,
    pub empresa: String,
    pub fecha_ingreso: String,
    pub gafete: Option<String>,
}

// Función principal
pub async fn registrar_ingreso(
    input_domain: crate::domain::ingreso_proveedor::CreateIngresoProveedorInput,
) -> Result<crate::domain::ingreso_proveedor::IngresoProveedor, IngresoProveedorError> {
    // Mapear input legacy a input nuevo
    let input = CreateIngresoProveedorInput {
        cedula: input_domain.cedula,
        nombre: input_domain.nombre,
        apellido: input_domain.apellido,
        empresa_id: input_domain.empresa_id,
        area_visitada: input_domain.area_visitada,
        motivo: input_domain.motivo,
        tipo_autorizacion: input_domain.tipo_autorizacion,
        modo_ingreso: input_domain.modo_ingreso,
        vehiculo_placa: input_domain.placa_vehiculo,
        gafete_numero: input_domain.gafete,
        observaciones: input_domain.observaciones,
        usuario_ingreso_id: input_domain.usuario_ingreso_id,
    };

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

    // 4. Datos extras (simulados o consultados)
    let proveedor_data = json!({
        "nombre": "Empresa Externa" // Placeholder
    });

    // 5. Crear
    let nuevo_ingreso = db::insert(input, &proveedor_data)
        .await
        .map_err(|e| IngresoProveedorError::Database(e.to_string()))?
        .ok_or(IngresoProveedorError::Database("Error creando ingreso".to_string()))?;

    // 6. Marcar gafete
    if let Some(ref g) = nuevo_ingreso.gafete_numero {
        let _ = gafete_service::marcar_en_uso(g, "proveedor").await;
    }

    let esta_adentro = nuevo_ingreso.fecha_hora_salida.is_none();

    Ok(crate::domain::ingreso_proveedor::IngresoProveedor {
        id: nuevo_ingreso.id,
        cedula: nuevo_ingreso.cedula,
        nombre: nuevo_ingreso.nombre,
        apellido: nuevo_ingreso.apellido,
        proveedor_id: None,
        empresa_id: nuevo_ingreso.empresa_nombre.clone(),
        empresa_nombre: nuevo_ingreso.empresa_nombre,
        area_visitada: nuevo_ingreso.area_visitada.unwrap_or_default(),
        motivo: nuevo_ingreso.motivo.unwrap_or_default(),
        tipo_autorizacion: Some(nuevo_ingreso.tipo_autorizacion),
        modo_ingreso: Some(nuevo_ingreso.modo_ingreso),
        placa_vehiculo: nuevo_ingreso.placa_temporal,
        gafete: nuevo_ingreso.gafete_numero,
        fecha_ingreso: nuevo_ingreso.fecha_hora_ingreso,
        fecha_salida: nuevo_ingreso.fecha_hora_salida,
        estado: if esta_adentro { "activo".to_string() } else { "finalizado".to_string() },
        observaciones: nuevo_ingreso.observaciones,
        usuario_ingreso_id: nuevo_ingreso.usuario_ingreso_id,
        usuario_salida_id: nuevo_ingreso.usuario_salida_id,
        usuario_ingreso_nombre: String::new(),
        usuario_salida_nombre: String::new(),
        created_at: nuevo_ingreso.created_at,
        updated_at: nuevo_ingreso.updated_at,
    })
}

pub async fn registrar_salida(
    id: String,
    usuario_id: String,
    observaciones: Option<String>,
    devolvio_gafete: bool,
) -> Result<(), IngresoProveedorError> {
    let actualizado = db::update_salida(&id, &usuario_id, observaciones)
        .await
        .map_err(|e| IngresoProveedorError::Database(e.to_string()))?
        .ok_or(IngresoProveedorError::Validation(
            "Ingreso no encontrado o ya cerrado".to_string(),
        ))?;

    if devolvio_gafete {
        if let Some(g) = actualizado.gafete_numero {
            let _ = gafete_service::liberar_gafete(&g, "proveedor").await;
        }
    }
    Ok(())
}

pub async fn get_activos(
) -> Result<Vec<crate::domain::ingreso_proveedor::IngresoProveedor>, IngresoProveedorError> {
    Ok(vec![])
}

pub async fn get_historial(
) -> Result<Vec<crate::domain::ingreso_proveedor::IngresoProveedor>, IngresoProveedorError> {
    Ok(vec![])
}

pub async fn search_proveedores(_q: &str) -> Result<Vec<serde_json::Value>, IngresoProveedorError> {
    Ok(vec![])
}

pub async fn validar_ingreso(
    _proveedor_id: String,
) -> Result<serde_json::Value, IngresoProveedorError> {
    Ok(serde_json::json!({"puedeIngresar":true}))
}
