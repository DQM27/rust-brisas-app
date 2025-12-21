use sysinfo::{Pid, System};
use tauri::command;

#[command]
pub fn get_app_memory_usage() -> Result<u64, String> {
    // Inicializar sistema LIMPIO (sin cargar procesos ni nada)
    let mut sys = System::new();

    // Obtener PID actual
    let pid = std::process::id();
    let sys_pid = Pid::from_u32(pid);

    // Refrescar SOLO la información de este proceso específico y memoria del sistema
    // Esto evita cargar la info de los otros 300+ procesos de Windows
    sys.refresh_process(sys_pid);

    match sys.process(sys_pid) {
        Some(process) => {
            // Retorna memoria en bytes
            Ok(process.memory())
        }
        None => {
            // Fallback: Si no lo encuentra (raro en v0.30+ con refresh_process), intentar refresh total
            // Solo como último recurso
            sys.refresh_processes();
            match sys.process(sys_pid) {
                Some(p) => Ok(p.memory()),
                None => Err(format!("No se pudo encontrar el proceso con PID {}", pid)),
            }
        }
    }
}
