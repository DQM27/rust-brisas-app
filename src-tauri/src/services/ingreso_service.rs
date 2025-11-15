// ==========================================
// src/services/ingreso_service.rs
// ==========================================

use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::{Utc, NaiveDateTime};
use crate::models::ingreso::*;
use crate::domain::{
    errors::IngresoError,
    ingreso::validations::IngresoValidator,
};
use crate::db::{ingreso as db, gafete as gafete_db};
use crate::services::gafete_service::GafeteService;
use crate::services::gafete_perdido_service::GafetePerdidoService;

pub struct IngresoService;

impl IngresoService {
    // ==========================================
    // VALIDACIONES
    // ==========================================
    
    /// Valida si un contratista puede ingresar
    pub async fn validar_ingreso_contratista(
        pool: &SqlitePool,
        cedula: String,
    ) -> Result<ValidacionIngresoResponse, IngresoError> {
        IngresoValidator::validar_puede_ingresar(pool, &cedula).await
    }
    
    // ==========================================
    // CREAR INGRESOS
    // ==========================================
    
    /// Crea un ingreso para un contratista existente
    pub async fn crear_ingreso_contratista(
        pool: &SqlitePool,
        input: CreateIngresoContratistaInput,
    ) -> Result<IngresoResponse, IngresoError> {
        // 1. Validar
        let validated = IngresoValidator::validar_creacion_contratista(pool, &input).await?;
        
        // 2. Obtener número de gafete
        let gafete = gafete_db::find_by_id(pool, &input.gafete_id).await?;
        
        // 3. Determinar modo de ingreso
        let modo_ingreso = if validated.vehiculo_id.is_some() {
            ModoIngreso::Vehiculo
        } else {
            ModoIngreso::Caminando
        };
        
        // 4. Preparar datos
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let timestamp = Utc::now().to_rfc3339();
        
        let data = db::CreateIngresoData {
            contratista_id: Some(validated.contratista_id),
            cedula: validated.cedula,
            nombre: validated.nombre,
            apellido: validated.apellido,
            empresa_nombre: validated.empresa_nombre,
            tipo_ingreso: TipoIngreso::Contratista,
            tipo_autorizacion: TipoAutorizacion::Praind,
            modo_ingreso,
            vehiculo_id: validated.vehiculo_id,
            placa_temporal: None,
            gafete_id: input.gafete_id.clone(),
            gafete_numero: gafete.numero.clone(),
            fecha_hora_ingreso: now,
            usuario_ingreso_id: validated.usuario_ingreso_id,
            praind_vigente_al_ingreso: Some(validated.praind_vigente),
            estado_contratista_al_ingreso: Some(validated.estado),
            observaciones: validated.observaciones,
            timestamp,
        };
        
        // 5. Insertar en transacción
        let mut tx = pool.begin().await?;
        db::insertar(&mut tx, &id, &data).await?;
        
        // 6. Asignar gafete si no es S/G
        if gafete.numero.to_uppercase() != GAFETE_SIN_GAFETE {
            GafeteService::asignar_tx(
                &mut tx,
                &input.gafete_id,
                Some(input.contratista_id),
                &id,
            ).await?;
        }
        
        tx.commit().await?;
        
        // 7. Retornar
        db::find_by_id(pool, &id).await
    }
    
