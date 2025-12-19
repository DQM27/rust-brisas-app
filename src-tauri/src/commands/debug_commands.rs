use sysinfo::{Pid, System};
use tauri::command;

#[command]
pub fn get_app_memory_usage() -> Result<u64, String> {
    // Inicializar sistema
    let mut sys = System::new_all();
    sys.refresh_all();

    // Obtener PID actual usando std (más seguro y directo)
    let pid = std::process::id();
    let sys_pid = Pid::from_u32(pid);

    match sys.process(sys_pid) {
        Some(process) => {
            // Retorna memoria en bytes
            Ok(process.memory())
        }
        None => Err(format!("No se pudo encontrar el proceso con PID {}", pid)),
    }
}
