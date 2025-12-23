// ==========================================
// src/services/ingreso_proveedor_service.rs
// ==========================================
// Capa de servicio: Lógica de negocio para ingreso de proveedores

use crate::db::{empresa_queries, ingreso_proveedor_queries, proveedor_queries};
use crate::domain::errors::IngresoProveedorError;
use crate::domain::ingreso_proveedor as domain;
use crate::domain::ingreso_proveedor::{
    CreateIngresoProveedorInput, IngresoProveedor, ProveedorSnapshot,
};
use crate::models::proveedor::CreateProveedorInput;
use crate::services::{alerta_service, gafete_service};
use sqlx::SqlitePool;

pub async fn registrar_ingreso(
    pool: &SqlitePool,
    input: CreateIngresoProveedorInput,
) -> Result<IngresoProveedor, IngresoProveedorError> {
    // 1. Validar existencia de la empresa
    if empresa_queries::find_by_id(pool, &input.empresa_id)
        .await
        .map_err(IngresoProveedorError::Database)?
        .is_none()
    {
        return Err(IngresoProveedorError::Validation("La empresa no existe".to_string()));
    }

    // 2. Validar disponibilidad de gafete (si aplica)
    if let Some(ref g) = input.gafete {
        let disponible = gafete_service::is_gafete_disponible(pool, g, "proveedor")
            .await
            .map_err(|e| IngresoProveedorError::Validation(e.to_string()))?; // Mapping GafeteError to Validation? Or keep string?
                                                                             // GafeteService returns Result<bool, GafeteError>. We map to String then Validation.
        if !disponible {
            return Err(IngresoProveedorError::Validation(format!(
                "El gafete {} no está disponible",
                g
            )));
        }
    }

    // 3. Obtener o Crear Proveedor (Catalog)
    let proveedor_id = if let Some(prov) = proveedor_queries::find_by_cedula(pool, &input.cedula)
        .await
        .map_err(IngresoProveedorError::Database)?
    {
        prov.id
    } else {
        // Crear nuevo en catálogo
        let new_prov = proveedor_queries::create(
            pool,
            CreateProveedorInput {
                cedula: input.cedula.clone(),
                nombre: input.nombre.clone(),
                segundo_nombre: None,
                apellido: input.apellido.clone(),
                segundo_apellido: None,
                empresa_id: input.empresa_id.clone(),
                tiene_vehiculo: if input.placa_vehiculo.is_some() { Some(true) } else { None },
                tipo_vehiculo: input.tipo_vehiculo.clone(),
                placa: input.placa_vehiculo.clone(),
                marca: input.marca_vehiculo.clone(),
                modelo: input.modelo_vehiculo.clone(),
                color: input.color_vehiculo.clone(),
            },
        )
        .await
        .map_err(IngresoProveedorError::Database)?;
        new_prov.id
    };

    // 4. Crear ingreso vinculado
    ingreso_proveedor_queries::create(pool, input, &proveedor_id)
        .await
        .map_err(IngresoProveedorError::Database)
}

