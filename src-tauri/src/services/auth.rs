// src/services/auth.rs

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Algorithm, Argon2, Params, Version,
};
use std::env;

fn get_argon2_params() -> Params {
    let m_cost = env::var("ARGON2_MEMORY")
        .unwrap_or("19456".into())
        .parse()
        .unwrap_or(19456);
    let t_cost = env::var("ARGON2_ITERATIONS")
        .unwrap_or("2".into())
        .parse()
        .unwrap_or(2);
    let p_cost = env::var("ARGON2_PARALLELISM")
        .unwrap_or("1".into())
        .parse()
        .unwrap_or(1);

    Params::new(m_cost, t_cost, p_cost, Some(32)).unwrap_or_default()
}

/// Hashea una contraseña usando Argon2id con un secreto (pepper)
pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let secret = env::var("PASSWORD_SECRET").unwrap_or_default();

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

    let secret = env::var("PASSWORD_SECRET").unwrap_or_default();

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
