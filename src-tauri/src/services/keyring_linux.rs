// Implementación de keyring para Linux usando secret-tool directamente
// Esto evita problemas de persistencia de la librería keyring

use std::process::Command;

const SERVICE: &str = "brisas-app";

pub fn store_secret(key: &str, value: &str) -> Result<(), String> {
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
        Ok(())
    } else {
        Err(format!("secret-tool store falló con código: {:?}", output.code()))
    }
}

pub fn retrieve_secret(key: &str) -> Option<String> {
    let output = Command::new("secret-tool")
        .arg("lookup")
        .arg("service")
        .arg(SERVICE)
        .arg("username")
        .arg(key)
        .output()
        .ok()?;

    if output.status.success() {
        String::from_utf8(output.stdout).ok()
    } else {
        None
    }
}

pub fn delete_secret(key: &str) -> Result<(), String> {
    let output = Command::new("secret-tool")
        .arg("clear")
        .arg("service")
        .arg(SERVICE)
        .arg("username")
        .arg(key)
        .output()
        .map_err(|e| format!("Error ejecutando secret-tool clear: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!("secret-tool clear falló con código: {:?}", output.status.code()))
    }
}
