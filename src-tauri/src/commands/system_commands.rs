use crate::domain::errors::SystemError;
use tauri::command;

#[cfg(target_os = "windows")]
use windows::Win32::{
    System::SystemInformation::GetTickCount,
    UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO},
};

/// Get system-wide idle time in milliseconds
///
/// Uses Windows API GetLastInputInfo() to detect when the user last
/// interacted with ANY application on the system (not just this app).
///
/// Returns idle time in milliseconds, or 0 if detection fails.
#[command]
#[allow(unsafe_code)] // Required for Windows API FFI calls
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
                    "Failed to get last input info from Windows API".to_string(),
                ))
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        // On non-Windows platforms, return 0 (always active)
        // Could implement Linux/macOS alternatives in the future
        Ok(0)
    }
}
