// ==========================================
// src/domain/motor_validacion.rs
// ==========================================
// Motor de Validación Unificado para todos los tipos de ingreso
// Principio: Todas las personas ingresan por el mismo sistema,
// lo que cambia es su contexto, no las reglas.

use serde::{Deserialize, Serialize};

// ==========================================
// TIPOS DE INGRESO
// ==========================================

/// Tipo de persona que está ingresando
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TipoIngreso {
    Contratista,
    Proveedor,
    Visita,
}

impl TipoIngreso {
    pub fn as_str(&self) -> &'static str {
        match self {
            TipoIngreso::Contratista => "contratista",
            TipoIngreso::Proveedor => "proveedor",
            TipoIngreso::Visita => "visita",
        }
    }

    /// Indica si este tipo de ingreso requiere validación de estado
    pub fn requiere_estado(&self) -> bool {
        matches!(self, TipoIngreso::Contratista | TipoIngreso::Proveedor)
    }

    /// Indica si este tipo de ingreso puede usar PRAIND
    pub fn puede_usar_praind(&self) -> bool {
        matches!(self, TipoIngreso::Contratista)
    }
}

impl std::str::FromStr for TipoIngreso {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "contratista" => Ok(TipoIngreso::Contratista),
            "proveedor" => Ok(TipoIngreso::Proveedor),
            "visita" => Ok(TipoIngreso::Visita),
            _ => Err(format!("Tipo de ingreso inválido: {}", s)),
        }
    }
}

// ==========================================
// TIPOS DE AUTORIZACIÓN
// ==========================================

/// Cómo fue autorizado el ingreso (3 variantes acordadas)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "tipo", rename_all = "lowercase")]
pub enum TipoAutorizacion {
    /// Contratistas: certificado digital con fecha de vencimiento
    Praind { fecha_vencimiento: String },

    /// Proveedores/Visitas: autorización previa por correo
    Correo { referencia: Option<String> },

    /// Casos muy raros: requiere supervisor
    Excepcional { autorizado_por: String, motivo: String },
}

impl TipoAutorizacion {
    pub fn as_str(&self) -> &'static str {
        match self {
            TipoAutorizacion::Praind { .. } => "praind",
            TipoAutorizacion::Correo { .. } => "correo",
            TipoAutorizacion::Excepcional { .. } => "excepcional",
        }
    }
}

// ==========================================
// CONTEXTO DE INGRESO (INPUT DEL MOTOR)
// ==========================================

/// Representa la realidad completa del ingreso en un punto del tiempo.
/// Inmutable durante validación. Independiente de base de datos.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextoIngreso {
    /// Tipo de persona
    pub tipo_ingreso: TipoIngreso,

    /// Cédula de la persona
    pub cedula: String,

    /// Nombre completo (para mensajes)
    pub nombre_completo: String,

    /// Tipo de autorización
    pub autorizacion: TipoAutorizacion,

    // --- Estado de restricciones ---
    /// Si está en lista negra
    pub esta_bloqueado: bool,

    /// Motivo del bloqueo (si aplica)
    pub motivo_bloqueo: Option<String>,

    /// Si ya tiene un ingreso abierto (sin salida)
    pub tiene_ingreso_abierto: bool,

    // --- Solo aplica a Contratista ---
    /// Estado del contratista: "activo", "inactivo", "suspendido"
    /// None para proveedores y visitas
    pub estado_contratista: Option<String>,

    // --- Gafetes ---
    /// Cantidad de alertas de gafete pendientes
    pub cantidad_alertas_gafete: usize,
}

