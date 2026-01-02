# Workflow I: Auditoría de Configuration & Setup (Tauri v2)

**Versión**: 3.0  
**Idioma**: Español  
**Aplicación**: Brisas APP  
**Framework**: Tauri v2

---

## Objetivo

Garantizar que la configuración de la aplicación Tauri (main.rs, plugins, variables de entorno, logging) siga las mejores prácticas de Tauri v2 y sea mantenible, segura y bien documentada.

---

## Principios Fundamentales

1. **Setup Claro**: `main.rs` debe ser conciso y delegar a módulos
2. **Plugins Centralizados**: Configuración de plugins en un solo lugar
3. **Secrets Seguros**: Nunca en el código, usar keyring o env vars
4. **Logging Robusto**: tauri-plugin-log configurado correctamente
5. **Error Handling**: Panics claros en setup, Result en runtime

---

## FASE 0: ANÁLISIS PREVIO (⚠️ OBLIGATORIO)

### [ ] 0.1 Auditoría de main.rs

**Plantilla de análisis**:

```markdown
**Archivo analizado**: `src/main.rs` (o `src-tauri/src/main.rs`)
**LOC**: {número de líneas}

## ESTRUCTURA ACTUAL

### Setup de Builder
```rust
tauri::Builder::default()
    .plugin(/* ... */)
    .invoke_handler(/* ... */)
    .setup(/* ... */)
    .run(/* ... */)
```

### Problemas Detectados
- [ ] Lógica compleja en main.rs (debería estar en módulos)
- [ ] Plugins sin configuración (usando defaults)
- [ ] Setup de BD en main.rs (debería estar en módulo service)
- [ ] Sin manejo de errores en .setup()
- [ ] Logging no configurado o mal configurado
```

### [ ] 0.2 Auditoría de Plugins

```markdown
## PLUGINS DE TAURI

### Plugins Instalados (según Cargo.toml)
| Plugin | Versión | Configurado? | Usado? | Notas |
|--------|---------|--------------|--------|-------|
| tauri-plugin-log | 2.x | ⚠️ | ✅ | Falta configuración de rotación |
| tauri-plugin-dialog | 2.x | ✅ | ✅ | - |
| tauri-plugin-store | 2.x | ❌ | ⚠️ | Sin inicializar |
| tauri-plugin-single-instance | 2.x | ✅ | ✅ | - |
| tauri-plugin-updater | 2.x | ❌ | ❌ | Pendiente implementar |

### Plugins Faltantes (recomendados)
- [ ] tauri-plugin-shell (si se ejecutan comandos externos)
- [ ] tauri-plugin-fs (si se manejan archivos)
- [ ] tauri-plugin-notification (si se usan notificaciones)
```

### [ ] 0.3 Auditoría de Configuración

```markdown
## ARCHIVOS DE CONFIGURACIÓN

### tauri.conf.json
- [ ] ¿Configuración de seguridad (CSP)?
- [ ] ¿allowlist correctamente configurado?
- [ ] ¿Configuración de ventana?
- [ ] ¿Nombre y versión de la app?

### .env / Variables de Entorno
- [ ] ¿Se usan variables de entorno?
- [ ] ¿Están documentadas?
- [ ] ¿Ejemplo en .env.example?

### Secrets
- [ ] ¿Se usan claves API?
- [ ] ¿Se almacenan en keyring?
- [ ] ¿Se hardcodean en el código? ❌ CRÍTICO
```

### [ ] 0.4 Auditoría de Logging

```markdown
## CONFIGURACIÓN DE LOGGING

### Estado Actual
```rust
.plugin(tauri_plugin_log::Builder::default().build())
```

### Problemas
- [ ] Sin configuración de targets
- [ ] Sin rotación de archivos
- [ ] Sin nivel de log configurado
- [ ] Sin logging a WebView (DevTools)

### Configuración Recomendada
```rust
.plugin(
    tauri_plugin_log::Builder::default()
        .targets([
            LogTarget::LogDir,    // Archivos en disco
            LogTarget::Stdout,    // Consola
            LogTarget::Webview,   // DevTools del frontend
        ])
        .level(LevelFilter::Info)
        .level_for("brisas_app", LevelFilter::Debug)
        .rotation_strategy(RotationStrategy::KeepAll)
        .build()
)
```
```

