use crate::config::settings::AppConfigState;
use std::process::Command;
use tauri::{command, State};

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
                "$player = New-Object System.Media.SoundPlayer('{}'); $player.Play();",
                path
            );
            let _ = Command::new("powershell").args(["-NoProfile", "-Command", &cmd]).spawn();
        } else {
            let cmd = format!("[System.Media.SystemSounds]:: {}.Play()", sound);
            let _ = Command::new("powershell").args(["-NoProfile", "-Command", &cmd]).spawn();
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = Command::new("afplay").arg("/System/Library/Sounds/Sosumi.aiff").spawn();
    }

    Ok(())
}

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

#[tauri::command]
pub async fn play_sound(
    _sound: String,
    _custom_path: Option<String>,
    _use_custom: bool,
) -> Result<(), String> {
    Ok(())
}

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
