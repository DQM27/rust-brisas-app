// ==========================================
// src/services/vehiculo_service.rs
// ==========================================
// Capa de servicio: orquesta dominio, db
// Contiene la lógica de negocio completa

use crate::db::vehiculo_queries as db;
use crate::domain::vehiculo as domain;
use crate::models::vehiculo::{
    CreateVehiculoInput, TipoVehiculo, TipoVehiculoStats, UpdateVehiculoInput,
    VehiculoListResponse, VehiculoResponse,
};
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

// ==========================================
// CREAR VEHÍCULO
// ==========================================

pub async fn create_vehiculo(
    pool: &SqlitePool,
    input: CreateVehiculoInput,
) -> Result<VehiculoResponse, String> {
    // 1. Validar input
    domain::validar_create_input(&input)?;

    // 2. Normalizar datos
    let placa_normalizada = domain::normalizar_placa(&input.placa);
    let tipo_vehiculo = domain::validar_tipo_vehiculo(&input.tipo_vehiculo)?;

    let marca_normalizada = input
        .marca
        .as_ref()
        .map(|m| domain::normalizar_texto(m))
        .filter(|m| !m.is_empty());

    let modelo_normalizado = input
        .modelo
        .as_ref()
        .map(|m| domain::normalizar_texto(m))
        .filter(|m| !m.is_empty());

    let color_normalizado = input
        .color
        .as_ref()
        .map(|c| domain::normalizar_texto(c))
        .filter(|c| !c.is_empty());

    // 3. Verificar que el contratista exista
    if !db::contratista_exists(pool, &input.contratista_id).await? {
        return Err("El contratista especificado no existe".to_string());
    }

    // 4. Verificar que la placa no exista
    let count = db::count_by_placa(pool, &placa_normalizada).await?;
    if count > 0 {
        return Err(format!(
            "Ya existe un vehículo con la placa {}",
            placa_normalizada
        ));
    }

    // 5. Generar ID y timestamps
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    // 6. Insertar en DB
    db::insert(
        pool,
        &id,
        Some(&input.contratista_id),
        None, // proveedor_id
        tipo_vehiculo.as_str(),
        &placa_normalizada,
        marca_normalizada.as_deref(),
        modelo_normalizado.as_deref(),
        color_normalizado.as_deref(),
        &now,
        &now,
    )
    .await?;

    // 7. Retornar vehículo creado con datos completos
    get_vehiculo_by_id(pool, &id).await
}

// ==========================================
// OBTENER VEHÍCULO POR ID
// ==========================================

