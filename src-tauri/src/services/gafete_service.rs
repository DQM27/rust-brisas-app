// ==========================================
// src/services/gafete_service.rs
// ==========================================
// Orquesta dominio y DB - Lógica de negocio completa

use crate::db::gafete_queries as db;
use crate::domain::gafete as domain;
use crate::models::gafete::{
    CreateGafeteInput, CreateGafeteRangeInput, GafeteEstado, GafeteListResponse, GafeteResponse,
    StatsGafetes, StatsPorTipo, TipoGafete, UpdateGafeteInput,
};
use chrono::Utc;
use sqlx::SqlitePool;

// ==========================================
// CREAR GAFETE
// ==========================================

pub async fn create_gafete(
    pool: &SqlitePool,
    input: CreateGafeteInput,
) -> Result<GafeteResponse, String> {
    // 1. Validar input
    domain::validar_create_input(&input)?;

    // 2. Normalizar número
    let numero_normalizado = domain::normalizar_numero(&input.numero);

    // 3. Verificar que no exista con este número + tipo
    let tipo = TipoGafete::from_str(&input.tipo)?;
    let exists = db::exists_by_numero_and_tipo(pool, &numero_normalizado, tipo.as_str()).await?;
    if exists {
        return Err(format!(
            "Ya existe un gafete con el número {} de tipo {}",
            numero_normalizado, input.tipo
        ));
    }

    // 4. Parsear tipo
    let tipo = TipoGafete::from_str(&input.tipo)?;

    // 5. Timestamps
    let now = Utc::now().to_rfc3339();

    // 6. Insertar
    db::insert(pool, &numero_normalizado, tipo.as_str(), &now, &now).await?;

    // 7. Retornar
    get_gafete(pool, &numero_normalizado).await
}

pub async fn create_gafete_range(
    pool: &SqlitePool,
    input: CreateGafeteRangeInput,
) -> Result<Vec<String>, String> {
    let mut created_gafetes = Vec::new();

    if input.start > input.end {
        return Err("El inicio del rango debe ser menor o igual al fin del rango".to_string());
    }

    if (input.end - input.start) > 1000 {
        return Err("No se pueden crear más de 1000 gafetes en una sola operación".to_string());
    }

    let tipo = TipoGafete::from_str(&input.tipo)?;
    let now = Utc::now().to_rfc3339();
    let padding = input.padding.unwrap_or(2);
    let prefix = input.prefix.unwrap_or_default();

    for i in input.start..=input.end {
        // Formatear número: prefix + i con padding
        let numero_base = format!("{:0width$}", i, width = padding);
        let numero_completo = format!("{}{}", prefix, numero_base);

        // Intentar insertar - Ignorar errores de duplicados (continue)
        match db::insert(pool, &numero_completo, tipo.as_str(), &now, &now).await {
            Ok(_) => created_gafetes.push(numero_completo),
            Err(e) if e.contains("Ya existe") => continue, // Skip duplicates
            Err(e) => return Err(e),                       // Fail on other errors
        }
    }

    Ok(created_gafetes)
}

// ==========================================
// OBTENER GAFETE
// ==========================================

pub async fn get_gafete(pool: &SqlitePool, numero: &str) -> Result<GafeteResponse, String> {
    let gafete = db::find_by_numero(pool, numero).await?;
    let en_uso = db::is_en_uso(pool, numero).await?;
    let tiene_alerta = db::has_unresolved_alert(pool, numero).await?;

    let mut response = GafeteResponse::from(gafete.clone());

    // Obtener detalles de la alerta si existe
    if let Ok(Some((alerta_id, fecha, nombre, resuelto))) =
        db::get_recent_alert_for_gafete(pool, numero).await
    {
        response.alerta_id = Some(alerta_id);
        response.fecha_perdido = Some(fecha);
        response.quien_perdio = Some(nombre);
        response.alerta_resuelta = Some(resuelto);
    }

    // Determinar estado global (status) considerando estado físico + uso + alertas
    // Prioridad: Dañado/Extraviado (Físico) > Perdido (Alerta) > En Uso > Disponible
    // Prioridad: Dañado/Extraviado (Físico) > Perdido (Alerta) > En Uso > Disponible
    if gafete.estado == GafeteEstado::Danado {
        response.status = "danado".to_string();
        response.esta_disponible = false;
    } else if gafete.estado == GafeteEstado::Extraviado {
        response.status = "extraviado".to_string();
        response.esta_disponible = false;
    } else if tiene_alerta {
        response.status = "perdido".to_string(); // Alerta de extravío generada por sistema
        response.esta_disponible = false;
    } else if en_uso {
        response.status = "en_uso".to_string();
        response.esta_disponible = false;
    } else {
        response.status = "disponible".to_string();
        response.esta_disponible = true;
    }

    Ok(response)
}

// ==========================================
// OBTENER TODOS
// ==========================================

