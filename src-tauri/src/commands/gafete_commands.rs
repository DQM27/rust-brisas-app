/// Puertos de Entrada: Inventario y Control de Gafetes Físicos (Asset Bridge).
///
/// Este módulo gestiona el ciclo de vida de los gafetes, desde su alta masiva
/// hasta su asignación y control de estado (Daño, Extravío, Disponibilidad).
use crate::domain::errors::GafeteError;
use crate::models::gafete::{
    CreateGafeteInput, CreateGafeteRangeInput, GafeteListResponse, GafeteResponse, StatsGafetes,
    StatsPorTipo, UpdateGafeteInput, UpdateGafeteStatusInput,
};
use crate::services::gafete_service;
use crate::services::session::SessionState;
use tauri::{command, State};

/// Registra una nueva unidad de identificación en el sistema.
/// [Comando Tauri]
#[command]
pub async fn create_gafete(
    session: State<'_, SessionState>,
    input: CreateGafeteInput,
) -> Result<GafeteResponse, GafeteError> {
    require_perm!(session, "gafetes:create", "Registrando nuevo activo (Gafete)")?;
    gafete_service::create_gafete(input).await
}

/// Generación Masiva: Permite crear rangos de gafetes (Ej: del 100 al 200) de una sola vez.
/// [Comando Tauri]
#[command]
pub async fn create_gafete_range(
    input: CreateGafeteRangeInput,
) -> Result<Vec<String>, GafeteError> {
    // El servicio retorna cantidad (i32), el front espera Result simple (o tal vez mensajes).
    // Mantenemos la firma original devolviendo vec vacío por compatibilidad si el front lo ignora,
    // pero idealmente deberíamos devolver el count.
    gafete_service::create_gafete_range(input).await.map(|_| vec![])
}

/// [Comando Tauri]
#[command]
pub async fn get_gafete(id: String) -> Result<GafeteResponse, GafeteError> {
    gafete_service::get_gafete_by_id(&id).await?.ok_or(GafeteError::NotFound)
}

/// Auditoría de Inventario: Obtiene el estado actual y estadísticas de todos los gafetes.
/// [Comando Tauri]
#[command]
pub async fn get_all_gafetes(
    session: State<'_, SessionState>,
) -> Result<GafeteListResponse, GafeteError> {
    require_perm!(session, "gafetes:read")?;

    // El servicio ya retorna Vec<GafeteResponse>
    let list = gafete_service::get_all_gafetes().await?;
    let total = list.len();

    // TODO: Calcular stats reales si es necesario, por ahora placeholders o lógica simple
    let stats = StatsGafetes {
        total,
        disponibles: 0, // Se calcularían iterando la lista si fuera necesario
        en_uso: 0,
        danados: 0,
        extraviados: 0,
        por_tipo: StatsPorTipo { contratistas: 0, proveedores: 0, visitas: 0, otros: 0 },
    };

    Ok(GafeteListResponse { gafetes: list, total, stats })
}

/// Filtro dinámico para la selección de gafetes libres durante el registro de ingresos.
/// [Comando Tauri]
#[command]
pub async fn get_gafetes_disponibles(tipo: String) -> Result<Vec<GafeteResponse>, GafeteError> {
    gafete_service::get_gafetes_disponibles(&tipo).await
}

/// Comprobación de estado rápida para validaciones en caliente.
/// [Comando Tauri]
#[command]
pub async fn is_gafete_disponible(numero: String, tipo: String) -> Result<bool, GafeteError> {
    let numero_int = numero.parse::<i32>().map_err(|_| {
        GafeteError::Validation("El número de gafete debe ser numérico".to_string())
    })?;

    gafete_service::is_gafete_disponible(numero_int, &tipo).await
}

/// [Comando Tauri]
#[command]
pub async fn update_gafete(
    id: String,
    _input: UpdateGafeteInput,
) -> Result<GafeteResponse, GafeteError> {
    // Por ahora solo hace un get, revisar si se requiere update real en servicio
    gafete_service::get_gafete_by_id(&id).await?.ok_or(GafeteError::NotFound)
}

/// Actualización de Estado Operativo: Permite marcar un gafete como 'dañado' o 'extraviado'.
/// [Comando Tauri]
#[command]
pub async fn update_gafete_status(
    id: String,
    input: UpdateGafeteStatusInput,
    _usuario_id: Option<String>,
    _motivo: Option<String>,
) -> Result<GafeteResponse, GafeteError> {
    gafete_service::update_gafete_status(&id, input.estado).await
}

/// Baja definitiva de un activo del inventario.
/// [Comando Tauri]
#[command]
pub async fn delete_gafete(id: String, _usuario_id: Option<String>) -> Result<(), GafeteError> {
    gafete_service::delete_gafete(&id).await
}
