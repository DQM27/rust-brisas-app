// ==========================================
// src/services/proveedor_service.rs
// ==========================================
use crate::db::{empresa_queries, proveedor_queries, vehiculo_queries};
use crate::models::proveedor::{CreateProveedorInput, ProveedorResponse};
use chrono::Utc;
use sqlx::SqlitePool;
use std::sync::Arc;
use uuid::Uuid;

pub struct ProveedorService {
    pool: Arc<SqlitePool>, // Usar Arc para compartir pool si es necesario, o clone propio
}

impl ProveedorService {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }

    pub async fn create_proveedor(
        &self,
        input: CreateProveedorInput,
    ) -> Result<ProveedorResponse, String> {
        // 1. Validar que la empresa existe
        if empresa_queries::find_by_id(&self.pool, &input.empresa_id)
            .await
            .map_err(|e| e.to_string())?
            .is_none()
        {
            return Err("La empresa seleccionada no existe".to_string());
        }

        // 2. Validar duplicidad
        if let Some(_) = proveedor_queries::find_by_cedula(&self.pool, &input.cedula)
            .await
            .map_err(|e| e.to_string())?
        {
            return Err("Ya existe un proveedor con esa cédula".to_string());
        }

        // 3. Crear
        let proveedor = proveedor_queries::create(&self.pool, input.clone()) // Clone input needed or refactor query? Query takes ownership? create takes input.
            .await
            .map_err(|e| e.to_string())?;

        // 4. Crear Vehículo si aplica
        if let Some(true) = input.tiene_vehiculo {
            if let (Some(tipo), Some(placa)) = (&input.tipo_vehiculo, &input.placa) {
                if !tipo.is_empty() && !placa.is_empty() {
                    let vid = Uuid::new_v4().to_string();
                    let now = Utc::now().to_rfc3339();

                    vehiculo_queries::insert(
                        &self.pool,
                        &vid,
                        None,                // Contratista ID
                        Some(&proveedor.id), // Proveedor ID
                        tipo,
                        placa,
                        input.marca.as_deref(),
                        input.modelo.as_deref(),
                        input.color.as_deref(),
                        &now,
                        &now,
                    )
                    .await
                    .map_err(|e| e.to_string())?;
                }
            }
        }

        // 5. Enriquecer respuesta (nombre empresa y vehículo)
        self.populate_response(proveedor).await
    }

    pub async fn search_proveedores(&self, query: &str) -> Result<Vec<ProveedorResponse>, String> {
        let proveedores = proveedor_queries::search(&self.pool, query, 20)
            .await
            .map_err(|e| e.to_string())?;

        let mut responses = Vec::new();
        for p in proveedores {
            responses.push(self.populate_response(p).await?);
        }
        Ok(responses)
    }

    pub async fn get_proveedor_by_id(&self, id: &str) -> Result<Option<ProveedorResponse>, String> {
        let p = proveedor_queries::find_by_id(&self.pool, id)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(proveedor) = p {
            Ok(Some(self.populate_response(proveedor).await?))
        } else {
            Ok(None)
        }
    }

    pub async fn get_proveedor_by_cedula(
        &self,
        cedula: &str,
    ) -> Result<Option<ProveedorResponse>, String> {
        let p = proveedor_queries::find_by_cedula(&self.pool, cedula)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(proveedor) = p {
            Ok(Some(self.populate_response(proveedor).await?))
        } else {
            Ok(None)
        }
    }

    // Helper para llenar datos de empresa
    async fn populate_response(
        &self,
        proveedor: crate::models::proveedor::Proveedor,
    ) -> Result<ProveedorResponse, String> {
        let empresa = empresa_queries::find_by_id(&self.pool, &proveedor.empresa_id)
            .await
            .map_err(|e| e.to_string())?;

        let proveedor_id = proveedor.id.clone();
        let mut response: ProveedorResponse = proveedor.into();
        if let Some(e) = empresa {
            response.empresa_nombre = e.nombre;
        } else {
            response.empresa_nombre = "Empresa no encontrada".to_string();
        }

        // Buscar vehículos
        let vehiculos = vehiculo_queries::find_by_proveedor(&self.pool, &proveedor_id)
            .await
            .unwrap_or_default();

        if let Some(v) = vehiculos.first() {
            response.vehiculo_tipo = Some(v.tipo_vehiculo.to_string());
            response.vehiculo_placa = Some(v.placa.clone());
        }

        Ok(response)
    }
}
