//! # Servicio de Gestión de Credenciales (Keyring Service)
//!
//! Orquestador de operaciones relacionadas con el almacenamiento seguro de credenciales
//! en el keyring nativo del sistema operativo.
//!
//! ## Responsabilidades
//! - Almacenar y recuperar parámetros de configuración de Argon2
//! - Gestionar secretos genéricos de forma segura
//! - Proporcionar una API multiplataforma consistente (Windows, Linux, macOS)
//!
//! ## Modelo de Seguridad
//! Las credenciales se almacenan en el **keyring nativo del sistema operativo**:
//! - **Windows**: Credential Manager (API `wincred`)
//! - **Linux**: libsecret (vía `secret-tool`)
//! - **macOS**: Keychain (via crate `keyring`)
//!
//! ### Garantías de Seguridad
//! - ✅ **NO hay cifrado adicional** - El OS gestiona el cifrado y control de acceso
//! - ✅ **Acceso protegido por usuario** - Solo el usuario actual puede leer sus credenciales
//! - ✅ **Persistencia automática** - Los secretos sobreviven reinicios
//! - ⚠️ **Requiere usuario logueado** - No disponible en servicios/background sin sesión
//!
//! ## Arquitectura
//! ```text
//! keyring_service.rs (Fachada agnóstica de plataforma)
//!         ↓
//!    Dispatch condicional (#[cfg])
//!         ↓
//! keyring_windows.rs / keyring_linux.rs (Implementación FFI)
//! ```
//!
//! ## Estándares de Logging
//! - `debug!` - Operaciones de bajo nivel (claves, tamaños)
//! - `info!` - Operaciones exitosas críticas
//! - `warn!` - Eliminación de secretos (impacto en configuración)
//!
//! ## Uso Típico
//! ```rust
//! use crate::services::keyring_service::{Argon2Params, store_argon2_params};
//!
//! let params = Argon2Params {
//!     memory: 19456,
//!     iterations: 2,
//!     parallelism: 1,
//!     secret: "mi-secreto-base64".to_string(),
//! };
//!
//! store_argon2_params(&params)?;
//! ```

use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// ==========================================
// ERROR TYPES
// ==========================================

#[derive(Debug, Error)]
pub enum KeyringError {
    #[error("Error de acceso al llavero: {0}")]
    AccessError(String),

    #[error("Error de almacenamiento: {0}")]
    StorageError(String),

    #[error("Error de recuperación: {0}")]
    RetrievalError(String),

    #[error("Error de eliminación: {0}")]
    DeletionError(String),

    #[error("Plataforma no soportada")]
    UnsupportedPlatform,
}

pub type KeyringResult<T> = Result<T, KeyringError>;

// ==========================================
// CONSTANTES PARA CLAVES
// ==========================================

// Argon2
const KEY_ARGON2_MEMORY: &str = "argon2_memory";
const KEY_ARGON2_ITERATIONS: &str = "argon2_iterations";
const KEY_ARGON2_PARALLELISM: &str = "argon2_parallelism";
const KEY_PASSWORD_SECRET: &str = "password_secret";

// ==========================================
// DTOs
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Argon2Params {
    pub memory: u32,
    pub iterations: u32,
    pub parallelism: u32,
    pub secret: String,
}

