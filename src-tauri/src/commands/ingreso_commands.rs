// ==========================================
// src/commands/ingreso_commands.rs
// ==========================================
use crate::models::ingreso::{
    Ingreso, IngresoResponse, IngresoListResponse, ValidacionIngresoResponse,
    CreateIngresoContratistaInput, CreateIngresoTemporalInput, RegistrarSalidaInput,
    RegistrarSalidaConGafetePerdidoInput, TipoIngreso, TipoAutorizacion, ModoIngreso, validaciones,
};
use crate::models::gafete::EstadoGafete;
use crate::commands::gafete_commands::asignar_gafete;
use crate::commands::gafete_perdido_commands::reportar_gafete_perdido;
use crate::models::gafete::AsignarGafeteInput;
use crate::models::gafete_perdido::ReportarGafetePerdidoInput;
use sqlx::{SqlitePool, Row};
use tauri::State;
use uuid::Uuid;
use chrono::{NaiveDateTime, Utc};

/// Valida si un contratista puede ingresar (COMANDO CRÍTICO)
#[tauri::command]
pub async fn validar_ingreso_contratista(
    pool: State<'_, SqlitePool>,
    cedula: String,
) -> Result<ValidacionIngresoResponse, String> {
    validaciones::validar_cedula(&cedula)?;
    
    let mut alertas: Vec<String> = Vec::new();
    let mut puede_ingresar = true;
    let mut motivo_rechazo: Option<String> = None;
    
    // 1. Verificar si está en lista negra
    let blocked = sqlx::query(
        "SELECT COUNT(*) as count, motivo_bloqueo FROM lista_negra WHERE cedula = ? AND is_active = 1"
    )
    .bind(&cedula)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Error al verificar lista negra: {}", e))?;
    
    let blocked_count: i32 = blocked.get("count");
    if blocked_count > 0 {
        let motivo: String = blocked.get("motivo_bloqueo");
        puede_ingresar = false;
        motivo_rechazo = Some(format!("BLOQUEADO - En lista negra. Motivo: {}", motivo));
        
        return Ok(ValidacionIngresoResponse {
            puede_ingresar,
            motivo_rechazo,
            alertas,
            contratista: None,
            tiene_ingreso_abierto: false,
            ingreso_abierto: None,
        });
    }
    
    // 2. Buscar contratista
    let contratista_row = sqlx::query(
        r#"SELECT 
            c.id, c.cedula, c.nombre, c.apellido, c.empresa_id, 
            c.fecha_vencimiento_praind, c.estado,
            e.nombre as empresa_nombre
           FROM contratistas c
           INNER JOIN empresas e ON c.empresa_id = e.id
           WHERE c.cedula = ?"#
    )
    .bind(&cedula)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Error al buscar contratista: {}", e))?;
    
    if let Some(row) = contratista_row {
        let contratista_id: String = row.get("id");
        let estado: String = row.get("estado");
        let fecha_vencimiento_praind: String = row.get("fecha_vencimiento_praind");
        
        // 3. Validar estado activo
        if estado != "activo" {
            puede_ingresar = false;
            motivo_rechazo = Some(format!("DENEGADO - Contratista inactivo (estado: {})", estado));
        }
        
        // 4. Validar PRAIND vigente
        let fecha_venc = NaiveDateTime::parse_from_str(
            &format!("{} 00:00:00", fecha_vencimiento_praind),
            "%Y-%m-%d %H:%M:%S"
        ).map_err(|_| "Error al parsear fecha PRAIND")?;
        
        let hoy = Utc::now().naive_utc();
        
        if fecha_venc < hoy {
            puede_ingresar = false;
            motivo_rechazo = Some(format!("DENEGADO - PRAIND vencido (venció el: {})", fecha_vencimiento_praind));
        } else {
            let dias_restantes = (fecha_venc - hoy).num_days();
            if dias_restantes <= 30 {
                alertas.push(format!("⚠️ PRAIND vence pronto ({} días restantes)", dias_restantes));
            }
        }
        
        // 5. Verificar ingreso abierto
        let ingreso_abierto = sqlx::query(
            "SELECT id FROM ingresos WHERE cedula = ? AND fecha_hora_salida IS NULL"
        )
        .bind(&cedula)
        .fetch_optional(&*pool)
        .await
        .map_err(|e| format!("Error al verificar ingreso abierto: {}", e))?;
        
        let (tiene_ingreso_abierto, ingreso_abierto_data) = if let Some(ing_row) = ingreso_abierto {
            let ing_id: String = ing_row.get("id");
            let ingreso_data = get_ingreso_by_id(pool.clone(), ing_id).await.ok();
            (true, ingreso_data)
        } else {
            (false, None)
        };
        
        // 6. Verificar deudas de gafetes
        let deudas = sqlx::query(
            r#"SELECT gp.monto_cobro, g.numero 
               FROM gafetes_perdidos gp
               INNER JOIN gafetes g ON gp.gafete_id = g.id
               WHERE gp.contratista_id = ? AND gp.estado_pago = 'pendiente'"#
        )
        .bind(&contratista_id)
        .fetch_all(&*pool)
        .await
        .map_err(|e| format!("Error al verificar deudas: {}", e))?;
        
        for deuda in deudas {
            let monto: f64 = deuda.get("monto_cobro");
            let numero: String = deuda.get("numero");
            alertas.push(format!("⚠️ Debe gafete #{} - Monto: ${:.2}", numero, monto));
        }
        
        // Construir objeto contratista
        let contratista_json = serde_json::json!({
            "id": contratista_id,
            "cedula": row.get::<String, _>("cedula"),
            "nombre": row.get::<String, _>("nombre"),
            "apellido": row.get::<String, _>("apellido"),
            "empresa_nombre": row.get::<String, _>("empresa_nombre"),
            "fecha_vencimiento_praind": fecha_vencimiento_praind,
            "estado": estado,
        });
        
        Ok(ValidacionIngresoResponse {
            puede_ingresar,
            motivo_rechazo,
            alertas,
            contratista: Some(contratista_json),
            tiene_ingreso_abierto,
            ingreso_abierto: ingreso_abierto_data,
        })
    } else {
        // Contratista NO existe
        Ok(ValidacionIngresoResponse {
            puede_ingresar: true,  // Puede hacer ingreso temporal o registrarse
            motivo_rechazo: None,
            alertas: vec!["ℹ️ Contratista no registrado. Puede crear registro o ingreso temporal.".to_string()],
            contratista: None,
            tiene_ingreso_abierto: false,
            ingreso_abierto: None,
        })
    }
}

