// ==========================================
// src/models/contratista.rs
// ==========================================
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contratista {
    pub id: Thing,
    pub cedula: String,
    pub nombre: String,
    pub segundo_nombre: Option<String>,
    pub apellido: String,
    pub segundo_apellido: Option<String>,
    pub empresa: Thing,
    pub fecha_vencimiento_praind: DateTime<Utc>,
    pub estado: EstadoContratista,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EstadoContratista {
    Activo,
    Inactivo,
    Suspendido,
}

impl EstadoContratista {
    pub fn as_str(&self) -> &str {
        match self {
            EstadoContratista::Activo => "activo",
            EstadoContratista::Inactivo => "inactivo",
            EstadoContratista::Suspendido => "suspendido",
        }
    }
}

impl std::str::FromStr for EstadoContratista {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "activo" => Ok(EstadoContratista::Activo),
            "inactivo" => Ok(EstadoContratista::Inactivo),
            "suspendido" => Ok(EstadoContratista::Suspendido),
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
}

#[derive(Debug, Deserialize, Default)]
pub struct UpdateContratistaInput {
    pub nombre: Option<String>,
    pub segundo_nombre: Option<String>,
    pub apellido: Option<String>,
    pub segundo_apellido: Option<String>,
    pub empresa_id: Option<String>,
    pub fecha_vencimiento_praind: Option<String>,
    pub tiene_vehiculo: Option<bool>,
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
    pub empresa: Thing,
    pub fecha_vencimiento_praind: DateTime<Utc>,
    pub estado: EstadoContratista,
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
}

impl From<Contratista> for ContratistaResponse {
    fn from(c: Contratista) -> Self {
        let hoy = Utc::now();
        let dias_hasta_vencimiento = (c.fecha_vencimiento_praind - hoy).num_days();
        let praind_vencido = c.fecha_vencimiento_praind < hoy;
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
            fecha_vencimiento_praind: c.fecha_vencimiento_praind.format("%Y-%m-%d").to_string(),
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
            created_at: c.created_at.to_rfc3339(),
            updated_at: c.updated_at.to_rfc3339(),
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