    /// Crea un ingreso temporal (autorización por correo)
    pub async fn crear_ingreso_temporal(
        pool: &SqlitePool,
        input: CreateIngresoTemporalInput,
    ) -> Result<IngresoResponse, IngresoError> {
        // 1. Validar
        let validated = IngresoValidator::validar_creacion_temporal(pool, &input).await?;
        
        // 2. Obtener número de gafete
        let gafete = gafete_db::find_by_id(pool, &input.gafete_id).await?;
        
        // 3. Determinar modo de ingreso
        let modo_ingreso = if validated.placa_temporal.is_some() {
            ModoIngreso::VehiculoTemporal
        } else {
            ModoIngreso::Caminando
        };
        
        // 4. Preparar datos
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let timestamp = Utc::now().to_rfc3339();
        
        let data = db::CreateIngresoData {
            contratista_id: None,
            cedula: validated.cedula,
            nombre: validated.nombre,
            apellido: validated.apellido,
            empresa_nombre: validated.empresa_nombre,
            tipo_ingreso: TipoIngreso::Temporal,
            tipo_autorizacion: TipoAutorizacion::Correo,
            modo_ingreso,
            vehiculo_id: None,
            placa_temporal: validated.placa_temporal,
            gafete_id: input.gafete_id.clone(),
            gafete_numero: gafete.numero.clone(),
            fecha_hora_ingreso: now,
            usuario_ingreso_id: validated.usuario_ingreso_id,
            praind_vigente_al_ingreso: None,
            estado_contratista_al_ingreso: None,
            observaciones: validated.observaciones,
            timestamp,
        };
        
        // 5. Insertar en transacción
        let mut tx = pool.begin().await?;
        db::insertar(&mut tx, &id, &data).await?;
        
        // 6. Asignar gafete si no es S/G
        if gafete.numero.to_uppercase() != GAFETE_SIN_GAFETE {
            GafeteService::asignar_tx(
                &mut tx,
                &input.gafete_id,
                None, // Sin contratista_id para temporales
                &id,
            ).await?;
        }
        
        tx.commit().await?;
        
        // 7. Retornar
        db::find_by_id(pool, &id).await
    }
    
    // ==========================================
    // LEER
    // ==========================================
    
    pub async fn obtener_por_id(
        pool: &SqlitePool,
        id: String,
    ) -> Result<IngresoResponse, IngresoError> {
        db::find_by_id(pool, &id).await
    }
    
    pub async fn obtener_por_gafete(
        pool: &SqlitePool,
        gafete_numero: String,
    ) -> Result<IngresoResponse, IngresoError> {
        db::find_by_gafete(pool, &gafete_numero).await
    }
    
    pub async fn listar_todos(
        pool: &SqlitePool,
    ) -> Result<IngresoListResponse, IngresoError> {
        let items = db::find_all(pool).await?;
        Ok(IngresoListResponse::new(items))
    }
    
    pub async fn listar_abiertos(
        pool: &SqlitePool,
    ) -> Result<Vec<IngresoResponse>, IngresoError> {
        db::find_abiertos(pool).await
    }
    
    // ==========================================
    // SALIDAS
    // ==========================================
    
    /// Registra salida normal (con devolución de gafete)
    pub async fn registrar_salida(
        pool: &SqlitePool,
        ingreso_id: String,
        input: RegistrarSalidaInput,
    ) -> Result<IngresoResponse, IngresoError> {
        // 1. Obtener datos del ingreso
        let ingreso = db::find_by_id(pool, &ingreso_id).await?;
        
        // 2. Calcular tiempo de permanencia
        let fecha_ingreso = NaiveDateTime::parse_from_str(
            &ingreso.fecha_hora_ingreso,
            "%Y-%m-%d %H:%M:%S"
        ).map_err(|e| IngresoError::ParseError(e.to_string()))?;
        
        let fecha_salida = Utc::now().naive_utc();
        let duracion = fecha_salida - fecha_ingreso;
        let tiempo_permanencia_minutos = duracion.num_minutes();
        
        let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let timestamp = Utc::now().to_rfc3339();
        
        let data = db::RegistrarSalidaData {
            fecha_hora_salida: now,
            tiempo_permanencia_minutos,
            usuario_salida_id: input.usuario_salida_id,
            observaciones_salida: input.observaciones_salida,
            timestamp,
        };
        
        // 3. Registrar salida
        db::registrar_salida(pool, &ingreso_id, &data).await?;
        
        // 4. Liberar gafete si lo devolvió
        if ingreso.gafete_numero.to_uppercase() != GAFETE_SIN_GAFETE && input.devolvio_gafete {
            GafeteService::liberar(pool, ingreso.gafete_id).await?;
        }
        
        // 5. Retornar
        db::find_by_id(pool, &ingreso_id).await
    }
    