impl ContextoIngreso {
    /// Crea un contexto de ingreso para un contratista
    pub fn new_contratista(
        cedula: String,
        nombre_completo: String,
        fecha_vencimiento_praind: &str,
        esta_bloqueado: bool,
        motivo_bloqueo: Option<String>,
        tiene_ingreso_abierto: bool,
        estado_contratista: String,
        cantidad_alertas_gafete: usize,
    ) -> Self {
        Self {
            tipo_ingreso: TipoIngreso::Contratista,
            cedula,
            nombre_completo,
            autorizacion: TipoAutorizacion::Praind {
                fecha_vencimiento: fecha_vencimiento_praind.to_string(),
            },
            esta_bloqueado,
            motivo_bloqueo,
            tiene_ingreso_abierto,
            estado_contratista: Some(estado_contratista),
            cantidad_alertas_gafete,
        }
    }

    /// Crea un contexto de ingreso para un proveedor
    pub fn new_proveedor(
        cedula: String,
        nombre_completo: String,
        referencia_correo: Option<String>,
        esta_bloqueado: bool,
        motivo_bloqueo: Option<String>,
        tiene_ingreso_abierto: bool,
        estado: String,
        cantidad_alertas_gafete: usize,
    ) -> Self {
        Self {
            tipo_ingreso: TipoIngreso::Proveedor,
            cedula,
            nombre_completo,
            autorizacion: TipoAutorizacion::Correo { referencia: referencia_correo },
            esta_bloqueado,
            motivo_bloqueo,
            tiene_ingreso_abierto,
            estado_contratista: Some(estado.to_lowercase()),
            cantidad_alertas_gafete,
        }
    }

    /// Crea un contexto de ingreso para una visita
    pub fn new_visita(
        cedula: String,
        nombre_completo: String,
        referencia_correo: Option<String>,
        esta_bloqueado: bool,
        motivo_bloqueo: Option<String>,
        tiene_ingreso_abierto: bool,
        cantidad_alertas_gafete: usize,
    ) -> Self {
        Self {
            tipo_ingreso: TipoIngreso::Visita,
            cedula,
            nombre_completo,
            autorizacion: TipoAutorizacion::Correo { referencia: referencia_correo },
            esta_bloqueado,
            motivo_bloqueo,
            tiene_ingreso_abierto,
            estado_contratista: None,
            cantidad_alertas_gafete,
        }
    }

    /// Crea un contexto para ingreso excepcional (supervisor autorizó manualmente)
    pub fn new_excepcional(
        tipo_ingreso: TipoIngreso,
        cedula: String,
        nombre_completo: String,
        autorizado_por: String,
        motivo: String,
        cantidad_alertas_gafete: usize,
    ) -> Self {
        Self {
            tipo_ingreso,
            cedula,
            nombre_completo,
            autorizacion: TipoAutorizacion::Excepcional { autorizado_por, motivo },
            esta_bloqueado: false, // Si es excepcional, el bloqueo fue anulado
            motivo_bloqueo: None,
            tiene_ingreso_abierto: false, // También anulado
            estado_contratista: if tipo_ingreso == TipoIngreso::Contratista {
                Some("activo".to_string()) // Forzar activo
            } else {
                None
            },
            cantidad_alertas_gafete,
        }
    }
}

// ==========================================
// RESULTADO DE VALIDACIÓN (OUTPUT DEL MOTOR)
// ==========================================

/// Resultado unificado de la validación
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultadoValidacion {
    /// Si puede ingresar o no
    pub puede_ingresar: bool,

    /// Lista de bloqueos (si los hay)
    pub bloqueos: Vec<MotivoBloqueo>,

    /// Lista de alertas (warnings, no bloquean)
    pub alertas: Vec<String>,

    /// Reportes que se deben generar
    pub reportes_pendientes: Vec<TipoReporte>,
}

impl ResultadoValidacion {
    /// Crea un resultado exitoso (puede ingresar)
    pub fn ok() -> Self {
        Self {
            puede_ingresar: true,
            bloqueos: Vec::new(),
            alertas: Vec::new(),
            reportes_pendientes: Vec::new(),
        }
    }