### [ ] 0.5 Auditoría de Handlers

```markdown
## INVOKE HANDLERS

### Estado Actual
```rust
.invoke_handler(tauri::generate_handler![
    // Lista de comandos
])
```

### Problemas
- [ ] >50 comandos en un solo handler (difícil de mantener)
- [ ] Sin organización por módulo
- [ ] Comandos sin prefix (colisiones potenciales)

### Recomendación
Organizar por módulos con macros helper:
```rust
macro_rules! handlers {
    ($($mod:ident::$cmd:ident),* $(,)?) => {
        tauri::generate_handler![$($mod::$cmd),*]
    };
}

.invoke_handler(handlers![
    // Contratistas
    contratista_commands::create_contratista,
    contratista_commands::get_all_contratistas,
    
    // Ingresos
    ingreso_commands::registrar_ingreso,
    ingreso_commands::get_ingresos_abiertos,
])
```
```

---

### 📋 Reporte Final de Análisis

```markdown
# Reporte de Análisis FASE 0 - Configuration

**Archivo principal**: src/main.rs
**LOC**: {número}

## PROBLEMAS CRÍTICOS
1. [CRÍTICO] Secrets hardcodeados en código
2. [CRÍTICO] Logging mal configurado

## PROBLEMAS MAYORES
3. [ALTO] Plugins sin configurar correctamente
4. [ALTO] Setup de BD en main.rs (separar a módulo)

## MEJORAS RECOMENDADAS
5. [MEDIO] Handlers sin organización
6. [BAJO] Sin .env.example documentado

## ESTIMACIÓN
- Separar setup a módulos: X horas
- Configurar plugins: Y horas
- Setup de logging: Z horas
- **TOTAL**: T horas

## ¿Proceder?
Esperar aprobación del usuario.
```

---

## FASE 1-7: EJECUCIÓN (Solo después de aprobar Fase 0)

### 1. [ ] Estructura de main.rs (Limpio y Conciso)

**Objetivo**: main.rs solo orquesta, la lógica está en módulos.

**Estructura recomendada**:

```rust
// src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Importar módulos
mod commands;
mod db;
mod domain;
mod models;
mod services;
mod common;

use commands::register_handlers;
use services::surrealdb_service;
use services::session::SessionState;
use tauri::Manager;

/// Punto de entrada de la aplicación Brisas.
///
/// ## Responsabilidades
/// 1. Configurar plugins de Tauri
/// 2. Inicializar base de datos
/// 3. Registrar comandos
/// 4. Gestionar estado de sesión
/// 5. Configurar logging
///
/// ## Panics
/// La aplicación hace panic si:
/// - No se puede inicializar la base de datos
/// - La configuración de Tauri es inválida
/// - Los plugins no se pueden cargar
#[tokio::main]
async fn main() {
    // Inicializar logging lo más pronto posible
    let _logger = setup_logging();
    
    log::info!("Iniciando Brisas APP v{}", env!("CARGO_PKG_VERSION"));
    
    tauri::Builder::default()
        // ====== PLUGINS ======
        .plugin(setup_log_plugin())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            log::info!("Ya existe una instancia de Brisas APP en ejecución");
            // Enfocar la ventana existente
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_focus();
            }
        }))
        
        // ====== STATE MANAGEMENT ======
        .manage(SessionState::default())
        
        // ====== SETUP ======
        .setup(|app| {
            log::info!("Ejecutando setup de aplicación");
            
            // Inicializar base de datos
            let db_path = get_database_path(app)?;
            log::info!("Inicializando base de datos en: {:?}", db_path);
            
            tauri::async_runtime::block_on(async {
                surrealdb_service::init_db(&db_path)
                    .await
                    .map_err(|e| {
                        log::error!("Error al inicializar BD: {}", e);
                        format!("No se pudo inicializar la base de datos: {}", e)
                    })?;
                
                log::info!("Base de datos inicializada correctamente");
                Ok(())
            })
        })
        
        // ====== COMMANDS ======
        .invoke_handler(register_handlers())
        
        // ====== RUN ======
        .run(tauri::generate_context!())
        .expect("Error al iniciar Brisas APP");
}

/// Configura el plugin de logging con rotación y múltiples targets.
fn setup_log_plugin() -> tauri_plugin_log::Builder {
    use tauri_plugin_log::{LogTarget, RotationStrategy};
    
    let log_level = if cfg!(debug_assertions) {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };
    
    tauri_plugin_log::Builder::default()
        .targets([
            LogTarget::LogDir,      // Archivos en disco
            LogTarget::Stdout,      // Consola (desarrollo)
            LogTarget::Webview,     // DevTools del frontend
        ])
        .level(log_level)
        // Nivel específico para nuestra app
        .level_for("brisas_app", log::LevelFilter::Debug)
        // Reducir noise de crates externos
        .level_for("surrealdb", log::LevelFilter::Warn)
        .level_for("tantivy", log::LevelFilter::Warn)
        // Rotar logs para no llenar disco
        .rotation_strategy(RotationStrategy::KeepAll)
}

/// Determina la ruta de la base de datos según el entorno.
///
/// ## Ubicaciones
/// - **Desarrollo**: `./db/brisas.db` (en directorio del proyecto)
/// - **Producción**: `{AppData}/brisas-app/db/brisas.db`
fn get_database_path(app: &tauri::AppHandle) -> Result<String, String> {
    if cfg!(debug_assertions) {
        // Desarrollo: usar directorio local
        Ok("./db/brisas.db".to_string())
    } else {
        // Producción: usar AppData
        let app_data = app
            .path()
            .app_data_dir()
            .map_err(|e| format!("No se pudo obtener directorio de datos: {}", e))?;
        
        let db_dir = app_data.join("db");
        std::fs::create_dir_all(&db_dir)
            .map_err(|e| format!("No se pudo crear directorio de BD: {}", e))?;
        
        let db_path = db_dir.join("brisas.db");
        Ok(db_path.to_string_lossy().to_string())
    }
}

/// Inicializa logging temprano (antes de Tauri).
///
/// Esto permite capturar logs incluso si Tauri falla al inicializar.
fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    {
        env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .init();
    }
    Ok(())
}
```

