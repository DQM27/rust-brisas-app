/// Gestión de Notificaciones Auditivas y Personalización de Sonidos.
///
/// Este módulo permite la reproducción de alertas sonoras del sistema y la gestión
/// de archivos de audio personalizados, facilitando la identificación auditiva
/// de eventos críticos en portería.
use crate::config::settings::AppConfigState;
use std::process::Command;
use tauri::{command, State};

/// Reproduce un sonido de alerta basado en la configuración actual.
/// Soporta sonidos nativos del sistema y archivos WAV personalizados en Windows.
#[command]
pub async fn play_alert_sound(config: State<'_, AppConfigState>) -> Result<(), String> {
    #[allow(unused_variables)]
    let (sound, custom_path, use_custom) = {
        let config_guard = config.read().expect("Error reading config");
        (
            config_guard.audio.alert_sound.clone(),
            config_guard.audio.custom_sound_path.clone(),
            config_guard.audio.use_custom,
        )
    };

    #[cfg(target_os = "windows")]
    {
        if use_custom && custom_path.is_some() {
            let path = custom_path.unwrap();
            let cmd = format!(
                "$player = New-Object System.Media.SoundPlayer('{path}'); $player.Play();"
            );
            let _ = Command::new("powershell").args(["-NoProfile", "-Command", &cmd]).spawn();
        } else {
            let cmd = format!("[System.Media.SystemSounds]:: {sound}.Play()");
            let _ = Command::new("powershell").args(["-NoProfile", "-Command", &cmd]).spawn();
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = Command::new("afplay").arg("/System/Library/Sounds/Sosumi.aiff").spawn();
    }

    Ok(())
}

/// Sube y establece un archivo de sonido personalizado para las alertas.
/// Copia el archivo al directorio de datos local para persistencia.
#[command]
pub async fn upload_custom_sound(
    config: State<'_, AppConfigState>,
    file_path: String,
) -> Result<String, String> {
    use std::fs;
    use std::path::Path;

    let source = Path::new(&file_path);
    if !source.exists() {
        return Err("El archivo de sonido no existe".to_string());
    }

    let data_dir = if let Some(dir) = dirs::data_local_dir() {
        dir.join("Brisas").join("sounds")
    } else {
        return Err("No se pudo determinar el directorio de datos".to_string());
    };

    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
    }

    let dest = data_dir.join("alert.wav");
    fs::copy(source, &dest).map_err(|e| e.to_string())?;

    let dest_str = dest.to_string_lossy().to_string();

    {
        let mut config_guard = config.write().map_err(|e| e.to_string())?;
        config_guard.audio.custom_sound_path = Some(dest_str.clone());
        config_guard.audio.use_custom = true;
        let config_path = if let Some(d) = dirs::data_local_dir() {
            d.join("Brisas").join("brisas.toml")
        } else {
            std::path::PathBuf::from("./config/brisas.toml")
        };

        crate::config::manager::save_config(&config_guard, &config_path)
            .map_err(|e| e.to_string())?;
    }

    Ok(dest_str)
}

/// Comando auxiliar para reproducción directa de archivos (Stub).
#[tauri::command]
pub async fn play_sound(
    _sound: String,
    _custom_path: Option<String>,
    _use_custom: bool,
) -> Result<(), String> {
    Ok(())
}

/// Activa o desactiva el uso del sonido personalizado en la configuración.
#[command]
pub async fn set_use_custom_sound(
    config: State<'_, AppConfigState>,
    use_custom: bool,
) -> Result<(), String> {
    let mut config_guard = config.write().map_err(|e| e.to_string())?;
    config_guard.audio.use_custom = use_custom;

    let config_path = if let Some(d) = dirs::data_local_dir() {
        d.join("Brisas").join("brisas.toml")
    } else {
        std::path::PathBuf::from("./config/brisas.toml")
    };

    crate::config::manager::save_config(&config_guard, &config_path).map_err(|e| e.to_string())?;
    Ok(())
}
