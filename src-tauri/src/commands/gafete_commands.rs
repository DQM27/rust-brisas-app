// ==========================================
// src/commands/gafete_commands.rs
// ==========================================
use crate::models::gafete::{
    Gafete, GafeteResponse, GafeteListResponse, GafeteStockResponse,
    CreateGafeteInput, UpdateGafeteInput, AsignarGafeteInput,
    EstadoGafete, validaciones,
};
use sqlx::{SqlitePool, Row};
use tauri::State;
use uuid::Uuid;
use chrono::Utc;

/// Crea un nuevo gafete
#[tauri::command]
pub async fn create_gafete(
    pool: State<'_, SqlitePool>,
    input: CreateGafeteInput,
) -> Result<GafeteResponse, String> {
    // Validar input
    validaciones::validar_create_input(&input)?;
    
    // Normalizar número (uppercase, trim)
    let numero_normalizado = input.numero.trim().to_uppercase();
    
    // Verificar que el número no exista
    let existe = sqlx::query(
        "SELECT COUNT(*) as count FROM gafetes WHERE numero = ?"
    )
    .bind(&numero_normalizado)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Error al verificar número de gafete: {}", e))?;
    
    let count: i32 = existe.get("count");
    if count > 0 {
        return Err(format!("Ya existe un gafete con el número {}", numero_normalizado));
    }
    
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    sqlx::query(
        r#"INSERT INTO gafetes 
           (id, numero, estado, contratista_asignado_id, ingreso_actual_id, created_at, updated_at)
           VALUES (?, ?, ?, NULL, NULL, ?, ?)"#
    )
    .bind(&id)
    .bind(&numero_normalizado)
    .bind(EstadoGafete::Disponible.as_str())
    .bind(&now)
    .bind(&now)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Error al crear gafete: {}", e))?;
    
    get_gafete_by_id(pool, id).await
}