---

### 2. [ ] Registro de Handlers (Organizado por Módulos)

**Archivo**: `src/commands/mod.rs`

```rust
//! # Commands Registry
//!
//! Registro centralizado de todos los comandos Tauri.

pub mod contratista_commands;
pub mod ingreso_commands;
pub mod alerta_commands;
pub mod user_commands;
pub mod search_commands;
// ... otros módulos

use tauri::generate_handler;

/// Registra todos los comandos de la aplicación.
///
/// Los comandos están organizados por módulos para facilitar mantenimiento.
///
/// ## Convenciones de Naming
/// - Comandos de lectura: `get_{entidad}_by_{criterio}`
/// - Comandos de escritura: `create_{entidad}`, `update_{entidad}`, `delete_{entidad}`
/// - Operaciones especiales: `{verbo}_{entidad}`
pub fn register_handlers() -> impl Fn(tauri::Invoke) + Send + Sync + 'static {
    generate_handler![
        // ====== CONTRATISTAS ======
        contratista_commands::create_contratista,
        contratista_commands::get_all_contratistas,
        contratista_commands::get_contratista_by_id,
        contratista_commands::get_contratista_by_cedula,
        contratista_commands::update_contratista,
        contratista_commands::delete_contratista,
        contratista_commands::cambiar_estado_contratista,
        
        // ====== INGRESOS ======
        ingreso_commands::registrar_ingreso,
        ingreso_commands::registrar_salida,
        ingreso_commands::get_all_ingresos,
        ingreso_commands::get_ingresos_abiertos,
        ingreso_commands::get_ingreso_by_id,
        ingreso_commands::get_ingreso_by_gafete,
        
        // ====== ALERTAS ======
        alerta_commands::get_all_alertas_gafetes,
        alerta_commands::get_alertas_pendientes_by_cedula,
        alerta_commands::resolver_alerta_gafete,
        
        // ====== USUARIOS & AUTH ======
        user_commands::login,
        user_commands::logout,
        user_commands::get_current_user,
        user_commands::cambiar_contrasena,
        
        // ====== BÚSQUEDA ======
        search_commands::search_contratistas,
        search_commands::search_visitantes,
    ]
}
```

---

### 3. [ ] Configuración de Variables de Entorno

**Archivo**: `.env.example`

