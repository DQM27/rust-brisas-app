/// Criptografía y Autenticación de Usuarios.
///
/// Implementamos Argon2id, el ganador de la Password Hashing Competition, por su
/// resistencia superior contra ataques de diccionario y de fuerza bruta (GPU/ASIC).
/// Complementamos la seguridad usando un "secreto" o pepper almacenado en el
/// Keyring del sistema operativo, asegurando que incluso si la base de datos es
/// comprometida, las contraseñas no puedan ser atacadas sin acceso físico al hardware.
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
    .map_err(|e| UserError::Internal(format!("Parámetros Argon2 no válidos: {}", e)))
}

/// Obtiene el "pepper" (secreto adicional) desde el almacén seguro de credenciales.
fn get_password_secret() -> String {
    keyring_service::get_argon2_params().secret
}

/// Genera un hash seguro para una contraseña en texto plano.
///
/// El proceso utiliza un 'salt' aleatorio por cada contraseña para evitar ataques de
/// tablas Rainbow, y el 'secret' global para añadir una capa de protección basada en hardware.
pub fn hash_password(password: &str) -> Result<String, UserError> {
    let salt = SaltString::generate(&mut OsRng);
    let secret = get_password_secret();

    let argon2 = Argon2::new_with_secret(
        secret.as_bytes(),
        Algorithm::Argon2id,
        Version::V0x13,
        get_argon2_params()?,
    )
    .map_err(|e| UserError::Auth(format!("Error en la inicialización de Argon2: {}", e)))?;

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| UserError::Auth(format!("Fallo crítico al hashear la contraseña: {}", e)))
}

/// Verifica si una contraseña entregada coincide con el hash almacenado.
///
/// Al igual que en el hasheo, se requiere el mismo 'secret' del Keyring para
/// poder reconstruir la validación correctamente.
pub fn verify_password(password: &str, hash: &str) -> Result<bool, UserError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| UserError::Auth(format!("El formato del hash es inválido: {}", e)))?;

    let secret = get_password_secret();

    let argon2 = Argon2::new_with_secret(
        secret.as_bytes(),
        Algorithm::Argon2id,
        Version::V0x13,
        get_argon2_params()?,
    )
    .map_err(|e| UserError::Auth(format!("Error de configuración en la verificación: {}", e)))?;

    // La comparación se realiza en tiempo constante para mitigar ataques de temporización.
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
