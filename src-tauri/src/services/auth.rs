/// Servicio: Criptografía y Autenticación.
///
/// Orquestador de primitivas criptográficas para la seguridad de contraseñas.
/// Utiliza Argon2id (v1.3) + Pepper (Keyring) para máxima protección.
///
/// Responsabilidades:
/// - Hashear contraseñas de forma segura (Salt + Pepper).
/// - Verificar credenciales contra hashes almacenados.
/// - Gestionar parámetros de seguridad globales.
use super::keyring_service;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Algorithm, Argon2, Params, Version,
};

use crate::domain::errors::UserError;

/// Recupera los parámetros de configuración para el algoritmo Argon2.
/// Estos valores (memoria, iteraciones, paralelismo) se extraen del Keyring
/// para permitir ajustes de seguridad globales sin cambiar el código.
fn get_argon2_params() -> Result<Params, UserError> {
    let keyring_params = keyring_service::get_argon2_params();

    // El equilibrio entre seguridad y performance se define aquí.
    Params::new(
        keyring_params.memory,
        keyring_params.iterations,
        keyring_params.parallelism,
        Some(32), // Longitud del hash de salida (256 bits).
    )
    .map_err(|e| UserError::Internal(format!("Parámetros Argon2 no válidos: {e}")))
}

/// Obtiene el "pepper" (secreto adicional) desde el almacén seguro de credenciales.
fn get_password_secret() -> String {
    keyring_service::get_argon2_params().secret
}

/// Genera un hash seguro para una contraseña en texto plano utilizando Argon2id.
///
/// # Arguments
///
/// * `password` - Contraseña en texto plano a proteger.
///
/// # Returns
///
/// Retorna el hash PHC string completo (incluye salt y parámetros) o un error.
///
/// # Errors
///
/// * `UserError::Auth`: Fallo en la inicialización del hasher.
pub fn hash_password(password: &str) -> Result<String, UserError> {
    let salt = SaltString::generate(&mut OsRng);
    let secret = get_password_secret();

    let argon2 = Argon2::new_with_secret(
        secret.as_bytes(),
        Algorithm::Argon2id,
        Version::V0x13,
        get_argon2_params()?,
    )
    .map_err(|e| UserError::Auth(format!("Error en la inicialización de Argon2: {e}")))?;

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| UserError::Auth(format!("Fallo crítico al hashear la contraseña: {e}")))
}

/// Verifica si una contraseña coincide con el hash almacenado.
///
/// Utiliza comparación en tiempo constante para evitar ataques de timing.
///
/// # Arguments
///
/// * `password` - Contraseña candidata enviada por el usuario.
/// * `hash` - Hash PHC almacenado en la base de datos.
///
/// # Returns
///
/// `Ok(true)` si coincide, `Ok(false)` si no, o Error si el hash está corrupto.
///
/// # Errors
///
/// * `UserError::Auth`: Si el formato del hash no es válido.
pub fn verify_password(password: &str, hash: &str) -> Result<bool, UserError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| UserError::Auth(format!("El formato del hash es inválido: {e}")))?;

    let secret = get_password_secret();

    let argon2 = Argon2::new_with_secret(
        secret.as_bytes(),
        Algorithm::Argon2id,
        Version::V0x13,
        get_argon2_params()?,
    )
    .map_err(|e| UserError::Auth(format!("Error de configuración en la verificación: {e}")))?;

    // La comparación se realiza en tiempo constante para mitigar ataques de temporización.
    Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
}