```env
# ================================
# BRISAS APP - CONFIGURACIÓN
# ================================

# Entorno de ejecución (development | production)
RUST_ENV=development

# Nivel de logging (trace | debug | info | warn | error)
RUST_LOG=info

# Configuración de Base de Datos
# En desarrollo, se puede usar una ruta relativa
# En producción, se usa automáticamente AppData
DB_PATH=./db/brisas.db

# Configuración de Búsqueda (Tantivy)
SEARCH_INDEX_PATH=./search_index

# (Opcional) Configuración de API keys si se integran servicios externos
# API_KEY_SERVICIO_EXTERNO=

# (Opcional) Configuración de SMTP para emails
# SMTP_HOST=
# SMTP_PORT=
# SMTP_USER=
# SMTP_PASSWORD=

# ================================
# NOTAS DE SEGURIDAD
# ================================
# - NUNCA commitear el archivo .env real
# - Usar keyring para secrets sensibles
# - Este es solo un ejemplo, copiar a .env y modificar
```

**Archivo**: `.gitignore` (asegurar que incluya)

```gitignore
# Variables de entorno
.env
.env.local

# Base de datos de desarrollo
db/
*.db
*.db-shm
*.db-wal

# Índices de búsqueda
search_index/

# Logs
*.log
logs/
```

---

### 4. [ ] Gestión de Secrets (Keyring)

**Uso del keyring para secrets sensibles**:

```rust
// src/services/secrets_service.rs

//! # Secrets Management
//!
//! Gestión segura de secretos usando el keyring del sistema operativo.

use keyring::Entry;
use thiserror::Error;

/// Errores de gestión de secretos.
#[derive(Debug, Error)]
pub enum SecretError {
    #[error("Error al acceder al keyring: {0}")]
    KeyringError(#[from] keyring::Error),
    
    #[error("Secret no encontrado: {0}")]
    NotFound(String),
}

/// Service para gestionar secretos en el keyring del sistema.
///
/// ## Seguridad
/// - Windows: Usa Credential Manager
/// - macOS: Usa Keychain
/// - Linux: Usa Secret Service (libsecret)
pub struct SecretsService {
    service_name: String,
}

impl SecretsService {
    /// Crea un nuevo servicio de secretos.
    ///
    /// ## Parámetros
    /// * `service_name` - Nombre del servicio (ej: "brisas-app")
    pub fn new(service_name: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
        }
    }
    
    /// Guarda un secret en el keyring.
    ///
    /// ## Ejemplo
    /// ```rust
    /// let secrets = SecretsService::new("brisas-app");
    /// secrets.set_secret("api_key", "sk_12345...")?;
    /// ```
    pub fn set_secret(&self, key: &str, value: &str) -> Result<(), SecretError> {
        let entry = Entry::new(&self.service_name, key)?;
        entry.set_password(value)?;
        log::info!("Secret '{}' guardado en keyring", key);
        Ok(())
    }
    
    /// Obtiene un secret del keyring.
    ///
    /// ## Ejemplo
    /// ```rust
    /// let secrets = SecretsService::new("brisas-app");
    /// let api_key = secrets.get_secret("api_key")?;
    /// ```
    pub fn get_secret(&self, key: &str) -> Result<String, SecretError> {
        let entry = Entry::new(&self.service_name, key)?;
        entry.get_password().map_err(|e| match e {
            keyring::Error::NoEntry => SecretError::NotFound(key.to_string()),
            other => SecretError::KeyringError(other),
        })
    }
    
    /// Elimina un secret del keyring.
    pub fn delete_secret(&self, key: &str) -> Result<(), SecretError> {
        let entry = Entry::new(&self.service_name, key)?;
        entry.delete_password()?;
        log::info!("Secret '{}' eliminado del keyring", key);
        Ok(())
    }
}

