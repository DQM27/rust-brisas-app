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
            let disponible = gafete_service::is_gafete_disponible(&self.pool, g, "proveedor")
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
                    tipo_vehiculo: input.tipo_vehiculo.clone(), // Must be captured from input
                    placa: input.placa_vehiculo.clone(),
                    marca: input.marca_vehiculo.clone(),
                    modelo: input.modelo_vehiculo.clone(),
                    color: input.color_vehiculo.clone(),
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
        devolvio_gafete: bool,
    ) -> Result<(), String> {
        // 1. Obtener el ingreso para verificar gafete (lectura antes de tx o dentro, da igual en sqlite sin WAL estricto)
        let ingreso = ingreso_proveedor_queries::find_by_id(&self.pool, &id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("Ingreso no encontrado")?;

        let now = chrono::Utc::now().to_rfc3339();

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        // 2. Registrar salida en DB
        sqlx::query(
            r#"
            UPDATE ingresos_proveedores 
            SET estado = 'SALIO', 
                fecha_salida = ?, 
                usuario_salida_id = ?, 
                observaciones = COALESCE(?, observaciones),
                updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&now)
        .bind(&usuario_id)
        .bind(observaciones.as_deref())
        .bind(&now)
        .bind(&id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        // 3. Verificar Gafete y crear alerta si no se devolvió
        if let Some(gafete_num) = ingreso.gafete {
            if !devolvio_gafete {
                // Generar alerta de Gafete No Devuelto
                let nombre_completo = format!("{} {}", ingreso.nombre, ingreso.apellido);
                let alerta_id = uuid::Uuid::new_v4().to_string();

                // Query directa para usar TX
                sqlx::query(
                    r#"INSERT INTO alertas_gafetes 
                    (id, persona_id, cedula, nombre_completo, gafete_numero, 
                    ingreso_contratista_id, ingreso_proveedor_id,
                    fecha_reporte, resuelto, fecha_resolucion, notas, reportado_por,
                    created_at, updated_at)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, 0, NULL, ?, ?, ?, ?)"#,
                )
                .bind(&alerta_id)
                .bind(None::<&str>)
                .bind(&ingreso.cedula)
                .bind(&nombre_completo)
                .bind(&gafete_num)
                .bind(None::<&str>) // ingreso_contratista_id
                .bind(&id) // ingreso_proveedor_id
                .bind(&now)
                .bind(Some("Gafete no devuelto por proveedor al salir"))
                .bind(&usuario_id)
                .bind(&now)
                .bind(&now)
                .execute(&mut *tx)
                .await
                .map_err(|e| format!("Error creando alerta de gafete: {}", e))?;
            }
        }

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn get_activos(&self) -> Result<Vec<IngresoProveedor>, String> {
        ingreso_proveedor_queries::find_actives(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn get_historial(&self) -> Result<Vec<IngresoProveedor>, String> {
        ingreso_proveedor_queries::find_history(&self.pool)
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

    pub async fn validar_ingreso(
        &self,
        proveedor_id: String,
    ) -> Result<crate::domain::ingreso_proveedor::ValidacionIngresoProveedorResponse, String> {
        use crate::domain::ingreso_proveedor::ValidacionIngresoProveedorResponse;

        // 1. Verificar ingreso abierto
        let ingreso_abierto =
            ingreso_proveedor_queries::find_open_by_proveedor(&self.pool, &proveedor_id)
                .await
                .map_err(|e| e.to_string())?;

        if let Some(ingreso) = ingreso_abierto {
            return Ok(ValidacionIngresoProveedorResponse {
                puede_ingresar: false,
                motivo_rechazo: Some("El proveedor ya tiene un ingreso abierto".to_string()),
                alertas: vec![],
                proveedor: None,
                tiene_ingreso_abierto: true,
                ingreso_abierto: Some(ingreso),
            });
        }

        // 2. Obtener datos del proveedor
        let proveedor_opt = proveedor_queries::find_by_id(&self.pool, &proveedor_id)
            .await
            .map_err(|e| e.to_string())?;

        let proveedor = match proveedor_opt {
            Some(p) => p,
            None => return Err("Proveedor no encontrado".to_string()),
        };

        // 3. Obtener vehículos
        // Necesitamos vehicular_queries::find_by_proveedor (which returns Vec<Vehiculo>)
        let vehiculos = crate::db::vehiculo_queries::find_by_proveedor(&self.pool, &proveedor_id)
            .await
            .unwrap_or_default();

        // 4. Construir respuesta JSON
        let proveedor_json = serde_json::json!({
            "id": proveedor.id,
            "cedula": proveedor.cedula,
            "nombre": proveedor.nombre,
            "segundo_nombre": proveedor.segundo_nombre,
            "apellido": proveedor.apellido,
            "segundo_apellido": proveedor.segundo_apellido,
            "empresa_id": proveedor.empresa_id,
            "estado": proveedor.estado.as_str(),
            // Incluir lista de vehículos
            "vehiculos": vehiculos
        });

        Ok(ValidacionIngresoProveedorResponse {
            puede_ingresar: true, // Por defecto true para proveedores (no PRAIND check yet)
            motivo_rechazo: None,
            alertas: vec![],
            proveedor: Some(proveedor_json),
            tiene_ingreso_abierto: false,
            ingreso_abierto: None,
        })
    }
}
