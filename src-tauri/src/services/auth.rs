// src/services/auth.rs

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Algorithm, Argon2, Params, Version,
};
use super::keyring_service;

fn get_argon2_params() -> Params {
    // Obtener parámetros desde el keyring del sistema
    let keyring_params = keyring_service::get_argon2_params();

    Params::new(
        keyring_params.memory,
        keyring_params.iterations,
        keyring_params.parallelism,
        Some(32)
    ).unwrap_or_default()
}

fn get_password_secret() -> String {
    keyring_service::get_argon2_params().secret
}

/// Hashea una contraseña usando Argon2id con un secreto (pepper)
pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let secret = get_password_secret();

    let argon2 = Argon2::new_with_secret(
        secret.as_bytes(),
        Algorithm::Argon2id,
        Version::V0x13,
        get_argon2_params(),
    )
    .map_err(|e| format!("Error al configurar Argon2: {}", e))?;

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| format!("Error al hashear contraseña: {}", e))
}

/// Verifica una contraseña contra un hash usando el mismo secreto
pub fn verify_password(password: &str, hash: &str) -> Result<bool, String> {
    let parsed_hash = PasswordHash::new(hash).map_err(|e| format!("Hash inválido: {}", e))?;

    let secret = get_password_secret();

    let argon2 = Argon2::new_with_secret(
        secret.as_bytes(),
        Algorithm::Argon2id,
        Version::V0x13,
        get_argon2_params(),
    )
    .map_err(|e| format!("Error al configurar Argon2: {}", e))?;

    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
