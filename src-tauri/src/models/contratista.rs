/// Modelo de Base de Datos: Contratista.
///
/// Este módulo define la estructura para personal externo (contratistas)
/// que requiere acceso a las instalaciones.
use crate::models::empresa::Empresa;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, RecordId};

// --------------------------------------------------------------------------
// MODELO PRINCIPAL
// --------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contratista {
    pub id: RecordId,
    pub cedula: String,
    pub nombre: String,
    #[serde(alias = "segundo_nombre")]
    pub segundo_nombre: Option<String>,
    pub apellido: String,
    #[serde(alias = "segundo_apellido")]
    pub segundo_apellido: Option<String>,
    pub empresa: RecordId,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CambiarEstadoInput {
    pub estado: String,
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
        let hoy = Utc::now();
        // Access inner DateTime<Utc> directly (assuming tuple struct or Deref)
        // Si falla la compilación, ajustaremos a conversión Into estricta
        let raw_date_str = c.fecha_vencimiento_praind.to_string();
        // Limpiar formato literal de SurrealDB (ej: d'2022-01-01...')
        let raw_date = raw_date_str.trim_start_matches("d'").trim_end_matches('\'');

        let fecha_venc: DateTime<Utc> = raw_date.parse().or_else(|_| {
            // Intenta parsear como fecha simple (YYYY-MM-DD)
            chrono::NaiveDate::parse_from_str(raw_date, "%Y-%m-%d")
                .map(|d| d.and_hms_opt(0, 0, 0).unwrap().and_local_timezone(Utc).unwrap())
        }).unwrap_or_else(|_| {
            println!(">>> CRITICAL: Error parseando fecha '{}' (original: '{}'). Usando fecha actual.", raw_date, raw_date_str);
            hoy
        });

        // Usar fechas calendario para ignorar componentes de hora (que causan diffs de 0 días y vencimientos prematuros)
        let venc_date = fecha_venc.date_naive();
        let hoy_date = hoy.date_naive();

        let dias_hasta_vencimiento = (venc_date - hoy_date).num_days();

        // Vencido solo si la fecha es estrictamente en el pasado (ayer o antes)
        let praind_vencido = venc_date < hoy_date;

        // Requiere atención si faltan 30 días o menos (y no está vencido o venció hoy/ayer?)
        // dias_hasta_vencimiento >= 0 cubre hoy y futuro.
        let requiere_atencion = dias_hasta_vencimiento <= 30 && dias_hasta_vencimiento >= 0;
        let puede_ingresar = c.estado == EstadoContratista::Activo && !praind_vencido;

        let mut nombre_completo = c.nombre.clone();
        if let Some(segundo) = &c.segundo_nombre {
            nombre_completo.push(' ');
            nombre_completo.push_str(segundo);
        }
        nombre_completo.push(' ');
        nombre_completo.push_str(&c.apellido);
        if let Some(segundo) = &c.segundo_apellido {
            nombre_completo.push(' ');
            nombre_completo.push_str(segundo);
        }

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
            fecha_vencimiento_praind: fecha_venc.to_rfc3339(),
            estado: c.estado,
            puede_ingresar,
            praind_vencido,
            esta_bloqueado: false,
            dias_hasta_vencimiento,
            requiere_atencion,
            vehiculo_tipo: None,
            vehiculo_placa: None,
            vehiculo_marca: None,
            vehiculo_modelo: None,
            vehiculo_color: None,
            created_at: c.created_at.to_string(), // Mantener default o cambiar a rfc3339 si es necesario
            updated_at: c.updated_at.to_string(),
            deleted_at: c.deleted_at.map(|d| d.to_string()),
        }
    }
}

impl ContratistaResponse {
    pub fn from_fetched(c: ContratistaFetched) -> Self {
        let hoy = Utc::now();
        let raw_date_str = c.fecha_vencimiento_praind.to_string();
        let raw_date = raw_date_str.trim_start_matches("d'").trim_end_matches('\'');

        let fecha_venc: DateTime<Utc> = raw_date
            .parse()
            .or_else(|_| {
                chrono::NaiveDate::parse_from_str(raw_date, "%Y-%m-%d").map(|d| {
                    d.and_hms_opt(0, 0, 0).unwrap().and_local_timezone(Utc).unwrap()
                })
            })
            .unwrap_or_else(|_| {
                println!(
                    ">>> CRITICAL: Error parseando fecha '{}' (original: '{}'). Usando fecha actual.",
                    raw_date, raw_date_str
                );
                hoy
            });

        let venc_date = fecha_venc.date_naive();
        let hoy_date = hoy.date_naive();
        let dias_hasta_vencimiento = (venc_date - hoy_date).num_days();
        let praind_vencido = venc_date < hoy_date;
        let requiere_atencion = dias_hasta_vencimiento <= 30 && dias_hasta_vencimiento >= 0;
        let puede_ingresar = c.estado == EstadoContratista::Activo && !praind_vencido;

        let mut nombre_completo = c.nombre.clone();
        if let Some(segundo) = &c.segundo_nombre {
            nombre_completo.push(' ');
            nombre_completo.push_str(segundo);
        }
        nombre_completo.push(' ');
        nombre_completo.push_str(&c.apellido);
        if let Some(segundo) = &c.segundo_apellido {
            nombre_completo.push(' ');
            nombre_completo.push_str(segundo);
        }

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
            fecha_vencimiento_praind: fecha_venc.to_rfc3339(),
            estado: c.estado,
            puede_ingresar,
            praind_vencido,
            esta_bloqueado: false,
            dias_hasta_vencimiento,
            requiere_atencion,
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