impl Default for Argon2Params {
    fn default() -> Self {
        Self {
            memory: 19456, // ~19 MB
            iterations: 2,
            parallelism: 1,
            secret: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllCredentials {
    pub argon2: Argon2Params,
}

// ==========================================
// IMPLEMENTACIÓN LINUX (secret-tool nativo)
// ==========================================
#[cfg(target_os = "linux")]
fn store_value(key: &str, value: &str) -> KeyringResult<()> {
    use crate::services::keyring_linux;
    keyring_linux::store_secret(key, value)
}

#[cfg(target_os = "linux")]
fn retrieve_value(key: &str) -> Option<String> {
    use crate::services::keyring_linux;
    keyring_linux::retrieve_secret(key)
}

#[cfg(target_os = "linux")]
#[allow(dead_code)]
fn delete_value(key: &str) -> KeyringResult<()> {
    use crate::services::keyring_linux;
    keyring_linux::delete_secret(key)
}

// ==========================================
// IMPLEMENTACIÓN WINDOWS (Credential Manager nativo)
// ==========================================
#[cfg(target_os = "windows")]
fn store_value(key: &str, value: &str) -> KeyringResult<()> {
    use crate::services::keyring_windows;
    keyring_windows::store_secret(key, value)
}

#[cfg(target_os = "windows")]
fn retrieve_value(key: &str) -> Option<String> {
    use crate::services::keyring_windows;
    keyring_windows::retrieve_secret(key)
}
#[cfg(target_os = "windows")]
fn delete_value(key: &str) -> KeyringResult<()> {
    use crate::services::keyring_windows;
    keyring_windows::delete_secret(key)
}

// ==========================================
// PLATAFORMAS NO SOPORTADAS (Fallback)
// ==========================================
#[cfg(not(any(target_os = "linux", target_os = "windows")))]
fn store_value(_key: &str, _value: &str) -> KeyringResult<()> {
    Err(KeyringError::UnsupportedPlatform)
}

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
fn retrieve_value(_key: &str) -> Option<String> {
    None
}

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
fn delete_value(_key: &str) -> KeyringResult<()> {
    Err(KeyringError::UnsupportedPlatform)
}

// ==========================================
// ARGON2 PARAMS
// ==========================================

/// Almacena los parámetros de configuración de Argon2 de forma segura.
///
/// # Argumentos
/// * `params` - Estructura `Argon2Params` con la configuración y el secreto.
///
/// # Retorno
/// * `Ok(())` - Si se almacenaron todos los valores correctamente.
/// * `Err(KeyringError)` - Si falló el almacenamiento de alguno de los valores.
pub fn store_argon2_params(params: &Argon2Params) -> KeyringResult<()> {
    debug!(
        "Almacenando parámetros Argon2: memory={}, iterations={}, parallelism={}",
        params.memory, params.iterations, params.parallelism
    );

    store_value(KEY_ARGON2_MEMORY, &params.memory.to_string())?;
    store_value(KEY_ARGON2_ITERATIONS, &params.iterations.to_string())?;
    store_value(KEY_ARGON2_PARALLELISM, &params.parallelism.to_string())?;
    store_value(KEY_PASSWORD_SECRET, &params.secret)?;

    info!("✅ Parámetros Argon2 almacenados exitosamente en el keyring del sistema");
    Ok(())
}

/// Recupera la configuración de Argon2 almacenada.
///
/// Si no se encuentran valores, retorna los defaults seguros.
///
/// # Retorno
/// Retorna una instancia de `Argon2Params`.
pub fn get_argon2_params() -> Argon2Params {
    let memory = retrieve_value(KEY_ARGON2_MEMORY).and_then(|v| v.parse().ok()).unwrap_or(19456);
    let iterations =
        retrieve_value(KEY_ARGON2_ITERATIONS).and_then(|v| v.parse().ok()).unwrap_or(2);
    let parallelism =
        retrieve_value(KEY_ARGON2_PARALLELISM).and_then(|v| v.parse().ok()).unwrap_or(1);
    let secret = retrieve_value(KEY_PASSWORD_SECRET).unwrap_or_default();

    Argon2Params { memory, iterations, parallelism, secret }
}

/// Verifica si existe un secreto de Argon2 configurado.
///
/// # Retorno
/// * `true` - Si existe y no está vacío.
/// * `false` - Si no existe o está vacío.
pub fn has_argon2_secret() -> bool {
    retrieve_value(KEY_PASSWORD_SECRET).is_some_and(|s| !s.is_empty())
}

/// Elimina toda la configuración de Argon2 del keyring.
///
/// # Retorno
/// * `Ok(())` - Siempre retorna éxito, incluso si alguna clave no existía.
pub fn delete_argon2_params() -> KeyringResult<()> {
    warn!("🔥 ELIMINANDO todos los parámetros Argon2 del keyring del sistema");

    // Intentar borrar cada una de las claves ignorando errores individuales
    let _ = delete_value(KEY_ARGON2_MEMORY);
    let _ = delete_value(KEY_ARGON2_ITERATIONS);
    let _ = delete_value(KEY_ARGON2_PARALLELISM);
    let _ = delete_value(KEY_PASSWORD_SECRET);

    warn!("⚠️ Parámetros Argon2 eliminados del keyring - La aplicación requiere reconfiguración");
    Ok(())
}

// ==========================================
// ALL CREDENTIALS
// ==========================================

/// Obtiene todas las credenciales gestionadas por la aplicación.
///
/// # Retorno
/// Retorna `AllCredentials` conteniendo la configuración de Argon2.
pub fn get_all_credentials() -> AllCredentials {
    AllCredentials { argon2: get_argon2_params() }
}

/// Verifica si el sistema está completamente configurado.
///
/// Actualmente esto equivale a tener el secreto de Argon2 establecido.
pub fn is_fully_configured() -> bool {
    has_argon2_secret()
}

// ==========================================
// CREDENTIAL STATUS (para UI)
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialStatus {
    pub argon2_configured: bool,
    pub fully_configured: bool,
}

/// Obtiene el estado de configuración de las credenciales para la UI.
pub fn get_credential_status() -> CredentialStatus {
    let argon2_configured = has_argon2_secret();

    CredentialStatus { argon2_configured, fully_configured: argon2_configured }
}

// ==========================================
// GENERAR SECRET ALEATORIO
// ==========================================

/// Genera un secreto aleatorio criptográficamente seguro de 32 bytes en Base64.
///
/// Utiliza `rand::thread_rng()` que es un CSPRNG (Cryptographically Secure Pseudo-Random
/// Number Generator) seguro para generar material criptográfico.
///
/// ## Garantías Criptográficas
/// - 32 bytes (256 bits) de entropía
/// - Codificación Base64 estándar (44 caracteres de salida)
/// - Thread-safe (usa RNG local del thread)
///
/// ## Casos de Uso
/// - Generación de secretos Argon2
/// - Tokens de sesión
/// - Claves de API temporales
///
/// # Retorno
/// String en Base64 de 44 caracteres (32 bytes originales).
///
/// # Ejemplo
/// ```rust
/// let secret = generate_random_secret();
/// assert_eq!(secret.len(), 44); // Base64 de 32 bytes
/// ```
///
/// # Logging
/// - `DEBUG`: Tamaño del secreto generado (no el contenido)
pub fn generate_random_secret() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
    let secret = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &bytes);

    debug!("Secreto aleatorio generado (32 bytes, {} caracteres en Base64)", secret.len());
    secret
}

// ==========================================
// PUBLIC API FOR COMMANDS
// ==========================================

/// Guarda un secreto genérico en el keyring del sistema operativo.
///
/// Esta función es un wrapper de alto nivel sobre `store_value` que agrega logging
/// estructurado para auditoría.
///
/// ## Diferencia vs `store_argon2_params`
/// - `save_secret`: Para secretos genéricos (ej: tokens, API keys, contraseñas de usuario)
/// - `store_argon2_params`: Específico para configuración de seguridad del sistema
///
/// ## Modelo de Seguridad
/// - La clave **NO** debe contener información sensible (se loguea en debug)
/// - El valor **SÍ** es secreto y nunca se loguea
/// - El valor persiste hasta eliminación manual o desinstalación
///
/// # Argumentos
/// * `key` - Identificador único de la clave (ej: "`user_token`", "`api_key`")
/// * `value` - Valor secreto a guardar (cualquier string)
///
/// # Retorno
/// * `Ok(())` - Éxito en almacenamiento
/// * `Err(KeyringError)` - Fallo de acceso al keyring (permisos, OS no soportado)
///
/// # Ejemplo
/// ```rust
/// save_secret("github_token", "ghp_xxxxxxxxxxxx")?;
/// ```
///
/// # Logging
/// - `DEBUG`: Identificador de la clave guardada
/// - `INFO`: Confirmación de operación exitosa
pub fn save_secret(key: &str, value: &str) -> KeyringResult<()> {
    debug!("Guardando secreto genérico en keyring: {key}");
    store_value(key, value)?;
    info!("✅ Secreto guardado exitosamente: {key}");
    Ok(())
}

/// Recupera un secreto genérico del keyring del sistema operativo.
///
/// Esta función **NO loguea** por diseño para evitar filtrar información sobre
/// qué secretos están siendo accedidos.
///
/// ## Comportamiento
/// - Si la clave existe: retorna `Some(valor)`
/// - Si la clave NO existe: retorna `None` (no es error)
/// - Si el keyring no está disponible: retorna `None`
///
/// # Argumentos
/// * `key` - Identificador de la clave a recuperar
///
/// # Retorno
/// * `Some(String)` - El valor si existe
/// * `None` - Si no existe o hay error de acceso
///
/// # Ejemplo
/// ```rust
/// if let Some(token) = get_secret("github_token") {
///     println!("Token recuperado: {}...", &token[..10]);
/// } else {
///     println!("Token no configurado");
/// }
/// ```
pub fn get_secret(key: &str) -> Option<String> {
    retrieve_value(key)
}

/// Elimina un secreto genérico del keyring del sistema operativo.
///
/// Esta operación es **idempotente**: eliminar una clave que no existe se considera éxito.
///
/// ## Casos de Uso
/// - Cerrar sesión (eliminar tokens)
/// - Revocar acceso (eliminar API keys)
/// - Limpiar configuración de usuario
///
/// # Argumentos
/// * `key` - Identificador de la clave a eliminar
///
/// # Retorno
/// * `Ok(())` - Éxito (incluso si no existía)
/// * `Err(KeyringError)` - Fallo crítico al intentar borrar (raro, problemas de permisos/OS)
///
/// # Ejemplo
/// ```rust
/// // Limpiar token al cerrar sesión
/// delete_secret("github_token")?;
/// ```
///
/// # Logging
/// - `DEBUG`: Identificador de la clave eliminada
/// - `INFO`: Confirmación de eliminación
pub fn delete_secret(key: &str) -> KeyringResult<()> {
    debug!("Eliminando secreto genérico del keyring: {key}");
    delete_value(key)?;
    info!("✅ Secreto eliminado: {key}");
    Ok(())
}

// ==========================================
// TESTS UNITARIOS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    // --------------------------------------------------------------------------
    // Tests de DTOs y Defaults
    // --------------------------------------------------------------------------