// Comando Tauri para configurar secrets desde el frontend
#[tauri::command]
pub async fn set_secret(key: String, value: String) -> Result<(), String> {
    let secrets = SecretsService::new("brisas-app");
    secrets
        .set_secret(&key, &value)
        .map_err(|e| e.to_string())
}
```

---

### 5. [ ] Configuración de tauri.conf.json

**Secciones importantes**:

```json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devPath": "http://localhost:1420",
    "distDir": "../dist"
  },
  "package": {
    "productName": "Brisas APP",
    "version": "1.0.0"
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "shell": {
        "all": false,
        "open": true
      },
      "dialog": {
        "all": true,
        "ask": true,
        "confirm": true,
        "message": true,
        "open": true,
        "save": true
      },
      "fs": {
        "all": false,
        "readFile": true,
        "writeFile": true,
        "readDir": true,
        "createDir": true,
        "removeDir": true,
        "removeFile": true,
        "scope": ["$APPDATA/brisas-app/**"]
      }
    },
    "bundle": {
      "active": true,
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ],
      "identifier": "com.brisas.app",
      "longDescription": "Sistema ERP de Control de Acceso",
      "shortDescription": "Control de Acceso",
      "targets": "all"
    },
    "security": {
      "csp": "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; font-src 'self' data:"
    },
    "windows": [
      {
        "fullscreen": false,
        "height": 800,
        "resizable": true,
        "title": "Brisas APP",
        "width": 1200,
        "minHeight": 600,
        "minWidth": 800
      }
    ]
  }
}
```

---

### 6. [ ] Documentación de Setup

**Archivo**: `SETUP.md`

```markdown
# Setup de Brisas APP

## Requisitos Previos

- Rust 1.70+
- Node.js 18+
- Sistema operativo: Windows 10+, macOS 11+, o Linux (Ubuntu 20.04+)

## Instalación (Desarrollo)

1. **Clonar repositorio**:
   ```bash
   git clone https://github.com/tu-org/brisas-app.git
   cd brisas-app
   ```

2. **Instalar dependencias de Rust**:
   ```bash
   cd src-tauri
   cargo build
   ```

3. **Instalar dependencias de frontend**:
   ```bash
   npm install
   ```

4. **Configurar variables de entorno**:
   ```bash
   cp .env.example .env
   # Editar .env con tus valores
   ```

5. **Inicializar base de datos**:
   ```bash
   # La BD se inicializa automáticamente al primer run
   cargo tauri dev
   ```

## Estructura de Directorios

```
brisas-app/
├── src/               # Frontend (React/Vue/etc)
├── src-tauri/         # Backend (Rust + Tauri)
│   ├── src/
│   │   ├── main.rs    # Punto de entrada
│   │   ├── commands/  # Comandos Tauri
│   │   ├── services/  # Lógica de negocio
│   │   ├── domain/    # Reglas de negocio
│   │   ├── models/    # Estructuras de datos
│   │   └── db/        # Queries de base de datos
│   ├── Cargo.toml
│   └── tauri.conf.json
├── .env.example       # Plantilla de variables de entorno
└── README.md
```

## Variables de Entorno

Ver `.env.example` para lista completa de variables configurables.

## Logging

Los logs se guardan en:
- **Windows**: `%APPDATA%\brisas-app\logs\`
- **macOS**: `~/Library/Application Support/brisas-app/logs/`
- **Linux**: `~/.local/share/brisas-app/logs/`

## Troubleshooting

### Error: "No se pudo inicializar la base de datos"
- Verificar permisos de escritura en el directorio de AppData
- Revisar logs en `{AppData}/brisas-app/logs/`

### Error: "Sesión no válida"
- El keyring del sistema puede no estar configurado
- En Linux, instalar `gnome-keyring` o `libsecret`
```

---

### 7. [ ] Verificación Final

**Checklist de Configuration**:

- [ ] main.rs es conciso (<150 líneas)
- [ ] Setup de BD delegado a módulo service
- [ ] Logging configurado con rotación
- [ ] Plugins configurados (no solo default)
- [ ] Handlers organizados por módulo
- [ ] Variables de entorno documentadas (.env.example)
- [ ] .gitignore incluye .env, db/, logs/
- [ ] Secrets usan keyring (no hardcoded)
- [ ] tauri.conf.json tiene allowlist restrictivo
- [ ] SETUP.md documenta instalación

---

## Plantilla de Commit

```
refactor(config): mejorar setup y configuración de Tauri v2

- Reorganizar main.rs (delegar setup a módulos)
- Configurar tauri-plugin-log con rotación
- Documentar variables de entorno (.env.example)
- Implementar gestión de secrets con keyring
- Organizar handlers por módulos
- Agregar SETUP.md con instrucciones

Closes #{numero_issue}
```

---

**Fin del Workflow I - Configuration & Setup**
