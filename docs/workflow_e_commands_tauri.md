# Workflow E: Auditoría y Refactorización de Commands Tauri

**Versión**: 3.0  
**Idioma**: Español  
**Aplicación**: Brisas APP

---

## Objetivo

Garantizar que los comandos Tauri actúen como **adaptadores puros** entre el frontend y los servicios, sin lógica de negocio, con manejo robusto de errores y validación de seguridad adecuada.

---

## Principios Fundamentales

1. **Adaptador Puro**: Commands solo adaptan entre TypeScript y Rust, CERO lógica de negocio
2. **Thin Layer**: Máximo 5-10 líneas por comando (parsear → validar → llamar servicio → mapear)
3. **Errores Serializables**: Manejo específico para que el frontend los entienda
4. **Validación de Sesión**: Commands sensibles deben verificar auth/permisos
5. **Sin Dependencias Directas**: Commands NO llaman a repositorios ni domain directamente

---

## FASE 0: ANÁLISIS PREVIO (⚠️ OBLIGATORIO)

### [ ] 0.1 Auditoría de Responsabilidad

**Plantilla de análisis**:

```markdown
**Archivo analizado**: `src/commands/{modulo}_commands.rs`
**LOC**: {número de líneas}
**Número de comandos**: {N}

## ❌ VIOLACIONES DE THIN LAYER

### Lógica de Negocio en Commands (mover a services/)

- [ ] Línea XX: Cálculo/transformación de datos → Mover a servicio
- [ ] Línea YY: Validación de reglas de negocio → Mover a domain
- [ ] Línea ZZ: Construcción de queries → Mover a servicio

### Dependencias Directas Incorrectas

- [ ] Importa `crate::db::` → ❌ CRÍTICO, debe usar servicios
- [ ] Importa `crate::domain::` directamente → ⚠️ Usar a través de servicios
- [ ] Llamadas a repositorios → ❌ CRÍTICO, usar servicios

### Commands con >15 Líneas de Lógica

| Comando              | LOC | Problema                      | Acción             |
| -------------------- | --- | ----------------------------- | ------------------ |
| `comando_complejo()` | 25  | Mucha transformación de datos | Extraer a servicio |
| `otro_comando()`     | 18  | Validaciones complejas        | Mover a domain     |
```

### [ ] 0.2 Auditoría de Seguridad y Sesión

```markdown
## VALIDACIÓN DE SESIÓN

### Commands que Modifican Datos (requieren auth)

| Comando             | Valida Sesión? | Nivel Requerido | Estado      |
| ------------------- | -------------- | --------------- | ----------- |
| `create_*()`        | ❌             | Usuario         | ❌ CRÍTICO  |
| `update_*()`        | ❌             | Usuario         | ❌ CRÍTICO  |
| `delete_*()`        | ❌             | Admin           | ❌ CRÍTICO  |
| `resolver_alerta()` | ✅             | Supervisor      | ✅ Correcto |

### Commands de Solo Lectura (evaluar si requieren auth)

| Comando       | Valida Sesión? | ¿Debe validar? | Acción              |
| ------------- | -------------- | -------------- | ------------------- |
| `get_all_*()` | ❌             | Depende        | Evaluar con negocio |
| `get_by_id()` | ❌             | Probablemente  | Agregar validación  |

### Validación de Permisos

- [ ] ¿Se validan roles específicos? (admin, supervisor, usuario)
- [ ] ¿Se valida propiedad de recursos? (ej: usuario solo ve sus datos)
- [ ] ¿Se auditan operaciones sensibles?
```

### [ ] 0.3 Auditoría de Manejo de Errores

````markdown
## MANEJO DE ERRORES

### Mapeo Genérico (mejorar)

| Comando      | Línea | Patrón Actual                   | Problema                        |
| ------------ | ----- | ------------------------------- | ------------------------------- |
| `get_*()`    | XX    | `.map_err(\|e\| e.to_string())` | Pierde contexto de error        |
| `create_*()` | YY    | `.map_err(\|e\| e.to_string())` | Frontend no puede manejar tipos |

### Mensajes de Error para el Usuario

- [ ] ¿Errores son comprensibles para no-técnicos?
- [ ] ¿Se expone información sensible? (IDs internos, stack traces)
- [ ] ¿Errores están en español?

### Sugerencia: Crear enum de errores serializables

```rust
#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum CommandError {
    NotFound(String),
    Unauthorized(String),
    ValidationError(String),
    ServerError(String),
}
```
````

