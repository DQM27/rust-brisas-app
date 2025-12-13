# 🔐 Solución a Problemas de Keyring

## Problema Identificado

El sistema de almacenamiento seguro de credenciales (keyring) no está funcionando en Linux ni en Windows porque **faltan dependencias del sistema**.

## Diagnóstico Realizado

Ejecutamos `./diagnose-keyring.sh` y encontramos:

- ✅ gnome-keyring-daemon está corriendo
- ✅ DBUS configurado correctamente
- ✅ Dependencia `keyring = "3"` en Cargo.toml
- ❌ **`secret-tool` NO está instalado** ← PROBLEMA PRINCIPAL
- ❌ **`libsecret-tools` NO está instalado**

## Solución para Linux (Ubuntu/Debian)

### Paso 1: Instalar dependencias del sistema

```bash
sudo apt install libsecret-tools libsecret-1-dev
```

### Paso 2: Verificar instalación

```bash
secret-tool --version
```

### Paso 3: Probar manualmente

```bash
# Guardar un secreto de prueba
echo "mi-password" | secret-tool store --label="test-brisas" app "brisas-test" key "test"

# Leer el secreto
secret-tool lookup app "brisas-test" key "test"

# Eliminar el secreto
secret-tool clear app "brisas-test" key "test"
```

Si estos comandos funcionan, el keyring está funcionando correctamente.

### Paso 4: Probar la aplicación

Ejecuta la aplicación Tauri:

```bash
npm run tauri dev
```

Y usa los comandos desde el frontend o prueba directamente con el archivo [test-keyring.html](test-keyring.html).

## Solución para Windows

En Windows, el problema es diferente. Windows usa **Windows Credential Manager** que viene incluido en el sistema, PERO puede haber problemas si:

### Problema 1: Servicio de Credential Manager no está activo

1. Abre "Servicios" (ejecuta `services.msc`)
2. Busca "Credential Manager"
3. Asegúrate de que esté **Iniciado** y configurado como **Automático**

### Problema 2: Permisos insuficientes

La aplicación necesita ejecutarse con permisos de usuario normal (NO como administrador). Si ejecutas como administrador, el Credential Manager puede usar un contenedor diferente.

### Problema 3: Antivirus bloqueando acceso

Algunos antivirus bloquean el acceso al Credential Manager. Agrega una excepción para tu aplicación.

### Verificar en Windows

Abre PowerShell y ejecuta:

```powershell
# Ver credenciales guardadas (puede requerir permisos)
cmdkey /list

# La aplicación debería crear entradas como:
# Target: brisas-app:smtp_host
# Target: brisas-app:smtp_user
# etc.
```

## Solución para Fedora/RedHat

```bash
sudo dnf install libsecret libsecret-devel
```

## Solución para Arch Linux

```bash
sudo pacman -S libsecret
```

## Scripts de Diagnóstico Incluidos

### 1. `diagnose-keyring.sh`
Script completo de diagnóstico que verifica:
- Sistema operativo y desktop environment
- Servicios de keyring corriendo
- Herramientas instaladas
- Configuración de D-Bus
- Prueba de lectura/escritura con secret-tool

Ejecutar:
```bash
./diagnose-keyring.sh
```

### 2. `test-keyring.html`
Página HTML con interfaz para probar:
- Test de diagnóstico del keyring (crea, lee, elimina credenciales de prueba)
- Guardar credenciales SMTP
- Leer credenciales SMTP

Usar dentro de la aplicación Tauri en desarrollo.

## Comando Tauri de Diagnóstico

Agregamos un nuevo comando `test_keyring()` en [src-tauri/src/commands/keyring_commands.rs](src-tauri/src/commands/keyring_commands.rs:233-305) que:

1. Crea una entrada de prueba en el keyring
2. Guarda una contraseña
3. Lee la contraseña y verifica que coincida
4. Elimina la credencial
5. Verifica que fue eliminada

Este comando está disponible desde el frontend:

```javascript
const result = await invoke('test_keyring');
console.log(result);
```

## Por Qué Falla el Keyring

La librería Rust `keyring` necesita acceso al **Secret Service API** de freedesktop.org, que es implementado por:

- **GNOME Keyring** (GNOME, Ubuntu, etc.)
- **KWallet** (KDE, Kubuntu, etc.)
- **Windows Credential Manager** (Windows)
- **Keychain** (macOS)

Sin las herramientas del sistema (`libsecret` en Linux), la librería no puede comunicarse con el servicio de secretos del SO.

## Verificación Post-Instalación

Después de instalar las dependencias, ejecuta nuevamente:

```bash
./diagnose-keyring.sh
```

Deberías ver:
```
✓ secret-tool: /usr/bin/secret-tool
✓ Puede escribir secretos
✓ Puede leer secretos correctamente
✓ Puede eliminar secretos
```

## Archivos Modificados/Creados

1. ✅ [src-tauri/src/commands/keyring_commands.rs](src-tauri/src/commands/keyring_commands.rs) - Agregado comando `test_keyring()`
2. ✅ [src-tauri/src/commands/handlers.rs](src-tauri/src/commands/handlers.rs) - Registrado nuevo comando
3. ✅ [diagnose-keyring.sh](diagnose-keyring.sh) - Script de diagnóstico
4. ✅ [test-keyring.html](test-keyring.html) - Interfaz de prueba
5. ✅ Este documento

## Notas Importantes

- **NO ejecutes la aplicación como root/administrador** - El keyring del usuario normal es diferente al de root
- **Asegúrate de estar en una sesión gráfica** - El keyring no funciona en SSH sin X11 forwarding
- **El keyring es por usuario** - Cada usuario tiene su propio almacén de secretos
- **En producción**, considera agregar instrucciones de instalación de `libsecret-tools` en tu documentación de instalación

## Soporte Multi-Plataforma

La librería `keyring` automáticamente usa el backend correcto:

| Plataforma | Backend | Requiere |
|------------|---------|----------|
| Linux | Secret Service (libsecret) | `libsecret-tools`, `libsecret-1-dev` |
| Windows | Credential Manager | Viene incluido |
| macOS | Keychain | Viene incluido |

## Próximos Pasos

1. ✅ Instalar `libsecret-tools` en tu sistema Linux
2. ✅ Ejecutar `./diagnose-keyring.sh` para verificar
3. ✅ Compilar la aplicación: `cd src-tauri && cargo build`
4. ✅ Probar con `npm run tauri dev`
5. ✅ Usar la interfaz de configuración de la app para guardar credenciales
6. ✅ Documentar requisitos de instalación para usuarios finales

## Para Distribución

Si estás creando un instalador/paquete para Linux, debes agregar `libsecret-1-0` como dependencia en tu paquete `.deb`, `.rpm`, o AppImage.

Ejemplo para `.deb`:
```
Depends: libsecret-1-0
```
