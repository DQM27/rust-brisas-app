// ==========================================
// src/services/empresa_service.rs
// ==========================================

use crate::db::empresa_queries as db;
use crate::domain::empresa as domain;
use crate::models::empresa::{
    CreateEmpresaInput, EmpresaListResponse, EmpresaResponse, UpdateEmpresaInput,
};
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

// ==========================================
// CREAR EMPRESA
// ==========================================

pub async fn create_empresa(
    pool: &SqlitePool,
    input: CreateEmpresaInput,
) -> Result<EmpresaResponse, String> {
    // 1. Validar input
    domain::validar_create_input(&input)?;

    // 2. Normalizar nombre
    let nombre_normalizado = domain::normalizar_nombre(&input.nombre);

    // 3. Verificar que no exista
    let count = db::count_by_nombre(pool, &nombre_normalizado).await?;
    if count > 0 {
        return Err("Ya existe una empresa con este nombre".to_string());
    }

    // 4. Crear
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    db::insert(pool, &id, &nombre_normalizado, &now, &now).await?;

    // 5. Retornar
    get_empresa_by_id(pool, id).await
}

// ==========================================
// OBTENER EMPRESA
// ==========================================

pub async fn get_empresa_by_id(
    pool: &SqlitePool,
    id: String,
) -> Result<EmpresaResponse, String> {
    let empresa_opt = db::find_by_id(pool, &id).await?;
    let empresa = empresa_opt.ok_or("Empresa no encontrada")?;
    let total_contratistas = db::count_contratistas(pool, &id).await?;

    let mut response = EmpresaResponse::from(empresa);
    response.total_contratistas = total_contratistas as usize;

    Ok(response)
}

// ==========================================
// OBTENER TODAS
// ==========================================

pub async fn get_all_empresas(pool: &SqlitePool) -> Result<EmpresaListResponse, String> {
    let empresas = db::find_all(pool).await?;

    let mut responses = Vec::new();
    for empresa in empresas {
        let total_contratistas = db::count_contratistas(pool, &empresa.id).await?;
        let mut response = EmpresaResponse::from(empresa);
        response.total_contratistas = total_contratistas as usize;
        responses.push(response);
    }

    let total = responses.len();
    let activas = responses.iter().filter(|e| e.is_active).count();

    Ok(EmpresaListResponse {
        empresas: responses,
        total,
        activas,
    })
}

// ==========================================
// OBTENER ACTIVAS
// ==========================================

pub async fn get_empresas_activas(pool: &SqlitePool) -> Result<Vec<EmpresaResponse>, String> {
    let empresas = db::find_activas(pool).await?;

    let mut responses = Vec::new();
    for empresa in empresas {
        let total_contratistas = db::count_contratistas(pool, &empresa.id).await?;
        let mut response = EmpresaResponse::from(empresa);
        response.total_contratistas = total_contratistas as usize;
        responses.push(response);
    }

    Ok(responses)
}

// ==========================================
// ACTUALIZAR
// ==========================================

pub async fn update_empresa(
    pool: &SqlitePool,
    id: String,
    input: UpdateEmpresaInput,
) -> Result<EmpresaResponse, String> {
    // 1. Validar input
    domain::validar_update_input(&input)?;

    // 2. Verificar que existe
    let _ = db::find_by_id(pool, &id).await?.ok_or("Empresa no encontrada")?;

    // 3. Normalizar y verificar nombre si viene
    let nombre_normalizado = if let Some(ref nombre) = input.nombre {
        let normalizado = domain::normalizar_nombre(nombre);

        let count = db::count_by_nombre_excluding_id(pool, &normalizado, &id).await?;
        if count > 0 {
            return Err("Ya existe otra empresa con este nombre".to_string());
        }

        Some(normalizado)
    } else {
        None
    };

    // 4. Actualizar
    let now = Utc::now().to_rfc3339();
    db::update(pool, &id, nombre_normalizado.as_deref(), input.is_active, &now).await?;

    // 5. Retornar
    get_empresa_by_id(pool, id).await
}

// ==========================================
// ELIMINAR
// ==========================================

pub async fn delete_empresa(pool: &SqlitePool, id: String) -> Result<(), String> {
    // 1. Verificar que existe
    let _ = db::find_by_id(pool, &id).await?.ok_or("Empresa no encontrada")?;

    // 2. Verificar que no tenga contratistas
    let count = db::count_contratistas(pool, &id).await?;
    if count > 0 {
        return Err(format!(
            "No se puede eliminar la empresa porque tiene {} contratista(s) asociado(s)",
            count
        ));
    }

    // 3. Eliminar
    db::delete(pool, &id).await
}