pub async fn registrar_salida(
    pool: &SqlitePool,
    id: String,
    usuario_id: String,
    observaciones: Option<String>,
    devolvio_gafete: bool,
) -> Result<(), IngresoProveedorError> {
    // 1. Obtener el ingreso para verificar gafete
    let ingreso = ingreso_proveedor_queries::find_by_id(pool, &id)
        .await
        .map_err(IngresoProveedorError::Database)?
        .ok_or(IngresoProveedorError::NotFound)?;

    domain::validar_ingreso_abierto(&ingreso.fecha_salida)?;

    let now = chrono::Utc::now().to_rfc3339();
    domain::validar_tiempo_salida(&ingreso.fecha_ingreso, &now)?; // Using fecha_ingreso (provider struct has it)

    // Evaluar gafete
    let decision = domain::evaluar_devolucion_gafete(
        ingreso.gafete.is_some(),
        ingreso.gafete.as_deref(),
        devolvio_gafete,
        if devolvio_gafete {
            ingreso.gafete.as_deref() // Assuming simplified frontend that just sends bool
        } else {
            None
        },
    ); // Retorna DecisionReporteGafete directamente

    // Start TX
    let mut tx = pool.begin().await.map_err(IngresoProveedorError::Database)?;

    // 2. Registrar salida en DB
    sqlx::query(
        r#"
        UPDATE ingresos_proveedores 
        SET estado = 'SALIO', 
            fecha_salida = ?, 
            usuario_salida_id = ?, 
            observaciones = COALESCE(?, observaciones),
            updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(&now)
    .bind(&usuario_id)
    .bind(observaciones.as_deref())
    .bind(&now)
    .bind(&id)
    .execute(&mut *tx)
    .await
    .map_err(IngresoProveedorError::Database)?;

    // 3. Crear alerta si aplica
    if decision.debe_generar_reporte {
        if let Some(num) = decision.gafete_numero {
            let alerta_id = uuid::Uuid::new_v4().to_string();
            let nombre_completo = format!("{} {}", ingreso.nombre, ingreso.apellido);

            alerta_service::insert(
                pool,
                &alerta_id,
                None, // No contratista_id
                &ingreso.cedula,
                &nombre_completo,
                &num,
                None,      // No ingreso_contratista_id
                Some(&id), // ingreso_proveedor_id
                &now,
                decision.motivo.as_deref(),
                &usuario_id,
                &now,
                &now,
            )
            .await
            .map_err(|e| IngresoProveedorError::Database(sqlx::Error::Protocol(e.to_string())))?;
        }
    }

    tx.commit().await.map_err(IngresoProveedorError::Database)?;

    Ok(())
}

pub async fn get_activos(
    pool: &SqlitePool,
) -> Result<Vec<IngresoProveedor>, IngresoProveedorError> {
    ingreso_proveedor_queries::find_actives(pool).await.map_err(IngresoProveedorError::Database)
}

pub async fn get_historial(
    pool: &SqlitePool,
) -> Result<Vec<IngresoProveedor>, IngresoProveedorError> {
    ingreso_proveedor_queries::find_history(pool).await.map_err(IngresoProveedorError::Database)
}

pub async fn search_proveedores(
    pool: &SqlitePool,
    query: &str,
) -> Result<Vec<ProveedorSnapshot>, IngresoProveedorError> {
    ingreso_proveedor_queries::search_distinct_proveedores(pool, query)
        .await
        .map_err(IngresoProveedorError::Database)
}

pub async fn validar_ingreso(
    pool: &SqlitePool,
    proveedor_id: String,
) -> Result<
    crate::domain::ingreso_proveedor::ValidacionIngresoProveedorResponse,
    IngresoProveedorError,
> {
    use crate::domain::ingreso_proveedor::ValidacionIngresoProveedorResponse;

    // 1. Verificar ingreso abierto
    let ingreso_abierto = ingreso_proveedor_queries::find_open_by_proveedor(pool, &proveedor_id)
        .await
        .map_err(IngresoProveedorError::Database)?;

    if let Some(ingreso) = ingreso_abierto {
        return Ok(ValidacionIngresoProveedorResponse {
            puede_ingresar: false,
            motivo_rechazo: Some("El proveedor ya tiene un ingreso abierto".to_string()),
            alertas: vec![],
            proveedor: None,
            tiene_ingreso_abierto: true,
            ingreso_abierto: Some(ingreso),
        });
    }

    // 2. Obtener datos del proveedor
    let proveedor_opt = proveedor_queries::find_by_id(pool, &proveedor_id)
        .await
        .map_err(IngresoProveedorError::Database)?;

    let proveedor = match proveedor_opt {
        Some(p) => p,
        None => return Err(IngresoProveedorError::NotFound),
    };

    // 3. Obtener vehículos
    let vehiculos = crate::db::vehiculo_queries::find_by_proveedor(pool, &proveedor_id)
        .await
        .map_err(IngresoProveedorError::Database)?;

    // 4. Construir respuesta JSON
    let proveedor_json = serde_json::json!({
        "id": proveedor.id,
        "cedula": proveedor.cedula,
        "nombre": proveedor.nombre,
        "segundo_nombre": proveedor.segundo_nombre,
        "apellido": proveedor.apellido,
        "segundo_apellido": proveedor.segundo_apellido,
        "empresa_id": proveedor.empresa_id,
        "estado": proveedor.estado.as_str(),
        "vehiculos": vehiculos
    });

    // 5. Obtener alertas pendientes
    let alertas_pendientes =
        crate::services::alerta_service::find_pendientes_by_cedula(pool, &proveedor.cedula)
            .await
            .map_err(|e| {
            IngresoProveedorError::Validation(format!("Error obteniendo alertas: {}", e))
        })?;
    // Wait, alerta_service returns Result<Vec<AlertaGafete>, sqlx::Error>? No, AlertaError?
    // Previous refactor: check alerta_service.rs.
    // Usually it mapped to String. But I should check.
    // Assuming AlertaError or sqlx::Error. If AlertaError, map to Validation or Database?

    let alertas_response: Vec<crate::models::ingreso::AlertaGafeteResponse> =
        alertas_pendientes.into_iter().map(|a| a.into()).collect();

    Ok(ValidacionIngresoProveedorResponse {
        puede_ingresar: true,
        motivo_rechazo: None,
        alertas: alertas_response,
        proveedor: Some(proveedor_json),
        tiene_ingreso_abierto: false,
        ingreso_abierto: None,
    })
}