/// Crea ingreso para contratista existente
#[tauri::command]
pub async fn create_ingreso_contratista(
    pool: State<'_, SqlitePool>,
    input: CreateIngresoContratistaInput,
) -> Result<IngresoResponse, String> {
    // Obtener datos del contratista
    let contratista_row = sqlx::query(
        r#"SELECT 
            c.cedula, c.nombre, c.apellido, c.fecha_vencimiento_praind, c.estado,
            e.nombre as empresa_nombre
           FROM contratistas c
           INNER JOIN empresas e ON c.empresa_id = e.id
           WHERE c.id = ?"#
    )
    .bind(&input.contratista_id)
    .fetch_one(&*pool)
    .await
    .map_err(|_| "Contratista no encontrado".to_string())?;
    
    let cedula: String = contratista_row.get("cedula");
    let nombre: String = contratista_row.get("nombre");
    let apellido: String = contratista_row.get("apellido");
    let empresa_nombre: String = contratista_row.get("empresa_nombre");
    let fecha_vencimiento_praind: String = contratista_row.get("fecha_vencimiento_praind");
    let estado: String = contratista_row.get("estado");
    
    // Validar que no tenga ingreso abierto
    let ingreso_existe = sqlx::query(
        "SELECT COUNT(*) as count FROM ingresos WHERE cedula = ? AND fecha_hora_salida IS NULL"
    )
    .bind(&cedula)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Error al verificar ingreso: {}", e))?;
    
    let count: i32 = ingreso_existe.get("count");
    if count > 0 {
        return Err("Ya tiene un ingreso abierto. Debe cerrar el anterior primero.".to_string());
    }
    
    // Determinar modo de ingreso
    let (modo_ingreso, vehiculo_id, placa_temporal): (ModoIngreso, Option<String>, Option<String>) = if let Some(ref veh_id) = input.vehiculo_id {
        (ModoIngreso::Vehiculo, Some(veh_id.clone()), None)
    } else {
        (ModoIngreso::Caminando, None, None)
    };
    
    // Obtener número de gafete
    let gafete_row = sqlx::query(
        "SELECT numero FROM gafetes WHERE id = ?"
    )
    .bind(&input.gafete_id)
    .fetch_one(&*pool)
    .await
    .map_err(|_| "Gafete no encontrado".to_string())?;
    
    let gafete_numero: String = gafete_row.get("numero");
    
    // Validar PRAIND vigente
    let fecha_venc = NaiveDateTime::parse_from_str(
        &format!("{} 00:00:00", fecha_vencimiento_praind),
        "%Y-%m-%d %H:%M:%S"
    ).map_err(|_| "Error al parsear fecha PRAIND")?;
    
    let praind_vigente = fecha_venc >= Utc::now().naive_utc();
    
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let now_rfc = Utc::now().to_rfc3339();
    
    // Crear ingreso
    sqlx::query(
        r#"INSERT INTO ingresos 
           (id, contratista_id, cedula, nombre, apellido, empresa_nombre,
            tipo_ingreso, tipo_autorizacion, modo_ingreso, vehiculo_id, placa_temporal,
            gafete_id, gafete_numero, fecha_hora_ingreso, fecha_hora_salida, tiempo_permanencia_minutos,
            usuario_ingreso_id, usuario_salida_id, praind_vigente_al_ingreso, estado_contratista_al_ingreso,
            observaciones, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NULL, NULL, ?, NULL, ?, ?, ?, ?, ?)"#
    )
    .bind(&id)
    .bind(&input.contratista_id)
    .bind(&cedula)
    .bind(&nombre)
    .bind(&apellido)
    .bind(&empresa_nombre)
    .bind(TipoIngreso::Contratista.as_str())
    .bind(TipoAutorizacion::Praind.as_str())
    .bind(modo_ingreso.as_str())
    .bind(&vehiculo_id)
    .bind(&placa_temporal)
    .bind(&input.gafete_id)
    .bind(&gafete_numero)
    .bind(&now)
    .bind(&input.usuario_ingreso_id)
    .bind(praind_vigente)
    .bind(&estado)
    .bind(input.observaciones.as_deref())
    .bind(&now_rfc)
    .bind(&now_rfc)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Error al crear ingreso: {}", e))?;
    
    // Si asignó gafete (no es S/G), llamar a asignar_gafete
    if gafete_numero.to_uppercase() != "S/G" {
        let asignar_input = AsignarGafeteInput {
            contratista_id: input.contratista_id.clone(),
            ingreso_id: id.clone(),
        };
        
        asignar_gafete(pool.clone(), input.gafete_id.clone(), asignar_input)
            .await
            .map_err(|e| format!("Error al asignar gafete: {}", e))?;
    }
    
    get_ingreso_by_id(pool, id).await
}

