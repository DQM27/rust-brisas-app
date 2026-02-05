use crate::config::settings::AppConfigState;
use crate::domain::errors::ConfigError;
use kira::manager::{backend::cpal::CpalBackend, AudioManager, AudioManagerSettings};
use kira::sound::static_sound::StaticSoundData;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tauri::{command, State};

/// Gestor global de audio para evitar reinicializar el stream en cada sonido.
static AUDIO_MANAGER: Lazy<Mutex<Option<AudioManager<CpalBackend>>>> = Lazy::new(|| {
    let manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).ok();
    Mutex::new(manager)
});

/// Reproduce un sonido de alerta basado en la configuración actual.
/// Soporta sonidos nativos (mapeados a internos) y archivos WAV/MP3 personalizados.
#[command]
pub async fn play_alert_sound(
    config: State<'_, AppConfigState>,
    sound_type: Option<String>,
) -> Result<(), ConfigError> {
    let (custom_path, use_custom, alert_sound_setting) = {
        let config_guard = config.read().map_err(|e| ConfigError::Message(e.to_string()))?;
        (
            config_guard.audio.custom_sound_path.clone(),
            config_guard.audio.use_custom,
            config_guard.audio.alert_sound.clone(),
        )
    };

    let type_str = sound_type.unwrap_or_else(|| "error".to_string());

    let sound_path = if use_custom && custom_path.is_some() {
        std::path::PathBuf::from(custom_path.unwrap())
    } else {
        // Mapeo de tipos a sonidos de sistema de Windows
        let media_path = std::path::PathBuf::from("C:\\Windows\\Media");
        match type_str.as_str() {
            "success" => media_path.join("Windows Notify System Generic.wav"),
            "info" => media_path.join("Windows Background.wav"),
            _ => {
                // Fallback a la configuración de alert_sound (Hand, Exclamation, etc.)
                match alert_sound_setting.as_str() {
                    "Hand" => media_path.join("Windows Foreground.wav"),
                    "Exclamation" => media_path.join("Windows Exclamation.wav"),
                    "Beep" => media_path.join("Windows Background.wav"),
                    "Question" => media_path.join("Windows Navigation Start.wav"),
                    "Asterisk" => media_path.join("Windows Background.wav"),
                    _ => media_path.join("Windows Background.wav"),
                }
            }
        }
    };

    if !sound_path.exists() {
        return Ok(());
    }

    // Reproducción nativa con Kira
    let mut manager_guard =
        AUDIO_MANAGER.lock().map_err(|_| ConfigError::Message("Audio mutex error".to_string()))?;
    if let Some(manager) = manager_guard.as_mut() {
        if let Ok(sound_data) = StaticSoundData::from_file(&sound_path) {
            let _ = manager.play(sound_data);
        }
    }

    Ok(())
}

/// Sube y establece un archivo de sonido personalizado para las alertas.
/// Copia el archivo al directorio de datos local para persistencia.
#[command]
pub async fn upload_custom_sound(
    config: State<'_, AppConfigState>,
    file_path: String,
) -> Result<String, ConfigError> {
    use std::fs;
    use std::path::Path;

    let source = Path::new(&file_path);
    if !source.exists() {
        return Err(ConfigError::Io("El archivo de sonido no existe".to_string()));
    }

    let data_dir =
        dirs::data_local_dir().map(|dir| dir.join("Brisas").join("sounds")).ok_or_else(|| {
            ConfigError::Io("No se pudo determinar el directorio de datos".to_string())
        })?;

    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)?;
    }

    // Mantener la extensión original
    let extension = source.extension().and_then(|s| s.to_str()).unwrap_or("wav");
    let dest = data_dir.join(format!("alert.{extension}"));

    fs::copy(source, &dest)?;

    let dest_str = dest.to_string_lossy().to_string();

    {
        let mut config_guard = config.write().map_err(|e| ConfigError::Message(e.to_string()))?;
        config_guard.audio.custom_sound_path = Some(dest_str.clone());
        config_guard.audio.use_custom = true;

        let config_path = if let Some(d) = dirs::data_local_dir() {
            d.join("Brisas").join("brisas.toml")
        } else {
            std::path::PathBuf::from("./config/brisas.toml")
        };

        crate::config::manager::save_config(&config_guard, &config_path)?;
    }

    Ok(dest_str)
}

/// Activa o desactiva el uso del sonido personalizado en la configuración.
#[command]
pub async fn set_use_custom_sound(
    config: State<'_, AppConfigState>,
    use_custom: bool,
) -> Result<(), ConfigError> {
    let mut config_guard = config.write().map_err(|e| ConfigError::Message(e.to_string()))?;
    config_guard.audio.use_custom = use_custom;

    let config_path = if let Some(d) = dirs::data_local_dir() {
        d.join("Brisas").join("brisas.toml")
    } else {
        std::path::PathBuf::from("./config/brisas.toml")
    };

    crate::config::manager::save_config(&config_guard, &config_path)?;
    Ok(())
}
