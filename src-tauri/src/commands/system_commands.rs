/// Integración con Funciones Nativa del Sistema Operativo.
///
/// Este módulo proporciona comandos para interactuar con el hardware y
/// el entorno del SO, facilitando tareas como el monitoreo de actividad
/// global para el auto-bloqueo por inactividad.
use crate::domain::errors::SystemError;
use tauri::command;

#[cfg(target_os = "windows")]
use windows::Win32::{
    System::SystemInformation::GetTickCount,
    UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO},
};

/// Obtiene el tiempo de inactividad global del sistema en milisegundos.
///
/// Utiliza la API de Windows `GetLastInputInfo()` para detectar el tiempo transcurrido
/// desde la última interacción del usuario con CUALQUIER aplicación del sistema.
/// Útil para implementar cierres de sesión automáticos por seguridad.
#[command]
#[allow(unsafe_code)] // Requerido para llamadas FFI a las APIs de Windows.
pub fn get_system_idle_time() -> Result<u32, SystemError> {
    #[cfg(target_os = "windows")]
    {
        unsafe {
            let mut last_input_info =
                LASTINPUTINFO { cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32, dwTime: 0 };

            if GetLastInputInfo(&mut last_input_info).as_bool() {
                let current_tick = GetTickCount();
                let idle_ms = current_tick - last_input_info.dwTime;
                Ok(idle_ms)
            } else {
                Err(SystemError::Process(
                    "Error al obtener información de entrada de la API de Windows".to_string(),
                ))
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        // En plataformas que no son Windows, retorna 0 (siempre activo para esta versión).
        Ok(0)
    }
}
