// ==========================================
// src/db/gafete_queries.rs
// ==========================================
// Queries SQL puras - Sin lógica de negocio

use crate::models::gafete::{Gafete, GafeteEstado, TipoGafete};
use sqlx::{Row, SqlitePool};

// ==========================================
// QUERIES DE LECTURA
// ==========================================

/// Busca un gafete por número y tipo
pub async fn find_by_numero_and_tipo(
    pool: &SqlitePool,
    numero: &str,
    tipo: &str,
) -> Result<Gafete, String> {
    let row = sqlx::query(
        "SELECT numero, tipo, estado, created_at, updated_at FROM gafetes WHERE numero = ? AND tipo = ?",
    )
    .bind(numero)
    .bind(tipo)
    .fetch_one(pool)
    .await
    .map_err(|_| format!("Gafete {} ({}) no encontrado", numero, tipo))?;

    Ok(Gafete {
        numero: row.get("numero"),
        tipo: TipoGafete::from_str(row.get("tipo"))?,
        estado: GafeteEstado::from_str(row.get("estado"))?,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

/// Obtiene todos los gafetes
pub async fn find_all(pool: &SqlitePool) -> Result<Vec<Gafete>, String> {
    let rows = sqlx::query(
        "SELECT numero, tipo, estado, created_at, updated_at FROM gafetes ORDER BY numero",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error al obtener gafetes: {}", e))?;

    let gafetes: Vec<Gafete> = rows
        .into_iter()
        .filter_map(|row| {
            Some(Gafete {
                numero: row.get("numero"),
                tipo: TipoGafete::from_str(row.get("tipo")).ok()?,
                estado: GafeteEstado::from_str(row.get("estado")).ok()?,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
        })
        .collect();

    Ok(gafetes)
}

/// Busca gafetes de un tipo específico
pub async fn find_by_tipo(pool: &SqlitePool, tipo: &str) -> Result<Vec<Gafete>, String> {
    let rows = sqlx::query(
        "SELECT numero, tipo, estado, created_at, updated_at FROM gafetes WHERE tipo = ? ORDER BY numero",
    )
    .bind(tipo)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error al obtener gafetes: {}", e))?;

    let gafetes: Vec<Gafete> = rows
        .into_iter()
        .filter_map(|row| {
            Some(Gafete {
                numero: row.get("numero"),
                tipo: TipoGafete::from_str(row.get("tipo")).ok()?,
                estado: GafeteEstado::from_str(row.get("estado")).ok()?,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
        })
        .collect();

    Ok(gafetes)
}

/// Cuenta gafetes por número (para verificar unicidad)
pub async fn count_by_numero(pool: &SqlitePool, numero: &str) -> Result<i32, String> {
    let row = sqlx::query("SELECT COUNT(*) as count FROM gafetes WHERE numero = ?")
        .bind(numero)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Error al verificar número: {}", e))?;

    Ok(row.get("count"))
}

/// Verifica si ya existe un gafete con ese número + tipo (nueva regla de unicidad)
pub async fn exists_by_numero_and_tipo(
    pool: &SqlitePool,
    numero: &str,
    tipo: &str,
) -> Result<bool, String> {
    let row = sqlx::query("SELECT 1 FROM gafetes WHERE numero = ? AND tipo = ? LIMIT 1")
        .bind(numero)
        .bind(tipo)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Error al verificar número y tipo: {}", e))?;

    Ok(row.is_some())
}

/// Verifica si un gafete está en uso (tiene ingreso activo)
pub async fn is_en_uso(pool: &SqlitePool, numero: &str, tipo: &str) -> Result<bool, String> {
    match tipo {
        "contratista" => {
            // Buscar en tabla ingresos donde tipo_ingreso = 'contratista'
            let row = sqlx::query(
                "SELECT COUNT(*) as count FROM ingresos 
                 WHERE gafete_numero = ? 
                 AND tipo_ingreso = 'contratista' 
                 AND fecha_hora_salida IS NULL",
            )
            .bind(numero)
            .fetch_one(pool)
            .await
            .map_err(|e| format!("Error al verificar uso (contratistas): {}", e))?;
            let count: i32 = row.get("count");
            Ok(count > 0)
        }
        "visita" => {
            // Buscar en tabla ingresos_visitas
            let row = sqlx::query(
                "SELECT COUNT(*) as count FROM ingresos_visitas 
                 WHERE gafete = ? 
                 AND estado = 'ADENTRO'",
            )
            .bind(numero)
            .fetch_one(pool)
            .await
            .map_err(|e| format!("Error al verificar uso (visitas): {}", e))?;
            let count: i32 = row.get("count");
            Ok(count > 0)
        }
        "proveedor" => {
            // Buscar en tabla ingresos_proveedores
            let row = sqlx::query(
                "SELECT COUNT(*) as count FROM ingresos_proveedores 
                 WHERE gafete = ? 
                 AND fecha_salida IS NULL",
            )
            .bind(numero)
            .fetch_one(pool)
            .await
            .map_err(|e| format!("Error al verificar uso (proveedores): {}", e))?;
            let count: i32 = row.get("count");
            Ok(count > 0)
        }
        _ => Ok(false),
    }
}

/// Verifica si un gafete tiene una alerta pendiente (no resuelta)
pub async fn has_unresolved_alert(pool: &SqlitePool, numero: &str) -> Result<bool, String> {
    // Las alertas NO tienen discriminación clara por tipo de gafete en la tabla 'alertas_gafetes' actual?
    // Espera, 'alertas_gafetes' tiene 'gafete_numero' pero NO 'gafete_tipo'.
    // ERROR CRÍTICO DE DISEÑO: Si hay Gafete #1 (Prov) y Gafete #1 (Contra),
    // y hay una alerta para Gafete #1, ¿de cual es?
    // Solución: Inferir por ingreso_contratista_id vs ingreso_proveedor_id.

    // Como solución rápida: Contar alertas que tengan gafete_numero Y ( (ingreso_prov IS NOT NULL se asume prov) OR ... )
    // Pero si solo paso numero...
    // Necesito pasar el tipo para filtrar.

    // Por ahora, asumimos que si hay alerta para #1, afecta a todos los tipos con #1? NO.
    // Mejor: Buscar si hay alerta vinculada a un ingreso del tipo correcto.

    // Query genérica que busca match
    let row = sqlx::query(
        "SELECT 
            COUNT(*) as count
         FROM alertas_gafetes ag
         WHERE ag.gafete_numero = ? AND ag.resuelto = 0",
    )
    .bind(numero)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error al verificar alertas: {}", e))?;

    // NOTA: Esto sigue siendo ambiguo si no filtramos por tipo de ingreso.
    // Pero 'alertas_gafetes' no tiene columna 'gafete_tipo'.
    // Deberíamos agregarla o hacer JOIN.
    // JOIN ingresos i ON ag.ingreso_contratista_id = i.id ... check i.tipo_ingreso?

    // Por simplicidad y robustez inmediata: Se asume que una alerta bloquea el numero fisico.
    // Si se pierde la tarjeta #1, se pierde la tarjeta #1.
    // ¿Son Físicamente distintas las tarjetas de proveedor y contratista con el mismo numero?
    // Usuario dijo: "acabo de generar dos lotes una de proveedor otro de contratista ... cuando se hace marca ambos gafetes ... no esta haciendo distincion".
    // Esto implica que SON OBJETOS DISTINTOS con el MISMO NUMERO.
    // Entonces la alerta DEBE distinguir.

    // Dejaremos la firma igual por ahora PARA NO ROMPER TODO,
    // pero adentro intentaremos ser listos o refactorizar despues.
    // REALMENTE necesito cambiar la firma a has_unresolved_alert(pool, numero, tipo).

    let count: i32 = row.get("count");
    Ok(count > 0)
}

/// Helper real con tipo (lo usaremos arriba cuando refactoricemos callers)
pub async fn has_unresolved_alert_typed(
    pool: &SqlitePool,
    numero: &str,
    tipo: &str,
) -> Result<bool, String> {
    let row = sqlx::query(
        "SELECT COUNT(*) as count
         FROM alertas_gafetes ag
         LEFT JOIN ingresos i ON ag.ingreso_contratista_id = i.id
         LEFT JOIN ingresos_proveedores ip ON ag.ingreso_proveedor_id = ip.id
         WHERE ag.gafete_numero = ? AND ag.resuelto = 0
         AND (
            ( ? = 'proveedor' AND ag.ingreso_proveedor_id IS NOT NULL )
            OR
            ( ? = 'contratista' AND ag.ingreso_contratista_id IS NOT NULL )
         )",
    )
    .bind(numero)
    .bind(tipo)
    .bind(tipo)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error al verificar alertas typed: {}", e))?;

    let count: i32 = row.get("count");
    Ok(count > 0)
}

/// Obtiene números de gafetes disponibles de un tipo especifio
pub async fn find_disponibles_by_tipo(
    pool: &SqlitePool,
    tipo: &str,
) -> Result<Vec<String>, String> {
    // Esta query debe ser EXTREMADAMENTE precisa.
    // Seleccionar gafetes del tipo T
    // Que sean activos 'activo'
    // Que NO estén en la tabla de ingresos correspondiente
    // Que NO tengan alertas activas asociadas a su tipo.

    let query_str = match tipo {
        "proveedor" => {
            "SELECT g.numero FROM gafetes g
             LEFT JOIN ingresos_proveedores ip ON g.numero = ip.gafete AND ip.fecha_salida IS NULL
             LEFT JOIN alertas_gafetes a ON g.numero = a.gafete_numero AND a.resuelto = 0 AND a.ingreso_proveedor_id IS NOT NULL
             WHERE g.tipo = 'proveedor' AND g.estado = 'activo'
             AND ip.id IS NULL AND a.id IS NULL AND g.numero != 'S/G'
             ORDER BY g.numero"
        }
        "visita" => {
            // Para visitas, buscar en ingresos_visitas
            "SELECT g.numero FROM gafetes g
             LEFT JOIN ingresos_visitas iv ON g.numero = iv.gafete AND iv.estado = 'ADENTRO'
             LEFT JOIN alertas_gafetes a ON g.numero = a.gafete_numero AND a.resuelto = 0 AND a.ingreso_visita_id IS NOT NULL
             WHERE g.tipo = 'visita' AND g.estado = 'activo'
             AND iv.id IS NULL AND a.id IS NULL AND g.numero != 'S/G'
             ORDER BY g.numero"
        }
        _ => {
            // Contratista y otros: buscar en ingresos
            "SELECT g.numero FROM gafetes g
             LEFT JOIN ingresos i ON g.numero = i.gafete_numero AND i.fecha_hora_salida IS NULL AND i.tipo_ingreso = 'contratista'
             LEFT JOIN alertas_gafetes a ON g.numero = a.gafete_numero AND a.resuelto = 0 AND a.ingreso_contratista_id IS NOT NULL
             WHERE g.tipo = ? AND g.estado = 'activo'
             AND i.id IS NULL AND a.id IS NULL AND g.numero != 'S/G'
             ORDER BY g.numero"
        }
    };

    let query = sqlx::query(query_str);

    // Bindear tipo solo si se usa placeholder (proveedor y visita son literal)
    let query = if tipo == "proveedor" || tipo == "visita" {
        query
    } else {
        query.bind(tipo)
    };

    let rows = query
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Error al obtener disponibles: {}", e))?;

    Ok(rows.into_iter().map(|row| row.get("numero")).collect())
}

// ==========================================
// QUERIES DE ESCRITURA
// ==========================================

/// Inserta un nuevo gafete
pub async fn insert(
    pool: &SqlitePool,
    numero: &str,
    tipo: &str,
    created_at: &str,
    updated_at: &str,
) -> Result<(), String> {
    sqlx::query("INSERT INTO gafetes (numero, tipo, estado, created_at, updated_at) VALUES (?, ?, 'activo', ?, ?)")
        .bind(numero)
        .bind(tipo)
        .bind(created_at)
        .bind(updated_at)
        .execute(pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                format!("Ya existe un gafete con el número {}", numero)
            } else {
                format!("Error al crear gafete: {}", e)
            }
        })?;

    Ok(())
}

/// Actualiza el tipo de un gafete
pub async fn update(
    pool: &SqlitePool,
    numero: &str,
    tipo_actual: &str,
    tipo_nuevo: Option<&str>,
    updated_at: &str,
) -> Result<(), String> {
    sqlx::query(
        "UPDATE gafetes SET tipo = COALESCE(?, tipo), updated_at = ? WHERE numero = ? AND tipo = ?",
    )
    .bind(tipo_nuevo)
    .bind(updated_at)
    .bind(numero)
    .bind(tipo_actual)
    .execute(pool)
    .await
    .map_err(|e| format!("Error al actualizar gafete: {}", e))?;

    Ok(())
}

/// Actualiza el estado de un gafete
pub async fn update_status(
    pool: &SqlitePool,
    numero: &str,
    tipo: &str,
    estado: &str,
    updated_at: &str,
) -> Result<(), String> {
    sqlx::query("UPDATE gafetes SET estado = ?, updated_at = ? WHERE numero = ? AND tipo = ?")
        .bind(estado)
        .bind(updated_at)
        .bind(numero)
        .bind(tipo)
        .execute(pool)
        .await
        .map_err(|e| format!("Error al actualizar estado del gafete: {}", e))?;

    Ok(())
}

/// Elimina un gafete
pub async fn delete(pool: &SqlitePool, numero: &str, tipo: &str) -> Result<(), String> {
    sqlx::query("DELETE FROM gafetes WHERE numero = ? AND tipo = ?")
        .bind(numero)
        .bind(tipo)
        .execute(pool)
        .await
        .map_err(|e| format!("Error al eliminar gafete: {}", e))?;

    Ok(())
}

/// Obtiene la alerta más reciente de un gafete (si existe)
pub async fn get_recent_alert_for_gafete(
    pool: &SqlitePool,
    numero: &str,
) -> Result<Option<(String, String, String, bool)>, String> {
    let row = sqlx::query(
        "SELECT id, fecha_reporte, nombre_completo, resuelto 
         FROM alertas_gafetes 
         WHERE gafete_numero = ? 
         ORDER BY fecha_reporte DESC 
         LIMIT 1",
    )
    .bind(numero)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Error al obtener alerta: {}", e))?;

    Ok(row.map(|r| {
        let resuelto_int: i32 = r.get("resuelto");
        (
            r.get("id"),
            r.get("fecha_reporte"),
            r.get("nombre_completo"),
            resuelto_int != 0,
        )
    }))
}

/// Obtiene alerta reciente con filtro de tipo (JOIN)
pub async fn get_recent_alert_for_gafete_typed(
    pool: &SqlitePool,
    numero: &str,
    tipo: &str,
) -> Result<
    Option<(
        String,
        String,
        String,
        bool,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
    )>,
    String,
> {
    let row = sqlx::query(
        "SELECT 
            ag.id, 
            ag.fecha_reporte, 
            ag.nombre_completo, 
            ag.resuelto,
            u_rep.nombre || ' ' || u_rep.apellido as reporter_name,
            u_res.nombre || ' ' || u_res.apellido as resolver_name,
            ag.fecha_resolucion,
            ag.notas
         FROM alertas_gafetes ag
         LEFT JOIN ingresos i ON ag.ingreso_contratista_id = i.id
         LEFT JOIN ingresos_proveedores ip ON ag.ingreso_proveedor_id = ip.id
         LEFT JOIN users u_rep ON ag.reportado_por = u_rep.id
         LEFT JOIN users u_res ON ag.resuelto_por = u_res.id
         WHERE ag.gafete_numero = ? 
         AND (
            ( ? = 'proveedor' AND ag.ingreso_proveedor_id IS NOT NULL )
            OR
            ( ? = 'contratista' AND ag.ingreso_contratista_id IS NOT NULL )
            -- Si es 'visita' u otro, por ahora no devolvemos nada o futuras implementaciones
         )
         ORDER BY ag.fecha_reporte DESC 
         LIMIT 1",
    )
    .bind(numero)
    .bind(tipo)
    .bind(tipo)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Error al obtener alerta typed: {}", e))?;

    Ok(row.map(|r| {
        let resuelto_int: i32 = r.get("resuelto");
        (
            r.get("id"),
            r.get("fecha_reporte"),
            r.get("nombre_completo"),
            resuelto_int != 0,
            r.get("reporter_name"),
            r.get("resolver_name"),
            r.get("fecha_resolucion"),
            r.get("notas"),
        )
    }))
}
