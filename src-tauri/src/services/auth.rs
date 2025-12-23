// src/services/auth.rs

use super::keyring_service;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Algorithm, Argon2, Params, Version,
};

use crate::domain::errors::UserError;

fn get_argon2_params() -> Result<Params, UserError> {
    // Obtener parámetros desde el keyring del sistema
    let keyring_params = keyring_service::get_argon2_params();

    Params::new(
        keyring_params.memory,
        keyring_params.iterations,
        keyring_params.parallelism,
        Some(32),
    )
    .map_err(|e| UserError::Internal(format!("Parámetros Argon2 inválidos: {}", e)))
}

fn get_password_secret() -> String {
    keyring_service::get_argon2_params().secret
}

/// Hashea una contraseña usando Argon2id con un secreto (pepper)
pub fn hash_password(password: &str) -> Result<String, UserError> {
    let salt = SaltString::generate(&mut OsRng);
    let secret = get_password_secret();

    let argon2 = Argon2::new_with_secret(
        secret.as_bytes(),
        Algorithm::Argon2id,
        Version::V0x13,
        get_argon2_params()?,
    )
    .map_err(|e| UserError::Auth(format!("Error al configurar Argon2: {}", e)))?;

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| UserError::Auth(format!("Error al hashear contraseña: {}", e)))
}

/// Verifica una contraseña contra un hash usando el mismo secreto
pub fn verify_password(password: &str, hash: &str) -> Result<bool, UserError> {
    let parsed_hash =
        PasswordHash::new(hash).map_err(|e| UserError::Auth(format!("Hash inválido: {}", e)))?;

    let secret = get_password_secret();

    let argon2 = Argon2::new_with_secret(
        secret.as_bytes(),
        Algorithm::Argon2id,
        Version::V0x13,
        get_argon2_params()?,
    )
    .map_err(|e| UserError::Auth(format!("Error al configurar Argon2: {}", e)))?;

    Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_password() {
        let password = "SecretPassword123!";
        let hash = hash_password(password).unwrap();

        // El hash no debe ser igual a la clave original
        assert_ne!(password, hash);

        // Verificación exitosa
        let is_valid = verify_password(password, &hash).unwrap();
        assert!(is_valid);

        // Verificación fallida con clave incorrecta
        let is_invalid = verify_password("WrongPassword", &hash).unwrap();
        assert!(!is_invalid);
    }

    #[test]
    fn test_invalid_hash_format() {
        let result = verify_password("some_password", "not_a_valid_argon2_hash");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Hash inválido"));
    }
}
