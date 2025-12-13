# 🪟 Solución a Problemas de Keyring en Windows

## Problema en Windows

El keyring no guarda credenciales en Windows. Aquí están las causas más comunes y sus soluciones.

## Diagnóstico Rápido en Windows

### Verificar el Servicio de Credential Manager

1. Presiona `Win + R`
2. Escribe `services.msc` y presiona Enter
3. Busca **"Credential Manager"** o **"Administrador de credenciales"**
4. Verifica que:
   - Estado: **Iniciado**
   - Tipo de inicio: **Automático**

Si no está iniciado:
- Click derecho → **Iniciar**
- Click derecho → **Propiedades** → Tipo de inicio: **Automático**

### Ver credenciales guardadas manualmente

```cmd
cmdkey /list
```

Deberías ver entradas como:
```
Target: brisas-app:smtp_host
Type: Generic
```

### Ver credenciales en la interfaz gráfica

1. Presiona `Win + R`
2. Escribe `control /name Microsoft.CredentialManager`
3. Ve a **Credenciales de Windows**
4. Busca entradas que empiecen con `brisas-app:`

## Causas Comunes del Problema

### 1. Aplicación ejecutándose como Administrador

**Problema**: Cuando ejecutas la app como administrador, usa un almacén de credenciales diferente al de tu usuario normal.

**Solución**:
- NO ejecutes la aplicación como administrador
- Ejecuta con tu usuario normal
- Si necesitas permisos, configura permisos específicos en lugar de usar "Ejecutar como administrador"

**Verificar**:
```cmd
# En CMD normal (NO como admin), ejecuta:
whoami

# Y luego:
npm run tauri dev
```

### 2. Política de Grupo bloqueando acceso

**Problema**: En computadoras corporativas, las políticas de grupo pueden bloquear el acceso al Credential Manager.

**Solución**:
1. Presiona `Win + R`
2. Escribe `gpedit.msc` (solo en Windows Pro/Enterprise)
3. Ve a: **Configuración del equipo** → **Plantillas administrativas** → **Sistema** → **Administración de credenciales**
4. Asegúrate de que **"Permitir guardar credenciales"** esté **Habilitado** o **No configurado**

### 3. Antivirus o Windows Defender bloqueando

**Problema**: El antivirus puede bloquear el acceso al Credential Manager.

**Solución**:
1. Abre **Windows Security** (Seguridad de Windows)
2. Ve a **Protección contra virus y amenazas**
3. **Configuración de protección contra virus y amenazas**
4. Agrega una exclusión para tu aplicación:
   - Ruta: `C:\Users\TuUsuario\AppData\Local\com.brisas.app\` (o donde esté instalada)

### 4. Perfil de Usuario corrupto

**Problema**: El perfil de usuario de Windows puede estar corrupto.

**Solución**:
```cmd
# Intenta reparar el perfil:
sfc /scannow

# O crea un nuevo usuario de Windows y prueba ahí
```

### 5. Windows Home Edition

**Problema**: Windows Home no tiene algunas características de seguridad completas.

**Verificar versión**:
```cmd
winver
```

**Solución**: Debería funcionar igual, pero si tienes problemas persistentes, considera actualizar a Windows Pro.

## Script de Diagnóstico para Windows

Crea un archivo `diagnose-keyring.bat`:

```batch
@echo off
echo ====================================
echo    DIAGNOSTICO DE KEYRING - WINDOWS
echo ====================================
echo.

echo 1. Version de Windows:
ver
echo.

echo 2. Usuario actual:
whoami
echo.

echo 3. Verificando si se esta ejecutando como administrador:
net session >nul 2>&1
if %errorLevel% == 0 (
    echo    [!] ADVERTENCIA: Ejecutandose como Administrador
    echo    Esto puede causar problemas con el Credential Manager
    echo    Cierra y ejecuta sin permisos de administrador
) else (
    echo    [OK] Ejecutandose como usuario normal
)
echo.

echo 4. Credenciales guardadas en Credential Manager:
cmdkey /list | findstr /C:"brisas-app"
if %errorLevel% == 0 (
    echo    [OK] Se encontraron credenciales de brisas-app
) else (
    echo    [INFO] No se encontraron credenciales de brisas-app
    echo    Esto es normal si aun no has guardado credenciales
)
echo.

echo 5. Verificando servicio de Credential Manager:
sc query VaultSvc | findstr STATE
echo.

echo ====================================
echo    SUGERENCIAS
echo ====================================
echo.
echo - NO ejecutes la app como administrador
echo - Verifica que el servicio VaultSvc este iniciado
echo - Si usas antivirus, agrega una excepcion para la app
echo - Si estas en una red corporativa, verifica politicas de grupo
echo.
pause
```

Guarda como `diagnose-keyring.bat` y ejecútalo (NO como administrador).

## Probar el Keyring en Windows

### Desde PowerShell:

```powershell
# Guardar una credencial de prueba
cmdkey /generic:"test-brisas" /user:"testuser" /pass:"testpass"

