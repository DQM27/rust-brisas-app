/// Gestión de Credenciales y Seguridad del Llavero (Keyring Bridge).
///
/// Este módulo es responsable del manejo seguro de secretos, parámetros de encriptación
/// y la configuración inicial del sistema (Wizard). Integra el llavero nativo del OS
/// (Windows Credential Manager, macOS Keychain) para proteger llaves maestras y tokens.
use crate::config::save_config;
use crate::config::settings::AppConfigState;
use crate::domain::errors::KeyringError;
use crate::services::keyring_service as ks;
use crate::services::keyring_service::{Argon2Params, CredentialStatus};
use crate::services::session::SessionState;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tauri::{command, State};

// ==========================================
// DTOs PARA COMANDOS
// ==========================================

/// Datos necesarios para el asistente de configuración inicial.
#[derive(Debug, Deserialize)]
pub struct SetupCredentialsInput {
    pub argon2: Argon2Params,
    pub terminal_name: String,
    pub terminal_location: String,
}

/// Resultado de la operación de configuración inicial.
#[derive(Debug, Serialize)]
pub struct SetupResult {
    pub success: bool,
    pub message: String,
}

// ==========================================
// COMANDOS DE ESTADO
// ==========================================

/// Obtiene el estado actual de las credenciales del sistema.
#[command]
pub fn get_credential_status() -> CredentialStatus {
    ks::get_credential_status()
}

/// Verifica si la aplicación tiene su configuración básica y secreta completa.
#[command]
pub fn is_app_configured(config: State<'_, AppConfigState>) -> bool {
    if let Ok(guard) = config.read() {
        guard.setup.is_configured && ks::is_fully_configured()
    } else {
        false
    }
}

/// Determina si es necesario mostrar el asistente de configuración (Wizard).
#[command]
pub fn needs_setup(config: State<'_, AppConfigState>) -> bool {
    if let Ok(guard) = config.read() {
        !guard.setup.is_configured || !ks::is_fully_configured()
    } else {
        true
    }
}

// ==========================================
// COMANDOS DE SETUP INICIAL
// ==========================================

/// Orquesta la configuración inicial: llavero, archivos TOML y siembra de DB.
#[command]
pub async fn setup_credentials(
    _session: State<'_, SessionState>, // No se usa - setup es previo al login
    input: SetupCredentialsInput,
    config: State<'_, crate::config::settings::AppConfigState>,
) -> Result<SetupResult, KeyringError> {
<<<<<<< HEAD
    // Nota: No usamos require_perm! aquí porque el setup ocurre ANTES del login.
    // La protección es que el wizard solo se muestra si needs_setup() retorna true.
=======
    // Verificar si la app ya está configurada.
    // Si NO está configurada (Wizard inicial), permitimos acceso sin sesión.
    // Si SI está configurada, exigimos permisos de administrador.
    let is_initial_setup = {
        let guard = config.read().map_err(|e| KeyringError::Message(e.to_string()))?;
        !guard.setup.is_configured
    };

    if !is_initial_setup {
        require_perm!(session, "config:update", "Configuración de credenciales")
            .map_err(|e| KeyringError::Message(e.to_string()))?;
    }
>>>>>>> feature/domain-layer-refactor

    let mut final_argon2 = input.argon2.clone();
    if ks::has_argon2_secret() {
        let existing = ks::get_argon2_params();
        if !existing.secret.is_empty() {
            log::info!("🔐 Detectado secreto existente en Keyring. Reutilizando para mantener compatibilidad.");
            final_argon2.secret = existing.secret;
        }
    }

    ks::store_argon2_params(&final_argon2).map_err(|e| KeyringError::StoreError(e.to_string()))?;

    {
        let mut config_guard = config
            .write()
            .map_err(|e| KeyringError::Message(format!("Error escribiendo configuración: {e}")))?;

        config_guard.setup.is_configured = true;
        config_guard.setup.configured_at = Some(Utc::now().to_rfc3339());
        config_guard.setup.configured_version = Some(env!("CARGO_PKG_VERSION").to_string());

        config_guard.terminal.nombre = input.terminal_name;
        config_guard.terminal.ubicacion = input.terminal_location;

        let config_path = if let Some(data_dir) = dirs::data_local_dir() {
            data_dir.join("Brisas").join("brisas.toml")
        } else {
            std::path::PathBuf::from("brisas.toml")
        };

        crate::config::save_config(&config_guard, &config_path)
            .map_err(|e| KeyringError::Message(format!("Error guardando configuración: {e}")))?;
    }

    log::info!("🌱 Configuración completada. Ejecutando seed de base de datos...");
    if let Err(e) = crate::config::seed::seed_db().await {
        log::error!("❌ Error al sembrar base de datos tras setup: {e}");
        return Err(KeyringError::Message(format!("Error en seed: {e}")));
    }
    log::info!("✅ Seed completado correctamente");

    Ok(SetupResult {
        success: true,
        message: "Configuración inicial completada correctamente".to_string(),
    })
}

