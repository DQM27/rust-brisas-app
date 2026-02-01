use crate::config::settings::AppConfigState;
use crate::db::surrealdb_audit_queries::{get_sys_logs, insert_sys_log, SysLogEntry};
use crate::domain::errors::SystemError;
use tauri::{command, State};

#[command]
pub async fn log_system_event(
    config_state: State<'_, AppConfigState>,
    user_name: String,
    event_type: String,
    duration: Option<String>,
    ip_address: Option<String>,
    details: Option<String>,
) -> Result<(), SystemError> {
    // 1. Get Terminal Info from Config State
    // 1. Get Terminal Info from Config State
    let (terminal_id, terminal_name) = {
        let config = config_state
            .read()
            .map_err(|e| SystemError::Message(format!("Failed to read config: {e}")))?;
        (config.terminal.id.clone(), config.terminal.nombre.clone())
    };

    // 2. Insert Log
    insert_sys_log(
        terminal_id,
        terminal_name,
        user_name,
        event_type,
        duration,
        ip_address,
        details,
    )
    .await
    .map_err(|e| SystemError::Message(e.to_string()))?;

    Ok(())
}

#[command]
pub async fn fetch_system_logs(
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<Vec<SysLogEntry>, SystemError> {
    get_sys_logs(limit, offset).await.map_err(|e| SystemError::Message(e.to_string()))
}
