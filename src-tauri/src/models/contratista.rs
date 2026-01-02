//! # Models: Contratista
//!
//! Estructuras de datos para el dominio de contratistas externos.
//!
//! ## Entidades Principales
//! - [`Contratista`]: Representación completa desde base de datos
//! - [`ContratistaFetched`]: Con empresa expandida (fetch)
//! - [`ContratistaResponse`]: DTO de salida al frontend
//!
//! ## DTOs de Entrada
//! - [`CreateContratistaInput`]: Input desde frontend para creación
//! - [`UpdateContratistaInput`]: Input para actualización parcial
//! - [`CambiarEstadoInput`]: Input para cambio de estado
//!
//! ## Convenciones de Fechas
//! - Campos `*_at`: Timestamps en formato SurrealDB `Datetime`
//! - Campo `fecha_vencimiento_praind`: Fecha en formato YYYY-MM-DD
//! - La validación de formatos ocurre en [`crate::domain::contratista`]
//!
//! ## Enums de Estado
//! [`EstadoContratista`] usa `lowercase` para serialización y es compatible con SurrealDB.

use crate::models::empresa::Empresa;
use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, RecordId};

// --------------------------------------------------------------------------
// MODELO PRINCIPAL
// --------------------------------------------------------------------------

/// Representa un contratista externo registrado en el sistema.
///
/// ## Ciclo de Vida
/// 1. Creado con estado [`EstadoContratista::Activo`] y PRAIND válido
/// 2. Cambia a [`EstadoContratista::Inactivo`] si PRAIND vence
/// 3. Puede ser [`EstadoContratista::Bloqueado`] por decisión administrativa
///
/// ## Relaciones
/// - Pertenece a una [`Empresa`] (campo `empresa`)
/// - Puede tener vehículos asociados
/// - Puede estar en lista negra
///
/// ## Campos Críticos para Seguridad
/// - `cedula`: Identificador único, validado en [`crate::domain::contratista`]
/// - `fecha_vencimiento_praind`: Determina si puede ingresar
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contratista {
    /// ID único en SurrealDB (formato: contratista:ulid)
    pub id: RecordId,
    /// Cédula de identidad (formato validado: números y guiones)
    pub cedula: String,
    /// Primer nombre del contratista
    pub nombre: String,
    /// Segundo nombre (opcional)
    #[serde(alias = "segundo_nombre")]
    pub segundo_nombre: Option<String>,
    /// Primer apellido del contratista
    pub apellido: String,
    /// Segundo apellido (opcional)
    #[serde(alias = "segundo_apellido")]
    pub segundo_apellido: Option<String>,
    /// Referencia a la empresa empleadora
    pub empresa: RecordId,
    /// Fecha de vencimiento de certificación PRAIND
    #[serde(alias = "fecha_vencimiento_praind")]
    pub fecha_vencimiento_praind: Datetime,
    /// Estado actual del contratista
    pub estado: EstadoContratista,
    /// Timestamp de creación
    #[serde(alias = "created_at")]
    pub created_at: Datetime,
    /// Timestamp de última actualización
    #[serde(alias = "updated_at")]
    pub updated_at: Datetime,
    /// Timestamp de eliminación lógica (soft delete)
    #[serde(alias = "deleted_at")]
    pub deleted_at: Option<Datetime>,
}

// --------------------------------------------------------------------------
// MODELO FETCHED
// --------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContratistaFetched {
    pub id: RecordId,
    pub cedula: String,
    pub nombre: String,
    #[serde(alias = "segundo_nombre")]
    pub segundo_nombre: Option<String>,
    pub apellido: String,
    #[serde(alias = "segundo_apellido")]
    pub segundo_apellido: Option<String>,
    pub empresa: Empresa,
    #[serde(alias = "fecha_vencimiento_praind")]
    pub fecha_vencimiento_praind: Datetime,
    pub estado: EstadoContratista,
    #[serde(alias = "created_at")]
    pub created_at: Datetime,
    #[serde(alias = "updated_at")]
    pub updated_at: Datetime,
    #[serde(alias = "deleted_at")]
    pub deleted_at: Option<Datetime>,
}

// --------------------------------------------------------------------------
// ENUMS DE DOMINIO
// --------------------------------------------------------------------------

/// Estados posibles de un contratista en el sistema.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")] // "activo", "inactivo", "bloqueado"
pub enum EstadoContratista {
    Activo,
    Inactivo,
    Bloqueado,
}

impl EstadoContratista {
    pub fn as_str(&self) -> &str {
        match self {
            EstadoContratista::Activo => "activo",
            EstadoContratista::Inactivo => "inactivo",
            EstadoContratista::Bloqueado => "bloqueado",
        }
    }
}

impl std::str::FromStr for EstadoContratista {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "activo" => Ok(EstadoContratista::Activo),
            "inactivo" => Ok(EstadoContratista::Inactivo),
            "bloqueado" => Ok(EstadoContratista::Bloqueado),
            _ => Err(format!("Estado desconocido: {}", s)),
        }
    }
}

