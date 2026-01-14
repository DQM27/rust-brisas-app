/// Puertos de Entrada: Gestión de Admisión de Visitantes Ocasionales (Secure Bridge).
///
/// A diferencia de contratistas y proveedores, este módulo integra validaciones
/// de permisos (RBAC) a nivel de comando, asegurando que solo personal autorizado
/// pueda registrar o visualizar movimientos de visitas.
use crate::domain::errors::IngresoVisitaError;
use crate::models::ingreso::{CreateIngresoVisitaInput, IngresoResponse};
use crate::services::ingreso_visita_service as service;
use crate::services::session::SessionState;
use tauri::{command, State};

/// Registra físicamente la entrada de una visita.
/// Requiere permiso 'ingresos:create'.
#[command]
pub async fn crear_ingreso_visita(
    session: State<'_, SessionState>,
    input: CreateIngresoVisitaInput,
) -> Result<IngresoResponse, IngresoVisitaError> {
    require_perm!(session, "ingresos:create", "Iniciando registro de ingreso para visitante")?;
    let user = session
        .get_user()
        .ok_or(IngresoVisitaError::Unauthorized("Sesión requerida".to_string()))?;
    service::registrar_ingreso(input, user.id).await
}

/// Validación Preventiva: Comprueba requisitos de seguridad antes del acceso físico.
#[command]
pub async fn validar_ingreso_visita(
    visitante_id: String,
) -> Result<crate::domain::ingreso_visita::ValidacionIngresoVisitaResponse, IngresoVisitaError> {
    service::validar_ingreso(&visitante_id).await
}

#[command]
pub async fn get_ingresos_visita_activos() -> Result<Vec<IngresoResponse>, IngresoVisitaError> {
    service::get_activos().await
}

/// Cierre de Registro: Registra la salida física del visitante.
#[command]
pub async fn registrar_salida_visita(
    ingreso_id: String,
    devolvio_gafete: bool,
    observaciones: Option<String>,
    session: State<'_, SessionState>,
) -> Result<IngresoResponse, IngresoVisitaError> {
    let user = session
        .get_user()
        .ok_or(IngresoVisitaError::Unauthorized("Sesión requerida".to_string()))?;
    service::registrar_salida(ingreso_id, user.id, devolvio_gafete, observaciones).await
}