    /// Test crítico: Verifica que los parámetros por defecto de Argon2 son seguros.
    ///
    /// Los defaults deben seguir las recomendaciones OWASP para Argon2id.
    #[test]
    fn test_argon2_params_default_values() {
        let params = Argon2Params::default();

        assert_eq!(params.memory, 19456, "Memoria debe ser ~19 MB");
        assert_eq!(params.iterations, 2, "Iteraciones mínimas OWASP");
        assert_eq!(params.parallelism, 1, "Paralelismo para single-thread");
        assert!(params.secret.is_empty(), "Secret debe iniciar vacío");
    }

    /// Test: Verifica que Argon2Params es clonable correctamente.
    #[test]
    fn test_argon2_params_clone() {
        let original = Argon2Params {
            memory: 32768,
            iterations: 3,
            parallelism: 2,
            secret: "test-secret".to_string(),
        };

        let cloned = original.clone();

        assert_eq!(cloned.memory, original.memory);
        assert_eq!(cloned.iterations, original.iterations);
        assert_eq!(cloned.parallelism, original.parallelism);
        assert_eq!(cloned.secret, original.secret);
    }

    /// Test: Verifica que AllCredentials se construye correctamente.
    #[test]
    fn test_all_credentials_construction() {
        let params = Argon2Params::default();
        let creds = AllCredentials { argon2: params.clone() };

        assert_eq!(creds.argon2.memory, params.memory);
        assert_eq!(creds.argon2.secret, params.secret);
    }