/// Obtiene un gafete por ID
#[tauri::command]
pub async fn get_gafete_by_id(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<GafeteResponse, String> {
    let row = sqlx::query(
        r#"SELECT 
            g.id, g.numero, g.estado, g.contratista_asignado_id, g.ingreso_actual_id,
            g.created_at, g.updated_at,
            c.nombre as contratista_nombre,
            c.apellido as contratista_apellido,
            c.cedula as contratista_cedula
           FROM gafetes g
           LEFT JOIN contratistas c ON g.contratista_asignado_id = c.id
           WHERE g.id = ?"#
    )
    .bind(&id)
    .fetch_one(&*pool)
    .await
    .map_err(|_| "Gafete no encontrado".to_string())?;
    
    let gafete = Gafete {
        id: row.get("id"),
        numero: row.get("numero"),
        estado: row.get("estado"),
        contratista_asignado_id: row.get("contratista_asignado_id"),
        ingreso_actual_id: row.get("ingreso_actual_id"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    };
    
    let mut response = GafeteResponse::from(gafete);
    
    // Llenar datos del contratista si está asignado
    if response.contratista_asignado_id.is_some() {
        let nombre: Option<String> = row.get("contratista_nombre");
        let apellido: Option<String> = row.get("contratista_apellido");
        
        if let (Some(n), Some(a)) = (nombre, apellido) {
            response.contratista_nombre = Some(format!("{} {}", n, a));
        }
        response.contratista_cedula = row.get("contratista_cedula");
    }
    
    Ok(response)
}

/// Obtiene un gafete por número
#[tauri::command]
pub async fn get_gafete_by_numero(
    pool: State<'_, SqlitePool>,
    numero: String,
) -> Result<GafeteResponse, String> {
    let numero_normalizado = numero.trim().to_uppercase();
    
    let row = sqlx::query(
        r#"SELECT 
            g.id, g.numero, g.estado, g.contratista_asignado_id, g.ingreso_actual_id,
            g.created_at, g.updated_at,
            c.nombre as contratista_nombre,
            c.apellido as contratista_apellido,
            c.cedula as contratista_cedula
           FROM gafetes g
           LEFT JOIN contratistas c ON g.contratista_asignado_id = c.id
           WHERE g.numero = ?"#
    )
    .bind(&numero_normalizado)
    .fetch_one(&*pool)
    .await
    .map_err(|_| format!("Gafete {} no encontrado", numero_normalizado))?;
    
    let gafete = Gafete {
        id: row.get("id"),
        numero: row.get("numero"),
        estado: row.get("estado"),
        contratista_asignado_id: row.get("contratista_asignado_id"),
        ingreso_actual_id: row.get("ingreso_actual_id"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    };
    
    let mut response = GafeteResponse::from(gafete);
    
    if response.contratista_asignado_id.is_some() {
        let nombre: Option<String> = row.get("contratista_nombre");
        let apellido: Option<String> = row.get("contratista_apellido");
        
        if let (Some(n), Some(a)) = (nombre, apellido) {
            response.contratista_nombre = Some(format!("{} {}", n, a));
        }
        response.contratista_cedula = row.get("contratista_cedula");
    }
    
    Ok(response)
}

/// Obtiene todos los gafetes
#[tauri::command]
pub async fn get_all_gafetes(
    pool: State<'_, SqlitePool>,
) -> Result<GafeteListResponse, String> {
    let rows = sqlx::query(
        r#"SELECT 
            g.id, g.numero, g.estado, g.contratista_asignado_id, g.ingreso_actual_id,
            g.created_at, g.updated_at,
            c.nombre as contratista_nombre,
            c.apellido as contratista_apellido,
            c.cedula as contratista_cedula
           FROM gafetes g
           LEFT JOIN contratistas c ON g.contratista_asignado_id = c.id
           ORDER BY g.numero"#
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error al obtener gafetes: {}", e))?;
    
    let gafetes: Vec<GafeteResponse> = rows.into_iter()
        .map(|row| {
            let gafete = Gafete {
                id: row.get("id"),
                numero: row.get("numero"),
                estado: row.get("estado"),
                contratista_asignado_id: row.get("contratista_asignado_id"),
                ingreso_actual_id: row.get("ingreso_actual_id"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            
            let mut response = GafeteResponse::from(gafete);
            
            if response.contratista_asignado_id.is_some() {
                let nombre: Option<String> = row.get("contratista_nombre");
                let apellido: Option<String> = row.get("contratista_apellido");
                
                if let (Some(n), Some(a)) = (nombre, apellido) {
                    response.contratista_nombre = Some(format!("{} {}", n, a));
                }
                response.contratista_cedula = row.get("contratista_cedula");
            }
            
            response
        })
        .collect();
    
    let total = gafetes.len();
    let disponibles = gafetes.iter().filter(|g| g.estado == EstadoGafete::Disponible).count();
    let asignados = gafetes.iter().filter(|g| g.estado == EstadoGafete::Asignado).count();
    let perdidos = gafetes.iter().filter(|g| g.estado == EstadoGafete::Perdido).count();
    
    Ok(GafeteListResponse {
        gafetes,
        total,
        disponibles,
        asignados,
        perdidos,
    })
}

/// Obtiene solo gafetes disponibles (para asignar)
#[tauri::command]
pub async fn get_gafetes_disponibles(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<GafeteResponse>, String> {
    let rows = sqlx::query(
        r#"SELECT 
            id, numero, estado, contratista_asignado_id, ingreso_actual_id,
            created_at, updated_at
           FROM gafetes
           WHERE estado = ? AND numero != 'S/G'
           ORDER BY numero"#
    )
    .bind(EstadoGafete::Disponible.as_str())
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error al obtener gafetes disponibles: {}", e))?;
    
    let gafetes = rows.into_iter()
        .map(|row| {
            let gafete = Gafete {
                id: row.get("id"),
                numero: row.get("numero"),
                estado: row.get("estado"),
                contratista_asignado_id: row.get("contratista_asignado_id"),
                ingreso_actual_id: row.get("ingreso_actual_id"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            GafeteResponse::from(gafete)
        })
        .collect();
    
    Ok(gafetes)
}

/// Obtiene el stock de gafetes (estadísticas)
#[tauri::command]
pub async fn get_stock_gafetes(
    pool: State<'_, SqlitePool>,
) -> Result<GafeteStockResponse, String> {
    let row = sqlx::query(
        r#"SELECT 
            COUNT(*) as total,
            SUM(CASE WHEN estado = 'disponible' THEN 1 ELSE 0 END) as disponibles,
            SUM(CASE WHEN estado = 'asignado' THEN 1 ELSE 0 END) as asignados,
            SUM(CASE WHEN estado = 'perdido' THEN 1 ELSE 0 END) as perdidos
           FROM gafetes
           WHERE numero != 'S/G'"#
    )
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Error al obtener stock: {}", e))?;
    
    let total: i32 = row.get("total");
    let disponibles: i32 = row.get("disponibles");
    let asignados: i32 = row.get("asignados");
    let perdidos: i32 = row.get("perdidos");
    
    let porcentaje_disponibilidad = if total > 0 {
        (disponibles as f64 / total as f64) * 100.0
    } else {
        0.0
    };
    
    Ok(GafeteStockResponse {
        total_gafetes: total as usize,
        disponibles: disponibles as usize,
        asignados: asignados as usize,
        perdidos: perdidos as usize,
        porcentaje_disponibilidad,
    })
}

/// Asigna un gafete a un contratista (se usa al crear ingreso)
#[tauri::command]
pub async fn asignar_gafete(
    pool: State<'_, SqlitePool>,
    gafete_id: String,
    input: AsignarGafeteInput,
) -> Result<GafeteResponse, String> {
    // Verificar que el gafete esté disponible
    let gafete_row = sqlx::query(
        "SELECT estado, numero FROM gafetes WHERE id = ?"
    )
    .bind(&gafete_id)
    .fetch_one(&*pool)
    .await
    .map_err(|_| "Gafete no encontrado".to_string())?;
    
    let estado: String = gafete_row.get("estado");
    let numero: String = gafete_row.get("numero");
    
    if numero.to_uppercase() == "S/G" {
        return Err("No se puede asignar el gafete S/G".to_string());
    }
    
    if estado != EstadoGafete::Disponible.as_str() {
        return Err(format!("El gafete no está disponible (estado actual: {})", estado));
    }
    
    let now = Utc::now().to_rfc3339();
    
    sqlx::query(
        r#"UPDATE gafetes SET
            estado = ?,
            contratista_asignado_id = ?,
            ingreso_actual_id = ?,
            updated_at = ?
        WHERE id = ?"#
    )
    .bind(EstadoGafete::Asignado.as_str())
    .bind(&input.contratista_id)
    .bind(&input.ingreso_id)
    .bind(&now)
    .bind(&gafete_id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Error al asignar gafete: {}", e))?;
    
    get_gafete_by_id(pool, gafete_id).await
}

/// Libera un gafete (se usa al registrar salida)
#[tauri::command]
pub async fn liberar_gafete(
    pool: State<'_, SqlitePool>,
    gafete_id: String,
) -> Result<GafeteResponse, String> {
    let now = Utc::now().to_rfc3339();
    
    sqlx::query(
        r#"UPDATE gafetes SET
            estado = ?,
            contratista_asignado_id = NULL,
            ingreso_actual_id = NULL,
            updated_at = ?
        WHERE id = ?"#
    )
    .bind(EstadoGafete::Disponible.as_str())
    .bind(&now)
    .bind(&gafete_id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Error al liberar gafete: {}", e))?;
    
    get_gafete_by_id(pool, gafete_id).await
}

/// Actualiza información de un gafete
#[tauri::command]
pub async fn update_gafete(
    pool: State<'_, SqlitePool>,
    id: String,
    input: UpdateGafeteInput,
) -> Result<GafeteResponse, String> {
    if let Some(ref numero) = input.numero {
        validaciones::validar_numero(numero)?;
        
        // Verificar que el nuevo número no exista
        let numero_normalizado = numero.trim().to_uppercase();
        let existe = sqlx::query(
            "SELECT COUNT(*) as count FROM gafetes WHERE numero = ? AND id != ?"
        )
        .bind(&numero_normalizado)
        .bind(&id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| format!("Error al verificar número: {}", e))?;
        
        let count: i32 = existe.get("count");
        if count > 0 {
            return Err(format!("Ya existe otro gafete con el número {}", numero_normalizado));
        }
    }
    
    let now = Utc::now().to_rfc3339();
    
    if let Some(numero) = input.numero {
        let numero_normalizado = numero.trim().to_uppercase();
        sqlx::query(
            "UPDATE gafetes SET numero = ?, updated_at = ? WHERE id = ?"
        )
        .bind(&numero_normalizado)
        .bind(&now)
        .bind(&id)
        .execute(&*pool)
        .await
        .map_err(|e| format!("Error al actualizar gafete: {}", e))?;
    }
    
    get_gafete_by_id(pool, id).await
}

/// Elimina un gafete (solo si está disponible)
#[tauri::command]
pub async fn delete_gafete(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    // Verificar que esté disponible
    let row = sqlx::query(
        "SELECT estado, numero FROM gafetes WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&*pool)
    .await
    .map_err(|_| "Gafete no encontrado".to_string())?;
    
    let estado: String = row.get("estado");
    let numero: String = row.get("numero");
    
    if numero.to_uppercase() == "S/G" {
        return Err("No se puede eliminar el gafete S/G".to_string());
    }
    
    if estado != EstadoGafete::Disponible.as_str() {
        return Err("No se puede eliminar un gafete que no está disponible".to_string());
    }
    
    sqlx::query("DELETE FROM gafetes WHERE id = ?")
        .bind(&id)
        .execute(&*pool)
        .await
        .map_err(|e| format!("Error al eliminar gafete: {}", e))?;
    
    Ok(())
}