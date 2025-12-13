// Implementación de keyring para Linux usando secret-tool directamente
// Esto evita problemas de persistencia de la librería keyring

use std::process::Command;

const SERVICE: &str = "brisas-app";

pub fn store_secret(key: &str, value: &str) -> Result<(), String> {
    eprintln!("[KEYRING LINUX] Guardando con secret-tool: service={}, username={}", SERVICE, key);

    let output = Command::new("secret-tool")
        .arg("store")
        .arg("--label")
        .arg(format!("Brisas App - {}", key))
        .arg("service")
        .arg(SERVICE)
        .arg("username")
        .arg(key)
        .stdin(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(value.as_bytes())?;
            }
            child.wait()
        })
        .map_err(|e| format!("Error ejecutando secret-tool store: {}", e))?;

    if output.success() {
        eprintln!("[KEYRING LINUX] ✓ Guardado exitoso");
        Ok(())
    } else {
        Err(format!("secret-tool store falló con código: {:?}", output.code()))
    }
}

pub fn retrieve_secret(key: &str) -> Option<String> {
    eprintln!("[KEYRING LINUX] Leyendo con secret-tool: service={}, username={}", SERVICE, key);

    let output = Command::new("secret-tool")
        .arg("lookup")
        .arg("service")
        .arg(SERVICE)
        .arg("username")
        .arg(key)
        .output()
        .ok()?;

    if output.status.success() {
        let value = String::from_utf8(output.stdout).ok()?;
        eprintln!("[KEYRING LINUX] ✓ Valor recuperado, len={}", value.len());
        Some(value)
    } else {
        eprintln!("[KEYRING LINUX] ✗ No se encontró la clave");
        None
    }
}

pub fn delete_secret(key: &str) -> Result<(), String> {
    eprintln!("[KEYRING LINUX] Eliminando con secret-tool: service={}, username={}", SERVICE, key);

    let output = Command::new("secret-tool")
        .arg("clear")
        .arg("service")
        .arg(SERVICE)
        .arg("username")
        .arg(key)
        .output()
        .map_err(|e| format!("Error ejecutando secret-tool clear: {}", e))?;

    if output.status.success() {
        eprintln!("[KEYRING LINUX] ✓ Eliminado exitosamente");
        Ok(())
    } else {
        Err(format!("secret-tool clear falló con código: {:?}", output.status.code()))
    }
}