// ==========================================
// COMANDOS ARGON2 (Administración Avanzada)
// ==========================================

/// Recupera la configuración técnica de Argon2 (excluyendo el secreto).
#[command]
pub async fn get_argon2_config(
    session: State<'_, SessionState>,
) -> Result<Argon2ParamsSafe, KeyringError> {
    require_perm!(session, "config:read").map_err(|e| KeyringError::Message(e.to_string()))?;

    let params = ks::get_argon2_params();
    Ok(Argon2ParamsSafe {
        memory: params.memory,
        iterations: params.iterations,
        parallelism: params.parallelism,
        has_secret: !params.secret.is_empty(),
    })
}

#[derive(Debug, Serialize)]
pub struct Argon2ParamsSafe {
    pub memory: u32,
    pub iterations: u32,
    pub parallelism: u32,
    pub has_secret: bool,
}

/// Actualiza los parámetros de seguridad para las derivaciones futuras de llaves.
#[command]
pub async fn update_argon2_params(
    session: State<'_, SessionState>,
    params: Argon2Params,
) -> Result<(), KeyringError> {
    require_perm!(session, "config:update", "Actualizando parámetros Argon2")
        .map_err(|e| KeyringError::Message(e.to_string()))?;
    ks::store_argon2_params(&params).map_err(|e| KeyringError::StoreError(e.to_string()))
}

/// Genera un secreto aleatorio compatible con la encriptación del sistema.
#[command]
pub fn generate_argon2_secret() -> String {
    ks::generate_random_secret()
}

// ==========================================
// COMANDOS DE UTILIDAD Y DIAGNÓSTICO
// ==========================================

#[command]
pub fn generate_random_secret() -> String {
    ks::generate_random_secret()
}