    /// Registra salida con gafete perdido
    pub async fn registrar_salida_con_gafete_perdido(
        pool: &SqlitePool,
        ingreso_id: String,
        input: RegistrarSalidaConGafetePerdidoInput,
    ) -> Result<IngresoResponse, IngresoError> {
        // 1. Obtener datos del ingreso
        let ingreso = db::find_by_id(pool, &ingreso_id).await?;
        
        // Validar que no sea S/G
        if ingreso.gafete_numero.to_uppercase() == GAFETE_SIN_GAFETE {
            return Err(IngresoError::ValidationError(
                "No se puede reportar como perdido el gafete S/G".to_string()
            ));
        }
        
        // Validar que tenga contratista_id
        let contratista_id = ingreso.contratista_id
            .ok_or_else(|| IngresoError::ValidationError(
                "No se puede reportar gafete perdido para ingreso temporal sin contratista".to_string()
            ))?;
        
        // 2. Calcular tiempo de permanencia
        let fecha_ingreso = NaiveDateTime::parse_from_str(
            &ingreso.fecha_hora_ingreso,
            "%Y-%m-%d %H:%M:%S"
        ).map_err(|e| IngresoError::ParseError(e.to_string()))?;
        
        let fecha_salida = Utc::now().naive_utc();
        let duracion = fecha_salida - fecha_ingreso;
        let tiempo_permanencia_minutos = duracion.num_minutes();
        
        let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let timestamp = Utc::now().to_rfc3339();
        
        let data = db::RegistrarSalidaData {
            fecha_hora_salida: now,
            tiempo_permanencia_minutos,
            usuario_salida_id: input.usuario_salida_id.clone(),
            observaciones_salida: input.observaciones.clone(),
            timestamp,
        };
        
        // 3. Registrar salida
        db::registrar_salida(pool, &ingreso_id, &data).await?;
        
        // 4. Reportar gafete como perdido
        GafetePerdidoService::reportar(
            pool,
            crate::models::gafete_perdido::ReportarGafetePerdidoInput {
                gafete_id: ingreso.gafete_id,
                contratista_id,
                ingreso_id: Some(ingreso_id.clone()),
                monto_cobro: input.monto_cobro,
                observaciones: input.observaciones,
                reportado_por: input.usuario_salida_id,
            }
        ).await?;
        
        // 5. Retornar
        db::find_by_id(pool, &ingreso_id).await
    }
    
    /// Cierra ingreso anterior automáticamente
    pub async fn cerrar_ingreso_anterior(
        pool: &SqlitePool,
        cedula: String,
        usuario_salida_id: String,
    ) -> Result<(), IngresoError> {
        // Buscar ingreso abierto
        if let Ok(ingreso) = db::find_abierto_by_cedula(pool, &cedula).await {
            // Calcular tiempo
            let fecha_ingreso = NaiveDateTime::parse_from_str(
                &ingreso.fecha_hora_ingreso,
                "%Y-%m-%d %H:%M:%S"
            ).map_err(|e| IngresoError::ParseError(e.to_string()))?;
            
            let fecha_salida = Utc::now().naive_utc();
            let duracion = fecha_salida - fecha_ingreso;
            let tiempo_permanencia_minutos = duracion.num_minutes();
            
            let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let timestamp = Utc::now().to_rfc3339();
            
            let data = db::CerrarAutomaticoData {
                fecha_hora_salida: now,
                tiempo_permanencia_minutos,
                usuario_salida_id,
                timestamp,
            };
            
            // Cerrar ingreso
            db::cerrar_automatico(pool, &cedula, &data).await?;
            
            // Liberar gafete si tenía
            if ingreso.gafete_numero.to_uppercase() != GAFETE_SIN_GAFETE {
                let _ = GafeteService::liberar(pool, ingreso.gafete_id).await;
            }
        }
        
        Ok(())
    }
}