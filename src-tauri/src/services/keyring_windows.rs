// Implementación de keyring para Windows usando Windows Credential Manager
// Utiliza la API nativa de Windows para almacenar credenciales de forma segura

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
    OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

/// Genera el nombre completo del target para la credencial
fn get_target_name(key: &str) -> String {
    format!("{}:{}", TARGET_PREFIX, key)
}

/// Almacena un secreto en el Credential Manager de Windows
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
        LastWritten: unsafe { std::mem::zeroed() },
        CredentialBlobSize: value_bytes.len() as u32,
        CredentialBlob: value_bytes.as_ptr() as *mut _,
        Persist: CRED_PERSIST_LOCAL_MACHINE,
        AttributeCount: 0,
        Attributes: ptr::null_mut(),
        TargetAlias: ptr::null_mut(),
        UserName: username_wide.as_ptr() as *mut _,
    };

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

/// Recupera un secreto del Credential Manager de Windows
pub fn retrieve_secret(key: &str) -> Option<String> {
    let target_name = get_target_name(key);
    let target_wide = to_wide_string(&target_name);
    let mut credential_ptr: PCREDENTIALW = ptr::null_mut();

    let result = unsafe { CredReadW(target_wide.as_ptr(), CRED_TYPE_GENERIC, 0, &mut credential_ptr) };

    if result == 0 {
        return None;
    }

    unsafe {
        let credential = &*credential_ptr;
        let blob_slice = std::slice::from_raw_parts(
            credential.CredentialBlob,
            credential.CredentialBlobSize as usize,
        );
        let value = String::from_utf8_lossy(blob_slice).to_string();

        // Liberar la credencial
        winapi::um::wincred::CredFree(credential_ptr as *mut _);

        Some(value)
    }
}

/// Elimina un secreto del Credential Manager de Windows
pub fn delete_secret(key: &str) -> Result<(), String> {
    let target_name = get_target_name(key);
    let target_wide = to_wide_string(&target_name);

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
