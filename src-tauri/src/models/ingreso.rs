// ==========================================
// src/models/ingreso.rs
// ==========================================
// Solo modelos, DTOs y enums - SIN validaciones ni lógica

use serde::{Deserialize, Serialize};

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

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "contratista" => Ok(TipoIngreso::Contratista),
            "visita" => Ok(TipoIngreso::Visita),
            "proveedor" => Ok(TipoIngreso::Proveedor),
            _ => Err(format!("Tipo de ingreso desconocido: {}", s)),
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

    pub fn from_str(s: &str) -> Result<Self, String> {
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

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "caminando" => Ok(ModoIngreso::Caminando),
            "vehiculo" => Ok(ModoIngreso::Vehiculo),
            _ => Err(format!("Modo de ingreso desconocido: {}", s)),
        }
    }

    pub fn display(&self) -> &str {
        match self {
            ModoIngreso::Caminando => "Caminando",
            ModoIngreso::Vehiculo => "Vehículo",
        }
    }
}

// ==========================================
// MODELO DE DOMINIO
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ingreso {
    pub id: String,
    pub contratista_id: Option<String>,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub empresa_nombre: String,
    pub tipo_ingreso: String,
    pub tipo_autorizacion: String,
    pub modo_ingreso: String,
    pub vehiculo_id: Option<String>,
    pub placa_temporal: Option<String>, // Mantener para compatibilidad futura
    pub gafete_numero: Option<String>,  // NULL = sin gafete
    pub fecha_hora_ingreso: String,
    pub fecha_hora_salida: Option<String>,
    pub tiempo_permanencia_minutos: Option<i64>,
    pub usuario_ingreso_id: String,
    pub usuario_salida_id: Option<String>,
    pub praind_vigente_al_ingreso: Option<bool>,
    pub estado_contratista_al_ingreso: Option<String>,
    pub observaciones: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// ==========================================
// DTOs DE ENTRADA
// ==========================================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIngresoContratistaInput {
    pub contratista_id: String,
    pub vehiculo_id: Option<String>,
    pub gafete_numero: Option<String>, // NULL = sin gafete
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
    pub vehiculo_placa: Option<String>, // Placa temporal
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
    pub empresa_id: String, // FK a tabla empresas
    pub area_visitada: String,
    pub motivo: String,
    pub tipo_autorizacion: String,
    pub modo_ingreso: String,
    pub vehiculo_placa: Option<String>, // Placa temporal
    pub gafete_numero: Option<String>,
    pub observaciones: Option<String>,
    pub usuario_ingreso_id: String,
}

/// Input unificado usando tagged union
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

impl From<Ingreso> for IngresoResponse {
    fn from(i: Ingreso) -> Self {
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

        let tipo_ingreso =
            TipoIngreso::from_str(&i.tipo_ingreso).unwrap_or(TipoIngreso::Contratista);
        let tipo_autorizacion =
            TipoAutorizacion::from_str(&i.tipo_autorizacion).unwrap_or(TipoAutorizacion::Praind);
        let modo_ingreso = ModoIngreso::from_str(&i.modo_ingreso).unwrap_or(ModoIngreso::Caminando);

        Self {
            id: i.id,
            contratista_id: i.contratista_id,
            cedula: i.cedula.clone(),
            nombre: i.nombre.clone(),
            apellido: i.apellido.clone(),
            nombre_completo: format!("{} {}", i.nombre, i.apellido),
            empresa_nombre: i.empresa_nombre,
            tipo_ingreso: tipo_ingreso.clone(),
            tipo_ingreso_display: "Contratista".to_string(),
            tipo_autorizacion: tipo_autorizacion.clone(),
            tipo_autorizacion_display: match tipo_autorizacion {
                TipoAutorizacion::Praind => "PRAIND",
                TipoAutorizacion::Correo => "Correo",
            }
            .to_string(),
            modo_ingreso: modo_ingreso.clone(),
            modo_ingreso_display: modo_ingreso.display().to_string(),
            vehiculo_id: i.vehiculo_id,
            vehiculo_placa: None, // Se llena con JOIN
            placa_temporal: i.placa_temporal,
            gafete_numero: i.gafete_numero,
            fecha_hora_ingreso: i.fecha_hora_ingreso,
            fecha_hora_salida: i.fecha_hora_salida,
            tiempo_permanencia_minutos: i.tiempo_permanencia_minutos,
            tiempo_permanencia_texto,
            usuario_ingreso_id: i.usuario_ingreso_id,
            usuario_ingreso_nombre: String::new(), // Se llena con JOIN
            usuario_salida_id: i.usuario_salida_id,
            usuario_salida_nombre: None, // Se llena con JOIN
            praind_vigente_al_ingreso: i.praind_vigente_al_ingreso,
            estado_contratista_al_ingreso: i.estado_contratista_al_ingreso,
            observaciones: i.observaciones,
            esta_adentro,
            tiene_gafete_asignado,
            created_at: i.created_at,
            updated_at: i.updated_at,
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
    pub id: String,
    pub persona_id: Option<String>, // contratista_id si existe
    pub cedula: String,
    pub nombre_completo: String,
    pub gafete_numero: String,
    pub ingreso_contratista_id: Option<String>,
    pub ingreso_proveedor_id: Option<String>,
    pub fecha_reporte: String,
    pub resuelto: bool,
    pub fecha_resolucion: Option<String>,
    pub resuelto_por: Option<String>,
    pub notas: Option<String>,
    pub reportado_por: String,
    pub created_at: String,
    pub updated_at: String,
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
    pub fecha_reporte: String,
    pub resuelto: bool,
    pub fecha_resolucion: Option<String>,
    pub notas: Option<String>,
    pub reportado_por: String,
    pub reportado_por_nombre: String, // JOIN con users
    pub created_at: String,
    pub updated_at: String,
}

impl From<AlertaGafete> for AlertaGafeteResponse {
    fn from(a: AlertaGafete) -> Self {
        Self {
            id: a.id,
            persona_id: a.persona_id,
            cedula: a.cedula,
            nombre_completo: a.nombre_completo,
            gafete_numero: a.gafete_numero,
            ingreso_contratista_id: a.ingreso_contratista_id,
            ingreso_proveedor_id: a.ingreso_proveedor_id,
            fecha_reporte: a.fecha_reporte,
            resuelto: a.resuelto,
            fecha_resolucion: a.fecha_resolucion,
            notas: a.notas,
            reportado_por: a.reportado_por,
            reportado_por_nombre: String::new(), // Se llena con JOIN
            created_at: a.created_at,
            updated_at: a.updated_at,
        }
    }
}
