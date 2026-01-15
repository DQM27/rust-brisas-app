use crate::domain::common::datetime_to_iso; // Import shared utility
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

/// Calcula la duración entre dos fechas y devuelve (minutos, texto formateado)
fn calculate_duration(start: &Datetime, end: Option<&Datetime>) -> (Option<i64>, Option<String>) {
    let end_dt = match end {
        Some(dt) => dt,
        None => return (None, None),
    };

    // Convertir a i64 (unix millis) para cálculo, asumiendo que Datetime de Surreal
    // maneja internamente nano/milisegundos o se puede convertir.
    // Surrealdb::Datetime usa chrono::DateTime<Utc> internamente o algo similar.
    // La forma más segura sin depender de métodos internos es conversión a string y re-parsing
    // O usar métodos públicos si existen. SurrealDB Datetime is a wrapper struct.
    // Viendo la doc, `Datetime` implementa `Into<chrono::DateTime<Utc>>` o similar usualmente.
    // Pero si no tenemos acceso fácil, parseamos las strings ISO.

    // Método robusto: Usar chrono::DateTime
    let start_iso = datetime_to_iso(start);
    let end_iso = datetime_to_iso(end_dt);

    let start_parsed = match chrono::DateTime::parse_from_rfc3339(&start_iso) {
        Ok(dt) => dt,
        Err(_) => {
            match chrono::NaiveDateTime::parse_from_str(&start_iso, "%Y-%m-%dT%H:%M:%S.%fZ") {
                Ok(dt) => dt.and_local_timezone(chrono::Utc).unwrap().into(),
                Err(_) => return (None, None), // Fallback
            }
        }
    };

    let end_parsed = match chrono::DateTime::parse_from_rfc3339(&end_iso) {
        Ok(dt) => dt,
        Err(_) => return (None, None),
    };

    let duration = end_parsed.signed_duration_since(start_parsed);
    let total_minutes = duration.num_minutes();

    let hours = total_minutes / 60;
    let minutes = total_minutes % 60;

    let text = format!("{hours}h {minutes}m");

    (Some(total_minutes), Some(text))
}

fn is_time_exceeded(start: &Datetime, end: Option<&Datetime>) -> Option<bool> {
    let (minutos, _) = calculate_duration(start, end);
    minutos.map(|m| m >= crate::domain::ingreso_contratista::TIEMPO_MAXIMO_MINUTOS)
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alerta_tiempo_excedido: Option<bool>,
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
            alerta_tiempo_excedido: None,
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
            fecha_hora_ingreso: datetime_to_iso(&i.fecha_hora_ingreso),
            fecha_hora_salida: i.fecha_hora_salida.as_ref().map(datetime_to_iso),
            tiempo_permanencia_minutos: calculate_duration(
                &i.fecha_hora_ingreso,
                i.fecha_hora_salida.as_ref(),
            )
            .0,
            tiempo_permanencia_texto: calculate_duration(
                &i.fecha_hora_ingreso,
                i.fecha_hora_salida.as_ref(),
            )
            .1,
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
            alerta_tiempo_excedido: is_time_exceeded(
                &i.fecha_hora_ingreso,
                i.fecha_hora_salida.as_ref(),
            ),
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
            fecha_hora_salida: i.fecha_hora_salida.as_ref().map(datetime_to_iso),
            tiempo_permanencia_minutos: calculate_duration(
                &i.fecha_hora_ingreso,
                i.fecha_hora_salida.as_ref(),
            )
            .0,
            tiempo_permanencia_texto: calculate_duration(
                &i.fecha_hora_ingreso,
                i.fecha_hora_salida.as_ref(),
            )
            .1,
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
            alerta_tiempo_excedido: None,
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
            fecha_hora_salida: i.fecha_hora_salida.as_ref().map(datetime_to_iso),
            tiempo_permanencia_minutos: calculate_duration(
                &i.fecha_hora_ingreso,
                i.fecha_hora_salida.as_ref(),
            )
            .0,
            tiempo_permanencia_texto: calculate_duration(
                &i.fecha_hora_ingreso,
                i.fecha_hora_salida.as_ref(),
            )
            .1,
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
            alerta_tiempo_excedido: None,
        }
    }
}