/// Crea ingreso temporal (autorización por correo)
#[tauri::command]
pub async fn create_ingreso_temporal(
    pool: State<'_, SqlitePool>,
    input: CreateIngresoTemporalInput,
) -> Result<IngresoResponse, String> {
    // Validar input
    validaciones::validar_create_temporal_input(&input)?;
    
    // Validar que no esté en lista negra
    let blocked = sqlx::query(
        "SELECT COUNT(*) as count FROM lista_negra WHERE cedula = ? AND is_active = 1"
    )
    .bind(&input.cedula)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Error al verificar lista negra: {}", e))?;
    
    let blocked_count: i32 = blocked.get("count");
    if blocked_count > 0 {
        return Err("No se puede crear ingreso. La persona está en lista negra.".to_string());
    }
    
    // Validar que no tenga ingreso abierto
    let ingreso_existe = sqlx::query(
        "SELECT COUNT(*) as count FROM ingresos WHERE cedula = ? AND fecha_hora_salida IS NULL"
    )
    .bind(&input.cedula)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Error al verificar ingreso: {}", e))?;
    
    let count: i32 = ingreso_existe.get("count");
    if count > 0 {
        return Err("Ya tiene un ingreso abierto. Debe cerrar el anterior primero.".to_string());
    }
    
    // Determinar modo de ingreso
    let (modo_ingreso, placa_temporal) = if input.placa_temporal.is_some() {
        (ModoIngreso::VehiculoTemporal, input.placa_temporal.clone())
    } else {
        (ModoIngreso::Caminando, None)
    };
    
    // Obtener número de gafete
    let gafete_row = sqlx::query(
        "SELECT numero FROM gafetes WHERE id = ?"
    )
    .bind(&input.gafete_id)
    .fetch_one(&*pool)
    .await
    .map_err(|_| "Gafete no encontrado".to_string())?;
    
    let gafete_numero: String = gafete_row.get("numero");
    
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let now_rfc = Utc::now().to_rfc3339();
    
    // Crear ingreso temporal
    sqlx::query(
        r#"INSERT INTO ingresos 
           (id, contratista_id, cedula, nombre, apellido, empresa_nombre,
            tipo_ingreso, tipo_autorizacion, modo_ingreso, vehiculo_id, placa_temporal,
            gafete_id, gafete_numero, fecha_hora_ingreso, fecha_hora_salida, tiempo_permanencia_minutos,
            usuario_ingreso_id, usuario_salida_id, praind_vigente_al_ingreso, estado_contratista_al_ingreso,
            observaciones, created_at, updated_at)
           VALUES (?, NULL, ?, ?, ?, ?, ?, ?, ?, NULL, ?, ?, ?, ?, NULL, NULL, ?, NULL, NULL, NULL, ?, ?, ?)"#
    )
    .bind(&id)
    .bind(input.cedula.trim())
    .bind(input.nombre.trim())
    .bind(input.apellido.trim())
    .bind(input.empresa_nombre.trim())
    .bind(TipoIngreso::Temporal.as_str())
    .bind(TipoAutorizacion::Correo.as_str())
    .bind(modo_ingreso.as_str())
    .bind(&placa_temporal)
    .bind(&input.gafete_id)
    .bind(&gafete_numero)
    .bind(&now)
    .bind(&input.usuario_ingreso_id)
    .bind(input.observaciones.as_deref())
    .bind(&now_rfc)
    .bind(&now_rfc)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Error al crear ingreso temporal: {}", e))?;
    
    // Si asignó gafete (no es S/G), asignar pero sin contratista_id
    if gafete_numero.to_uppercase() != "S/G" {
        // Para temporales, actualizamos manualmente el gafete
        let update_time = Utc::now().to_rfc3339();
        sqlx::query(
            r#"UPDATE gafetes SET
                estado = ?,
                ingreso_actual_id = ?,
                updated_at = ?
            WHERE id = ?"#
        )
        .bind(EstadoGafete::Asignado.as_str())
        .bind(&id)
        .bind(&update_time)
        .bind(&input.gafete_id)
        .execute(&*pool)
        .await
        .map_err(|e| format!("Error al asignar gafete: {}", e))?;
    }
    
    get_ingreso_by_id(pool, id).await
}