````

### [ ] 0.4 Auditoría de Validación de Input

```markdown
## VALIDACIÓN DE INPUTS

### Comandos que Reciben Datos del Frontend
| Comando | Input | Valida Formato? | Valida Negocio? | Acción |
|---------|-------|-----------------|-----------------|--------|
| `get_salidas_en_rango()` | `fecha_inicio`, `fecha_fin` | ❌ | ❌ | Validar RFC 3339 |
| `create_*()` | DTO | ⚠️ | ❌ | Llamar `domain::validar_*()` |
| `update_*()` | DTO | ⚠️ | ❌ | Llamar `domain::validar_*()` |

### Problemas Comunes
- [ ] Fechas sin validar formato → Usar `common::validar_fecha_*`
- [ ] IDs sin validar formato → Verificar que no estén vacíos
- [ ] Strings sin trim() → Normalizar antes de pasar a servicio
- [ ] Números sin validar rangos → Verificar positivos, límites
````

### [ ] 0.5 Auditoría de Documentación

```markdown
## DOCUMENTACIÓN

| Comando                    | Tiene `///`? | Explica propósito? | Documenta auth? | Idioma  |
| -------------------------- | ------------ | ------------------ | --------------- | ------- |
| `get_ingreso_by_id()`      | ✅           | ✅                 | ❌              | Español |
| `get_all_ingresos()`       | ✅           | ✅                 | ❌              | Español |
| `resolver_alerta_gafete()` | ✅           | ✅                 | ✅              | Español |

**Cobertura**: X/Y comandos documentados (Z%)

### Elementos faltantes en docs:

- [ ] Requisitos de autenticación/autorización
- [ ] Ejemplos de llamada desde TypeScript
- [ ] Posibles errores retornados
- [ ] Formato esperado de inputs (fechas, etc)
```

### [ ] 0.6 Auditoría de Naming y Convenciones

```markdown
## CONVENCIONES DE NAMING

### Prefijos de Comandos

| Tipo Operación       | Prefijo                       | Ejemplos             | Estado |
| -------------------- | ----------------------------- | -------------------- | ------ |
| Obtener uno          | `get_{entidad}_by_{criterio}` | `get_ingreso_by_id`  | ✅     |
| Obtener todos        | `get_all_{entidades}`         | `get_all_ingresos`   | ✅     |
| Crear                | `create_{entidad}`            | `create_contratista` | ✅     |
| Actualizar           | `update_{entidad}`            | `update_contratista` | ✅     |
| Eliminar             | `delete_{entidad}`            | `delete_contratista` | ✅     |
| Operación específica | `{verbo}_{entidad}`           | `resolver_alerta`    | ✅     |

### Inconsistencias Detectadas

- [ ] Comando con nombre poco claro: `{nombre}` → Sugerir renombre
- [ ] Prefijo incorrecto: `fetch_*` → Debe ser `get_*`
```

---

### 📋 Reporte Final de Análisis

```markdown
# Reporte de Análisis FASE 0 - Commands

**Archivo**: src/commands/{modulo}\_commands.rs
**LOC**: {número}
**Comandos**: {N}

## PROBLEMAS CRÍTICOS

1. [CRÍTICO] N comandos sin validación de sesión en operaciones de escritura
2. [CRÍTICO] M comandos con lógica de negocio → Extraer a servicios

## PROBLEMAS MAYORES

3. [ALTO] K comandos con validación de input insuficiente
4. [ALTO] P comandos con mapeo de errores genérico

## MEJORAS RECOMENDADAS

5. [MEDIO] Q comandos sin documentar requisitos de auth
6. [BAJO] R inconsistencias en naming

## ESTIMACIÓN

- Validación de sesión: X horas
- Extraer lógica a servicios: Y horas
- Validación de inputs: Z horas
- Mejora de errores: W horas
- Documentación: V horas
- **TOTAL**: T horas

## ¿Proceder?