// ==========================================
// DTOs de entrada (Commands/Input)
// ==========================================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateContratistaInput {
    pub cedula: String,
    pub nombre: String,
    pub segundo_nombre: Option<String>,
    pub apellido: String,
    pub segundo_apellido: Option<String>,
    pub empresa_id: String,
    pub fecha_vencimiento_praind: String,

    // Campos del Vehículo
    pub tiene_vehiculo: Option<bool>,
    pub tipo_vehiculo: Option<String>,
    pub placa: Option<String>,
    pub marca: Option<String>,
    pub modelo: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateContratistaInput {
    pub nombre: Option<String>,
    pub segundo_nombre: Option<String>,
    pub apellido: Option<String>,
    pub segundo_apellido: Option<String>,
    pub empresa_id: Option<String>,
    pub fecha_vencimiento_praind: Option<String>,
    pub tiene_vehiculo: Option<bool>,

    // Vehículo
    pub tipo_vehiculo: Option<String>,
    pub placa: Option<String>,
    pub marca: Option<String>,
    pub modelo: Option<String>,
    pub color: Option<String>,
}

/// Input para cambiar el estado de un contratista.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CambiarEstadoInput {
    /// Nuevo estado a aplicar
    pub estado: EstadoContratista,
}

/// Input para actualizar PRAIND con motivo para historial.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActualizarPraindInput {
    pub contratista_id: String,
    pub nueva_fecha_praind: String,
    pub motivo: Option<String>,
}

/// Input para cambiar estado con registro histórico detallado.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CambiarEstadoConHistorialInput {
    pub contratista_id: String,
    pub nuevo_estado: EstadoContratista,
    pub motivo: String,
}

// ==========================================
// VALUE OBJECTS DE DOMINIO (Resultados de Cálculos)
// ==========================================

/// Resultado del análisis del estado PRAIND de un contratista.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstadoPraind {
    /// Días restantes hasta el vencimiento (negativo si ya venció)
    pub dias_hasta_vencimiento: i64,
    /// True si el PRAIND ya venció (fecha estrictamente en el pasado)
    pub vencido: bool,
    /// True si faltan 30 días o menos (y no está vencido)
    pub requiere_atencion: bool,
}

// ==========================================
// DTOs PARA PERSISTENCIA (Service -> DB)
// ==========================================

#[derive(Debug, Serialize)]
pub struct ContratistaCreateDTO {
    pub cedula: String,
    pub nombre: String,
    pub segundo_nombre: Option<String>,
    pub apellido: String,
    pub segundo_apellido: Option<String>,
    pub empresa: RecordId,
    pub fecha_vencimiento_praind: Datetime,
    pub estado: EstadoContratista,
}

/// DTO tipado para actualizaciones parciales (PATCH) - los campos None se omiten.
#[derive(Debug, Serialize, Default)]
pub struct ContratistaUpdateDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apellido: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_apellido: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empresa: Option<RecordId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fecha_vencimiento_praind: Option<Datetime>,
}