/// Obtiene un ingreso por ID con JOINs completos
#[tauri::command]
pub async fn get_ingreso_by_id(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<IngresoResponse, String> {
    let row = sqlx::query(
        r#"SELECT 
            i.id, i.contratista_id, i.cedula, i.nombre, i.apellido, i.empresa_nombre,
            i.tipo_ingreso, i.tipo_autorizacion, i.modo_ingreso, i.vehiculo_id, i.placa_temporal,
            i.gafete_id, i.gafete_numero, i.fecha_hora_ingreso, i.fecha_hora_salida,
            i.tiempo_permanencia_minutos, i.usuario_ingreso_id, i.usuario_salida_id,
            i.praind_vigente_al_ingreso, i.estado_contratista_al_ingreso, i.observaciones,
            i.created_at, i.updated_at,
            u_ing.username as usuario_ingreso_nombre,
            u_sal.username as usuario_salida_nombre,
            v.placa as vehiculo_placa
           FROM ingresos i
           LEFT JOIN users u_ing ON i.usuario_ingreso_id = u_ing.id
           LEFT JOIN users u_sal ON i.usuario_salida_id = u_sal.id
           LEFT JOIN vehiculos v ON i.vehiculo_id = v.id
           WHERE i.id = ?"#
    )
    .bind(&id)
    .fetch_one(&*pool)
    .await
    .map_err(|_| "Ingreso no encontrado".to_string())?;
    
    let ingreso = Ingreso {
        id: row.get("id"),
        contratista_id: row.get("contratista_id"),
        cedula: row.get("cedula"),
        nombre: row.get("nombre"),
        apellido: row.get("apellido"),
        empresa_nombre: row.get("empresa_nombre"),
        tipo_ingreso: row.get("tipo_ingreso"),
        tipo_autorizacion: row.get("tipo_autorizacion"),
        modo_ingreso: row.get("modo_ingreso"),
        vehiculo_id: row.get("vehiculo_id"),
        placa_temporal: row.get("placa_temporal"),
        gafete_id: row.get("gafete_id"),
        gafete_numero: row.get("gafete_numero"),
        fecha_hora_ingreso: row.get("fecha_hora_ingreso"),
        fecha_hora_salida: row.get("fecha_hora_salida"),
        tiempo_permanencia_minutos: row.get("tiempo_permanencia_minutos"),
        usuario_ingreso_id: row.get("usuario_ingreso_id"),
        usuario_salida_id: row.get("usuario_salida_id"),
        praind_vigente_al_ingreso: row.get("praind_vigente_al_ingreso"),
        estado_contratista_al_ingreso: row.get("estado_contratista_al_ingreso"),
        observaciones: row.get("observaciones"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    };
    
    let mut response = IngresoResponse::from(ingreso);
    response.usuario_ingreso_nombre = row.get("usuario_ingreso_nombre");
    response.usuario_salida_nombre = row.get("usuario_salida_nombre");
    response.vehiculo_placa = row.get("vehiculo_placa");
    
    Ok(response)
}

/// Obtiene todos los ingresos (con paginación implícita)
#[tauri::command]
pub async fn get_all_ingresos(
    pool: State<'_, SqlitePool>,
) -> Result<IngresoListResponse, String> {
    let rows = sqlx::query(
        r#"SELECT 
            i.id, i.contratista_id, i.cedula, i.nombre, i.apellido, i.empresa_nombre,
            i.tipo_ingreso, i.tipo_autorizacion, i.modo_ingreso, i.vehiculo_id, i.placa_temporal,
            i.gafete_id, i.gafete_numero, i.fecha_hora_ingreso, i.fecha_hora_salida,
            i.tiempo_permanencia_minutos, i.usuario_ingreso_id, i.usuario_salida_id,
            i.praind_vigente_al_ingreso, i.estado_contratista_al_ingreso, i.observaciones,
            i.created_at, i.updated_at,
            u_ing.username as usuario_ingreso_nombre,
            u_sal.username as usuario_salida_nombre,
            v.placa as vehiculo_placa
           FROM ingresos i
           LEFT JOIN users u_ing ON i.usuario_ingreso_id = u_ing.id
           LEFT JOIN users u_sal ON i.usuario_salida_id = u_sal.id
           LEFT JOIN vehiculos v ON i.vehiculo_id = v.id
           ORDER BY i.fecha_hora_ingreso DESC
           LIMIT 500"#
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error al obtener ingresos: {}", e))?;
    
    let ingresos: Vec<IngresoResponse> = rows.into_iter()
        .map(|row| {
            let ingreso = Ingreso {
                id: row.get("id"),
                contratista_id: row.get("contratista_id"),
                cedula: row.get("cedula"),
                nombre: row.get("nombre"),
                apellido: row.get("apellido"),
                empresa_nombre: row.get("empresa_nombre"),
                tipo_ingreso: row.get("tipo_ingreso"),
                tipo_autorizacion: row.get("tipo_autorizacion"),
                modo_ingreso: row.get("modo_ingreso"),
                vehiculo_id: row.get("vehiculo_id"),
                placa_temporal: row.get("placa_temporal"),
                gafete_id: row.get("gafete_id"),
                gafete_numero: row.get("gafete_numero"),
                fecha_hora_ingreso: row.get("fecha_hora_ingreso"),
                fecha_hora_salida: row.get("fecha_hora_salida"),
                tiempo_permanencia_minutos: row.get("tiempo_permanencia_minutos"),
                usuario_ingreso_id: row.get("usuario_ingreso_id"),
                usuario_salida_id: row.get("usuario_salida_id"),
                praind_vigente_al_ingreso: row.get("praind_vigente_al_ingreso"),
                estado_contratista_al_ingreso: row.get("estado_contratista_al_ingreso"),
                observaciones: row.get("observaciones"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            
            let mut response = IngresoResponse::from(ingreso);
            response.usuario_ingreso_nombre = row.get("usuario_ingreso_nombre");
            response.usuario_salida_nombre = row.get("usuario_salida_nombre");
            response.vehiculo_placa = row.get("vehiculo_placa");
            response
        })
        .collect();
    
    let total = ingresos.len();
    let adentro = ingresos.iter().filter(|i| i.esta_adentro).count();
    let salieron = total - adentro;
    
    Ok(IngresoListResponse {
        ingresos,
        total,
        adentro,
        salieron,
    })
}

/// Obtiene solo ingresos abiertos (personas adentro)
#[tauri::command]
pub async fn get_ingresos_abiertos(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<IngresoResponse>, String> {
    let rows = sqlx::query(
        r#"SELECT 
            i.id, i.contratista_id, i.cedula, i.nombre, i.apellido, i.empresa_nombre,
            i.tipo_ingreso, i.tipo_autorizacion, i.modo_ingreso, i.vehiculo_id, i.placa_temporal,
            i.gafete_id, i.gafete_numero, i.fecha_hora_ingreso, i.fecha_hora_salida,
            i.tiempo_permanencia_minutos, i.usuario_ingreso_id, i.usuario_salida_id,
            i.praind_vigente_al_ingreso, i.estado_contratista_al_ingreso, i.observaciones,
            i.created_at, i.updated_at,
            u_ing.username as usuario_ingreso_nombre,
            u_sal.username as usuario_salida_nombre,
            v.placa as vehiculo_placa
           FROM ingresos i
           LEFT JOIN users u_ing ON i.usuario_ingreso_id = u_ing.id
           LEFT JOIN users u_sal ON i.usuario_salida_id = u_sal.id
           LEFT JOIN vehiculos v ON i.vehiculo_id = v.id
           WHERE i.fecha_hora_salida IS NULL
           ORDER BY i.fecha_hora_ingreso DESC"#
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error al obtener ingresos abiertos: {}", e))?;
    
    let ingresos = rows.into_iter()
        .map(|row| {
            let ingreso = Ingreso {
                id: row.get("id"),
                contratista_id: row.get("contratista_id"),
                cedula: row.get("cedula"),
                nombre: row.get("nombre"),
                apellido: row.get("apellido"),
                empresa_nombre: row.get("empresa_nombre"),
                tipo_ingreso: row.get("tipo_ingreso"),
                tipo_autorizacion: row.get("tipo_autorizacion"),
                modo_ingreso: row.get("modo_ingreso"),
                vehiculo_id: row.get("vehiculo_id"),
                placa_temporal: row.get("placa_temporal"),
                gafete_id: row.get("gafete_id"),
                gafete_numero: row.get("gafete_numero"),
                fecha_hora_ingreso: row.get("fecha_hora_ingreso"),
                fecha_hora_salida: row.get("fecha_hora_salida"),
                tiempo_permanencia_minutos: row.get("tiempo_permanencia_minutos"),
                usuario_ingreso_id: row.get("usuario_ingreso_id"),
                usuario_salida_id: row.get("usuario_salida_id"),
                praind_vigente_al_ingreso: row.get("praind_vigente_al_ingreso"),
                estado_contratista_al_ingreso: row.get("estado_contratista_al_ingreso"),
                observaciones: row.get("observaciones"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            
            let mut response = IngresoResponse::from(ingreso);
            response.usuario_ingreso_nombre = row.get("usuario_ingreso_nombre");
            response.usuario_salida_nombre = row.get("usuario_salida_nombre");
            response.vehiculo_placa = row.get("vehiculo_placa");
            response
        })
        .collect();
    
    Ok(ingresos)
}

/// Busca ingreso abierto por número de gafete
#[tauri::command]
pub async fn get_ingreso_by_gafete(
    pool: State<'_, SqlitePool>,
    gafete_numero: String,
) -> Result<IngresoResponse, String> {
    let numero_normalizado = gafete_numero.trim().to_uppercase();
    
    let row = sqlx::query(
        r#"SELECT 
            i.id, i.contratista_id, i.cedula, i.nombre, i.apellido, i.empresa_nombre,
            i.tipo_ingreso, i.tipo_autorizacion, i.modo_ingreso, i.vehiculo_id, i.placa_temporal,
            i.gafete_id, i.gafete_numero, i.fecha_hora_ingreso, i.fecha_hora_salida,
            i.tiempo_permanencia_minutos, i.usuario_ingreso_id, i.usuario_salida_id,
            i.praind_vigente_al_ingreso, i.estado_contratista_al_ingreso, i.observaciones,
            i.created_at, i.updated_at,
            u_ing.username as usuario_ingreso_nombre,
            u_sal.username as usuario_salida_nombre,
            v.placa as vehiculo_placa
           FROM ingresos i
           LEFT JOIN users u_ing ON i.usuario_ingreso_id = u_ing.id
           LEFT JOIN users u_sal ON i.usuario_salida_id = u_sal.id
           LEFT JOIN vehiculos v ON i.vehiculo_id = v.id
           WHERE i.gafete_numero = ? AND i.fecha_hora_salida IS NULL"#
    )
    .bind(&numero_normalizado)
    .fetch_one(&*pool)
    .await
    .map_err(|_| format!("No se encontró ingreso abierto con gafete #{}", numero_normalizado))?;
    
    let ingreso = Ingreso {
        id: row.get("id"),
        contratista_id: row.get("contratista_id"),
        cedula: row.get("cedula"),
        nombre: row.get("nombre"),
        apellido: row.get("apellido"),
        empresa_nombre: row.get("empresa_nombre"),
        tipo_ingreso: row.get("tipo_ingreso"),
        tipo_autorizacion: row.get("tipo_autorizacion"),
        modo_ingreso: row.get("modo_ingreso"),
        vehiculo_id: row.get("vehiculo_id"),
        placa_temporal: row.get("placa_temporal"),
        gafete_id: row.get("gafete_id"),
        gafete_numero: row.get("gafete_numero"),
        fecha_hora_ingreso: row.get("fecha_hora_ingreso"),
        fecha_hora_salida: row.get("fecha_hora_salida"),
        tiempo_permanencia_minutos: row.get("tiempo_permanencia_minutos"),
        usuario_ingreso_id: row.get("usuario_ingreso_id"),
        usuario_salida_id: row.get("usuario_salida_id"),
        praind_vigente_al_ingreso: row.get("praind_vigente_al_ingreso"),
        estado_contratista_al_ingreso: row.get("estado_contratista_al_ingreso"),
        observaciones: row.get("observaciones"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    };
    
    let mut response = IngresoResponse::from(ingreso);
    response.usuario_ingreso_nombre = row.get("usuario_ingreso_nombre");
    response.usuario_salida_nombre = row.get("usuario_salida_nombre");
    response.vehiculo_placa = row.get("vehiculo_placa");
    
    Ok(response)
}

/// Registra salida normal (con devolución de gafete)
#[tauri::command]
pub async fn registrar_salida(
    pool: State<'_, SqlitePool>,
    ingreso_id: String,
    input: RegistrarSalidaInput,
) -> Result<IngresoResponse, String> {
    // Obtener datos del ingreso
    let ingreso_row = sqlx::query(
        "SELECT fecha_hora_ingreso, gafete_id, gafete_numero FROM ingresos WHERE id = ?"
    )
    .bind(&ingreso_id)
    .fetch_one(&*pool)
    .await
    .map_err(|_| "Ingreso no encontrado".to_string())?;
    
    let fecha_hora_ingreso_str: String = ingreso_row.get("fecha_hora_ingreso");
    let gafete_id: String = ingreso_row.get("gafete_id");
    let gafete_numero: String = ingreso_row.get("gafete_numero");
    
    // Calcular tiempo de permanencia
    let fecha_ingreso = NaiveDateTime::parse_from_str(&fecha_hora_ingreso_str, "%Y-%m-%d %H:%M:%S")
        .map_err(|_| "Error al parsear fecha de ingreso".to_string())?;
    
    let fecha_salida = Utc::now().naive_utc();
    let duracion = fecha_salida - fecha_ingreso;
    let tiempo_permanencia_minutos = duracion.num_minutes();
    
    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let now_rfc = Utc::now().to_rfc3339();
    
    // Actualizar observaciones si se proporcionaron
    let observaciones_update = input.observaciones_salida.as_deref();
    
    // Registrar salida
    sqlx::query(
        r#"UPDATE ingresos SET
            fecha_hora_salida = ?,
            tiempo_permanencia_minutos = ?,
            usuario_salida_id = ?,
            observaciones = COALESCE(?, observaciones),
            updated_at = ?
        WHERE id = ?"#
    )
    .bind(&now)
    .bind(tiempo_permanencia_minutos)
    .bind(&input.usuario_salida_id)
    .bind(observaciones_update)
    .bind(&now_rfc)
    .bind(&ingreso_id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Error al registrar salida: {}", e))?;
    
    // Si tenía gafete asignado y lo devolvió, liberar
    if gafete_numero.to_uppercase() != "S/G" && input.devolvio_gafete {
        use crate::commands::gafete_commands::liberar_gafete;
        liberar_gafete(pool.clone(), gafete_id)
            .await
            .map_err(|e| format!("Error al liberar gafete: {}", e))?;
    }
    
    get_ingreso_by_id(pool, ingreso_id).await
}

/// Registra salida con gafete perdido
#[tauri::command]
pub async fn registrar_salida_con_gafete_perdido(
    pool: State<'_, SqlitePool>,
    ingreso_id: String,
    input: RegistrarSalidaConGafetePerdidoInput,
) -> Result<IngresoResponse, String> {
    // Obtener datos del ingreso
    let ingreso_row = sqlx::query(
        "SELECT fecha_hora_ingreso, gafete_id, gafete_numero, contratista_id FROM ingresos WHERE id = ?"
    )
    .bind(&ingreso_id)
    .fetch_one(&*pool)
    .await
    .map_err(|_| "Ingreso no encontrado".to_string())?;
    
    let fecha_hora_ingreso_str: String = ingreso_row.get("fecha_hora_ingreso");
    let gafete_id: String = ingreso_row.get("gafete_id");
    let gafete_numero: String = ingreso_row.get("gafete_numero");
    let contratista_id: Option<String> = ingreso_row.get("contratista_id");
    
    if gafete_numero.to_uppercase() == "S/G" {
        return Err("No se puede reportar como perdido el gafete S/G".to_string());
    }
    
    // Calcular tiempo de permanencia
    let fecha_ingreso = NaiveDateTime::parse_from_str(&fecha_hora_ingreso_str, "%Y-%m-%d %H:%M:%S")
        .map_err(|_| "Error al parsear fecha de ingreso".to_string())?;
    
    let fecha_salida = Utc::now().naive_utc();
    let duracion = fecha_salida - fecha_ingreso;
    let tiempo_permanencia_minutos = duracion.num_minutes();
    
    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let now_rfc = Utc::now().to_rfc3339();
    
    // Registrar salida
    sqlx::query(
        r#"UPDATE ingresos SET
            fecha_hora_salida = ?,
            tiempo_permanencia_minutos = ?,
            usuario_salida_id = ?,
            observaciones = COALESCE(?, observaciones),
            updated_at = ?
        WHERE id = ?"#
    )
    .bind(&now)
    .bind(tiempo_permanencia_minutos)
    .bind(&input.usuario_salida_id)
    .bind(input.observaciones.as_deref())
    .bind(&now_rfc)
    .bind(&ingreso_id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Error al registrar salida: {}", e))?;
    
    // Reportar gafete como perdido
    // Si es temporal (sin contratista_id), necesitamos obtener cedula para crear contratista temporal
    let contratista_final_id = if let Some(cid) = contratista_id {
        cid
    } else {
        // Para temporales, necesitamos un contratista_id para el reporte
        // Opción: crear un registro temporal o manejar de otra forma
        return Err("No se puede reportar gafete perdido para ingreso temporal sin contratista registrado".to_string());
    };
    
    let reportar_input = ReportarGafetePerdidoInput {
        gafete_id: gafete_id.clone(),
        contratista_id: contratista_final_id,
        ingreso_id: Some(ingreso_id.clone()),
        monto_cobro: input.monto_cobro,
        observaciones: input.observaciones,
        reportado_por: input.usuario_salida_id.clone(),
    };
    
    reportar_gafete_perdido(pool.clone(), reportar_input)
        .await
        .map_err(|e| format!("Error al reportar gafete perdido: {}", e))?;
    
    get_ingreso_by_id(pool, ingreso_id).await
}

/// Cierra ingreso anterior (usado cuando detecta ingreso doble)
#[tauri::command]
pub async fn cerrar_ingreso_anterior(
    pool: State<'_, SqlitePool>,
    cedula: String,
    usuario_salida_id: String,
) -> Result<(), String> {
    // Buscar ingreso abierto
    let ingreso_row = sqlx::query(
        "SELECT id, fecha_hora_ingreso, gafete_id, gafete_numero FROM ingresos WHERE cedula = ? AND fecha_hora_salida IS NULL"
    )
    .bind(&cedula)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Error al buscar ingreso: {}", e))?;
    
    if let Some(row) = ingreso_row {
        let ingreso_id: String = row.get("id");
        let fecha_hora_ingreso_str: String = row.get("fecha_hora_ingreso");
        let gafete_id: String = row.get("gafete_id");
        let gafete_numero: String = row.get("gafete_numero");
        
        // Calcular tiempo
        let fecha_ingreso = NaiveDateTime::parse_from_str(&fecha_hora_ingreso_str, "%Y-%m-%d %H:%M:%S")
            .map_err(|_| "Error al parsear fecha".to_string())?;
        
        let fecha_salida = Utc::now().naive_utc();
        let duracion = fecha_salida - fecha_ingreso;
        let tiempo_permanencia_minutos = duracion.num_minutes();
        
        let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let now_rfc = Utc::now().to_rfc3339();
        
        // Cerrar ingreso
        sqlx::query(
            r#"UPDATE ingresos SET
                fecha_hora_salida = ?,
                tiempo_permanencia_minutos = ?,
                usuario_salida_id = ?,
                observaciones = 'Cerrado automáticamente por nuevo ingreso',
                updated_at = ?
            WHERE id = ?"#
        )
        .bind(&now)
        .bind(tiempo_permanencia_minutos)
        .bind(&usuario_salida_id)
        .bind(&now_rfc)
        .bind(&ingreso_id)
        .execute(&*pool)
        .await
        .map_err(|e| format!("Error al cerrar ingreso: {}", e))?;
        
        // Liberar gafete si tenía
        if gafete_numero.to_uppercase() != "S/G" {
            use crate::commands::gafete_commands::liberar_gafete;
            liberar_gafete(pool, gafete_id)
                .await
                .ok(); // Ignorar error si ya estaba liberado
        }
    }
    
    Ok(())
}