Esperar aprobación del usuario.
```

---

## FASE 1-7: EJECUCIÓN (Solo después de aprobar Fase 0)

### 1. [ ] Implementar Validación de Sesión

**Objetivo**: Proteger operaciones sensibles con autenticación.

**Patrón Estándar**:

````rust
use crate::services::session::SessionState;
use tauri::State;

/// Crea un nuevo contratista en el sistema.
///
/// ## Autenticación Requerida
/// Este comando requiere sesión activa de usuario con rol `Usuario` o superior.
///
/// ## Autorización
/// - `Usuario`: Puede crear contratistas de su propia empresa
/// - `Admin`: Puede crear contratistas de cualquier empresa
///
/// ## Parámetros
/// * `session` - Estado de sesión de Tauri (inyectado automáticamente)
/// * `input` - Datos del contratista a crear
///
/// ## Retorno
/// * `Ok(ContratistaResponse)` - Contratista creado exitosamente
///
/// ## Errores
/// * `"Sesión no válida o expirada"` - Usuario no autenticado
/// * `"Sin permisos suficientes"` - Usuario sin rol adecuado
/// * `"Cédula duplicada"` - Ya existe contratista con esa cédula
/// * `"Error del servidor: {detalle}"` - Error inesperado
///
/// ## Ejemplo desde TypeScript
/// ```typescript
/// import { invoke } from '@tauri-apps/api/tauri';
///
/// const contratista = await invoke('create_contratista', {
///   input: {
///     cedula: '1-2345-6789',
///     nombre: 'Juan',
///     empresaId: 'empresa:123',
///   }
/// });
/// ```
#[tauri::command]
pub async fn create_contratista(
    session: State<'_, SessionState>,
    input: CreateContratistaInput,
) -> Result<ContratistaResponse, String> {
    // 1. Validar sesión
    let user = session
        .get_user()
        .ok_or("Sesión no válida o expirada".to_string())?;

    // 2. Validar permisos (si aplica)
    if user.rol != "Admin" && input.empresa_id != user.empresa_id {
        return Err("Sin permisos para crear contratistas de otra empresa".to_string());
    }

    // 3. Llamar al servicio
    contratista_service::create_contratista(&search_service, input)
        .await
        .map_err(|e| match e {
            ContratistaError::CedulaExists => "Ya existe un contratista con esa cédula".to_string(),
            ContratistaError::EmpresaNotFound => "La empresa especificada no existe".to_string(),
            ContratistaError::Validation(msg) => format!("Error de validación: {}", msg),
            _ => format!("Error del servidor: {}", e),
        })
}
````

**Clasificación de Commands por Nivel de Seguridad**:

```rust
// ========== PÚBLICOS (Sin autenticación) ==========
// Solo si la app no tiene login, o son operaciones triviales
#[tauri::command]
pub async fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// ========== LECTURA AUTENTICADA (Sesión requerida) ==========
#[tauri::command]
pub async fn get_all_ingresos(
    session: State<'_, SessionState>,
) -> Result<IngresoListResponse, String> {
    let _user = session
        .get_user()
        .ok_or("Sesión no válida")?;

    ingreso_service::get_all_ingresos().await
        .map_err(|e| format!("Error al obtener ingresos: {}", e))
}

// ========== ESCRITURA AUTENTICADA (Sesión + validación) ==========
#[tauri::command]
pub async fn create_entidad(
    session: State<'_, SessionState>,
    input: CreateInput,
) -> Result<Response, String> {
    let user = session.get_user().ok_or("Sesión no válida")?;

    // Validación de autorización si aplica

    servicio::create(input).await
        .map_err(|e| e.to_string())
}

// ========== OPERACIONES PRIVILEGIADAS (Admin/Supervisor) ==========
#[tauri::command]
pub async fn delete_usuario(
    session: State<'_, SessionState>,
    id: String,
) -> Result<(), String> {
    let user = session.get_user().ok_or("Sesión no válida")?;

    // Verificar rol de admin
    if user.rol != "Admin" {
        return Err("Operación requiere permisos de administrador".to_string());
    }

    usuario_service::delete(&id).await
        .map_err(|e| format!("Error al eliminar usuario: {}", e))
}
```

---

### 2. [ ] Validación de Inputs

**Objetivo**: Verificar formato antes de pasar a servicios.

**Patrón**:

