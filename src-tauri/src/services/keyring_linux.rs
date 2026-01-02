//! # Implementación de Keyring para Linux
//!
//! Utiliza `secret-tool` (libsecret) a través de llamadas al sistema para evitar
//! problemas de compilación y enlazado con la librería nativa `keyring` en
//! entornos de desarrollo cruzado o inestables.

use crate::services::keyring_service::KeyringError;
use log::{debug, error, info};
use std::io::Write;
use std::process::Command;

const SERVICE_NAME: &str = "MegaBrisas";

/// Almacena un secreto en el anillo de llaves del sistema.
pub fn store_secret(key: &str, value: &str) -> Result<(), KeyringError> {
    debug!("Almacenando secreto para clave: {}", key);

    let mut child = Command::new("secret-tool")
        .arg("store")
        .arg("--label")
        .arg(format!("MegaBrisas - {}", key))
        .arg("service")
        .arg(SERVICE_NAME)
        .arg("username")
        .arg(key)
        .stdin(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| {
            error!("Fallo al iniciar secret-tool store: {}", e);
            KeyringError::AccessError(e.to_string())
        })?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(value.as_bytes()).map_err(|e| {
            error!("Fallo al escribir en stdin de secret-tool: {}", e);
            KeyringError::StorageError(e.to_string())
        })?;
    }

    let output = child.wait_with_output().map_err(|e| {
        error!("Fallo esperando secret-tool: {}", e);
        KeyringError::StorageError(e.to_string())
    })?;

    if output.status.success() {
        info!("Secreto almacenado exitosamente: {}", key);
        Ok(())
    } else {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        error!("secret-tool flalló al guardar: {}", err_msg);
        Err(KeyringError::StorageError(err_msg.to_string()))
    }
}

/// Recupera un secreto del anillo de llaves.
pub fn retrieve_secret(key: &str) -> Option<String> {
    debug!("Recuperando secreto para clave: {}", key);

    let output = Command::new("secret-tool")
        .arg("lookup")
        .arg("service")
        .arg(SERVICE_NAME)
        .arg("username")
        .arg(key)
        .output()
        .ok()?;

    if output.status.success() {
        // secret-tool devuelve el secreto en stdout sin newline extra (usualmente)
        let secret = String::from_utf8(output.stdout).ok()?;
        if secret.is_empty() {
            None
        } else {
            Some(secret)
        }
    } else {
        // Si falla (ej. no encontrado), secret-tool suele retornar código no-cero.
        // No logueamos error aquí porque puede ser simplemente que no existe (comportamiento normal).
        None
    }
}

/// Elimina un secreto del anillo de llaves.
pub fn delete_secret(key: &str) -> Result<(), KeyringError> {
    debug!("Eliminando secreto para clave: {}", key);

    let output = Command::new("secret-tool")
        .arg("clear")
        .arg("service")
        .arg(SERVICE_NAME)
        .arg("username")
        .arg(key)
        .output()
        .map_err(|e| {
            error!("Fallo ejecutando secret-tool clear: {}", e);
            KeyringError::AccessError(e.to_string())
        })?;

    if output.status.success() {
        info!("Secreto eliminado exitosamente: {}", key);
        Ok(())
    } else {
        // secret-tool clear puede fallar si no existe, pero para nosotros es "éxito" (ya no está).
        // Aún así, logueamos warning por si es otro tipo de error.
        let err_msg = String::from_utf8_lossy(&output.stderr);
        if !err_msg.is_empty() {
            debug!("secret-tool clear output: {}", err_msg);
        }
        Ok(())
    }
}

// --------------------------------------------------------------------------
// TESTS DE INTEGRACIÓN (SOLO LINUX)
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // Nota: Estos tests requieren que 'secret-tool' esté instalado y que haya un
    // keyring desbloqueado disponible (ej. gnome-keyring-daemon).
    // En entornos headless/CI sin DBus/Keyring, estos tests fallarán.

    fn get_test_key() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
        format!("test_key_{}", since_the_epoch.as_millis())
    }

    #[test]
    fn test_lifecycle() {
        let key = get_test_key();
        let secret = "my_super_secret_value";

        // 1. Store
        let store_res = store_secret(&key, secret);
        // Si no hay keyring disponible, esto podría fallar, lo cual es correcto.
        // Pero intentamos aserciones suaves o ignorar si falla por entorno.
        if let Err(e) = store_res {
            println!("Salatando test de keyring: No disponible ({})", e);
            return;
        }

        // 2. Retrieve
        let retrieved = retrieve_secret(&key);
        assert!(retrieved.is_some(), "Debería recuperar el secreto guardado");
        assert_eq!(retrieved.unwrap(), secret, "El secreto recuperado debe coincidir");

        // 3. Delete
        let del_res = delete_secret(&key);
        assert!(del_res.is_ok(), "Debería borrar sin errores");

        // 4. Retrieve again (should fail)
        let retrieved_after = retrieve_secret(&key);
        assert!(retrieved_after.is_none(), "El secreto ya no debería existir");
    }
}