# Listar para verificar
cmdkey /list | Select-String "test-brisas"

# Eliminar
cmdkey /delete:"test-brisas"
```

Si estos comandos funcionan, el Credential Manager está funcionando correctamente.

### Desde la aplicación Tauri:

1. Compila la app: `npm run tauri build`
2. Ejecuta el `.exe` generado (NO como administrador)
3. Ve a la configuración y guarda credenciales SMTP
4. Verifica con: `cmdkey /list`

## Código de Prueba en Rust (para depuración)

Si sigues teniendo problemas, crea este archivo `test-win-keyring.rs`:

```rust
use keyring::Entry;

fn main() {
    println!("=== Test Keyring en Windows ===\n");

    let service = "test-brisas-app";
    let username = "test-user";
    let password = "test-password-123";

    // 1. Crear entrada
    println!("1. Creando entrada...");
    let entry = match Entry::new(service, username) {
        Ok(e) => {
            println!("   ✓ OK");
            e
        }
        Err(e) => {
            println!("   ✗ Error: {}", e);
            println!("   Código de error Windows: {:?}", e);
            return;
        }
    };

    // 2. Guardar
    println!("\n2. Guardando contraseña...");
    match entry.set_password(password) {
        Ok(_) => println!("   ✓ OK"),
        Err(e) => {
            println!("   ✗ Error: {}", e);
            println!("   Código de error Windows: {:?}", e);

            // Errores comunes en Windows:
            // - ERROR_ACCESS_DENIED (5): Bloqueado por políticas/antivirus
            // - ERROR_INVALID_PARAMETER (87): Parámetros inválidos
            // - ERROR_NOT_SUPPORTED (50): Feature no soportada
            return;
        }
    }

    // 3. Leer
    println!("\n3. Leyendo contraseña...");
    match entry.get_password() {
        Ok(pass) => {
            println!("   ✓ OK: {}", pass);
            if pass == password {
                println!("   ✓ La contraseña coincide!");
            } else {
                println!("   ✗ ERROR: La contraseña NO coincide!");
            }
        }
        Err(e) => {
            println!("   ✗ Error: {}", e);
            return;
        }
    }

    // 4. Eliminar
    println!("\n4. Eliminando...");
    match entry.delete_credential() {
        Ok(_) => println!("   ✓ OK"),
        Err(e) => println!("   ⚠ Error: {}", e),
    }

    println!("\n=== Test completado ===");
}
```

Compila y ejecuta:
```cmd
rustc --edition 2021 test-win-keyring.rs
test-win-keyring.exe
```

## Errores Específicos de Windows y Soluciones

| Código Error | Descripción | Solución |
|--------------|-------------|----------|
| `ERROR_ACCESS_DENIED (5)` | Acceso denegado | Verifica antivirus, políticas de grupo, no uses admin |
| `ERROR_INVALID_PARAMETER (87)` | Parámetro inválido | Verifica que service/username no estén vacíos |
| `ERROR_NOT_SUPPORTED (50)` | No soportado | Tu versión de Windows puede no soportar esta API |
| `ERROR_NO_SUCH_LOGON_SESSION (1312)` | Sesión no válida | Reinicia sesión de Windows |

## Alternativa: Usar cifrado local en lugar de keyring

Si el Credential Manager sigue sin funcionar, puedes usar cifrado local como alternativa:

1. Cifra las credenciales con una clave derivada del hardware (HWID)
2. Guarda el archivo cifrado en `%APPDATA%\Brisas\`
3. Esto es menos seguro que usar el Credential Manager, pero funciona

## Requisitos Mínimos

- Windows 10 o superior
- Usuario con perfil completo (no cuenta temporal)
- Servicio VaultSvc (Credential Manager) iniciado
- NO ejecutar como administrador

## Para Distribución

En tu instalador (NSIS, WiX, etc.), agrega verificaciones:

```nsis
; Verificar que VaultSvc esté habilitado
System::Call 'advapi32::OpenSCManager(t0, t0, i0x1) i.r0'
System::Call 'advapi32::OpenService(ir0, t"VaultSvc", i0x4) i.r1'
; etc...
```

## Soporte

Si después de todo esto sigue sin funcionar:

1. Ejecuta `diagnose-keyring.bat` y guarda el resultado
2. Verifica en `cmdkey /list` si las credenciales se guardan
3. Prueba con un usuario de Windows diferente (nuevo)
4. Considera reportar el issue específico con los detalles del sistema

## Referencias

- [Windows Credential Manager API](https://learn.microsoft.com/en-us/windows/win32/api/wincred/)
- [keyring crate - Windows backend](https://github.com/hwchen/keyring-rs)
