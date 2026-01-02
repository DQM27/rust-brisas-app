/// Modelo de Dominio: Alertas de Gafete.
///
/// Gestiona las incidencias de seguridad relacionadas con el mal uso,
/// pérdida o discrepancias en la devolución de gafetes de acceso.
///
/// Responsabilidades:
/// - Registrar el estado de la alerta (resuelta/pendiente).
/// - Vincular la alerta con el ingreso específico (trazabilidad).
/// - Almacenar la auditoría de resolución.
// ==========================================
// src/models/ingreso/alerta.rs
// ==========================================
use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, RecordId};

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
    pub fecha_reporte: Datetime,
    pub resuelto: bool,
    pub fecha_resolucion: Option<Datetime>,
    pub resuelto_por: Option<RecordId>,
    pub notas: Option<String>,
    pub reportado_por: RecordId,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

/// DTO de entrada para la creación de una nueva alerta.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateAlertaInput {
    pub id: String,
    /// ID de la persona asociada (si existe).
    pub persona_id: Option<String>,
    pub cedula: String,
    pub nombre_completo: String,
    pub gafete_numero: i32,
    pub ingreso_contratista_id: Option<String>,
    pub ingreso_proveedor_id: Option<String>,
    pub ingreso_visita_id: Option<String>,
    /// Fecha/Hora del incidente.
    pub fecha_reporte: String,
    /// Detalles adicionales de la incidencia.
    pub notas: Option<String>,
    /// ID del usuario o sistema que reporta.
    pub reportado_por: String,
}

/// DTO para la resolución de una alerta.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolverAlertaInput {
    pub alerta_id: String,
    /// Notas de resolución o descargo.
    pub notas: Option<String>,
    /// ID del usuario que resuelve la alerta (Supervisor).
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
            persona_id: a.persona.as_ref().map(std::string::ToString::to_string),
            cedula: a.cedula,
            nombre_completo: a.nombre_completo,
            gafete_numero: a.gafete_numero,
            ingreso_contratista_id: a.ingreso_contratista.as_ref().map(std::string::ToString::to_string),
            ingreso_proveedor_id: a.ingreso_proveedor.as_ref().map(std::string::ToString::to_string),
            ingreso_visita_id: a.ingreso_visita.as_ref().map(std::string::ToString::to_string),
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
// ESTRUCTURAS COMUNES: GESTIÓN DE GAFETES
// ==========================================

/// Evaluación de la devolución de un gafete durante el proceso de salida.
///
/// Determina si el comportamiento del visitante requiere la generación automática
/// de un reporte de incidencia (alerta) por pérdida o discrepancia.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DecisionReporteGafete {
    /// Indica si se debe crear una alerta en el sistema.
    pub debe_generar_reporte: bool,
    /// Razón técnica o administrativa del reporte.
    pub motivo: Option<String>,
    /// El número del gafete que debería estar bajo custodia.
    pub gafete_numero: Option<i32>,
}
