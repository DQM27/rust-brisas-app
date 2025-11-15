// ==========================================
// src/commands/vehiculo_commands.rs
// ==========================================
use crate::models::vehiculo::{
    Vehiculo, VehiculoResponse, VehiculoListResponse,
    CreateVehiculoInput, UpdateVehiculoInput, validaciones,
};
use sqlx::{SqlitePool, Row};
use tauri::State;
use uuid::Uuid;
use chrono::Utc;

/// Crea un nuevo vehículo para un contratista
#[tauri::command]
pub async fn create_vehiculo(
    pool: State<'_, SqlitePool>,
    input: CreateVehiculoInput,
) -> Result<VehiculoResponse, String> {
    // Validar input
    validaciones::validar_create_input(&input)?;
    
    // Normalizar placa (uppercase, sin espacios extras)
    let placa_normalizada = input.placa.trim().to_uppercase();
    
    // Verificar que el contratista exista
    let contratista_existe = sqlx::query(
        "SELECT COUNT(*) as count FROM contratistas WHERE id = ?"
    )
    .bind(&input.contratista_id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Error al verificar contratista: {}", e))?;
    
    let count: i32 = contratista_existe.get("count");
    if count == 0 {
        return Err("El contratista especificado no existe".to_string());
    }
    
    // Verificar que la placa no exista (constraint: 1 placa = 1 contratista)
    let placa_existe = sqlx::query(
        "SELECT COUNT(*) as count FROM vehiculos WHERE placa = ?"
    )
    .bind(&placa_normalizada)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Error al verificar placa: {}", e))?;
    
    let placa_count: i32 = placa_existe.get("count");
    if placa_count > 0 {
        return Err(format!("Ya existe un vehículo con la placa {}", placa_normalizada));
    }
    
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    sqlx::query(
        r#"INSERT INTO vehiculos 
           (id, contratista_id, placa, marca, modelo, color, is_active, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, 1, ?, ?)"#
    )
    .bind(&id)
    .bind(&input.contratista_id)
    .bind(&placa_normalizada)
    .bind(input.marca.as_deref().map(|s| s.trim()))
    .bind(input.modelo.as_deref().map(|s| s.trim()))
    .bind(input.color.as_deref().map(|s| s.trim()))
    .bind(&now)
    .bind(&now)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Error al crear vehículo: {}", e))?;
    
    get_vehiculo_by_id(pool, id).await
}

/// Obtiene un vehículo por ID con datos del contratista
#[tauri::command]
pub async fn get_vehiculo_by_id(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<VehiculoResponse, String> {
    let row = sqlx::query(
        r#"SELECT 
            v.id, v.contratista_id, v.placa, v.marca, v.modelo, v.color,
            v.is_active, v.created_at, v.updated_at,
            c.nombre as contratista_nombre,
            c.apellido as contratista_apellido,
            c.cedula as contratista_cedula,
            e.nombre as empresa_nombre
           FROM vehiculos v
           INNER JOIN contratistas c ON v.contratista_id = c.id
           INNER JOIN empresas e ON c.empresa_id = e.id
           WHERE v.id = ?"#
    )
    .bind(&id)
    .fetch_one(&*pool)
    .await
    .map_err(|_| "Vehículo no encontrado".to_string())?;
    
    let vehiculo = Vehiculo {
        id: row.get("id"),
        contratista_id: row.get("contratista_id"),
        placa: row.get("placa"),
        marca: row.get("marca"),
        modelo: row.get("modelo"),
        color: row.get("color"),
        is_active: row.get("is_active"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    };
    
    let nombre: String = row.get("contratista_nombre");
    let apellido: String = row.get("contratista_apellido");
    
    let mut response = VehiculoResponse::from(vehiculo);
    response.contratista_nombre = format!("{} {}", nombre, apellido);
    response.contratista_cedula = row.get("contratista_cedula");
    response.empresa_nombre = row.get("empresa_nombre");
    
    Ok(response)
}

/// Obtiene un vehículo por placa
#[tauri::command]
pub async fn get_vehiculo_by_placa(
    pool: State<'_, SqlitePool>,
    placa: String,
) -> Result<VehiculoResponse, String> {
    let placa_normalizada = placa.trim().to_uppercase();
    
    let row = sqlx::query(
        r#"SELECT 
            v.id, v.contratista_id, v.placa, v.marca, v.modelo, v.color,
            v.is_active, v.created_at, v.updated_at,
            c.nombre as contratista_nombre,
            c.apellido as contratista_apellido,
            c.cedula as contratista_cedula,
            e.nombre as empresa_nombre
           FROM vehiculos v
           INNER JOIN contratistas c ON v.contratista_id = c.id
           INNER JOIN empresas e ON c.empresa_id = e.id
           WHERE v.placa = ?"#
    )
    .bind(&placa_normalizada)
    .fetch_one(&*pool)
    .await
    .map_err(|_| format!("Vehículo con placa {} no encontrado", placa_normalizada))?;
    
    let vehiculo = Vehiculo {
        id: row.get("id"),
        contratista_id: row.get("contratista_id"),
        placa: row.get("placa"),
        marca: row.get("marca"),
        modelo: row.get("modelo"),
        color: row.get("color"),
        is_active: row.get("is_active"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    };
    
    let nombre: String = row.get("contratista_nombre");
    let apellido: String = row.get("contratista_apellido");
    
    let mut response = VehiculoResponse::from(vehiculo);
    response.contratista_nombre = format!("{} {}", nombre, apellido);
    response.contratista_cedula = row.get("contratista_cedula");
    response.empresa_nombre = row.get("empresa_nombre");
    
    Ok(response)
}

/// Obtiene todos los vehículos del sistema
#[tauri::command]
pub async fn get_all_vehiculos(
    pool: State<'_, SqlitePool>,
) -> Result<VehiculoListResponse, String> {
    let rows = sqlx::query(
        r#"SELECT 
            v.id, v.contratista_id, v.placa, v.marca, v.modelo, v.color,
            v.is_active, v.created_at, v.updated_at,
            c.nombre as contratista_nombre,
            c.apellido as contratista_apellido,
            c.cedula as contratista_cedula,
            e.nombre as empresa_nombre
           FROM vehiculos v
           INNER JOIN contratistas c ON v.contratista_id = c.id
           INNER JOIN empresas e ON c.empresa_id = e.id
           ORDER BY v.created_at DESC"#
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error al obtener vehículos: {}", e))?;
    
    let vehiculos: Vec<VehiculoResponse> = rows.into_iter()
        .map(|row| {
            let vehiculo = Vehiculo {
                id: row.get("id"),
                contratista_id: row.get("contratista_id"),
                placa: row.get("placa"),
                marca: row.get("marca"),
                modelo: row.get("modelo"),
                color: row.get("color"),
                is_active: row.get("is_active"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            
            let nombre: String = row.get("contratista_nombre");
            let apellido: String = row.get("contratista_apellido");
            
            let mut response = VehiculoResponse::from(vehiculo);
            response.contratista_nombre = format!("{} {}", nombre, apellido);
            response.contratista_cedula = row.get("contratista_cedula");
            response.empresa_nombre = row.get("empresa_nombre");
            response
        })
        .collect();
    
    let total = vehiculos.len();
    let activos = vehiculos.iter().filter(|v| v.is_active).count();
    let inactivos = total - activos;
    
    Ok(VehiculoListResponse {
        vehiculos,
        total,
        activos,
        inactivos,
    })
}

/// Obtiene todos los vehículos activos
#[tauri::command]
pub async fn get_vehiculos_activos(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<VehiculoResponse>, String> {
    let rows = sqlx::query(
        r#"SELECT 
            v.id, v.contratista_id, v.placa, v.marca, v.modelo, v.color,
            v.is_active, v.created_at, v.updated_at,
            c.nombre as contratista_nombre,
            c.apellido as contratista_apellido,
            c.cedula as contratista_cedula,
            e.nombre as empresa_nombre
           FROM vehiculos v
           INNER JOIN contratistas c ON v.contratista_id = c.id
           INNER JOIN empresas e ON c.empresa_id = e.id
           WHERE v.is_active = 1
           ORDER BY v.placa"#
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error al obtener vehículos activos: {}", e))?;
    
    let vehiculos = rows.into_iter()
        .map(|row| {
            let vehiculo = Vehiculo {
                id: row.get("id"),
                contratista_id: row.get("contratista_id"),
                placa: row.get("placa"),
                marca: row.get("marca"),
                modelo: row.get("modelo"),
                color: row.get("color"),
                is_active: row.get("is_active"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            
            let nombre: String = row.get("contratista_nombre");
            let apellido: String = row.get("contratista_apellido");
            
            let mut response = VehiculoResponse::from(vehiculo);
            response.contratista_nombre = format!("{} {}", nombre, apellido);
            response.contratista_cedula = row.get("contratista_cedula");
            response.empresa_nombre = row.get("empresa_nombre");
            response
        })
        .collect();
    
    Ok(vehiculos)
}

/// Obtiene todos los vehículos de un contratista específico
#[tauri::command]
pub async fn get_vehiculos_by_contratista(
    pool: State<'_, SqlitePool>,
    contratista_id: String,
) -> Result<Vec<VehiculoResponse>, String> {
    let rows = sqlx::query(
        r#"SELECT 
            v.id, v.contratista_id, v.placa, v.marca, v.modelo, v.color,
            v.is_active, v.created_at, v.updated_at,
            c.nombre as contratista_nombre,
            c.apellido as contratista_apellido,
            c.cedula as contratista_cedula,
            e.nombre as empresa_nombre
           FROM vehiculos v
           INNER JOIN contratistas c ON v.contratista_id = c.id
           INNER JOIN empresas e ON c.empresa_id = e.id
           WHERE v.contratista_id = ?
           ORDER BY v.is_active DESC, v.placa"#
    )
    .bind(&contratista_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error al obtener vehículos del contratista: {}", e))?;
    
    let vehiculos = rows.into_iter()
        .map(|row| {
            let vehiculo = Vehiculo {
                id: row.get("id"),
                contratista_id: row.get("contratista_id"),
                placa: row.get("placa"),
                marca: row.get("marca"),
                modelo: row.get("modelo"),
                color: row.get("color"),
                is_active: row.get("is_active"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            
            let nombre: String = row.get("contratista_nombre");
            let apellido: String = row.get("contratista_apellido");
            
            let mut response = VehiculoResponse::from(vehiculo);
            response.contratista_nombre = format!("{} {}", nombre, apellido);
            response.contratista_cedula = row.get("contratista_cedula");
            response.empresa_nombre = row.get("empresa_nombre");
            response
        })
        .collect();
    
    Ok(vehiculos)
}

/// Actualiza información de un vehículo
#[tauri::command]
pub async fn update_vehiculo(
    pool: State<'_, SqlitePool>,
    id: String,
    input: UpdateVehiculoInput,
) -> Result<VehiculoResponse, String> {
    // Validar campos opcionales
    if let Some(ref marca) = input.marca {
        validaciones::validar_texto_opcional(marca, "Marca", 50)?;
    }
    if let Some(ref modelo) = input.modelo {
        validaciones::validar_texto_opcional(modelo, "Modelo", 50)?;
    }
    if let Some(ref color) = input.color {
        validaciones::validar_texto_opcional(color, "Color", 30)?;
    }
    
    let now = Utc::now().to_rfc3339();
    
    sqlx::query(
        r#"UPDATE vehiculos SET
            marca = COALESCE(?, marca),
            modelo = COALESCE(?, modelo),
            color = COALESCE(?, color),
            is_active = COALESCE(?, is_active),
            updated_at = ?
        WHERE id = ?"#
    )
    .bind(input.marca.as_deref().map(|s| s.trim()))
    .bind(input.modelo.as_deref().map(|s| s.trim()))
    .bind(input.color.as_deref().map(|s| s.trim()))
    .bind(input.is_active)
    .bind(&now)
    .bind(&id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Error al actualizar vehículo: {}", e))?;
    
    get_vehiculo_by_id(pool, id).await
}

/// Elimina un vehículo (eliminación física)
#[tauri::command]
pub async fn delete_vehiculo(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    sqlx::query("DELETE FROM vehiculos WHERE id = ?")
        .bind(&id)
        .execute(&*pool)
        .await
        .map_err(|e| format!("Error al eliminar vehículo: {}", e))?;
    
    Ok(())
}