    /// Crea un resultado bloqueado
    pub fn bloqueado(motivo: MotivoBloqueo) -> Self {
        Self {
            puede_ingresar: false,
            bloqueos: vec![motivo],
            alertas: Vec::new(),
            reportes_pendientes: Vec::new(),
        }
    }

    /// Agrega una alerta (warning)
    pub fn con_alerta(mut self, alerta: String) -> Self {
        self.alertas.push(alerta);
        self
    }

    /// Obtiene mensaje principal de bloqueo
    pub fn mensaje_bloqueo(&self) -> Option<String> {
        self.bloqueos.first().map(|b| b.mensaje())
    }
}

/// Motivos de bloqueo
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "tipo", rename_all = "snake_case")]
pub enum MotivoBloqueo {
    ListaNegra { motivo: String },
    IngresoActivo,
    EstadoInvalido { estado: String },
    AutorizacionInvalida { motivo: String },
    GafetesPendientes { cantidad: usize },
}

impl MotivoBloqueo {
    pub fn mensaje(&self) -> String {
        match self {
            MotivoBloqueo::ListaNegra { motivo } => {
                format!("BLOQUEADO: {}", motivo)
            }
            MotivoBloqueo::IngresoActivo => "Ya tiene un ingreso activo".to_string(),
            MotivoBloqueo::EstadoInvalido { estado } => {
                format!("Estado inválido: {}", estado)
            }
            MotivoBloqueo::AutorizacionInvalida { motivo } => {
                format!("Autorización inválida: {}", motivo)
            }
            MotivoBloqueo::GafetesPendientes { cantidad } => {
                format!("Debe {} gafetes. Regularice antes de ingresar", cantidad)
            }
        }
    }
}

/// Tipos de reportes a generar
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "tipo", rename_all = "snake_case")]
pub enum TipoReporte {
    IngresoExcepcional { detalles: String },
}

// ==========================================
// MOTOR DE VALIDACIÓN (FUNCIÓN PURA)
// ==========================================

/// Constantes de negocio
const DIAS_ALERTA_PRAIND: i64 = 30;
const LIMITE_GAFETES_PENDIENTES: usize = 2;

/// Valida un ingreso basado en el contexto proporcionado.
/// Función pura: no tiene efectos secundarios, solo evalúa reglas.
pub fn validar_ingreso(contexto: &ContextoIngreso) -> ResultadoValidacion {
    let mut bloqueos = Vec::new();
    let mut alertas = Vec::new();
    let reportes = Vec::new();

    // =========================================
    // REGLA 1: Lista Negra (BLOQUEA)
    // =========================================
    if contexto.esta_bloqueado {
        bloqueos.push(MotivoBloqueo::ListaNegra {
            motivo: contexto.motivo_bloqueo.clone().unwrap_or_else(|| "Sin motivo".to_string()),
        });
    }

    // =========================================
    // REGLA 2: Ingreso Abierto (BLOQUEA)
    // =========================================
    if contexto.tiene_ingreso_abierto {
        bloqueos.push(MotivoBloqueo::IngresoActivo);
    }

    // =========================================
    // REGLA 3: Estado (SOLO Contratistas)
    // =========================================
    if contexto.tipo_ingreso.requiere_estado() {
        if let Some(ref estado) = contexto.estado_contratista {
            if estado != "activo" {
                bloqueos.push(MotivoBloqueo::EstadoInvalido { estado: estado.clone() });
            }
        }
    }

    // =========================================
    // REGLA 4: Autorización
    // =========================================
    match &contexto.autorizacion {
        TipoAutorizacion::Praind { fecha_vencimiento } => {
            match validar_praind(fecha_vencimiento) {
                PraindStatus::Vencido => {
                    bloqueos.push(MotivoBloqueo::AutorizacionInvalida {
                        motivo: "PRAIND vencido".to_string(),
                    });
                }
                PraindStatus::PorVencer { dias } => {
                    alertas.push(format!("⚠️ PRAIND vence en {} días", dias));
                }
                PraindStatus::Vigente => {
                    // OK, no hacer nada
                }
            }
        }
        TipoAutorizacion::Correo { .. } => {
            // Siempre válido - ya fue autorizado previamente por correo
        }
        TipoAutorizacion::Excepcional { autorizado_por, motivo } => {
            alertas.push(format!(
                "⚠️ Ingreso excepcional autorizado por {} - {}",
                autorizado_por, motivo
            ));
        }
    }

    // =========================================
    // REGLA 5: Gafetes Pendientes
    // =========================================
    if contexto.cantidad_alertas_gafete >= LIMITE_GAFETES_PENDIENTES {
        bloqueos
            .push(MotivoBloqueo::GafetesPendientes { cantidad: contexto.cantidad_alertas_gafete });
    } else if contexto.cantidad_alertas_gafete == 1 {
        alertas.push("⚠️ Tiene 1 gafete pendiente de devolución".to_string());
    }

    // =========================================
    // RESULTADO FINAL
    // =========================================
    ResultadoValidacion {
        puede_ingresar: bloqueos.is_empty(),
        bloqueos,
        alertas,
        reportes_pendientes: reportes,
    }
}