```rust
/// Obtiene salidas de personal en un rango de fechas.
///
/// ## Autenticación
/// Requiere sesión activa.
///
/// ## Formato de Fechas
/// Las fechas deben estar en formato RFC 3339:
/// - `fecha_inicio`: "2026-01-01T00:00:00Z"
/// - `fecha_fin`: "2026-01-31T23:59:59Z"
///
/// ## Validaciones
/// - Ambas fechas deben ser válidas
/// - `fecha_fin` debe ser posterior a `fecha_inicio`
///
/// ## Parámetros
/// * `session` - Estado de sesión
/// * `fecha_inicio` - Fecha de inicio (RFC 3339)
/// * `fecha_fin` - Fecha de fin (RFC 3339)
///
/// ## Retorno
/// * `Ok(Vec<IngresoResponse>)` - Lista de salidas en el rango
///
/// ## Errores
/// * `"Sesión no válida"` - Usuario no autenticado
/// * `"Formato de fecha inválido"` - Fechas no cumplen RFC 3339
/// * `"Fecha de fin debe ser posterior a inicio"` - Rango inválido
#[tauri::command]
pub async fn get_salidas_en_rango(
    session: State<'_, SessionState>,
    fecha_inicio: String,
    fecha_fin: String,
) -> Result<Vec<IngresoResponse>, String> {
    // 1. Validar sesión
    let _user = session
        .get_user()
        .ok_or("Sesión no válida")?;

    // 2. Validar formato de fechas
    crate::common::validar_fecha_rfc3339(&fecha_inicio)
        .map_err(|_| "Formato de fecha de inicio inválido (debe ser RFC 3339)".to_string())?;

    crate::common::validar_fecha_rfc3339(&fecha_fin)
        .map_err(|_| "Formato de fecha de fin inválido (debe ser RFC 3339)".to_string())?;

    // 3. Validar que inicio < fin
    crate::common::validar_tiempo_salida(&fecha_inicio, &fecha_fin)
        .map_err(|_| "La fecha de fin debe ser posterior a la fecha de inicio".to_string())?;

    // 4. Llamar al servicio
    ingreso_service::get_salidas_en_rango(&fecha_inicio, &fecha_fin)
        .await
        .map_err(|e| format!("Error al obtener salidas: {}", e))
}
```

**Validaciones comunes en commands**:

```rust
// Validar que string no esté vacío
if input.campo.trim().is_empty() {
    return Err("El campo es obligatorio".to_string());
}

// Validar formato de ID
if !id.starts_with("entidad:") {
    return Err("Formato de ID inválido".to_string());
}

// Validar rangos numéricos
if input.cantidad < 1 || input.cantidad > 1000 {
    return Err("La cantidad debe estar entre 1 y 1000".to_string());
}

// Validar formato de fecha
crate::common::validar_fecha_rfc3339(&input.fecha)
    .map_err(|_| "Formato de fecha inválido".to_string())?;
```

---

### 3. [ ] Eliminar Lógica de Negocio

**Objetivo**: Commands solo adaptan, no transforman.

**Acción**:

```rust
// ❌ ANTES - Lógica en command
#[tauri::command]
pub async fn get_salidas_del_dia(fecha: String) -> Result<Vec<IngresoResponse>, String> {
    // ❌ Transformación de datos en el command
    let start = format!("{}T00:00:00Z", fecha);
    let end = format!("{}T23:59:59Z", fecha);

    ingreso_service::get_salidas_en_rango(&start, &end)
        .await
        .map_err(|e| e.to_string())
}

// ✅ DESPUÉS - Lógica en servicio
#[tauri::command]
pub async fn get_salidas_del_dia(
    session: State<'_, SessionState>,
    fecha: String,
) -> Result<Vec<IngresoResponse>, String> {
    let _user = session.get_user().ok_or("Sesión no válida")?;

    // Validar formato YYYY-MM-DD
    crate::common::validar_fecha_simple(&fecha)
        .map_err(|_| "Formato de fecha inválido (debe ser YYYY-MM-DD)".to_string())?;

    // El servicio se encarga de convertir a rango
    ingreso_service::get_salidas_del_dia(&fecha)
        .await
        .map_err(|e| format!("Error al obtener salidas: {}", e))
}

// Servicio con la lógica
pub async fn get_salidas_del_dia(fecha: &str) -> Result<Vec<IngresoResponse>, ServiceError> {
    let start = format!("{}T00:00:00Z", fecha);
    let end = format!("{}T23:59:59Z", fecha);
    get_salidas_en_rango(&start, &end).await
}
```

---

### 4. [ ] Mejorar Manejo de Errores

**Objetivo**: Errores comprensibles para el usuario final.

**Enum de Errores Serializables** (Opcional pero recomendado):

