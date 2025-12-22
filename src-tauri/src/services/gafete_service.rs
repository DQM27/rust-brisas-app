// ==========================================
// src/services/gafete_service.rs
// ==========================================
// Orquesta dominio y DB - Lógica de negocio completa
// Strict Mode: Return Result<T, GafeteError>

use crate::db::gafete_queries as db;
use crate::domain::errors::GafeteError;
use crate::domain::gafete as domain;
use crate::models::gafete::{
    CreateGafeteInput, CreateGafeteRangeInput, GafeteEstado, GafeteListResponse, GafeteResponse,
    StatsGafetes, StatsPorTipo, TipoGafete, UpdateGafeteInput,
};
use crate::services::alerta_service;
use chrono::Utc;
use sqlx::SqlitePool;

// ==========================================
// CREAR GAFETE
// ==========================================

pub async fn create_gafete(
    pool: &SqlitePool,
    input: CreateGafeteInput,
) -> Result<GafeteResponse, GafeteError> {
    // 1. Validar input
    domain::validar_create_input(&input).map_err(GafeteError::Validation)?;

    // 2. Normalizar número
    let numero_normalizado = domain::normalizar_numero(&input.numero);

    // 3. Verificar que no exista con este número + tipo
    let tipo: TipoGafete = input.tipo.parse().map_err(|e| GafeteError::Validation(e))?;
    let exists = db::exists_by_numero_and_tipo(pool, &numero_normalizado, tipo.as_str()).await?;
    if exists {
        return Err(GafeteError::AlreadyExists);
    }

    // 4. Timestamps
    let now = Utc::now().to_rfc3339();

    // 5. Insertar
    db::insert(pool, &numero_normalizado, tipo.as_str(), &now, &now).await?;

    // 6. Retornar
    get_gafete(pool, &numero_normalizado, tipo.as_str()).await
}

pub async fn create_gafete_range(
    pool: &SqlitePool,
    input: CreateGafeteRangeInput,
) -> Result<Vec<String>, GafeteError> {
    let mut created_gafetes = Vec::new();

    if input.start > input.end {
        return Err(GafeteError::Validation(
            "El inicio del rango debe ser menor o igual al fin del rango".to_string(),
        ));
    }

    if (input.end - input.start) > 1000 {
        return Err(GafeteError::Validation(
            "No se pueden crear más de 1000 gafetes en una sola operación".to_string(),
        ));
    }

    let tipo: TipoGafete = input.tipo.parse().map_err(GafeteError::Validation)?;
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
            Err(e) => {
                // Check unique violation properly
                if let Some(db_err) = e.as_database_error() {
                    if db_err.is_unique_violation() {
                        continue;
                    }
                }
                return Err(GafeteError::Database(e));
            }
        }
    }

    Ok(created_gafetes)
}

// ==========================================
// OBTENER GAFETE
// ==========================================

pub async fn get_gafete(
    pool: &SqlitePool,
    numero: &str,
    tipo: &str,
) -> Result<GafeteResponse, GafeteError> {
    // Si falla find, retorna sqlx error. Si es RowNotFound, retorna Error Database.
    // Deberíamos mapear RowNotFound a GafeteError::NotFound?
    // DB returns sqlx::Error::RowNotFound.
    // GafeteError::Database wraps it.
    // Frontend could check code/string.
    // Better: Helper map_not_found?
    // Using simple propagation first.
    let gafete = db::find_by_numero_and_tipo(pool, numero, tipo)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => GafeteError::NotFound,
            _ => GafeteError::Database(e),
        })?;

    let en_uso = db::is_en_uso(pool, numero, tipo).await?;
    let tiene_alerta = db::has_unresolved_alert_typed(pool, numero, tipo).await?;

    let mut response = GafeteResponse::from(gafete.clone());

    // Obtener detalles de la alerta si existe
    if let Ok(Some((
        alerta_id,
        fecha,
        nombre,
        resuelto,
        reporte_por,
        resuelto_por,
        fecha_res,
        notas,
    ))) = db::get_recent_alert_for_gafete_typed(pool, numero, tipo).await
    {
        response.alerta_id = Some(alerta_id);
        response.fecha_perdido = Some(fecha);
        response.quien_perdio = Some(nombre);
        response.alerta_resuelta = Some(resuelto);

        response.reportado_por_nombre = reporte_por;
        response.resuelto_por_nombre = resuelto_por;
        response.fecha_resolucion = fecha_res;
        response.notas = notas;
    }

    // Determinar estado global (status)
    if gafete.estado == GafeteEstado::Danado {
        response.status = "danado".to_string();
        response.esta_disponible = false;
    } else if gafete.estado == GafeteEstado::Extraviado {
        response.status = "extraviado".to_string();
        response.esta_disponible = false;
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

    Ok(response)
}

// ==========================================
// OBTENER TODOS
// ==========================================

