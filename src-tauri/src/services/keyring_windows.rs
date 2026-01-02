//! # Implementación de Keyring para Windows
//!
//! Utiliza el **Windows Credential Manager** (API nativa `wincred`) para almacenamiento seguro.
//!
//! ## Safety
//! Este módulo utiliza bloques `unsafe` para interactuar con la FFI de Windows (`winapi`).
//! Se han documentado las precondiciones y garantías de seguridad en cada función.

#![allow(unsafe_code)]

use crate::services::keyring_service::KeyringError;
use log::{debug, error, info};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use winapi::um::wincred::{
    CredDeleteW, CredReadW, CredWriteW, CREDENTIALW, CRED_PERSIST_LOCAL_MACHINE, CRED_TYPE_GENERIC,
    PCREDENTIALW,
};

const SERVICE_NAME: &str = "MegaBrisas";

/// Convierte un &str a un Vec<u16> (wide string) para Windows API
fn to_wide_string(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}

/// Genera el nombre completo del target para la credencial
fn get_target_name(key: &str) -> String {
    format!("{SERVICE_NAME}:{key}")
}

/// Almacena un secreto en el Credential Manager de Windows.
///
/// # Safety
/// Usa `winapi::CredWriteW` con punteros validados localmente.
pub fn store_secret(key: &str, value: &str) -> Result<(), KeyringError> {
    debug!("Almacenando secreto en Windows Credential Manager: {key}");

    let target_name = get_target_name(key);
    let target_wide = to_wide_string(&target_name);
    let username_wide = to_wide_string(key);
    let value_bytes = value.as_bytes();

    let mut credential = CREDENTIALW {
        Flags: 0,
        Type: CRED_TYPE_GENERIC,
        TargetName: target_wide.as_ptr().cast_mut(),
        Comment: ptr::null_mut(),
        // SAFETY: FILETIME es POD, zeroed es válido.
        LastWritten: unsafe { std::mem::zeroed() },
        CredentialBlobSize: value_bytes.len() as u32,
        CredentialBlob: value_bytes.as_ptr().cast_mut(),
        Persist: CRED_PERSIST_LOCAL_MACHINE,
        AttributeCount: 0,
        Attributes: ptr::null_mut(),
        TargetAlias: ptr::null_mut(),
        UserName: username_wide.as_ptr().cast_mut(),
    };

    // SAFETY: credential y sus punteros internos son válidos durante la llamada.
    let result = unsafe { CredWriteW(&raw mut credential, 0) };

    if result == 0 {
        let err = std::io::Error::last_os_error();
        error!("Fallo al escribir credencial '{key}': {err}");
        Err(KeyringError::StorageError(format!("Windows error: {err}")))
    } else {
        info!("Credencial almacenada exitosamente: {key}");
        Ok(())
    }
}

/// Recupera un secreto del Credential Manager de Windows.
///
/// # Safety
/// Usa `winapi::CredReadW` y `CredFree` para gestionar memoria asignada por el sistema.
pub fn retrieve_secret(key: &str) -> Option<String> {
    debug!("Recuperando secreto de Windows CM: {key}");

    let target_name = get_target_name(key);
    let target_wide = to_wide_string(&target_name);
    let mut credential_ptr: PCREDENTIALW = ptr::null_mut();

    // SAFETY: Punteros válidos. credential_ptr será sobrescrito.
    let result =
        unsafe { CredReadW(target_wide.as_ptr(), CRED_TYPE_GENERIC, 0, &raw mut credential_ptr) };

    if result == 0 {
        // No logueamos error aquí porque es normal que no exista
        return None;
    }

    // SAFETY: Windows garantiza puntero válido si result != 0.
    unsafe {
        let credential = &*credential_ptr;

        let value = if credential.CredentialBlob.is_null() || credential.CredentialBlobSize == 0 {
            String::new()
        } else {
            // SAFETY: Validamos blob y tamaño antes de crear slice.
            let blob_slice = std::slice::from_raw_parts(
                credential.CredentialBlob,
                credential.CredentialBlobSize as usize,
            );
            String::from_utf8_lossy(blob_slice).to_string()
        };

        // SAFETY: Liberar memoria del sistema.
        winapi::um::wincred::CredFree(credential_ptr.cast());

        Some(value)
    }
}

/// Elimina un secreto del Credential Manager de Windows.
///
/// # Safety
/// Usa `winapi::CredDeleteW`.
pub fn delete_secret(key: &str) -> Result<(), KeyringError> {
    debug!("Eliminando secreto de Windows CM: {key}");

    let target_name = get_target_name(key);
    let target_wide = to_wide_string(&target_name);

    // SAFETY: Puntero válido.
    let result = unsafe { CredDeleteW(target_wide.as_ptr(), CRED_TYPE_GENERIC, 0) };

    if result == 0 {
        let err = std::io::Error::last_os_error();
        let err_code = err.raw_os_error().unwrap_or(0);

        // ERROR_NOT_FOUND = 1168
        if err_code == 1168 {
            debug!("Intento de borrar credencial inexistente: {key} (ignorado)");
            Ok(())
        } else {
            error!("Fallo al eliminar credencial '{key}': {err}");
            Err(KeyringError::DeletionError(format!("Windows error: {err}")))
        }
    } else {
        info!("Credencial eliminada exitosamente: {key}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_key() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
        format!("test_key_{}", since_the_epoch.as_millis())
    }

    #[test]
    fn test_lifecycle_windows() {
        let key = get_test_key();
        let secret = "windows_secret_123";

        // 1. Store
        store_secret(&key, secret).expect("Store failed");

        // 2. Retrieve
        let retrieved = retrieve_secret(&key).expect("Retrieve failed");
        assert_eq!(retrieved, secret);

        // 3. Delete
        delete_secret(&key).expect("Delete failed");

        // 4. Retrieve again
        assert!(retrieve_secret(&key).is_none());
    }
}