// ==========================================
// HELPERS INTERNOS
// ==========================================

#[derive(Debug)]
enum PraindStatus {
    Vencido,
    PorVencer { dias: i64 },
    Vigente,
}

fn validar_praind(fecha_vencimiento_str: &str) -> PraindStatus {
    use chrono::{NaiveDate, Utc};

    let hoy = Utc::now().date_naive();

    // Intentar parsear la fecha
    let fecha_vencimiento = match NaiveDate::parse_from_str(fecha_vencimiento_str, "%Y-%m-%d") {
        Ok(f) => f,
        Err(_) => {
            // Si no se puede parsear, considerar vencido por seguridad
            return PraindStatus::Vencido;
        }
    };

    let dias_restantes = (fecha_vencimiento - hoy).num_days();

    if dias_restantes < 0 {
        PraindStatus::Vencido
    } else if dias_restantes <= DIAS_ALERTA_PRAIND {
        PraindStatus::PorVencer { dias: dias_restantes }
    } else {
        PraindStatus::Vigente
    }
}

// ==========================================
// TESTS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    fn contexto_base() -> ContextoIngreso {
        ContextoIngreso {
            tipo_ingreso: TipoIngreso::Contratista,
            cedula: "12345678".to_string(),
            nombre_completo: "Juan Pérez".to_string(),
            autorizacion: TipoAutorizacion::Praind { fecha_vencimiento: "2026-12-31".to_string() },
            esta_bloqueado: false,
            motivo_bloqueo: None,
            tiene_ingreso_abierto: false,
            estado_contratista: Some("activo".to_string()),
            cantidad_alertas_gafete: 0,
        }
    }

    #[test]
    fn test_ingreso_exitoso() {
        let ctx = contexto_base();
        let resultado = validar_ingreso(&ctx);

        assert!(resultado.puede_ingresar);
        assert!(resultado.bloqueos.is_empty());
    }

    #[test]
    fn test_bloqueo_lista_negra() {
        let mut ctx = contexto_base();
        ctx.esta_bloqueado = true;
        ctx.motivo_bloqueo = Some("Incidente de seguridad".to_string());

        let resultado = validar_ingreso(&ctx);

        assert!(!resultado.puede_ingresar);
        assert!(matches!(resultado.bloqueos[0], MotivoBloqueo::ListaNegra { .. }));
    }

    #[test]
    fn test_bloqueo_ingreso_activo() {
        let mut ctx = contexto_base();
        ctx.tiene_ingreso_abierto = true;

        let resultado = validar_ingreso(&ctx);

        assert!(!resultado.puede_ingresar);
        assert!(matches!(resultado.bloqueos[0], MotivoBloqueo::IngresoActivo));
    }

    #[test]
    fn test_bloqueo_estado_inactivo() {
        let mut ctx = contexto_base();
        ctx.estado_contratista = Some("suspendido".to_string());

        let resultado = validar_ingreso(&ctx);

        assert!(!resultado.puede_ingresar);
        assert!(matches!(resultado.bloqueos[0], MotivoBloqueo::EstadoInvalido { .. }));
    }

    #[test]
    fn test_estado_no_aplica_a_visita() {
        let mut ctx = contexto_base();
        ctx.tipo_ingreso = TipoIngreso::Visita;
        ctx.estado_contratista = Some("suspendido".to_string()); // No debería afectar
        ctx.autorizacion = TipoAutorizacion::Correo { referencia: None };

        let resultado = validar_ingreso(&ctx);

        // Visita no requiere estado, debería poder ingresar
        assert!(resultado.puede_ingresar);
    }

    #[test]
    fn test_bloqueo_praind_vencido() {
        let mut ctx = contexto_base();
        ctx.autorizacion = TipoAutorizacion::Praind {
            fecha_vencimiento: "2020-01-01".to_string(), // Fecha pasada
        };

        let resultado = validar_ingreso(&ctx);

        assert!(!resultado.puede_ingresar);
        assert!(matches!(resultado.bloqueos[0], MotivoBloqueo::AutorizacionInvalida { .. }));
    }

    #[test]
    fn test_alerta_praind_por_vencer() {
        let mut ctx = contexto_base();
        // Fecha a 15 días de hoy
        let fecha_proxima = (chrono::Utc::now().date_naive() + chrono::Duration::days(15))
            .format("%Y-%m-%d")
            .to_string();
        ctx.autorizacion = TipoAutorizacion::Praind { fecha_vencimiento: fecha_proxima };

        let resultado = validar_ingreso(&ctx);

        assert!(resultado.puede_ingresar);
        assert!(resultado.alertas.iter().any(|a| a.contains("vence")));
    }

    #[test]
    fn test_bloqueo_gafetes_pendientes() {
        let mut ctx = contexto_base();
        ctx.cantidad_alertas_gafete = 2;

        let resultado = validar_ingreso(&ctx);

        assert!(!resultado.puede_ingresar);
        assert!(matches!(resultado.bloqueos[0], MotivoBloqueo::GafetesPendientes { .. }));
    }

    #[test]
    fn test_alerta_un_gafete_pendiente() {
        let mut ctx = contexto_base();
        ctx.cantidad_alertas_gafete = 1;

        let resultado = validar_ingreso(&ctx);

        assert!(resultado.puede_ingresar); // Solo alerta, no bloquea
        assert!(resultado.alertas.iter().any(|a| a.contains("gafete")));
    }

    #[test]
    fn test_ingreso_excepcional_genera_alerta() {
        let mut ctx = contexto_base();
        ctx.autorizacion = TipoAutorizacion::Excepcional {
            autorizado_por: "Supervisor García".to_string(),
            motivo: "Emergencia operativa".to_string(),
        };

        let resultado = validar_ingreso(&ctx);

        assert!(resultado.puede_ingresar);
        assert!(resultado.alertas.iter().any(|a| a.contains("excepcional")));
    }

    #[test]
    fn test_proveedor_con_correo() {
        let ctx = ContextoIngreso {
            tipo_ingreso: TipoIngreso::Proveedor,
            cedula: "87654321".to_string(),
            nombre_completo: "María López".to_string(),
            autorizacion: TipoAutorizacion::Correo { referencia: Some("REF-123".to_string()) },
            esta_bloqueado: false,
            motivo_bloqueo: None,
            tiene_ingreso_abierto: false,
            estado_contratista: None, // Proveedores no tienen estado
            cantidad_alertas_gafete: 0,
        };

        let resultado = validar_ingreso(&ctx);

        assert!(resultado.puede_ingresar);
        assert!(resultado.bloqueos.is_empty());
        assert!(resultado.alertas.is_empty());
    }
}
