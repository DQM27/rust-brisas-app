// ==========================================
// src/models/contratista.rs
// ==========================================
use crate::models::empresa::Empresa;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, RecordId};

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
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

    // Vehicle fields
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

/// DTO tipado para partial updates - campos None se omiten en merge
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
        // If compilation fails, we will adjust to strict Into conversion
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

        // Use calendar dates for comparison to ignore time components (which cause 0-day diffs and premature expiration)
        let venc_date = fecha_venc.date_naive();
        let hoy_date = hoy.date_naive();

        let dias_hasta_vencimiento = (venc_date - hoy_date).num_days();

        // Expired only if the date is strictly in the past (yesterday or earlier)
        let praind_vencido = venc_date < hoy_date;

        // Requiere atención si faltan 30 días o menos (y no está vencido o venció hoy/ayer?)
        // dias_hasta_vencimiento >= 0 covers today and future.
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
            empresa_nombre: String::new(), // Will be filled by service
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
            created_at: c.created_at.to_string(), // Keep default or change to rfc3339 if needed
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

// ==========================================
// Validaciones de dominio
// ==========================================

pub mod validaciones {
    use chrono::{DateTime, NaiveDate, TimeZone, Utc};

    pub fn validar_cedula(cedula: &str) -> Result<(), String> {
        let limpia = cedula.trim();

        if limpia.is_empty() {
            return Err("La cédula no puede estar vacía".to_string());
        }

        if !limpia.chars().all(|c| c.is_numeric() || c == '-') {
            return Err("La cédula solo puede contener números y guiones".to_string());
        }

        if limpia.len() < 7 || limpia.len() > 20 {
            return Err("La cédula debe tener entre 7 y 20 caracteres".to_string());
        }

        Ok(())
    }

    pub fn validar_nombre(nombre: &str) -> Result<(), String> {
        let limpio = nombre.trim();

        if limpio.is_empty() {
            return Err("El nombre no puede estar vacío".to_string());
        }

        if limpio.len() > 50 {
            return Err("El nombre no puede exceder 50 caracteres".to_string());
        }

        Ok(())
    }

    pub fn validar_segundo_nombre(segundo_nombre: Option<&String>) -> Result<(), String> {
        if let Some(nombre) = segundo_nombre {
            let limpio = nombre.trim();

            if !limpio.is_empty() && limpio.len() > 50 {
                return Err("El segundo nombre no puede exceder 50 caracteres".to_string());
            }
        }

        Ok(())
    }

    pub fn validar_apellido(apellido: &str) -> Result<(), String> {
        let limpio = apellido.trim();

        if limpio.is_empty() {
            return Err("El apellido no puede estar vacío".to_string());
        }

        if limpio.len() > 50 {
            return Err("El apellido no puede exceder 50 caracteres".to_string());
        }

        Ok(())
    }

    pub fn validar_segundo_apellido(segundo_apellido: Option<&String>) -> Result<(), String> {
        if let Some(apellido) = segundo_apellido {
            let limpio = apellido.trim();

            if !limpio.is_empty() && limpio.len() > 50 {
                return Err("El segundo apellido no puede exceder 50 caracteres".to_string());
            }
        }

        Ok(())
    }

    pub fn validar_empresa_id(empresa_id: &str) -> Result<(), String> {
        let limpia = empresa_id.trim();

        if limpia.is_empty() {
            return Err("Debe seleccionar una empresa".to_string());
        }

        Ok(())
    }

    pub fn validar_fecha(fecha_str: &str) -> Result<DateTime<Utc>, String> {
        let naive = NaiveDate::parse_from_str(fecha_str, "%Y-%m-%d")
            .map_err(|_| "Formato de fecha inválido. Use YYYY-MM-DD".to_string())?;

        Ok(Utc.from_utc_datetime(&naive.and_hms_opt(0, 0, 0).unwrap()))
    }

    pub fn validar_create_input(input: &super::CreateContratistaInput) -> Result<(), String> {
        validar_cedula(&input.cedula)?;
        validar_nombre(&input.nombre)?;
        validar_segundo_nombre(input.segundo_nombre.as_ref())?;
        validar_apellido(&input.apellido)?;
        validar_segundo_apellido(input.segundo_apellido.as_ref())?;
        validar_empresa_id(&input.empresa_id)?;
        validar_fecha(&input.fecha_vencimiento_praind)?;
        Ok(())
    }
}
