// ==========================================
// src/models/ingreso.rs
// ==========================================

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
// MODELO DE DOMINIO
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ingreso {
    pub id: RecordId,
    pub contratista: Option<RecordId>,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub empresa_nombre: String,
    pub tipo_ingreso: String,
    pub tipo_autorizacion: String,
    pub modo_ingreso: String,
    pub vehiculo: Option<RecordId>,
    pub placa_temporal: Option<String>,
    pub gafete_numero: Option<String>,
    pub gafete_tipo: Option<String>,
    pub fecha_hora_ingreso: surrealdb::Datetime,
    pub fecha_hora_salida: Option<surrealdb::Datetime>,
    pub tiempo_permanencia_minutos: Option<i64>,
    pub usuario_ingreso: RecordId,
    pub usuario_salida: Option<RecordId>,
    pub praind_vigente_al_ingreso: Option<bool>,
    pub estado_contratista_al_ingreso: Option<String>,
    pub observaciones: Option<String>,
    pub anfitrion: Option<String>,
    pub area_visitada: Option<String>,
    pub motivo: Option<String>,
    pub created_at: surrealdb::Datetime,
    pub updated_at: surrealdb::Datetime,
}

// ==========================================
// DTOs DE ENTRADA
// ==========================================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIngresoContratistaInput {
    pub contratista_id: String,
    pub vehiculo_id: Option<String>,
    pub gafete_numero: Option<String>,
    pub gafete_tipo: Option<String>,
    pub tipo_autorizacion: String,
    pub modo_ingreso: String,
    pub observaciones: Option<String>,
    pub usuario_ingreso_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIngresoVisitaInput {
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub anfitrion: String,
    pub area_visitada: String,
    pub motivo_visita: String,
    pub tipo_autorizacion: String,
    pub modo_ingreso: String,
    pub vehiculo_placa: Option<String>,
    pub gafete_numero: Option<String>,
    pub observaciones: Option<String>,
    pub usuario_ingreso_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIngresoProveedorInput {
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub empresa_id: String,
    pub area_visitada: String,
    pub motivo: String,
    pub tipo_autorizacion: String,
    pub modo_ingreso: String,
    pub vehiculo_placa: Option<String>,
    pub gafete_numero: Option<String>,
    pub observaciones: Option<String>,
    pub usuario_ingreso_id: String,
}

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

// ==========================================
// DTO PARA PERSISTENCIA (Service -> DB)
// ==========================================

#[derive(Debug, Serialize)]
pub struct IngresoCreateDTO {
    pub contratista: Option<RecordId>,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub empresa_nombre: String,
    pub tipo_ingreso: String,
    pub tipo_autorizacion: String,
    pub modo_ingreso: String,
    pub vehiculo: Option<RecordId>,
    pub placa_temporal: Option<String>,
    pub gafete_numero: Option<String>,
    pub gafete_tipo: Option<String>,
    pub fecha_hora_ingreso: surrealdb::Datetime,
    pub usuario_ingreso: RecordId,
    pub praind_vigente_al_ingreso: Option<bool>,
    pub estado_contratista_al_ingreso: Option<String>,
    pub observaciones: Option<String>,
    pub anfitrion: Option<String>,
    pub area_visitada: Option<String>,
    pub motivo: Option<String>,
}

// ==========================================
// DTOs DE SALIDA
// ==========================================

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
    pub gafete_numero: Option<String>,
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

impl TryFrom<Ingreso> for IngresoResponse {
    type Error = String;

    fn try_from(i: Ingreso) -> Result<Self, Self::Error> {
        let esta_adentro = i.fecha_hora_salida.is_none();
        let tiene_gafete_asignado = i.gafete_numero.is_some();

        let tiempo_permanencia_texto = i.tiempo_permanencia_minutos.map(|mins| {
            let horas = mins / 60;
            let minutos = mins % 60;
            if horas > 0 {
                format!("{}h {}m", horas, minutos)
            } else {
                format!("{}m", minutos)
            }
        });

        let tipo_ingreso: TipoIngreso = i.tipo_ingreso.parse()?;
        let tipo_autorizacion: TipoAutorizacion = i.tipo_autorizacion.parse()?;
        let modo_ingreso: ModoIngreso = i.modo_ingreso.parse()?;

        Ok(Self {
            id: i.id.to_string(),
            contratista_id: i.contratista.as_ref().map(|t| t.to_string()),
            cedula: i.cedula.clone(),
            nombre: i.nombre.clone(),
            apellido: i.apellido.clone(),
            nombre_completo: format!("{} {}", i.nombre, i.apellido),
            empresa_nombre: i.empresa_nombre,
            tipo_ingreso: tipo_ingreso.clone(),
            tipo_ingreso_display: tipo_ingreso.display().to_string(),
            tipo_autorizacion: tipo_autorizacion.clone(),
            tipo_autorizacion_display: match tipo_autorizacion {
                TipoAutorizacion::Praind => "PRAIND",
                TipoAutorizacion::Correo => "Correo",
            }
            .to_string(),
            modo_ingreso: modo_ingreso.clone(),
            modo_ingreso_display: modo_ingreso.display().to_string(),
            vehiculo_id: i.vehiculo.as_ref().map(|t| t.to_string()),
            vehiculo_placa: None,
            placa_temporal: i.placa_temporal,
            gafete_numero: i.gafete_numero,
            fecha_hora_ingreso: i.fecha_hora_ingreso.to_string(),
            fecha_hora_salida: i.fecha_hora_salida.map(|d| d.to_string()),
            tiempo_permanencia_minutos: i.tiempo_permanencia_minutos,
            tiempo_permanencia_texto,
            usuario_ingreso_id: i.usuario_ingreso.to_string(),
            usuario_ingreso_nombre: String::new(),
            usuario_salida_id: i.usuario_salida.as_ref().map(|t| t.to_string()),
            usuario_salida_nombre: None,
            praind_vigente_al_ingreso: i.praind_vigente_al_ingreso,
            estado_contratista_al_ingreso: i.estado_contratista_al_ingreso,
            observaciones: i.observaciones,
            esta_adentro,
            tiene_gafete_asignado,
            created_at: i.created_at.to_string(),
            updated_at: i.updated_at.to_string(),
        })
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
    pub gafete_numero: String,
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
    pub gafete_numero: String,
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
