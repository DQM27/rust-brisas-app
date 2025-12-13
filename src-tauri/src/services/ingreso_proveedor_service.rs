use crate::db::{empresa_queries, ingreso_proveedor_queries, proveedor_queries};
use crate::domain::ingreso_proveedor::{CreateIngresoProveedorInput, IngresoProveedor};
use crate::models::proveedor::CreateProveedorInput;
use crate::services::gafete_service;
use sqlx::SqlitePool;

pub struct IngresoProveedorService {
    pool: SqlitePool,
}

impl IngresoProveedorService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn registrar_ingreso(
        &self,
        input: CreateIngresoProveedorInput,
    ) -> Result<IngresoProveedor, String> {
        // 1. Validar existencia de la empresa
        if empresa_queries::find_by_id(&self.pool, &input.empresa_id)
            .await
            .map_err(|e| e.to_string())?
            .is_none()
        {
            return Err("La empresa no existe".to_string());
        }

        // 2. Validar disponibilidad de gafete (si aplica)
        if let Some(ref g) = input.gafete {
            let disponible = gafete_service::is_gafete_disponible(&self.pool, g)
                .await
                .map_err(|e| e.to_string())?;
            if !disponible {
                return Err(format!("El gafete {} no está disponible", g));
            }
        }

        // 3. Obtener o Crear Proveedor (Catalog)
        let proveedor_id = if let Some(prov) =
            proveedor_queries::find_by_cedula(&self.pool, &input.cedula)
                .await
                .map_err(|e| e.to_string())?
        {
            prov.id
        } else {
            // Crear nuevo en catálogo
            let new_prov = proveedor_queries::create(
                &self.pool,
                CreateProveedorInput {
                    cedula: input.cedula.clone(),
                    nombre: input.nombre.clone(),
                    segundo_nombre: None,
                    apellido: input.apellido.clone(),
                    segundo_apellido: None,
                    empresa_id: input.empresa_id.clone(),
                    tiene_vehiculo: if input.placa_vehiculo.is_some() {
                        Some(true)
                    } else {
                        None
                    },
                    tipo_vehiculo: if input.placa_vehiculo.is_some() {
                        Some("automovil".to_string())
                    } else {
                        None
                    }, // Default a automovil si viene por ingreso rápido
                    placa: input.placa_vehiculo.clone(),
                    marca: None,
                    modelo: None,
                    color: None,
                },
            )
            .await
            .map_err(|e| e.to_string())?;
            new_prov.id
        };

        // 4. Crear ingreso vinculado
        ingreso_proveedor_queries::create(&self.pool, input, &proveedor_id)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn registrar_salida(
        &self,
        id: String,
        usuario_id: String,
        observaciones: Option<String>,
    ) -> Result<(), String> {
        ingreso_proveedor_queries::registrar_salida(
            &self.pool,
            &id,
            &usuario_id,
            observaciones.as_deref(),
        )
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn get_activos(&self) -> Result<Vec<IngresoProveedor>, String> {
        ingreso_proveedor_queries::find_actives(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn search_proveedores(
        &self,
        query: &str,
    ) -> Result<Vec<crate::domain::ingreso_proveedor::ProveedorSnapshot>, String> {
        ingreso_proveedor_queries::search_distinct_proveedores(&self.pool, query)
            .await
            .map_err(|e| e.to_string())
    }
}
