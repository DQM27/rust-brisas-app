// Implementación de keyring para Windows usando Windows Credential Manager
// Utiliza la API nativa de Windows para almacenar credenciales de forma segura
//
// SAFETY: Este módulo requiere unsafe para FFI con Windows Credential Manager API.
// Cada bloque unsafe tiene documentación explicando por qué es seguro.
#![allow(unsafe_code)]

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use winapi::um::wincred::{
    CredDeleteW, CredReadW, CredWriteW, CREDENTIALW, CRED_PERSIST_LOCAL_MACHINE, CRED_TYPE_GENERIC,
    PCREDENTIALW,
};

const TARGET_PREFIX: &str = "brisas-app";

/// Convierte un &str a un Vec<u16> (wide string) para Windows API
fn to_wide_string(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}

/// Genera el nombre completo del target para la credencial
fn get_target_name(key: &str) -> String {
    format!("{}:{}", TARGET_PREFIX, key)
}

/// Almacena un secreto en el Credential Manager de Windows.
///
/// # Safety
///
/// Esta función usa `unsafe` para interactuar con la API de Windows Credential Manager:
/// - `std::mem::zeroed()` para inicializar `LastWritten` (FILETIME) - seguro porque
///   FILETIME es un struct POD sin invariantes.
/// - `CredWriteW` es una función FFI de Windows que requiere punteros válidos.
///   Los punteros `target_wide`, `username_wide` y `value_bytes` se mantienen vivos
///   durante toda la llamada porque son variables locales en scope.
pub fn store_secret(key: &str, value: &str) -> Result<(), String> {
    let target_name = get_target_name(key);
    let target_wide = to_wide_string(&target_name);
    let username_wide = to_wide_string(key);
    let value_bytes = value.as_bytes();

    let mut credential = CREDENTIALW {
        Flags: 0,
        Type: CRED_TYPE_GENERIC,
        TargetName: target_wide.as_ptr() as *mut _,
        Comment: ptr::null_mut(),
        // SAFETY: FILETIME es un struct POD (Plain Old Data), zeroed es un estado válido
        LastWritten: unsafe { std::mem::zeroed() },
        CredentialBlobSize: value_bytes.len() as u32,
        CredentialBlob: value_bytes.as_ptr() as *mut _,
        Persist: CRED_PERSIST_LOCAL_MACHINE,
        AttributeCount: 0,
        Attributes: ptr::null_mut(),
        TargetAlias: ptr::null_mut(),
        UserName: username_wide.as_ptr() as *mut _,
    };

    // SAFETY: CredWriteW es una función FFI segura cuando se le pasa una estructura
    // CREDENTIALW válida. Todos los punteros en `credential` apuntan a datos válidos
    // que permanecen vivos durante la llamada.
    let result = unsafe { CredWriteW(&mut credential, 0) };

    if result == 0 {
        Err(format!(
            "Error almacenando credencial '{}' en Windows Credential Manager: {}",
            key,
            std::io::Error::last_os_error()
        ))
    } else {
        Ok(())
    }
}

/// Recupera un secreto del Credential Manager de Windows.
///
/// # Safety
///
/// Esta función usa `unsafe` para interactuar con la API de Windows:
/// - `CredReadW` escribe un puntero a memoria asignada por Windows en `credential_ptr`.
/// - Después de una llamada exitosa, `credential_ptr` apunta a memoria válida que
///   debe ser liberada con `CredFree`.
/// - `std::slice::from_raw_parts` requiere que el puntero sea válido y el tamaño correcto.
///   Validamos que `CredentialBlob` no sea null y que `CredentialBlobSize > 0` antes de usarlo.
pub fn retrieve_secret(key: &str) -> Option<String> {
    let target_name = get_target_name(key);
    let target_wide = to_wide_string(&target_name);
    let mut credential_ptr: PCREDENTIALW = ptr::null_mut();

    // SAFETY: CredReadW es seguro cuando se le pasan punteros válidos.
    // `target_wide` permanece vivo durante la llamada.
    // `credential_ptr` es inicializado a null y CredReadW lo sobrescribe en caso de éxito.
    let result =
        unsafe { CredReadW(target_wide.as_ptr(), CRED_TYPE_GENERIC, 0, &mut credential_ptr) };

    if result == 0 {
        return None;
    }

    // SAFETY: Si result != 0, Windows garantiza que credential_ptr apunta a una
    // estructura CREDENTIALW válida asignada por el sistema.
    unsafe {
        let credential = &*credential_ptr;

        // Validar que el puntero y tamaño son válidos antes de crear el slice
        let value = if credential.CredentialBlob.is_null() || credential.CredentialBlobSize == 0 {
            String::new()
        } else {
            // SAFETY: Después de validar que CredentialBlob no es null y CredentialBlobSize > 0,
            // podemos crear un slice seguro. Windows garantiza que el blob contiene
            // exactamente CredentialBlobSize bytes válidos.
            let blob_slice = std::slice::from_raw_parts(
                credential.CredentialBlob,
                credential.CredentialBlobSize as usize,
            );
            String::from_utf8_lossy(blob_slice).to_string()
        };

        // SAFETY: CredFree debe ser llamado para liberar la memoria asignada por CredReadW.
        // Después de esta llamada, credential_ptr ya no es válido.
        winapi::um::wincred::CredFree(credential_ptr as *mut _);

        Some(value)
    }
}

/// Elimina un secreto del Credential Manager de Windows.
///
/// # Safety
///
/// Esta función usa `unsafe` para llamar a `CredDeleteW`, una función FFI de Windows.
/// `target_wide` permanece vivo durante toda la llamada, garantizando que el puntero sea válido.
#[allow(dead_code)]
pub fn delete_secret(key: &str) -> Result<(), String> {
    let target_name = get_target_name(key);
    let target_wide = to_wide_string(&target_name);

    // SAFETY: CredDeleteW es seguro cuando se le pasa un puntero a wide string válido.
    // `target_wide` permanece vivo durante la llamada.
    let result = unsafe { CredDeleteW(target_wide.as_ptr(), CRED_TYPE_GENERIC, 0) };

    if result == 0 {
        let err = std::io::Error::last_os_error();
        let err_code = err.raw_os_error().unwrap_or(0);

        // ERROR_NOT_FOUND = 1168, no es un error crítico si la credencial no existe
        if err_code == 1168 {
            Ok(())
        } else {
            Err(format!(
                "Error eliminando credencial '{}' en Windows Credential Manager: {}",
                key, err
            ))
        }
    } else {
        Ok(())
    }
}
