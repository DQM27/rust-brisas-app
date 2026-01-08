/// Modelo de Respuesta: Ingreso Unificado.
///
/// Define las estructuras de transferencia de datos (DTOs) que representan
/// la información "aplanada" y lista para consumo por el frontend.
///
/// Este módulo se encarga de:
/// - Unificar las distintas variantes de ingreso (`Contratista`, `Visita`, `Proveedor`).
/// - Formatear campos complejos (fechas, enumeraciones).
/// - Exponer campos calculados (tiempo de permanencia, nombres completos).
// ==========================================
// src/models/ingreso/response.rs
// ==========================================
use crate::models::ingreso::{
    IngresoContratista, IngresoContratistaFetched, IngresoProveedorFetched, IngresoVisitaFetched,
    ModoIngreso, TipoAutorizacion, TipoIngreso,
};
use serde::Serialize;
use surrealdb::Datetime;

/// Convierte un surrealdb::Datetime a formato ISO8601 compatible con JavaScript
fn datetime_to_iso(dt: &Datetime) -> String {
    // La representación string de surrealdb::Datetime es "d'YYYY-MM-DD...'"
    // Limpiamos los decoradores para obtener solo la fecha ISO
    let raw = dt.to_string();
    let clean = raw.trim_start_matches("d'").trim_end_matches('\'').to_string();
    log::info!("Date conversion: raw='{}' -> clean='{}'", raw, clean);
    clean
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IngresoResponse {
    pub id: String,
    pub contratista_id: Option<String>,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_apellido: Option<String>,
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

        let nombre = i.nombre.clone();
        let segundo_nombre = i.segundo_nombre.clone();
        let apellido = i.apellido.clone();
        let segundo_apellido = i.segundo_apellido.clone();

        let mut nombre_completo = nombre.clone();
        if let Some(ref s) = segundo_nombre {
            nombre_completo.push(' ');
            nombre_completo.push_str(s);
        }
        nombre_completo.push(' ');
        nombre_completo.push_str(&apellido);
        if let Some(ref s) = segundo_apellido {
            nombre_completo.push(' ');
            nombre_completo.push_str(s);
        }

        Self {
            id: i.id.to_string(),
            contratista_id: Some(i.contratista.to_string()),
            cedula: i.cedula.clone(),
            nombre,
            apellido,
            segundo_nombre,
            segundo_apellido,
            nombre_completo,
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
            fecha_hora_ingreso: datetime_to_iso(&i.fecha_hora_ingreso),
            fecha_hora_salida: i.fecha_hora_salida.as_ref().map(datetime_to_iso),
            tiempo_permanencia_minutos: None,
            tiempo_permanencia_texto: None,
            usuario_ingreso_id: i.usuario_ingreso.to_string(),
            usuario_ingreso_nombre: String::new(),
            usuario_salida_id: i.usuario_salida.as_ref().map(std::string::ToString::to_string),
            usuario_salida_nombre: None,
            praind_vigente_al_ingreso: None,
            estado_contratista_al_ingreso: None,
            observaciones: i.observaciones,
            esta_adentro,
            tiene_gafete_asignado,
            created_at: datetime_to_iso(&i.created_at),
            updated_at: datetime_to_iso(&i.updated_at),
        }
    }

    pub fn from_contratista_fetched(i: IngresoContratistaFetched) -> Result<Self, String> {
        let esta_adentro = i.fecha_hora_salida.is_none();
        let tiene_gafete_asignado = i.gafete_numero.is_some();

        let nombre = i.nombre.clone();
        let segundo_nombre = i.segundo_nombre.clone();
        let apellido = i.apellido.clone();
        let segundo_apellido = i.segundo_apellido.clone();

        let mut nombre_completo = nombre.clone();
        if let Some(ref s) = segundo_nombre {
            nombre_completo.push(' ');
            nombre_completo.push_str(s);
        }
        nombre_completo.push(' ');
        nombre_completo.push_str(&apellido);
        if let Some(ref s) = segundo_apellido {
            nombre_completo.push(' ');
            nombre_completo.push_str(s);
        }

        Ok(Self {
            id: i.id.to_string(),
            contratista_id: Some(i.contratista.id.to_string()),
            cedula: i.cedula.clone(),
            nombre,
            apellido,
            segundo_nombre,
            segundo_apellido,
            nombre_completo,
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

        let nombre = i.nombre.clone();
        let segundo_nombre = i.segundo_nombre.clone();
        let apellido = i.apellido.clone();
        let segundo_apellido = i.segundo_apellido.clone();

        let mut nombre_completo = nombre.clone();
        if let Some(ref s) = segundo_nombre {
            nombre_completo.push(' ');
            nombre_completo.push_str(s);
        }
        nombre_completo.push(' ');
        nombre_completo.push_str(&apellido);
        if let Some(ref s) = segundo_apellido {
            nombre_completo.push(' ');
            nombre_completo.push_str(s);
        }

        Self {
            id: i.id.to_string(),
            contratista_id: None,
            cedula: i.cedula.clone(),
            nombre,
            apellido,
            segundo_nombre,
            segundo_apellido,
            nombre_completo,
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
            fecha_hora_ingreso: datetime_to_iso(&i.fecha_hora_ingreso),
            fecha_hora_salida: i.fecha_hora_salida.map(|d| datetime_to_iso(&d)),
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
            created_at: datetime_to_iso(&i.created_at),
            updated_at: datetime_to_iso(&i.updated_at),
        }
    }

    pub fn from_proveedor_fetched(i: IngresoProveedorFetched) -> Self {
        let esta_adentro = i.fecha_hora_salida.is_none();
        let tiene_gafete_asignado = i.gafete_numero.is_some();

        let nombre = i.nombre.clone();
        let segundo_nombre = i.segundo_nombre.clone();
        let apellido = i.apellido.clone();
        let segundo_apellido = i.segundo_apellido.clone();

        let mut nombre_completo = nombre.clone();
        if let Some(ref s) = segundo_nombre {
            nombre_completo.push(' ');
            nombre_completo.push_str(s);
        }
        nombre_completo.push(' ');
        nombre_completo.push_str(&apellido);
        if let Some(ref s) = segundo_apellido {
            nombre_completo.push(' ');
            nombre_completo.push_str(s);
        }

        Self {
            id: i.id.to_string(),
            contratista_id: None,
            cedula: i.cedula.clone(),
            nombre,
            apellido,
            segundo_nombre,
            segundo_apellido,
            nombre_completo,
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
            fecha_hora_ingreso: datetime_to_iso(&i.fecha_hora_ingreso),
            fecha_hora_salida: i.fecha_hora_salida.map(|d| datetime_to_iso(&d)),
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
            created_at: datetime_to_iso(&i.created_at),
            updated_at: datetime_to_iso(&i.updated_at),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_datetime_format_debug() {
        let now = Utc::now();
        let dt = surrealdb::Datetime::from(now);
        let raw = dt.to_string();
        let clean = datetime_to_iso(&dt);
        println!("DEBUG_TEST: Raw: '{}'", raw);
        println!("DEBUG_TEST: Clean: '{}'", clean);

        // Assert basic ISO format (starts with 20 and has T)
        assert!(clean.starts_with("20"));
        assert!(clean.contains('T'));
        assert!(!clean.contains("d'"));
    }
}