```rust
// src/commands/errors.rs

use serde::Serialize;

/// Errores que pueden retornar los comandos Tauri.
///
/// Estos errores están diseñados para ser serializados y enviados al frontend,
/// donde pueden ser manejados apropiadamente en la UI.
#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "details")]
pub enum CommandError {
    /// Operación requiere autenticación
    #[serde(rename = "UNAUTHORIZED")]
    Unauthorized { message: String },

    /// Usuario no tiene permisos suficientes
    #[serde(rename = "FORBIDDEN")]
    Forbidden { message: String },

    /// Recurso no encontrado
    #[serde(rename = "NOT_FOUND")]
    NotFound { message: String },

    /// Error de validación de datos de entrada
    #[serde(rename = "VALIDATION_ERROR")]
    ValidationError { field: Option<String>, message: String },

    /// Conflicto (ej: recurso ya existe)
    #[serde(rename = "CONFLICT")]
    Conflict { message: String },

    /// Error interno del servidor
    #[serde(rename = "SERVER_ERROR")]
    ServerError { message: String },
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unauthorized { message } => write!(f, "No autorizado: {}", message),
            Self::Forbidden { message } => write!(f, "Acceso denegado: {}", message),
            Self::NotFound { message } => write!(f, "No encontrado: {}", message),
            Self::ValidationError { field, message } => {
                if let Some(field) = field {
                    write!(f, "Error en campo '{}': {}", field, message)
                } else {
                    write!(f, "Error de validación: {}", message)
                }
            }
            Self::Conflict { message } => write!(f, "Conflicto: {}", message),
            Self::ServerError { message } => write!(f, "Error del servidor: {}", message),
        }
    }
}
```

**Uso en Commands**:

```rust
use crate::commands::errors::CommandError;

#[tauri::command]
pub async fn create_contratista(
    session: State<'_, SessionState>,
    input: CreateContratistaInput,
) -> Result<ContratistaResponse, CommandError> {
    // Validar sesión
    let user = session
        .get_user()
        .ok_or(CommandError::Unauthorized {
            message: "Sesión no válida o expirada".to_string(),
        })?;

    // Llamar servicio con mapeo específico de errores
    contratista_service::create_contratista(&search_service, input)
        .await
        .map_err(|e| match e {
            ContratistaError::CedulaExists => CommandError::Conflict {
                message: "Ya existe un contratista con esa cédula".to_string(),
            },
            ContratistaError::EmpresaNotFound => CommandError::NotFound {
                message: "La empresa especificada no existe".to_string(),
            },
            ContratistaError::Validation(msg) => CommandError::ValidationError {
                field: None,
                message: msg,
            },
            _ => CommandError::ServerError {
                message: "Error inesperado del servidor".to_string(),
            },
        })
}
```

**Desde TypeScript**:

```typescript
try {
	const contratista = await invoke('create_contratista', { input });
} catch (error: any) {
	// error.type será "CONFLICT", "NOT_FOUND", etc.
	switch (error.type) {
		case 'CONFLICT':
			toast.error('La cédula ya está registrada');
			break;
		case 'NOT_FOUND':
			toast.error('Empresa no encontrada');
			break;
		case 'VALIDATION_ERROR':
			toast.error(`Error: ${error.details.message}`);
			break;
		default:
			toast.error('Error inesperado');
	}
}
```

---

### 5. [ ] Documentación Completa

**Objetivo**: Docs útiles tanto para Rust como para TypeScript.

**Plantilla**:

````rust
/// {Descripción breve de la operación}.
///
/// {Explicación más detallada del propósito de negocio}
///
/// ## Autenticación
/// {Requisitos de sesión y autorización}
/// - Requiere sesión activa: Sí/No
/// - Roles permitidos: Usuario, Admin, Supervisor
/// - Permisos adicionales: {descripción}
///
/// ## Formato de Inputs
/// {Explicar formatos esperados, especialmente fechas}
/// - `fecha`: Formato RFC 3339 ("2026-01-15T08:30:00Z")
/// - `id`: Formato "{tabla}:{key}" ("contratista:123")
///
/// ## Validaciones
/// {Lista de validaciones que se realizan}
/// - Campo X no puede estar vacío
/// - Fecha Y debe ser posterior a fecha X
/// - Usuario debe pertenecer a la misma empresa
///
/// ## Parámetros
/// * `session` - Estado de sesión de Tauri (inyectado automáticamente)
/// * `{param}` - {Descripción y formato}
///
/// ## Retorno
/// * `Ok({Tipo})` - {Descripción del resultado exitoso}
///
/// ## Errores
/// * `"{mensaje}"` - {Cuándo ocurre este error}
///
/// ## Ejemplo desde TypeScript
/// ```typescript
/// import { invoke } from '@tauri-apps/api/tauri';
///
/// try {
///   const resultado = await invoke('nombre_comando', {
///     parametro1: 'valor',
///     parametro2: 123,
///   });
///   console.log('Éxito:', resultado);
/// } catch (error) {
///   console.error('Error:', error);
/// }
/// ```
///
/// ## Eventos Emitidos (si aplica)
/// Este comando puede emitir los siguientes eventos:
/// - `{evento}:{accion}` - {Descripción}
#[tauri::command]
pub async fn nombre_comando(...) -> Result<...> {
    // implementación
}
````