// ==========================================
// DTOs de salida (Response/ViewModel)
// ==========================================

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContratistaResponse {
    pub id: String,
    pub cedula: String,
    pub nombre: String,
    pub segundo_nombre: Option<String>,
    pub apellido: String,
    pub segundo_apellido: Option<String>,
    pub nombre_completo: String,
    pub empresa_id: String,
    pub empresa_nombre: String,
    pub fecha_vencimiento_praind: String,
    pub estado: EstadoContratista,
    pub puede_ingresar: bool,
    pub praind_vencido: bool,
    pub esta_bloqueado: bool,
    pub dias_hasta_vencimiento: i64,
    pub requiere_atencion: bool,
    pub vehiculo_tipo: Option<String>,
    pub vehiculo_placa: Option<String>,
    pub vehiculo_marca: Option<String>,
    pub vehiculo_modelo: Option<String>,
    pub vehiculo_color: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

impl From<Contratista> for ContratistaResponse {
    fn from(c: Contratista) -> Self {
        use crate::domain::contratista::{
            calcular_estado_praind, construir_nombre_completo, puede_ingresar,
        };

        let raw_date_str = c.fecha_vencimiento_praind.to_string();
        // Clean SurrealDB format explicitly in infrastructure layer
        let clean_date_str = raw_date_str.trim_start_matches("d'").trim_end_matches('\'');

        // Delegar cálculo de estado PRAIND a domain (lógica pura)
        let estado_praind = calcular_estado_praind(clean_date_str);

        // Delegar construcción de nombre completo a domain
        let nombre_completo = construir_nombre_completo(
            &c.nombre,
            c.segundo_nombre.as_deref(),
            &c.apellido,
            c.segundo_apellido.as_deref(),
        );

        // Delegar regla de negocio "puede ingresar" a domain
        let puede = puede_ingresar(&c.estado, estado_praind.vencido);

        Self {
            id: c.id.to_string(),
            cedula: c.cedula.clone(),
            nombre: c.nombre.clone(),
            segundo_nombre: c.segundo_nombre.clone(),
            apellido: c.apellido.clone(),
            segundo_apellido: c.segundo_apellido.clone(),
            nombre_completo,
            empresa_id: c.empresa.to_string(),
            empresa_nombre: String::new(), // Será llenado por el servicio
            fecha_vencimiento_praind: clean_date_str.to_string(),
            estado: c.estado,
            puede_ingresar: puede,
            praind_vencido: estado_praind.vencido,
            esta_bloqueado: false,
            dias_hasta_vencimiento: estado_praind.dias_hasta_vencimiento,
            requiere_atencion: estado_praind.requiere_atencion,
            vehiculo_tipo: None,
            vehiculo_placa: None,
            vehiculo_marca: None,
            vehiculo_modelo: None,
            vehiculo_color: None,
            created_at: c.created_at.to_string(),
            updated_at: c.updated_at.to_string(),
            deleted_at: c.deleted_at.map(|d| d.to_string()),
        }
    }
}

impl ContratistaResponse {
    /// Construye un response desde un ContratistaFetched (con empresa expandida).
    pub fn from_fetched(c: ContratistaFetched) -> Self {
        use crate::domain::contratista::{
            calcular_estado_praind, construir_nombre_completo, puede_ingresar,
        };

        let raw_date_str = c.fecha_vencimiento_praind.to_string();
        // Clean SurrealDB format explicitly in infrastructure layer
        let clean_date_str = raw_date_str.trim_start_matches("d'").trim_end_matches('\'');

        // Delegar cálculo de estado PRAIND a domain (lógica pura)
        let estado_praind = calcular_estado_praind(clean_date_str);

        // Delegar construcción de nombre completo a domain
        let nombre_completo = construir_nombre_completo(
            &c.nombre,
            c.segundo_nombre.as_deref(),
            &c.apellido,
            c.segundo_apellido.as_deref(),
        );

        // Delegar regla de negocio "puede ingresar" a domain
        let puede = puede_ingresar(&c.estado, estado_praind.vencido);

        Self {
            id: c.id.to_string(),
            cedula: c.cedula.clone(),
            nombre: c.nombre.clone(),
            segundo_nombre: c.segundo_nombre.clone(),
            apellido: c.apellido.clone(),
            segundo_apellido: c.segundo_apellido.clone(),
            nombre_completo,
            empresa_id: c.empresa.id.to_string(),
            empresa_nombre: c.empresa.nombre.clone(),
            fecha_vencimiento_praind: clean_date_str.to_string(),
            estado: c.estado,
            puede_ingresar: puede,
            praind_vencido: estado_praind.vencido,
            esta_bloqueado: false,
            dias_hasta_vencimiento: estado_praind.dias_hasta_vencimiento,
            requiere_atencion: estado_praind.requiere_atencion,
            vehiculo_tipo: None,
            vehiculo_placa: None,
            vehiculo_marca: None,
            vehiculo_modelo: None,
            vehiculo_color: None,
            created_at: c.created_at.to_string(),
            updated_at: c.updated_at.to_string(),
            deleted_at: c.deleted_at.map(|d| d.to_string()),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContratistaListResponse {
    pub contratistas: Vec<ContratistaResponse>,
    pub total: usize,
    pub activos: usize,
    pub con_praind_vencido: usize,
    pub requieren_atencion: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::empresa::Empresa;
    use chrono::{Duration, Utc};
    use surrealdb::RecordId;

    /// Verifica que la conversión desde ContratistaFetched delegue correctamente
    /// los cálculos de estado (PRAIND, Ingreso) al dominio.
    #[test]
    fn test_contratista_response_from_fetched() {
        let created = Utc::now();
        let fecha_vencimiento = created + Duration::days(60); // Futuro

        let fetched = ContratistaFetched {
            id: RecordId::from(("contratista", "c1")),
            cedula: "12345678".to_string(),
            nombre: "Juan".to_string(),
            segundo_nombre: None,
            apellido: "Pérez".to_string(),
            segundo_apellido: None,
            empresa: Empresa {
                id: RecordId::from(("empresa", "e1")),
                nombre: "Empresa Test".to_string(),
                direccion: Some("Calle Falsa 123".to_string()),
                is_active: true,
                created_at: Some(created.into()),
                updated_at: Some(created.into()),
            },
            fecha_vencimiento_praind: fecha_vencimiento.into(),
            estado: EstadoContratista::Activo,
            created_at: created.into(),
            updated_at: created.into(),
            deleted_at: None,
        };

        let response = ContratistaResponse::from_fetched(fetched);

        // Validaciones directas de mapeo
        assert_eq!(response.id, "contratista:c1");
        assert_eq!(response.cedula, "12345678");
        assert_eq!(response.nombre_completo, "Juan Pérez");
        assert_eq!(response.empresa_nombre, "Empresa Test");
        assert_eq!(response.empresa_id, "empresa:e1");

        // Validaciones de lógica delegada al dominio
        assert!(response.puede_ingresar); // Activo y PRAIND futuro
        assert!(!response.praind_vencido);
        assert!(!response.requiere_atencion);
        assert_eq!(response.estado, EstadoContratista::Activo);
    }
}