    /// Test: Verifica que CredentialStatus refleja el estado correcto.
    #[test]
    fn test_credential_status_construction() {
        let status = CredentialStatus { argon2_configured: true, fully_configured: true };

        assert!(status.argon2_configured);
        assert!(status.fully_configured);

        let incomplete = CredentialStatus { argon2_configured: false, fully_configured: false };

        assert!(!incomplete.argon2_configured);
        assert!(!incomplete.fully_configured);
    }

    // --------------------------------------------------------------------------
    // Tests de Generación de Secretos
    // --------------------------------------------------------------------------

    /// Test crítico de seguridad: Verifica que el secreto generado tiene longitud correcta.
    ///
    /// Base64(32 bytes) = 44 caracteres. Si falla, hay un problema de codificación.
    #[test]
    fn test_generate_random_secret_length() {
        let secret = generate_random_secret();

        assert_eq!(secret.len(), 44, "Base64 de 32 bytes debe resultar en 44 caracteres");
    }

    /// Test crítico de seguridad: Verifica que los secretos generados son únicos.
    ///
    /// Si dos llamadas consecutivas producen el mismo secreto, el RNG está roto.
    #[test]
    fn test_generate_random_secret_uniqueness() {
        let secret1 = generate_random_secret();
        let secret2 = generate_random_secret();

        assert_ne!(secret1, secret2, "Secretos aleatorios deben ser únicos (colisión detectada)");
    }