/// Ejecuta una batería de pruebas sobre el llavero del sistema para verificar permisos.
/// Ejecuta una batería de pruebas sobre el llavero del sistema para verificar permisos.
#[command]
pub async fn test_keyring(session: State<'_, SessionState>) -> Result<String, KeyringError> {
    require_perm!(session, "config:read", "Diagnóstico de Keyring")
        .map_err(|e| KeyringError::Message(e.to_string()))?;

    let test_key = "test-brisas-diagnostic";
    let test_value = "test-password-123";
    let mut results = Vec::new();

    results.push(format!("Sistema operativo: {}", std::env::consts::OS));
    results.push(String::new());

    results.push("1. Guardando credencial de prueba...".to_string());
<<<<<<< HEAD
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    match keyring_impl::store_secret(test_key, test_value) {
        Ok(()) => results.push("   ✓ Credencial guardada correctamente".to_string()),
=======
    match ks::save_secret(test_key, test_value) {
        Ok(_) => results.push("   ✓ Credencial guardada correctamente".to_string()),
>>>>>>> feature/domain-layer-refactor
        Err(e) => {
            results.push(format!("   ✗ Error guardando credencial: {e}"));
            return Ok(results.join("\n"));
        }
    }

    results.push("2. Recuperando credencial...".to_string());
<<<<<<< HEAD
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    if let Some(password) = keyring_impl::retrieve_secret(test_key) {
        results.push(format!("   ✓ Credencial recuperada: {password}"));
        if password == test_value {
            results.push("   ✓ La credencial coincide!".to_string());
        } else {
            results.push(format!(
                "   ✗ La credencial NO coincide! Esperado: {test_value}, Obtenido: {password}"
            ));
=======
    match ks::get_secret(test_key) {
        Some(password) => {
            results
                .push(format!("   ✓ Credencial recuperada: [OCULTO, longitud={}]", password.len()));
            if password == test_value {
                results.push("   ✓ La credencial coincide!".to_string());
            } else {
                results.push(format!(
                    "   ✗ La credencial NO coincide! Esperado: {}, Obtenido: {}",
                    test_value, password
                ));
            }
        }
        None => {
            results.push("   ✗ Error recuperando credencial".to_string());
            return Ok(results.join("\n"));
>>>>>>> feature/domain-layer-refactor
        }
    } else {
        results.push("   ✗ Error recuperando credencial".to_string());
        return Ok(results.join("\n"));
    }

    results.push("3. Eliminando credencial...".to_string());
<<<<<<< HEAD
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    match keyring_impl::delete_secret(test_key) {
        Ok(()) => results.push("   ✓ Credencial eliminada correctamente".to_string()),
=======
    match ks::delete_secret(test_key) {
        Ok(_) => results.push("   ✓ Credencial eliminada correctamente".to_string()),
>>>>>>> feature/domain-layer-refactor
        Err(e) => {
            results.push(format!("   ✗ Error eliminando credencial: {e}"));
        }
    }

    results.push("4. Verificando eliminación...".to_string());
<<<<<<< HEAD
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    match keyring_impl::retrieve_secret(test_key) {
        Some(password) => {
            results.push(format!("   ✗ La credencial aún existe: {password}"));
=======
    match ks::get_secret(test_key) {
        Some(_) => {
            results.push("   ✗ La credencial aún existe".to_string());
>>>>>>> feature/domain-layer-refactor
        }
        None => {
            results.push("   ✓ La credencial fue eliminada correctamente".to_string());
        }
    }

<<<<<<< HEAD
    #[cfg(target_os = "macos")]
    {
        use keyring::Entry;
        let entry = Entry::new("test-brisas-diagnostic", test_key)
            .map_err(|e| format!("Error creando entrada: {}", e))?;
        match entry.get_password() {
            Ok(password) => {
                results.push(format!("   ✗ La credencial aún existe: {}", password));
            }
            Err(_) => {
                results.push("   ✓ La credencial fue eliminada correctamente".to_string());
            }
        }
    }

    results.push(String::new());
=======
    results.push("".to_string());
>>>>>>> feature/domain-layer-refactor
    results.push("✓ Test completado exitosamente".to_string());
    Ok(results.join("\n"))
}

/// Elimina toda la configuración de seguridad y secretos del sistema.
/// ¡ATENCIÓN!: Esta operación es destructiva y requiere reconfigurar la app.
#[command]
pub async fn reset_all_credentials(
    session: State<'_, SessionState>,
    confirm: bool,
    config: State<'_, AppConfigState>,
) -> Result<(), KeyringError> {
    require_perm!(session, "config:delete", "Reseteo total de credenciales")
        .map_err(|e| KeyringError::Message(e.to_string()))?;
    if !confirm {
        return Err(KeyringError::Message("Debes confirmar la operación".to_string()));
    }

    let _ = ks::delete_argon2_params();

    let mut config_guard = config
        .write()
        .map_err(|e| KeyringError::Message(format!("Error escribiendo configuración: {e}")))?;

    config_guard.setup.is_configured = false;
    config_guard.setup.configured_at = None;
    config_guard.setup.configured_version = None;

    config_guard.terminal.nombre = String::new();
    config_guard.terminal.ubicacion = String::new();

    let config_path = if let Some(data_dir) = dirs::data_local_dir() {
        data_dir.join("Brisas").join("brisas.toml")
    } else {
        std::path::PathBuf::from("brisas.toml")
    };

    save_config(&config_guard, &config_path)
        .map_err(|e| KeyringError::Message(format!("Error guardando configuración: {e}")))?;

    Ok(())
}

// ==========================================
// COMANDOS DE PERSISTENCIA SEGURA LOCAL
// ==========================================

/// Almacena un par clave-valor en el llavero seguro del sistema operativo.
#[command]
pub async fn save_secret(key: String, value: String) -> Result<(), KeyringError> {
    ks::save_secret(&key, &value).map_err(|e| KeyringError::StoreError(e.to_string()))
}

/// Recupera un valor secreto previamente guardado.
#[command]
pub async fn get_secret(key: String) -> Result<Option<String>, KeyringError> {
    Ok(ks::get_secret(&key))
}

/// Borra una clave del llavero seguro para que no pueda ser recuperada.
#[command]
pub async fn delete_secret(key: String) -> Result<(), KeyringError> {
    ks::delete_secret(&key).map_err(|e| KeyringError::DeleteError(e.to_string()))
}