pub async fn get_vehiculo_by_id(pool: &SqlitePool, id: &str) -> Result<VehiculoResponse, String> {
    // Obtener vehículo de DB
    let vehiculo = db::find_by_id(pool, id).await?;

    // Obtener datos del contratista con JOIN
    let row = sqlx::query(
        r#"SELECT 
            c.nombre as contratista_nombre,
            c.apellido as contratista_apellido,
            c.cedula as contratista_cedula,
            e.nombre as empresa_nombre
           FROM contratistas c
           INNER JOIN empresas e ON c.empresa_id = e.id
           WHERE c.id = ?"#,
    )
    .bind(&vehiculo.contratista_id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error al obtener datos del contratista: {}", e))?;

    use sqlx::Row;
    let nombre: String = row.get("contratista_nombre");
    let apellido: String = row.get("contratista_apellido");

    let mut response = VehiculoResponse::from(vehiculo);
    response.contratista_nombre = format!("{} {}", nombre, apellido);
    response.contratista_cedula = row.get("contratista_cedula");
    response.empresa_nombre = row.get("empresa_nombre");

    Ok(response)
}

// ==========================================
// OBTENER VEHÍCULO POR PLACA
// ==========================================

pub async fn get_vehiculo_by_placa(
    pool: &SqlitePool,
    placa: String,
) -> Result<VehiculoResponse, String> {
    let placa_normalizada = domain::normalizar_placa(&placa);

    // Obtener vehículo de DB
    let vehiculo = db::find_by_placa(pool, &placa_normalizada).await?;

    // Obtener datos del contratista con JOIN
    let row = sqlx::query(
        r#"SELECT 
            c.nombre as contratista_nombre,
            c.apellido as contratista_apellido,
            c.cedula as contratista_cedula,
            e.nombre as empresa_nombre
           FROM contratistas c
           INNER JOIN empresas e ON c.empresa_id = e.id
           WHERE c.id = ?"#,
    )
    .bind(&vehiculo.contratista_id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error al obtener datos del contratista: {}", e))?;

    use sqlx::Row;
    let nombre: String = row.get("contratista_nombre");
    let apellido: String = row.get("contratista_apellido");

    let mut response = VehiculoResponse::from(vehiculo);
    response.contratista_nombre = format!("{} {}", nombre, apellido);
    response.contratista_cedula = row.get("contratista_cedula");
    response.empresa_nombre = row.get("empresa_nombre");

    Ok(response)
}

// ==========================================
// OBTENER TODOS LOS VEHÍCULOS
// ==========================================

pub async fn get_all_vehiculos(pool: &SqlitePool) -> Result<VehiculoListResponse, String> {
    let vehiculos = db::find_all(pool).await?;

    // Obtener datos de contratistas para cada vehículo
    let mut vehiculo_responses = Vec::new();

    for vehiculo in vehiculos {
        let row = sqlx::query(
            r#"SELECT 
                c.nombre as contratista_nombre,
                c.apellido as contratista_apellido,
                c.cedula as contratista_cedula,
                e.nombre as empresa_nombre
               FROM contratistas c
               INNER JOIN empresas e ON c.empresa_id = e.id
               WHERE c.id = ?"#,
        )
        .bind(&vehiculo.contratista_id)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Error al obtener datos del contratista: {}", e))?;

        use sqlx::Row;
        let nombre: String = row.get("contratista_nombre");
        let apellido: String = row.get("contratista_apellido");

        let mut response = VehiculoResponse::from(vehiculo);
        response.contratista_nombre = format!("{} {}", nombre, apellido);
        response.contratista_cedula = row.get("contratista_cedula");
        response.empresa_nombre = row.get("empresa_nombre");

        vehiculo_responses.push(response);
    }

    // Calcular estadísticas
    let total = vehiculo_responses.len();
    let activos = vehiculo_responses.iter().filter(|v| v.is_active).count();
    let inactivos = total - activos;
    let motocicletas = vehiculo_responses
        .iter()
        .filter(|v| v.tipo_vehiculo == TipoVehiculo::Motocicleta)
        .count();
    let automóviles = vehiculo_responses
        .iter()
        .filter(|v| v.tipo_vehiculo == TipoVehiculo::Automóvil)
        .count();

    Ok(VehiculoListResponse {
        vehiculos: vehiculo_responses,
        total,
        activos,
        inactivos,
        por_tipo: TipoVehiculoStats {
            motocicletas,
            automóviles,
        },
    })
}

// ==========================================
// OBTENER VEHÍCULOS ACTIVOS
// ==========================================

pub async fn get_vehiculos_activos(pool: &SqlitePool) -> Result<Vec<VehiculoResponse>, String> {
    let vehiculos = db::find_activos(pool).await?;

    let mut vehiculo_responses = Vec::new();

    for vehiculo in vehiculos {
        let row = sqlx::query(
            r#"SELECT 
                c.nombre as contratista_nombre,
                c.apellido as contratista_apellido,
                c.cedula as contratista_cedula,
                e.nombre as empresa_nombre
               FROM contratistas c
               INNER JOIN empresas e ON c.empresa_id = e.id
               WHERE c.id = ?"#,
        )
        .bind(&vehiculo.contratista_id)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Error al obtener datos del contratista: {}", e))?;

        use sqlx::Row;
        let nombre: String = row.get("contratista_nombre");
        let apellido: String = row.get("contratista_apellido");

        let mut response = VehiculoResponse::from(vehiculo);
        response.contratista_nombre = format!("{} {}", nombre, apellido);
        response.contratista_cedula = row.get("contratista_cedula");
        response.empresa_nombre = row.get("empresa_nombre");

        vehiculo_responses.push(response);
    }

    Ok(vehiculo_responses)
}

// ==========================================
// OBTENER VEHÍCULOS POR CONTRATISTA
// ==========================================

pub async fn get_vehiculos_by_contratista(
    pool: &SqlitePool,
    contratista_id: String,
) -> Result<Vec<VehiculoResponse>, String> {
    let vehiculos = db::find_by_contratista(pool, &contratista_id).await?;

    let mut vehiculo_responses = Vec::new();

    for vehiculo in vehiculos {
        let row = sqlx::query(
            r#"SELECT 
                c.nombre as contratista_nombre,
                c.apellido as contratista_apellido,
                c.cedula as contratista_cedula,
                e.nombre as empresa_nombre
               FROM contratistas c
               INNER JOIN empresas e ON c.empresa_id = e.id
               WHERE c.id = ?"#,
        )
        .bind(&vehiculo.contratista_id)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Error al obtener datos del contratista: {}", e))?;

        use sqlx::Row;
        let nombre: String = row.get("contratista_nombre");
        let apellido: String = row.get("contratista_apellido");

        let mut response = VehiculoResponse::from(vehiculo);
        response.contratista_nombre = format!("{} {}", nombre, apellido);
        response.contratista_cedula = row.get("contratista_cedula");
        response.empresa_nombre = row.get("empresa_nombre");

        vehiculo_responses.push(response);
    }

    Ok(vehiculo_responses)
}

// ==========================================
// ACTUALIZAR VEHÍCULO
// ==========================================

pub async fn update_vehiculo(
    pool: &SqlitePool,
    id: String,
    input: UpdateVehiculoInput,
) -> Result<VehiculoResponse, String> {
    // 1. Validar input
    domain::validar_update_input(&input)?;

    // 2. Verificar que el vehículo existe
    let _ = db::find_by_id(pool, &id).await?;

    // 3. Normalizar y convertir tipo si viene
    let tipo_str = if let Some(ref t) = input.tipo_vehiculo {
        Some(domain::validar_tipo_vehiculo(t)?.as_str().to_string())
    } else {
        None
    };

    // 4. Normalizar textos si vienen
    let marca_normalizada = input
        .marca
        .as_ref()
        .map(|m| domain::normalizar_texto(m))
        .filter(|m| !m.is_empty());

    let modelo_normalizado = input
        .modelo
        .as_ref()
        .map(|m| domain::normalizar_texto(m))
        .filter(|m| !m.is_empty());

    let color_normalizado = input
        .color
        .as_ref()
        .map(|c| domain::normalizar_texto(c))
        .filter(|c| !c.is_empty());

    // 5. Timestamp de actualización
    let now = Utc::now().to_rfc3339();

    // 6. Convertir is_active a i32 si viene
    let is_active_int = input.is_active.map(|b| if b { 1 } else { 0 });

    // 7. Actualizar en DB
    db::update(
        pool,
        &id,
        tipo_str.as_deref(),
        marca_normalizada.as_deref(),
        modelo_normalizado.as_deref(),
        color_normalizado.as_deref(),
        is_active_int,
        &now,
    )
    .await?;

    // 8. Retornar vehículo actualizado
    get_vehiculo_by_id(pool, &id).await
}

// ==========================================
// ELIMINAR VEHÍCULO
// ==========================================

pub async fn delete_vehiculo(pool: &SqlitePool, id: String) -> Result<(), String> {
    // Verificar que existe antes de eliminar
    let _ = db::find_by_id(pool, &id).await?;

    // Eliminar
    db::delete(pool, &id).await
}