pub async fn get_all_gafetes(pool: &SqlitePool) -> Result<GafeteListResponse, GafeteError> {
    let gafetes = db::find_all(pool).await?;

    // Calcular disponibilidad y estado para cada uno
    let mut responses = Vec::with_capacity(gafetes.len());

    let mut stats_danados = 0;
    let mut stats_extraviados = 0;

    for gafete in gafetes {
        let tipo_str = gafete.tipo.as_str();
        let en_uso = db::is_en_uso(pool, &gafete.numero, tipo_str).await?;
        let tiene_alerta = db::has_unresolved_alert_typed(pool, &gafete.numero, tipo_str).await?;

        let mut response = GafeteResponse::from(gafete.clone());

        // Obtener detalles de la alerta si existe
        if let Ok(Some((
            alerta_id,
            fecha,
            nombre,
            resuelto,
            reporte_por,
            resuelto_por,
            fecha_res,
            notas,
        ))) = db::get_recent_alert_for_gafete_typed(pool, &response.numero, tipo_str).await
        {
            response.alerta_id = Some(alerta_id);
            response.fecha_perdido = Some(fecha);
            response.quien_perdio = Some(nombre);
            response.alerta_resuelta = Some(resuelto);
            response.reportado_por_nombre = reporte_por;
            response.resuelto_por_nombre = resuelto_por;
            response.fecha_resolucion = fecha_res;
            response.notas = notas;
        }

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

    let total = responses.len();
    let disponibles = responses.iter().filter(|g| g.esta_disponible).count();
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
) -> Result<Vec<GafeteResponse>, GafeteError> {
    let numeros = db::find_disponibles_by_tipo(pool, tipo.as_str()).await?;

    let mut responses = Vec::new();
    for numero in numeros {
        let gafete = db::find_by_numero_and_tipo(pool, &numero, tipo.as_str()).await?;
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

pub async fn is_gafete_disponible(
    pool: &SqlitePool,
    numero: &str,
    tipo: &str,
) -> Result<bool, GafeteError> {
    match db::find_by_numero_and_tipo(pool, numero, tipo).await {
        Ok(g) => {
            if g.estado != GafeteEstado::Activo {
                return Ok(false);
            }
        }
        Err(sqlx::Error::RowNotFound) => return Ok(false),
        Err(e) => return Err(GafeteError::Database(e)),
    }

    let en_uso = db::is_en_uso(pool, numero, tipo).await?;
    let tiene_alerta = db::has_unresolved_alert_typed(pool, numero, tipo).await?;
    Ok(!en_uso && !tiene_alerta)
}

// ==========================================
// ACTUALIZAR
// ==========================================

pub async fn update_gafete(
    pool: &SqlitePool,
    numero: String,
    tipo_actual: String,
    input: UpdateGafeteInput,
) -> Result<GafeteResponse, GafeteError> {
    domain::validar_update_input(&input).map_err(GafeteError::Validation)?;

    let _ = db::find_by_numero_and_tipo(pool, &numero, &tipo_actual)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => GafeteError::NotFound,
            _ => GafeteError::Database(e),
        })?;

    let tipo_str = if let Some(ref t) = input.tipo {
        Some(
            t.parse::<TipoGafete>()
                .map_err(GafeteError::Validation)?
                .as_str()
                .to_string(),
        )
    } else {
        None
    };

    let now = Utc::now().to_rfc3339();

    db::update(pool, &numero, &tipo_actual, tipo_str.as_deref(), &now).await?;

    let tipo_final = tipo_str.unwrap_or(tipo_actual);
    get_gafete(pool, &numero, &tipo_final).await
}

pub async fn update_gafete_status(
    pool: &SqlitePool,
    numero: String,
    tipo: String,
    estado: GafeteEstado,
    usuario_id: Option<String>,
) -> Result<GafeteResponse, GafeteError> {
    let _ = db::find_by_numero_and_tipo(pool, &numero, &tipo)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => GafeteError::NotFound,
            _ => GafeteError::Database(e),
        })?;

    let now = Utc::now().to_rfc3339();
    db::update_status(pool, &numero, &tipo, estado.as_str(), &now).await?;

    if estado == GafeteEstado::Activo {
        if let Ok(true) = db::has_unresolved_alert_typed(pool, &numero, &tipo).await {
            if let Ok(Some((id, _, _, _, _, _, _, _))) =
                db::get_recent_alert_for_gafete_typed(pool, &numero, &tipo).await
            {
                let resolver_id = usuario_id.unwrap_or_else(|| "sistema".to_string());
                alerta_service::resolver(
                    pool,
                    &id,
                    &now,
                    Some("Gafete marcado como activo manualmente"),
                    &resolver_id,
                    &now,
                )
                .await
                .map_err(|e| GafeteError::Validation(e.to_string()))?; // Map AlertaError
            }
        }
    }

    get_gafete(pool, &numero, &tipo).await
}

// ==========================================
// ELIMINAR
// ==========================================

pub async fn delete_gafete(
    pool: &SqlitePool,
    numero: String,
    tipo: String,
) -> Result<(), GafeteError> {
    let _ = db::find_by_numero_and_tipo(pool, &numero, &tipo)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => GafeteError::NotFound,
            _ => GafeteError::Database(e),
        })?;

    let en_uso = db::is_en_uso(pool, &numero, &tipo).await?;
    if en_uso {
        return Err(GafeteError::Validation(
            "No se puede eliminar un gafete que está actualmente en uso".to_string(),
        ));
    }

    db::delete(pool, &numero, &tipo).await?;
    Ok(())
}