pub async fn get_all_gafetes(pool: &SqlitePool) -> Result<GafeteListResponse, String> {
    let gafetes = db::find_all(pool).await?;

    // Calcular disponibilidad y estado para cada uno
    let mut responses = Vec::with_capacity(gafetes.len());

    // Contadores para stats rápidos
    let mut stats_danados = 0;
    let mut stats_extraviados = 0;

    for gafete in gafetes {
        let en_uso = db::is_en_uso(pool, &gafete.numero).await?;
        let tiene_alerta = db::has_unresolved_alert(pool, &gafete.numero).await?;

        let mut response = GafeteResponse::from(gafete.clone());

        // Obtener detalles de la alerta si existe
        if let Ok(Some((alerta_id, fecha, nombre, resuelto))) =
            db::get_recent_alert_for_gafete(pool, &response.numero).await
        {
            response.alerta_id = Some(alerta_id);
            response.fecha_perdido = Some(fecha);
            response.quien_perdio = Some(nombre);
            response.alerta_resuelta = Some(resuelto);
        }

        // Determinar status
        // Determinar status
        if gafete.estado == GafeteEstado::Danado {
            response.status = "danado".to_string();
            response.esta_disponible = false;
            stats_danados += 1;
        } else if gafete.estado == GafeteEstado::Extraviado {
            response.status = "extraviado".to_string();
            response.esta_disponible = false;
            stats_extraviados += 1;
        } else if tiene_alerta {
            response.status = "perdido".to_string();
            response.esta_disponible = false;
        } else if en_uso {
            response.status = "en_uso".to_string();
            response.esta_disponible = false;
        } else {
            response.status = "disponible".to_string();
            response.esta_disponible = true;
        }

        responses.push(response);
    }

    // Stats
    let total = responses.len();
    let disponibles = responses.iter().filter(|g| g.esta_disponible).count();
    // En uso real debe contar los que están en uso pero NO dañados/extraviados
    let en_uso = responses.iter().filter(|r| r.status == "en_uso").count();

    let contratistas = responses
        .iter()
        .filter(|g| g.tipo == TipoGafete::Contratista)
        .count();
    let proveedores = responses
        .iter()
        .filter(|g| g.tipo == TipoGafete::Proveedor)
        .count();
    let visitas = responses
        .iter()
        .filter(|g| g.tipo == TipoGafete::Visita)
        .count();
    let otros = responses
        .iter()
        .filter(|g| g.tipo == TipoGafete::Otro)
        .count();

    Ok(GafeteListResponse {
        gafetes: responses,
        total,
        stats: StatsGafetes {
            total,
            disponibles,
            en_uso,
            danados: stats_danados,
            extraviados: stats_extraviados,
            por_tipo: StatsPorTipo {
                contratistas,
                proveedores,
                visitas,
                otros,
            },
        },
    })
}

// ==========================================
// OBTENER DISPONIBLES POR TIPO
// ==========================================

pub async fn get_gafetes_disponibles(
    pool: &SqlitePool,
    tipo: TipoGafete,
) -> Result<Vec<GafeteResponse>, String> {
    let numeros = db::find_disponibles_by_tipo(pool, tipo.as_str()).await?;

    let mut responses = Vec::new();
    for numero in numeros {
        // Optimización: find_disponibles_by_tipo ya filtra por estado, uso y alertas
        // Pero necesitamos el objeto completo
        let gafete = db::find_by_numero(pool, &numero).await?;
        let mut response = GafeteResponse::from(gafete);
        response.esta_disponible = true;
        response.status = "disponible".to_string();
        responses.push(response);
    }

    Ok(responses)
}

// ==========================================
// VERIFICAR DISPONIBILIDAD
// ==========================================

pub async fn is_gafete_disponible(pool: &SqlitePool, numero: &str) -> Result<bool, String> {
    // Verificar existencia y estado físico
    match db::find_by_numero(pool, numero).await {
        Ok(g) => {
            if g.estado != GafeteEstado::Activo {
                return Ok(false);
            }
        }
        Err(_) => return Ok(false), // No existe
    }

    // Checking usage and alerts
    let en_uso = db::is_en_uso(pool, numero).await?;
    let tiene_alerta = db::has_unresolved_alert(pool, numero).await?;
    Ok(!en_uso && !tiene_alerta)
}

// ==========================================
// ACTUALIZAR
// ==========================================

pub async fn update_gafete(
    pool: &SqlitePool,
    numero: String,
    input: UpdateGafeteInput,
) -> Result<GafeteResponse, String> {
    // 1. Validar input
    domain::validar_update_input(&input)?;

    // 2. Verificar que existe
    let _ = db::find_by_numero(pool, &numero).await?;

    // 3. Parsear tipo si viene
    let tipo_str = if let Some(ref t) = input.tipo {
        Some(TipoGafete::from_str(t)?.as_str().to_string())
    } else {
        None
    };

    // 4. Timestamp
    let now = Utc::now().to_rfc3339();

    // 5. Actualizar
    db::update(pool, &numero, tipo_str.as_deref(), &now).await?;

    // 6. Retornar
    get_gafete(pool, &numero).await
}

pub async fn update_gafete_status(
    pool: &SqlitePool,
    numero: String,
    estado: GafeteEstado,
) -> Result<GafeteResponse, String> {
    // Validar estado (Implícita por tipo)

    // Verificar que existe
    let _ = db::find_by_numero(pool, &numero).await?;

    let now = Utc::now().to_rfc3339();
    db::update_status(pool, &numero, estado.as_str(), &now).await?;

    get_gafete(pool, &numero).await
}

// ==========================================
// ELIMINAR
// ==========================================

pub async fn delete_gafete(pool: &SqlitePool, numero: String) -> Result<(), String> {
    // 1. Verificar que existe
    let _ = db::find_by_numero(pool, &numero).await?;

    // 2. Verificar que no esté en uso (si está 'activo' o 'en_uso')
    let en_uso = db::is_en_uso(pool, &numero).await?;
    if en_uso {
        return Err("No se puede eliminar un gafete que está actualmente en uso".to_string());
    }

    // 3. Eliminar
    db::delete(pool, &numero).await
}