---

### 6. [ ] Organización del Archivo

**Objetivo**: Código fácil de navegar.

**Estructura estándar**:

```rust
//! # Commands: {Módulo}
//!
//! Comandos Tauri que exponen operaciones de {módulo} al frontend.
//!
//! ## Convenciones
//! - Todos los comandos están marcados con `#[tauri::command]`
//! - Commands de escritura requieren validación de sesión
//! - Errores están en español y son comprensibles para usuarios
//!
//! ## Categorías de Comandos
//! - **Consultas**: `get_*` - Operaciones de solo lectura
//! - **Creación**: `create_*` - Crear nuevas entidades
//! - **Actualización**: `update_*` - Modificar entidades existentes
//! - **Eliminación**: `delete_*` - Eliminar (soft delete) entidades
//! - **Operaciones Especiales**: Acciones específicas del dominio

use crate::commands::errors::CommandError;
use crate::services::session::SessionState;
use tauri::State;

// --------------------------------------------------------------------------
// COMANDOS DE CONSULTA (Solo Lectura)
// --------------------------------------------------------------------------

/// ...
#[tauri::command]
pub async fn get_by_id(...) { }

/// ...
#[tauri::command]
pub async fn get_all(...) { }

// --------------------------------------------------------------------------
// COMANDOS DE CREACIÓN
// --------------------------------------------------------------------------

/// ...
#[tauri::command]
pub async fn create(...) { }

// --------------------------------------------------------------------------
// COMANDOS DE ACTUALIZACIÓN
// --------------------------------------------------------------------------

/// ...
#[tauri::command]
pub async fn update(...) { }

// --------------------------------------------------------------------------
// COMANDOS DE ELIMINACIÓN
// --------------------------------------------------------------------------

/// ...
#[tauri::command]
pub async fn delete(...) { }

// --------------------------------------------------------------------------
// COMANDOS DE OPERACIONES ESPECIALES
// --------------------------------------------------------------------------

/// ...
#[tauri::command]
pub async fn operacion_especial(...) { }
```

---

### 7. [ ] Verificación Final

**Checklist de Commands**:

- [ ] Sin lógica de negocio (máximo 5-10 líneas por command)
- [ ] Validación de sesión en operaciones de escritura
- [ ] Validación de inputs (formatos, rangos)
- [ ] Manejo de errores comprensibles (español)
- [ ] Todas las funciones documentadas con `///`
- [ ] Ejemplos de TypeScript en documentación
- [ ] Separadores visuales entre categorías
- [ ] Naming consistente (`get_*`, `create_*`, `update_*`, `delete_*`)
- [ ] Sin imports directos a `db/` o `repositories/`
- [ ] Solo llama a servicios

**Compilación**:

```bash
# Verificar compilación
cargo check --package mega-brisas

# Verificar warnings
cargo clippy --package mega-brisas -- -D warnings

# Formatear
cargo fmt
```

---

## Plantilla de Commit

```
refactor(commands): mejorar {modulo}_commands con validación y seguridad

- Agregar validación de sesión en operaciones de escritura
- Validar formato de inputs (fechas, IDs, rangos)
- Extraer lógica de negocio a servicios
- Mejorar manejo de errores con mensajes comprensibles
- Documentar requisitos de autenticación y ejemplos TypeScript
- Organizar con separadores visuales

Closes #{numero_issue}
```

---

## Testing de Commands (End-to-End)

Los commands deben testearse desde el frontend idealmente, pero también puedes hacer tests unitarios:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::session::SessionState;
    use tauri::State;

    #[tokio::test]
    async fn test_comando_sin_sesion_debe_fallar() {
        let session = SessionState::default();
        let state = State::from(&session);

        let resultado = create_contratista(
            state,
            CreateContratistaInput { /* ... */ }
        ).await;

        assert!(resultado.is_err());
        assert_eq!(resultado.unwrap_err(), "Sesión no válida o expirada");
    }
}
```

---

**Fin del Workflow E - Commands/Tauri**
