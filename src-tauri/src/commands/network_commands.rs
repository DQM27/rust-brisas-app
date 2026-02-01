use local_ip_address::local_ip;

#[tauri::command]
pub fn get_local_ip() -> String {
    local_ip().map(|ip| ip.to_string()).unwrap_or_else(|_| "Unknown".to_string())
}
