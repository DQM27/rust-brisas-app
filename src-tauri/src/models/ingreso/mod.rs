// src/models/ingreso/mod.rs
pub mod contratista;
pub mod proveedor;
pub mod visita;

pub use contratista::*;
pub use proveedor::*;
pub use visita::*;

use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

// ==========================================
// ENUMS DE DOMINIO
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TipoIngreso {
    Contratista,
    Visita,
    Proveedor,
}

impl TipoIngreso {
    pub fn as_str(&self) -> &str {
        match self {
            TipoIngreso::Contratista => "contratista",
            TipoIngreso::Visita => "visita",
            TipoIngreso::Proveedor => "proveedor",
        }
    }

    pub fn display(&self) -> &str {
        match self {
            TipoIngreso::Contratista => "Contratista",
            TipoIngreso::Visita => "Visita",
            TipoIngreso::Proveedor => "Proveedor",
        }
    }
}

impl std::str::FromStr for TipoIngreso {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "contratista" => Ok(TipoIngreso::Contratista),
            "visita" => Ok(TipoIngreso::Visita),
            "proveedor" => Ok(TipoIngreso::Proveedor),
            _ => Err(format!("Tipo de ingreso desconocido: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TipoAutorizacion {
    Praind,
    Correo,
}

impl TipoAutorizacion {
    pub fn as_str(&self) -> &str {
        match self {
            TipoAutorizacion::Praind => "praind",
            TipoAutorizacion::Correo => "correo",
        }
    }
}

impl std::str::FromStr for TipoAutorizacion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "praind" => Ok(TipoAutorizacion::Praind),
            "correo" => Ok(TipoAutorizacion::Correo),
            _ => Err(format!("Tipo de autorización desconocido: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ModoIngreso {
    Caminando,
    Vehiculo,
}

impl ModoIngreso {
    pub fn as_str(&self) -> &str {
        match self {
            ModoIngreso::Caminando => "caminando",
            ModoIngreso::Vehiculo => "vehiculo",
        }
    }

    pub fn display(&self) -> &str {
        match self {
            ModoIngreso::Caminando => "Caminando",
            ModoIngreso::Vehiculo => "Vehículo",
        }
    }
}

impl std::str::FromStr for ModoIngreso {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "caminando" => Ok(ModoIngreso::Caminando),
            "vehiculo" => Ok(ModoIngreso::Vehiculo),
            _ => Err(format!("Modo de ingreso desconocido: {}", s)),
        }
    }
}

// ==========================================
// DTOs COMPARTIDOS
// ==========================================

#[derive(Debug, Deserialize)]
#[serde(tag = "tipo", rename_all = "lowercase")]
pub enum CreateIngresoInput {
    Contratista(CreateIngresoContratistaInput),
    Visita(CreateIngresoVisitaInput),
    Proveedor(CreateIngresoProveedorInput),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrarSalidaInput {
    pub ingreso_id: String,
    pub devolvio_gafete: bool,
    pub usuario_salida_id: String,
    pub observaciones_salida: Option<String>,
}

#[derive(Debug, Serialize, Default, Clone)]
pub struct IngresoUpdateDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fecha_hora_salida: Option<surrealdb::Datetime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usuario_salida: Option<RecordId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observaciones: Option<String>,
}

pub type IngresoContratistaUpdateDTO = IngresoUpdateDTO;
pub type IngresoProveedorUpdateDTO = IngresoUpdateDTO;
pub type IngresoVisitaUpdateDTO = IngresoUpdateDTO;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IngresoResponse {
    pub id: String,
    pub contratista_id: Option<String>,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub nombre_completo: String,
    pub empresa_nombre: String,
    pub tipo_ingreso: TipoIngreso,
    pub tipo_ingreso_display: String,
    pub tipo_autorizacion: TipoAutorizacion,
    pub tipo_autorizacion_display: String,
    pub modo_ingreso: ModoIngreso,
    pub modo_ingreso_display: String,
    pub vehiculo_id: Option<String>,
    pub vehiculo_placa: Option<String>,
    pub placa_temporal: Option<String>,
    pub gafete_numero: Option<i32>,
    pub fecha_hora_ingreso: String,
    pub fecha_hora_salida: Option<String>,
    pub tiempo_permanencia_minutos: Option<i64>,
    pub tiempo_permanencia_texto: Option<String>,
    pub usuario_ingreso_id: String,
    pub usuario_ingreso_nombre: String,
    pub usuario_salida_id: Option<String>,
    pub usuario_salida_nombre: Option<String>,
    pub praind_vigente_al_ingreso: Option<bool>,
    pub estado_contratista_al_ingreso: Option<String>,
    pub observaciones: Option<String>,
    pub esta_adentro: bool,
    pub tiene_gafete_asignado: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl IngresoResponse {
    pub fn from_contratista(i: IngresoContratista) -> Self {
        let esta_adentro = i.fecha_hora_salida.is_none();
        let tiene_gafete_asignado = i.gafete_numero.is_some();

        Self {
            id: i.id.to_string(),
            contratista_id: Some(i.contratista.to_string()),
            cedula: i.cedula.clone(),
            nombre: i.nombre.clone(),
            apellido: i.apellido.clone(),
            nombre_completo: format!("{} {}", i.nombre, i.apellido),
            empresa_nombre: String::new(),
            tipo_ingreso: TipoIngreso::Contratista,
            tipo_ingreso_display: "Contratista".to_string(),
            tipo_autorizacion: i.tipo_autorizacion.parse().unwrap_or(TipoAutorizacion::Praind),
            tipo_autorizacion_display: i.tipo_autorizacion.to_uppercase(),
            modo_ingreso: i.modo_ingreso.parse().unwrap_or(ModoIngreso::Caminando),
            modo_ingreso_display: i.modo_ingreso.clone(),
            vehiculo_id: None,
            vehiculo_placa: i.placa_vehiculo.clone(),
            placa_temporal: None,
            gafete_numero: i.gafete_numero,
            fecha_hora_ingreso: i.fecha_hora_ingreso.to_string(),
            fecha_hora_salida: i.fecha_hora_salida.map(|d| d.to_string()),
            tiempo_permanencia_minutos: None,
            tiempo_permanencia_texto: None,
            usuario_ingreso_id: i.usuario_ingreso.to_string(),
            usuario_ingreso_nombre: String::new(),
            usuario_salida_id: i.usuario_salida.as_ref().map(|t| t.to_string()),
            usuario_salida_nombre: None,
            praind_vigente_al_ingreso: None,
            estado_contratista_al_ingreso: None,
            observaciones: i.observaciones,
            esta_adentro,
            tiene_gafete_asignado,
            created_at: i.created_at.to_string(),
            updated_at: i.updated_at.to_string(),
        }
    }

    pub fn from_contratista_fetched(i: IngresoContratistaFetched) -> Result<Self, String> {
        let esta_adentro = i.fecha_hora_salida.is_none();
        let tiene_gafete_asignado = i.gafete_numero.is_some();

        Ok(Self {
            id: i.id.to_string(),
            contratista_id: Some(i.contratista.id.to_string()),
            cedula: i.cedula.clone(),
            nombre: i.nombre.clone(),
            apellido: i.apellido.clone(),
            nombre_completo: format!("{} {}", i.nombre, i.apellido),
            empresa_nombre: i.contratista.empresa.nombre.clone(),
            tipo_ingreso: TipoIngreso::Contratista,
            tipo_ingreso_display: "Contratista".to_string(),
            tipo_autorizacion: i.tipo_autorizacion.parse().unwrap_or(TipoAutorizacion::Praind),
            tipo_autorizacion_display: i.tipo_autorizacion.to_uppercase(),
            modo_ingreso: i.modo_ingreso.parse().unwrap_or(ModoIngreso::Caminando),
            modo_ingreso_display: i.modo_ingreso.clone(),
            vehiculo_id: None,
            vehiculo_placa: i.placa_vehiculo.clone(),
            placa_temporal: None,
            gafete_numero: i.gafete_numero,
            fecha_hora_ingreso: i.fecha_hora_ingreso.to_string(),
            fecha_hora_salida: i.fecha_hora_salida.map(|d| d.to_string()),
            tiempo_permanencia_minutos: None,
            tiempo_permanencia_texto: None,
            usuario_ingreso_id: i.usuario_ingreso.id.to_string(),
            usuario_ingreso_nombre: format!(
                "{} {}",
                i.usuario_ingreso.nombre, i.usuario_ingreso.apellido
            ),
            usuario_salida_id: i.usuario_salida.as_ref().map(|t| t.id.to_string()),
            usuario_salida_nombre: i
                .usuario_salida
                .as_ref()
                .map(|u| format!("{} {}", u.nombre, u.apellido)),
            praind_vigente_al_ingreso: None,
            estado_contratista_al_ingreso: None,
            observaciones: i.observaciones,
            esta_adentro,
            tiene_gafete_asignado,
            created_at: i.created_at.to_string(),
            updated_at: i.updated_at.to_string(),
        })
    }

    pub fn from_visita_fetched(i: IngresoVisitaFetched) -> Self {
        let esta_adentro = i.fecha_hora_salida.is_none();
        let tiene_gafete_asignado = i.gafete_numero.is_some();

        Self {
            id: i.id.to_string(),
            contratista_id: None,
            cedula: i.cedula.clone(),
            nombre: i.nombre.clone(),
            apellido: i.apellido.clone(),
            nombre_completo: format!("{} {}", i.nombre, i.apellido),
            empresa_nombre: String::new(),
            tipo_ingreso: TipoIngreso::Visita,
            tipo_ingreso_display: "Visita".to_string(),
            tipo_autorizacion: TipoAutorizacion::Correo,
            tipo_autorizacion_display: "Visita".to_string(),
            modo_ingreso: i.modo_ingreso.parse().unwrap_or(ModoIngreso::Caminando),
            modo_ingreso_display: i.modo_ingreso.clone(),
            vehiculo_id: None,
            vehiculo_placa: i.placa_vehiculo.clone(),
            placa_temporal: None,
            gafete_numero: i.gafete_numero,
            fecha_hora_ingreso: i.fecha_hora_ingreso.to_string(),
            fecha_hora_salida: i.fecha_hora_salida.map(|d| d.to_string()),
            tiempo_permanencia_minutos: None,
            tiempo_permanencia_texto: None,
            usuario_ingreso_id: i.usuario_ingreso.id.to_string(),
            usuario_ingreso_nombre: format!(
                "{} {}",
                i.usuario_ingreso.nombre, i.usuario_ingreso.apellido
            ),
            usuario_salida_id: i.usuario_salida.as_ref().map(|t| t.id.to_string()),
            usuario_salida_nombre: i
                .usuario_salida
                .as_ref()
                .map(|u| format!("{} {}", u.nombre, u.apellido)),
            praind_vigente_al_ingreso: None,
            estado_contratista_al_ingreso: None,
            observaciones: i.observaciones,
            esta_adentro,
            tiene_gafete_asignado,
            created_at: i.created_at.to_string(),
            updated_at: i.updated_at.to_string(),
        }
    }

    pub fn from_proveedor_fetched(i: IngresoProveedorFetched) -> Self {
        let esta_adentro = i.fecha_hora_salida.is_none();
        let tiene_gafete_asignado = i.gafete_numero.is_some();

        Self {
            id: i.id.to_string(),
            contratista_id: None,
            cedula: i.cedula.clone(),
            nombre: i.nombre.clone(),
            apellido: i.apellido.clone(),
            nombre_completo: format!("{} {}", i.nombre, i.apellido),
            empresa_nombre: i.proveedor.empresa.nombre.clone(),
            tipo_ingreso: TipoIngreso::Proveedor,
            tipo_ingreso_display: "Proveedor".to_string(),
            tipo_autorizacion: TipoAutorizacion::Correo,
            tipo_autorizacion_display: "Proveedor".to_string(),
            modo_ingreso: i.modo_ingreso.parse().unwrap_or(ModoIngreso::Caminando),
            modo_ingreso_display: i.modo_ingreso.clone(),
            vehiculo_id: None,
            vehiculo_placa: i.placa_vehiculo.clone(),
            placa_temporal: None,
            gafete_numero: i.gafete_numero,
            fecha_hora_ingreso: i.fecha_hora_ingreso.to_string(),
            fecha_hora_salida: i.fecha_hora_salida.map(|d| d.to_string()),
            tiempo_permanencia_minutos: None,
            tiempo_permanencia_texto: None,
            usuario_ingreso_id: i.usuario_ingreso.id.to_string(),
            usuario_ingreso_nombre: format!(
                "{} {}",
                i.usuario_ingreso.nombre, i.usuario_ingreso.apellido
            ),
            usuario_salida_id: i.usuario_salida.as_ref().map(|t| t.id.to_string()),
            usuario_salida_nombre: i
                .usuario_salida
                .as_ref()
                .map(|u| format!("{} {}", u.nombre, u.apellido)),
            praind_vigente_al_ingreso: None,
            estado_contratista_al_ingreso: None,
            observaciones: i.observaciones,
            esta_adentro,
            tiene_gafete_asignado,
            created_at: i.created_at.to_string(),
            updated_at: i.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IngresoListResponse {
    pub ingresos: Vec<IngresoResponse>,
    pub total: usize,
    pub adentro: usize,
    pub salieron: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidacionIngresoResponse {
    pub puede_ingresar: bool,
    pub motivo_rechazo: Option<String>,
    pub alertas: Vec<String>,
    pub contratista: Option<serde_json::Value>,
    pub tiene_ingreso_abierto: bool,
    pub ingreso_abierto: Option<IngresoResponse>,
}

// ==========================================
// MODELO DE ALERTA DE GAFETE
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertaGafete {
    pub id: RecordId,
    pub persona: Option<RecordId>,
    pub cedula: String,
    pub nombre_completo: String,
    pub gafete_numero: i32,
    pub ingreso_contratista: Option<RecordId>,
    pub ingreso_proveedor: Option<RecordId>,
    pub ingreso_visita: Option<RecordId>,
    pub fecha_reporte: surrealdb::Datetime,
    pub resuelto: bool,
    pub fecha_resolucion: Option<surrealdb::Datetime>,
    pub resuelto_por: Option<RecordId>,
    pub notas: Option<String>,
    pub reportado_por: RecordId,
    pub created_at: surrealdb::Datetime,
    pub updated_at: surrealdb::Datetime,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolverAlertaInput {
    pub alerta_id: String,
    pub notas: Option<String>,
    pub usuario_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AlertaGafeteResponse {
    pub id: String,
    pub persona_id: Option<String>,
    pub cedula: String,
    pub nombre_completo: String,
    pub gafete_numero: i32,
    pub ingreso_contratista_id: Option<String>,
    pub ingreso_proveedor_id: Option<String>,
    pub ingreso_visita_id: Option<String>,
    pub fecha_reporte: String,
    pub resuelto: bool,
    pub fecha_resolucion: Option<String>,
    pub notas: Option<String>,
    pub reportado_por: String,
    pub reportado_por_nombre: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<AlertaGafete> for AlertaGafeteResponse {
    fn from(a: AlertaGafete) -> Self {
        Self {
            id: a.id.to_string(),
            persona_id: a.persona.as_ref().map(|t| t.to_string()),
            cedula: a.cedula,
            nombre_completo: a.nombre_completo,
            gafete_numero: a.gafete_numero,
            ingreso_contratista_id: a.ingreso_contratista.as_ref().map(|t| t.to_string()),
            ingreso_proveedor_id: a.ingreso_proveedor.as_ref().map(|t| t.to_string()),
            ingreso_visita_id: a.ingreso_visita.as_ref().map(|t| t.to_string()),
            fecha_reporte: a.fecha_reporte.to_string(),
            resuelto: a.resuelto,
            fecha_resolucion: a.fecha_resolucion.map(|d| d.to_string()),
            notas: a.notas,
            reportado_por: a.reportado_por.to_string(),
            reportado_por_nombre: String::new(),
            created_at: a.created_at.to_string(),
            updated_at: a.updated_at.to_string(),
        }
    }
}

// ==========================================
// ESTRUCTURAS DE VALIDACIÓN
// ==========================================

/// Estados de validación para el motor unificado
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ValidationStatus {
    Allowed,
    Denied,
    Warning,
}

/// Razones de rechazo o advertencia
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationReason {
    None,
    Blacklisted,
    AlreadyInside,
    ExpiredDocuments,
    GafeteAlert,
}

/// Resultado unificado de validación
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationResult {
    pub status: ValidationStatus,
    pub reason: ValidationReason,
    pub message: String,
}

/// Contexto común para validación (usado por el motor)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonValidationContext {
    pub cedula: String,
    pub nombre_completo: String,
    pub tipo_identidad: String,
    pub lista_negra_info: Option<ListaNegraInfo>,
    pub ingreso_activo_id: Option<String>,
    pub fecha_ingreso_activo: Option<String>,
    pub gafete_activo_numero: Option<i32>,
    pub estado_autorizacion: String,
    pub alerta_gafete: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListaNegraInfo {
    pub motivo: String,
    pub severidad: crate::models::lista_negra::NivelSeveridad,
}

/// Resultado de validación de entrada (legacy - para contratistas)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultadoValidacionEntrada {
    pub puede_ingresar: bool,
    pub motivo_rechazo: Option<String>,
    pub alertas: Vec<String>,
}

// ==========================================
// ERRORES COMUNES DE DOMINIO
// ==========================================

/// Definición de errores comunes para validaciones transversales de dominio.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "message")]
pub enum CommonError {
    FechaIngresoInvalida,
    FechaSalidaInvalida,
    SalidaAnteriorAIngreso,
    GafeteNoCoincide { devuelto: String, asignado: String },
    Validation(String),
}

impl std::fmt::Display for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommonError::FechaIngresoInvalida => {
                write!(f, "Datos corruptos: fecha de ingreso inválida")
            }
            CommonError::FechaSalidaInvalida => write!(f, "Fecha de salida inválida"),
            CommonError::SalidaAnteriorAIngreso => {
                write!(f, "La fecha de salida no puede ser anterior a la de ingreso")
            }
            CommonError::GafeteNoCoincide { devuelto, asignado } => write!(
                f,
                "El gafete devuelto ({}) no coincide con el asignado ({})",
                devuelto, asignado
            ),
            CommonError::Validation(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for CommonError {}

// ==========================================
// ESTRUCTURAS COMUNES: GESTIÓN DE GAFETES
// ==========================================

/// Evaluación de la devolución de un gafete durante el proceso de salida.
///
/// Determina si el comportamiento del visitante requiere la generación automática
/// de un reporte de incidencia (alerta) por pérdida o discrepancia.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecisionReporteGafete {
    /// Indica si se debe crear una alerta en el sistema.
    pub debe_generar_reporte: bool,
    /// Razón técnica o administrativa del reporte.
    pub motivo: Option<String>,
    /// El número del gafete que debería estar bajo custodia.
    pub gafete_numero: Option<i32>,
}

impl Default for DecisionReporteGafete {
    fn default() -> Self {
        Self { debe_generar_reporte: false, motivo: None, gafete_numero: None }
    }
}