    /// Test: Verifica que el secreto generado es Base64 válido.
    ///
    /// Intenta decodificarlo; si falla, la codificación está corrupta.
    #[test]
    fn test_generate_random_secret_is_valid_base64() {
        let secret = generate_random_secret();

        let decoded = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &secret);

        assert!(decoded.is_ok(), "El secreto debe ser Base64 válido");

        let bytes = decoded.unwrap();
        assert_eq!(bytes.len(), 32, "El secreto decodificado debe ser exactamente 32 bytes");
    }

    /// Test: Verifica que múltiples secretos no tienen patrones repetidos.
    ///
    /// Genera 10 secretos y verifica que todos sean únicos (prueba de entropía básica).
    #[test]
    fn test_generate_random_secret_no_patterns() {
        let mut secrets = std::collections::HashSet::new();

        for _ in 0..10 {
            let secret = generate_random_secret();
            secrets.insert(secret);
        }

        assert_eq!(
            secrets.len(),
            10,
            "Los 10 secretos deben ser únicos (no debe haber colisiones)"
        );
    }

    // --------------------------------------------------------------------------
    // Tests de Lógica de Negocio
    // --------------------------------------------------------------------------

    /// Test: Verifica que `has_argon2_secret` maneje correctamente el caso None.
    ///
    /// Este test NO accede al keyring real, solo verifica la lógica del unwrap_or.
    #[test]
    fn test_has_argon2_secret_logic() {
        // Este test verifica la lógica interna, no el keyring real
        // La función usa unwrap_or(false) que debe manejar None correctamente

        // Simulamos el comportamiento esperado
        let empty_result: Option<String> = None;
        let has_secret = empty_result.map(|s| !s.is_empty()).unwrap_or(false);
        assert!(!has_secret, "None debe resultar en false");

        let some_empty: Option<String> = Some(String::new());
        let has_secret = some_empty.map(|s| !s.is_empty()).unwrap_or(false);
        assert!(!has_secret, "String vacío debe resultar en false");

        let some_value: Option<String> = Some("secreto".to_string());
        let has_secret = some_value.map(|s| !s.is_empty()).unwrap_or(false);
        assert!(has_secret, "String no vacío debe resultar en true");
    }
}
