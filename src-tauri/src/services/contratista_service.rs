// services/contratista_service.rs
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::contratista::*;
use crate::domain::{
    errors::ContratistaError,
    contratista::validations::ContratistaValidator,
};
use crate::db::contratista as db;

pub struct ContratistaService;

impl ContratistaService {
    // ==========================================
    // CREAR
    // ==========================================
    pub async fn crear(
        pool: &SqlitePool,
        input: CreateContratistaInput,
    ) -> Result<ContratistaResponse, ContratistaError> {
        // 1. Validar
        let validated = ContratistaValidator::validar_creacion(pool, &input).await?;
        
        // 2. Preparar datos
        let id = Uuid::new_v4().to_string();
        let timestamp = Utc::now().to_rfc3339();
        
        let data = db::CreateData {
            cedula: validated.cedula,
            nombre: validated.nombre,
            apellido: validated.apellido,
            empresa_id: validated.empresa_id,
            fecha_vencimiento_praind: validated.fecha_vencimiento_praind.to_string(),
            timestamp,
        };
        
        // 3. Insertar en transacción
        let mut tx = pool.begin().await?;
        db::insertar(&mut tx, &id, &data).await?;
        tx.commit().await?;
        
        // 4. Retornar
        db::find_by_id(pool, &id).await
    }
    
    // ==========================================
    // LEER
    // ==========================================
    pub async fn obtener_por_id(
        pool: &SqlitePool,
        id: String,
    ) -> Result<ContratistaResponse, ContratistaError> {
        db::find_by_id(pool, &id).await
    }
    
    pub async fn obtener_por_cedula(
        pool: &SqlitePool,
        cedula: String,
    ) -> Result<ContratistaResponse, ContratistaError> {
        db::find_by_cedula(pool, &cedula).await
    }
    
    pub async fn listar_todos(
        pool: &SqlitePool,
    ) -> Result<ContratistaListResponse, ContratistaError> {
        let items = db::find_all(pool).await?;
        Ok(ContratistaListResponse::new(items))
    }
    
    pub async fn listar_activos(
        pool: &SqlitePool,
    ) -> Result<Vec<ContratistaResponse>, ContratistaError> {
        db::find_activos(pool).await
    }
    
    // ==========================================
    // ACTUALIZAR
    // ==========================================
    pub async fn actualizar(
        pool: &SqlitePool,
        id: String,
        input: UpdateContratistaInput,
    ) -> Result<ContratistaResponse, ContratistaError> {
        // Validar campos opcionales
        // ... validaciones
        
        let data = db::UpdateData {
            nombre: input.nombre,
            apellido: input.apellido,
            empresa_id: input.empresa_id,
            fecha_vencimiento_praind: input.fecha_vencimiento_praind,
            timestamp: Utc::now().to_rfc3339(),
        };
        
        db::actualizar(pool, &id, &data).await?;
        db::find_by_id(pool, &id).await
    }
    
    pub async fn cambiar_estado(
        pool: &SqlitePool,
        id: String,
        input: CambiarEstadoInput,
    ) -> Result<ContratistaResponse, ContratistaError> {
        let estado = EstadoContratista::from_str(&input.estado)
            .map_err(ContratistaError::ParseError)?;
        let timestamp = Utc::now().to_rfc3339();
        
        db::cambiar_estado(pool, &id, estado, &timestamp).await?;
        db::find_by_id(pool, &id).await
    }
    
    // ==========================================
    // ELIMINAR
    // ==========================================
    pub async fn eliminar(
        pool: &SqlitePool,
        id: String,
    ) -> Result<(), ContratistaError> {
        db::eliminar(pool, &id).await
    }
}