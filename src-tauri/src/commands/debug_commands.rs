use crate::domain::errors::SystemError;
use sysinfo::{Pid, System};
use tauri::command;

#[command]
pub fn get_app_memory_usage() -> Result<u64, SystemError> {
    let mut sys = System::new();

    let pid = std::process::id();
    let sys_pid = Pid::from_u32(pid);

    sys.refresh_process(sys_pid);

    match sys.process(sys_pid) {
        Some(process) => Ok(process.memory()),
        None => {
            sys.refresh_processes();
            match sys.process(sys_pid) {
                Some(p) => Ok(p.memory()),
                None => Err(SystemError::Process(format!(
                    "No se pudo encontrar el proceso con PID {}",
                    pid
                ))),
            }
        }
    }
}
