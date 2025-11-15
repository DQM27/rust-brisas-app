// ==========================================
// src/commands/gafete_perdido_commands.rs
// ==========================================
use crate::models::gafete_perdido::{
    GafetePerdido, GafetePerdidoResponse, GafetesPerdidosListResponse, DeudasContratistaResponse,
    ReportarGafetePerdidoInput, RegistrarPagoInput, CondonarDeudaInput,
    EstadoPago, validaciones,
};
use crate::models::gafete::EstadoGafete;
use sqlx::{SqlitePool, Row};
use tauri::State;
use uuid::Uuid;
use chrono::Utc;

/// Reporta un gafete como perdido
#[tauri::command]
pub async fn reportar_gafete_perdido(
    pool: State<'_, SqlitePool>,
    input: ReportarGafetePerdidoInput,
) -> Result<GafetePerdidoResponse, String> {
    // Validar input
    validaciones::validar_reportar_input(&input)?;
    
    // Verificar que el gafete exista
    let gafete_existe = sqlx::query(
        "SELECT COUNT(*) as count, numero FROM gafetes WHERE id = ?"
    )
    .bind(&input.gafete_id)
    .fetch_one(&*pool)
    .await
    .map_err(|_| "Gafete no encontrado".to_string())?;
    
    let count: i32 = gafete_existe.get("count");
    if count == 0 {
        return Err("El gafete especificado no existe".to_string());
    }
    
    let numero: String = gafete_existe.get("numero");
    if numero.to_uppercase() == "S/G" {
        return Err("No se puede reportar como perdido el gafete S/G".to_string());
    }
    
    // Verificar que el contratista exista
    let contratista_existe = sqlx::query(
        "SELECT COUNT(*) as count FROM contratistas WHERE id = ?"
    )
    .bind(&input.contratista_id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Error al verificar contratista: {}", e))?;
    
    let cont_count: i32 = contratista_existe.get("count");
    if cont_count == 0 {
        return Err("El contratista especificado no existe".to_string());
    }
    
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    // Crear registro de pérdida
    sqlx::query(
        r#"INSERT INTO gafetes_perdidos 
           (id, gafete_id, contratista_id, ingreso_id, fecha_perdida, monto_cobro,
            estado_pago, fecha_pago, observaciones, reportado_por, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, NULL, ?, ?, ?, ?)"#
    )
    .bind(&id)
    .bind(&input.gafete_id)
    .bind(&input.contratista_id)
    .bind(&input.ingreso_id)
    .bind(&now)
    .bind(input.monto_cobro)
    .bind(EstadoPago::Pendiente.as_str())
    .bind(input.observaciones.as_deref())
    .bind(&input.reportado_por)
    .bind(&now)
    .bind(&now)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Error al reportar gafete perdido: {}", e))?;
    
    // Actualizar estado del gafete a 'perdido'
    let update_time = Utc::now().to_rfc3339();
    sqlx::query(
        r#"UPDATE gafetes SET
            estado = ?,
            contratista_asignado_id = NULL,
            ingreso_actual_id = NULL,
            updated_at = ?
        WHERE id = ?"#
    )
    .bind(EstadoGafete::Perdido.as_str())
    .bind(&update_time)
    .bind(&input.gafete_id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Error al actualizar estado del gafete: {}", e))?;
    
    get_gafete_perdido_by_id(pool, id).await
}

/// Obtiene un gafete perdido por ID
#[tauri::command]
pub async fn get_gafete_perdido_by_id(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<GafetePerdidoResponse, String> {
    let row = sqlx::query(
        r#"SELECT 
            gp.id, gp.gafete_id, gp.contratista_id, gp.ingreso_id,
            gp.fecha_perdida, gp.monto_cobro, gp.estado_pago, gp.fecha_pago,
            gp.observaciones, gp.reportado_por, gp.created_at, gp.updated_at,
            g.numero as gafete_numero,
            c.nombre as contratista_nombre,
            c.apellido as contratista_apellido,
            c.cedula as contratista_cedula,
            e.nombre as empresa_nombre,
            u.username as reportado_por_nombre
           FROM gafetes_perdidos gp
           INNER JOIN gafetes g ON gp.gafete_id = g.id
           INNER JOIN contratistas c ON gp.contratista_id = c.id
           INNER JOIN empresas e ON c.empresa_id = e.id
           INNER JOIN users u ON gp.reportado_por = u.id
           WHERE gp.id = ?"#
    )
    .bind(&id)
    .fetch_one(&*pool)
    .await
    .map_err(|_| "Registro no encontrado".to_string())?;
    
    let gafete_perdido = GafetePerdido {
        id: row.get("id"),
        gafete_id: row.get("gafete_id"),
        contratista_id: row.get("contratista_id"),
        ingreso_id: row.get("ingreso_id"),
        fecha_perdida: row.get("fecha_perdida"),
        monto_cobro: row.get("monto_cobro"),
        estado_pago: row.get("estado_pago"),
        fecha_pago: row.get("fecha_pago"),
        observaciones: row.get("observaciones"),
        reportado_por: row.get("reportado_por"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    };
    
    let nombre: String = row.get("contratista_nombre");
    let apellido: String = row.get("contratista_apellido");
    
    let mut response = GafetePerdidoResponse::from(gafete_perdido);
    response.gafete_numero = row.get("gafete_numero");
    response.contratista_nombre = format!("{} {}", nombre, apellido);
    response.contratista_cedula = row.get("contratista_cedula");
    response.empresa_nombre = row.get("empresa_nombre");
    response.reportado_por_nombre = row.get("reportado_por_nombre");
    
    Ok(response)
}

/// Obtiene todos los gafetes perdidos
#[tauri::command]
pub async fn get_all_gafetes_perdidos(
    pool: State<'_, SqlitePool>,
) -> Result<GafetesPerdidosListResponse, String> {
    let rows = sqlx::query(
        r#"SELECT 
            gp.id, gp.gafete_id, gp.contratista_id, gp.ingreso_id,
            gp.fecha_perdida, gp.monto_cobro, gp.estado_pago, gp.fecha_pago,
            gp.observaciones, gp.reportado_por, gp.created_at, gp.updated_at,
            g.numero as gafete_numero,
            c.nombre as contratista_nombre,
            c.apellido as contratista_apellido,
            c.cedula as contratista_cedula,
            e.nombre as empresa_nombre,
            u.username as reportado_por_nombre
           FROM gafetes_perdidos gp
           INNER JOIN gafetes g ON gp.gafete_id = g.id
           INNER JOIN contratistas c ON gp.contratista_id = c.id
           INNER JOIN empresas e ON c.empresa_id = e.id
           INNER JOIN users u ON gp.reportado_por = u.id
           ORDER BY gp.fecha_perdida DESC"#
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error al obtener gafetes perdidos: {}", e))?;
    
    let perdidos: Vec<GafetePerdidoResponse> = rows.into_iter()
        .map(|row| {
            let gafete_perdido = GafetePerdido {
                id: row.get("id"),
                gafete_id: row.get("gafete_id"),
                contratista_id: row.get("contratista_id"),
                ingreso_id: row.get("ingreso_id"),
                fecha_perdida: row.get("fecha_perdida"),
                monto_cobro: row.get("monto_cobro"),
                estado_pago: row.get("estado_pago"),
                fecha_pago: row.get("fecha_pago"),
                observaciones: row.get("observaciones"),
                reportado_por: row.get("reportado_por"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            
            let nombre: String = row.get("contratista_nombre");
            let apellido: String = row.get("contratista_apellido");
            
            let mut response = GafetePerdidoResponse::from(gafete_perdido);
            response.gafete_numero = row.get("gafete_numero");
            response.contratista_nombre = format!("{} {}", nombre, apellido);
            response.contratista_cedula = row.get("contratista_cedula");
            response.empresa_nombre = row.get("empresa_nombre");
            response.reportado_por_nombre = row.get("reportado_por_nombre");
            response
        })
        .collect();
    
    let total = perdidos.len();
    let pendientes = perdidos.iter().filter(|p| p.estado_pago == EstadoPago::Pendiente).count();
    let pagados = perdidos.iter().filter(|p| p.estado_pago == EstadoPago::Pagado).count();
    let condonados = perdidos.iter().filter(|p| p.estado_pago == EstadoPago::Condonado).count();
    
    let monto_total_pendiente: f64 = perdidos.iter()
        .filter(|p| p.estado_pago == EstadoPago::Pendiente)
        .map(|p| p.monto_cobro)
        .sum();
    
    Ok(GafetesPerdidosListResponse {
        perdidos,
        total,
        pendientes,
        pagados,
        condonados,
        monto_total_pendiente,
    })
}

/// Obtiene solo gafetes perdidos pendientes de pago
#[tauri::command]
pub async fn get_gafetes_perdidos_pendientes(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<GafetePerdidoResponse>, String> {
    let rows = sqlx::query(
        r#"SELECT 
            gp.id, gp.gafete_id, gp.contratista_id, gp.ingreso_id,
            gp.fecha_perdida, gp.monto_cobro, gp.estado_pago, gp.fecha_pago,
            gp.observaciones, gp.reportado_por, gp.created_at, gp.updated_at,
            g.numero as gafete_numero,
            c.nombre as contratista_nombre,
            c.apellido as contratista_apellido,
            c.cedula as contratista_cedula,
            e.nombre as empresa_nombre,
            u.username as reportado_por_nombre
           FROM gafetes_perdidos gp
           INNER JOIN gafetes g ON gp.gafete_id = g.id
           INNER JOIN contratistas c ON gp.contratista_id = c.id
           INNER JOIN empresas e ON c.empresa_id = e.id
           INNER JOIN users u ON gp.reportado_por = u.id
           WHERE gp.estado_pago = ?
           ORDER BY gp.fecha_perdida DESC"#
    )
    .bind(EstadoPago::Pendiente.as_str())
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error al obtener pendientes: {}", e))?;
    
    let perdidos = rows.into_iter()
        .map(|row| {
            let gafete_perdido = GafetePerdido {
                id: row.get("id"),
                gafete_id: row.get("gafete_id"),
                contratista_id: row.get("contratista_id"),
                ingreso_id: row.get("ingreso_id"),
                fecha_perdida: row.get("fecha_perdida"),
                monto_cobro: row.get("monto_cobro"),
                estado_pago: row.get("estado_pago"),
                fecha_pago: row.get("fecha_pago"),
                observaciones: row.get("observaciones"),
                reportado_por: row.get("reportado_por"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            
            let nombre: String = row.get("contratista_nombre");
            let apellido: String = row.get("contratista_apellido");
            
            let mut response = GafetePerdidoResponse::from(gafete_perdido);
            response.gafete_numero = row.get("gafete_numero");
            response.contratista_nombre = format!("{} {}", nombre, apellido);
            response.contratista_cedula = row.get("contratista_cedula");
            response.empresa_nombre = row.get("empresa_nombre");
            response.reportado_por_nombre = row.get("reportado_por_nombre");
            response
        })
        .collect();
    
    Ok(perdidos)
}

/// Obtiene deudas de un contratista específico
#[tauri::command]
pub async fn get_deudas_by_contratista(
    pool: State<'_, SqlitePool>,
    contratista_id: String,
) -> Result<DeudasContratistaResponse, String> {
    // Obtener datos del contratista
    let contratista_row = sqlx::query(
        "SELECT nombre, apellido, cedula FROM contratistas WHERE id = ?"
    )
    .bind(&contratista_id)
    .fetch_one(&*pool)
    .await
    .map_err(|_| "Contratista no encontrado".to_string())?;
    
    let nombre: String = contratista_row.get("nombre");
    let apellido: String = contratista_row.get("apellido");
    let cedula: String = contratista_row.get("cedula");
    
    // Obtener gafetes perdidos pendientes
    let rows = sqlx::query(
        r#"SELECT 
            gp.id, gp.gafete_id, gp.contratista_id, gp.ingreso_id,
            gp.fecha_perdida, gp.monto_cobro, gp.estado_pago, gp.fecha_pago,
            gp.observaciones, gp.reportado_por, gp.created_at, gp.updated_at,
            g.numero as gafete_numero,
            e.nombre as empresa_nombre,
            u.username as reportado_por_nombre
           FROM gafetes_perdidos gp
           INNER JOIN gafetes g ON gp.gafete_id = g.id
           INNER JOIN contratistas c ON gp.contratista_id = c.id
           INNER JOIN empresas e ON c.empresa_id = e.id
           INNER JOIN users u ON gp.reportado_por = u.id
           WHERE gp.contratista_id = ? AND gp.estado_pago = ?
           ORDER BY gp.fecha_perdida DESC"#
    )
    .bind(&contratista_id)
    .bind(EstadoPago::Pendiente.as_str())
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error al obtener deudas: {}", e))?;
    
    let gafetes_perdidos: Vec<GafetePerdidoResponse> = rows.into_iter()
        .map(|row| {
            let gafete_perdido = GafetePerdido {
                id: row.get("id"),
                gafete_id: row.get("gafete_id"),
                contratista_id: contratista_id.clone(),
                ingreso_id: row.get("ingreso_id"),
                fecha_perdida: row.get("fecha_perdida"),
                monto_cobro: row.get("monto_cobro"),
                estado_pago: row.get("estado_pago"),
                fecha_pago: row.get("fecha_pago"),
                observaciones: row.get("observaciones"),
                reportado_por: row.get("reportado_por"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            
            let mut response = GafetePerdidoResponse::from(gafete_perdido);
            response.gafete_numero = row.get("gafete_numero");
            response.contratista_nombre = format!("{} {}", nombre, apellido);
            response.contratista_cedula = cedula.clone();
            response.empresa_nombre = row.get("empresa_nombre");
            response.reportado_por_nombre = row.get("reportado_por_nombre");
            response
        })
        .collect();
    
    let total_deuda: f64 = gafetes_perdidos.iter().map(|g| g.monto_cobro).sum();
    let cantidad_gafetes_perdidos = gafetes_perdidos.len();
    
    Ok(DeudasContratistaResponse {
        contratista_id,
        contratista_nombre: format!("{} {}", nombre, apellido),
        contratista_cedula: cedula,
        gafetes_perdidos,
        total_deuda,
        cantidad_gafetes_perdidos,
    })
}

/// Registra el pago de un gafete perdido
#[tauri::command]
pub async fn registrar_pago_gafete(
    pool: State<'_, SqlitePool>,
    id: String,
    input: RegistrarPagoInput,
) -> Result<GafetePerdidoResponse, String> {
    let fecha_pago = input.fecha_pago.unwrap_or_else(|| {
        Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
    });
    
    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    sqlx::query(
        r#"UPDATE gafetes_perdidos SET
            estado_pago = ?,
            fecha_pago = ?,
            updated_at = ?
        WHERE id = ?"#
    )
    .bind(EstadoPago::Pagado.as_str())
    .bind(&fecha_pago)
    .bind(&now)
    .bind(&id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Error al registrar pago: {}", e))?;
    
    get_gafete_perdido_by_id(pool, id).await
}

/// Condona la deuda de un gafete perdido
#[tauri::command]
pub async fn condonar_deuda_gafete(
    pool: State<'_, SqlitePool>,
    id: String,
    input: CondonarDeudaInput,
) -> Result<GafetePerdidoResponse, String> {
    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    let observaciones_final = if let Some(obs) = input.observaciones {
        Some(format!("CONDONADO: {}", obs))
    } else {
        Some("DEUDA CONDONADA".to_string())
    };
    
    sqlx::query(
        r#"UPDATE gafetes_perdidos SET
            estado_pago = ?,
            observaciones = ?,
            updated_at = ?
        WHERE id = ?"#
    )
    .bind(EstadoPago::Condonado.as_str())
    .bind(observaciones_final)
    .bind(&now)
    .bind(&id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Error al condonar deuda: {}", e))?;
    
    get_gafete_perdido_by_id(pool, id).await
}

/// Elimina un registro de gafete perdido (solo Admin)
#[tauri::command]
pub async fn delete_gafete_perdido(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    sqlx::query("DELETE FROM gafetes_perdidos WHERE id = ?")
        .bind(&id)
        .execute(&*pool)
        .await
        .map_err(|e| format!("Error al eliminar registro: {}", e))?;
    
    Ok(())
}