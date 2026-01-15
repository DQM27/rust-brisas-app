# Workflow H: Jerarquía de Errores Idiomática (Rust + thiserror)

**Versión**: 3.0  
**Idioma**: Español  
**Aplicación**: Brisas APP  
**Paradigma**: Rust idiomático con `thiserror`

---

## Objetivo

Establecer una **jerarquía de errores clara y consistente** que fluya desde la base de datos hasta el frontend, con mensajes descriptivos en español y conversiones automáticas entre capas.

---

## Principios Fundamentales

1. **thiserror sobre anyhow**: `thiserror` para errores de biblioteca, `anyhow` solo en binarios
2. **Errores por Capa**: Cada capa tiene su propio tipo de error
3. **Conversiones Automáticas**: Usar `From` trait para conversiones entre capas
4. **Mensajes en Español**: Todos los errores orientados al usuario final
5. **Context Preservation**: No perder información al convertir errores

---

## Jerarquía de Errores en Brisas APP

```
┌─────────────────────────────────────────────────────────┐
│                   CommandError                           │
│            (Serializable para frontend)                  │
│   Tipos: Unauthorized, NotFound, ValidationError, etc.  │
└───────────────────┬─────────────────────────────────────┘
                    │ From<DomainError>
                    │ From<ServiceError>
┌───────────────────┴─────────────────────────────────────┐
│                  DomainError                             │
│           (Errores de lógica de negocio)                 │
│   Ej: CedulaInvalida, PraindVencido, etc.               │
└───────────────────┬─────────────────────────────────────┘
                    │ From<SurrealDbError>
┌───────────────────┴─────────────────────────────────────┐
│               SurrealDbError                             │
│              (Errores de persistencia)                   │
│   Ej: Connection, Query, NotFound, etc.                 │
└───────────────────┬─────────────────────────────────────┘
                    │ From<surrealdb::Error>
┌───────────────────┴─────────────────────────────────────┐
│              surrealdb::Error                            │
│          (Errores del crate externo)                     │
└─────────────────────────────────────────────────────────┘
```

---

## FASE 0: ANÁLISIS PREVIO (⚠️ OBLIGATORIO)

### [ ] 0.1 Auditoría de Errores Existentes

**Plantilla de análisis**:

```markdown
**Archivos de errores**:

- `src/domain/errors.rs` (o errores por módulo)
- `src/services/errors.rs`
- `src/commands/errors.rs`
- `src/db/errors.rs`

## ESTADO ACTUAL

### Errores de Dominio

| Módulo      | Archivo             | Usa thiserror? | Mensajes en español? | Estado      |
| ----------- | ------------------- | -------------- | -------------------- | ----------- |
| contratista | `domain/errors.rs`  | ✅             | ✅                   | ✅ Correcto |
| ingreso     | `domain/ingreso.rs` | ❌             | ⚠️                   | ⚠️ Mejorar  |

### Errores de Servicios

| Servicio            | Tipo de Error      | Conversiones? | Estado     |
| ------------------- | ------------------ | ------------- | ---------- |
| contratista_service | `ContratistaError` | ⚠️ Parcial    | ⚠️         |
| ingreso_service     | `String` genérico  | ❌            | ❌ CRÍTICO |

### Errores de Commands

| Command              | Tipo de Error | Serializable? | Estado |
| -------------------- | ------------- | ------------- | ------ |
| contratista_commands | `String`      | ❌            | ❌     |
| ingreso_commands     | `String`      | ❌            | ❌     |
```

### [ ] 0.2 Auditoría de Conversiones

```markdown
## CONVERSIONES ENTRE CAPAS

### DB → Domain

- [ ] `From<SurrealDbError> for ContratistaError` existe?
- [ ] Contexto preservado en conversión?

### Domain → Command

- [ ] `From<DomainError> for CommandError` existe?
- [ ] Tipos de error mapeados correctamente?

### Conversiones Faltantes

1. `SurrealDbError` → `ContratistaError`: ❌ Falta
2. `ContratistaError` → `CommandError`: ❌ Falta
```

### [ ] 0.3 Auditoría de Mensajes

```markdown
## CALIDAD DE MENSAJES DE ERROR

### Mensajes Genéricos (mejorar)

- [ ] "Error" → Agregar contexto
- [ ] "Invalid input" → Español: "Entrada inválida: {campo}"
- [ ] "Not found" → "No se encontró {entidad} con ID {id}"

### Mensajes Técnicos Expuestos (ocultar)

- [ ] Stack traces en errores de usuario
- [ ] IDs internos de BD
- [ ] Nombres de tablas/columnas

### Mensajes en Inglés (traducir)

- [ ] X mensajes requieren traducción a español
```

---

### 📋 Reporte Final de Análisis

```markdown
# Reporte de Análisis FASE 0 - Errors

## PROBLEMAS CRÍTICOS

1. [CRÍTICO] Commands usan `String` en lugar de tipos específicos
2. [CRÍTICO] Conversiones automáticas faltantes entre capas

## PROBLEMAS MAYORES

3. [ALTO] N errores sin thiserror
4. [ALTO] M mensajes genéricos sin contexto

## MEJORAS RECOMENDADAS

5. [MEDIO] K mensajes en inglés (traducir)
6. [BAJO] P errores exponiendo detalles técnicos

## ESTIMACIÓN

- Implementar jerarquía completa: X horas
- Conversiones automáticas: Y horas
- Mejorar mensajes: Z horas
- **TOTAL**: T horas

## ¿Proceder?

Esperar aprobación del usuario.
```

---

## FASE 1-6: EJECUCIÓN (Solo después de aprobar Fase 0)

### 1. [ ] Errores de Base de Datos (Capa Más Baja)

**Archivo**: `src/db/errors.rs`

````rust
//! # Errores de Persistencia (SurrealDB)
//!
//! Errores que pueden ocurrir al interactuar con la base de datos.

use thiserror::Error;

/// Errores específicos de operaciones con SurrealDB.
///
/// Estos errores representan problemas de infraestructura y persistencia,
/// y generalmente se convierten a errores de dominio en capas superiores.
#[derive(Debug, Error)]
pub enum SurrealDbError {
    /// Error al conectar a la base de datos.
    ///
    /// Puede ocurrir si el archivo de BD está corrupto, sin permisos,
    /// o si hay problemas de configuración.
    #[error("Error de conexión a la base de datos: {0}")]
    Connection(String),

    /// Error al ejecutar un query.
    ///
    /// Incluye el contexto del query que falló para facilitar debugging.
    #[error("Error en query: {0}")]
    Query(String),

    /// Error al deserializar resultado de query.
    ///
    /// Ocurre cuando el resultado de BD no coincide con el tipo esperado.
    #[error("Error de deserialización: {0}")]
    Deserialization(String),

    /// Registro no encontrado cuando se esperaba que existiera.
    ///
    /// Ejemplo: Buscar contratista por ID que no existe.
    #[error("Registro no encontrado: {0}")]
    NotFound(String),

    /// Error durante una transacción.
    ///
    /// Incluye información de qué operación falló dentro de la transacción.
    #[error("Error en transacción: {0}")]
    Transaction(String),

    /// Violación de constraint (ej: unicidad, foreign key).
    #[error("Violación de restricción de base de datos: {0}")]
    ConstraintViolation(String),

    /// Error genérico del crate surrealdb.
    ///
    /// Se usa cuando no se puede mapear a un error más específico.
    #[error("Error de SurrealDB: {0}")]
    Database(#[from] surrealdb::Error),
}

impl SurrealDbError {
    /// Helper para crear errores de query con contexto.
    ///
    /// ## Ejemplo
    /// ```rust
    /// return Err(SurrealDbError::query_error(
    ///     "SELECT * FROM contratista WHERE id = $id",
    ///     id,
    ///     err
    /// ));
    /// ```
    pub fn query_error(query: &str, context: impl std::fmt::Display, err: impl std::fmt::Display) -> Self {
        Self::Query(format!(
            "Query: '{}' | Contexto: {} | Error: {}",
            query, context, err
        ))
    }
}
````

---

### 2. [ ] Errores de Dominio (Lógica de Negocio)

**Archivo**: `src/domain/errors.rs` (o por módulo)

```rust
//! # Errores de Dominio
//!
//! Errores que representan violaciones de reglas de negocio.

use thiserror::Error;
use crate::db::SurrealDbError;

// --------------------------------------------------------------------------
// ERRORES DE CONTRATISTA
// --------------------------------------------------------------------------

/// Errores específicos del dominio de contratistas.
///
/// Estos errores representan violaciones de reglas de negocio y validaciones.
#[derive(Debug, Error)]
pub enum ContratistaError {
    /// La cédula proporcionada no cumple el formato costarricense.
    ///
    /// Formato esperado: X-XXXX-XXXX (ej: "1-2345-6789")
    #[error("Cédula inválida: {0}. Formato esperado: X-XXXX-XXXX")]
    CedulaInvalida(String),

    /// La cédula ya está registrada en el sistema.
    ///
    /// No se permiten duplicados de cédulas por razones de seguridad.
    #[error("Ya existe un contratista registrado con la cédula: {0}")]
    CedulaDuplicada(String),

    /// El campo obligatorio está vacío.
    #[error("El campo '{campo}' es obligatorio y no puede estar vacío")]
    CampoObligatorio { campo: String },

    /// La fecha de vencimiento del PRAIND no es válida.
    ///
    /// Debe estar en formato YYYY-MM-DD y ser una fecha futura.
    #[error("Fecha de vencimiento PRAIND inválida: {0}")]
    FechaVencimientoPraindInvalida(String),

    /// El PRAIND ha vencido.
    ///
    /// El contratista no puede ingresar con certificación vencida.
    #[error("PRAIND vencido. Fecha de vencimiento: {0}")]
    PraindVencido(String),

    /// La empresa especificada no existe en el sistema.
    #[error("No se encontró la empresa con ID: {0}")]
    EmpresaNoEncontrada(String),

    /// El contratista no fue encontrado.
    #[error("No se encontró el contratista con ID: {0}")]
    ContratistaNoEncontrado(String),

    /// Error genérico de validación.
    ///
    /// Usar solo cuando no hay un tipo de error más específico.
    #[error("Error de validación: {0}")]
    Validacion(String),

    /// Error de base de datos propagado.
    #[error("Error de base de datos: {0}")]
    Database(#[from] SurrealDbError),
}

// Implementar conversión desde SurrealDbError con contexto
impl From<SurrealDbError> for ContratistaError {
    fn from(err: SurrealDbError) -> Self {
        match err {
            SurrealDbError::NotFound(msg) => {
                // Intentar extraer si es empresa o contratista
                if msg.contains("empresa") {
                    Self::EmpresaNoEncontrada(msg)
                } else {
                    Self::ContratistaNoEncontrado(msg)
                }
            }
            SurrealDbError::ConstraintViolation(msg) if msg.contains("cedula") => {
                Self::CedulaDuplicada(msg)
            }
            other => Self::Database(other),
        }
    }
}

// --------------------------------------------------------------------------
// ERRORES DE INGRESO
// --------------------------------------------------------------------------

/// Errores específicos del dominio de ingresos.
#[derive(Debug, Error)]
pub enum IngresoError {
    /// La fecha/hora de ingreso no es válida.
    ///
    /// Debe estar en formato RFC 3339.
    #[error("Fecha/hora de ingreso inválida: {0}")]
    FechaIngresoInvalida(String),

    /// La fecha/hora de salida es anterior a la de ingreso.
    ///
    /// Esto es físicamente imposible y probablemente un error de captura.
    #[error("La fecha de salida no puede ser anterior a la de ingreso")]
    SalidaAnteriorAIngreso,

    /// El visitante/contratista está en la lista negra.
    ///
    /// No se permite el ingreso por motivos de seguridad.
    #[error("Ingreso bloqueado: La persona está en la lista negra. Motivo: {motivo}")]
    PersonaBloqueada { motivo: String },

    /// El visitante/contratista ya tiene un ingreso activo.
    ///
    /// Debe registrar salida antes de un nuevo ingreso.
    #[error("Ya existe un ingreso activo para la cédula: {0}")]
    IngresoActivoDuplicado(String),

    /// No hay gafetes disponibles.
    #[error("No hay gafetes disponibles. Liberar gafetes antes de registrar nuevo ingreso")]
    GafeteNoDisponible,

    /// El gafete especificado no existe.
    #[error("No se encontró el gafete con número: {0}")]
    GafeteNoEncontrado(String),

    /// El ingreso no fue encontrado.
    #[error("No se encontró el ingreso con ID: {0}")]
    IngresoNoEncontrado(String),

    /// Error de base de datos.
    #[error("Error de base de datos: {0}")]
    Database(#[from] SurrealDbError),
}

// --------------------------------------------------------------------------
// ERRORES DE ALERTA
// --------------------------------------------------------------------------

/// Errores del sistema de alertas de seguridad.
#[derive(Debug, Error)]
pub enum AlertaError {
    /// Error de validación de alerta.
    #[error("Error de validación: {0}")]
    Validacion(String),

    /// La alerta no fue encontrada.
    #[error("No se encontró la alerta con ID: {0}")]
    AlertaNoEncontrada(String),

    /// La alerta ya fue resuelta.
    #[error("La alerta ya fue resuelta previamente por: {usuario}")]
    AlertaYaResuelta { usuario: String },

    /// Error de base de datos.
    #[error("Error de base de datos: {0}")]
    Database(#[from] SurrealDbError),
}
```

---

### 3. [ ] Errores de Commands (Serializables para Frontend)

**Archivo**: `src/commands/errors.rs`

````rust
//! # Errores de Commands (Tauri)
//!
//! Errores serializables que se envían al frontend.

use serde::Serialize;
use crate::domain::errors::{ContratistaError, IngresoError, AlertaError};

/// Errores que pueden retornar los comandos Tauri.
///
/// Estos errores están diseñados para:
/// 1. Ser serializables a JSON
/// 2. Tener tipos que el frontend puede identificar
/// 3. Mensajes comprensibles para usuarios no técnicos
/// 4. Información suficiente para el frontend manejar el error apropiadamente
///
/// ## Ejemplo de Uso en TypeScript
/// ```typescript
/// try {
///   await invoke('create_contratista', { input });
/// } catch (error: CommandError) {
///   switch (error.type) {
///     case 'CONFLICT':
///       toast.error('La cédula ya está registrada');
///       break;
///     case 'NOT_FOUND':
///       toast.error('Empresa no encontrada');
///       break;
///     case 'VALIDATION_ERROR':
///       toast.error(error.details.message);
///       break;
///   }
/// }
/// ```
#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "details")]
pub enum CommandError {
    /// Usuario no autenticado o sesión expirada.
    ///
    /// El frontend debe redirigir al login.
    #[serde(rename = "UNAUTHORIZED")]
    Unauthorized {
        /// Mensaje descriptivo para el usuario
        message: String,
    },

    /// Usuario no tiene permisos para esta operación.
    ///
    /// El frontend debe mostrar mensaje de error y no permitir retry.
    #[serde(rename = "FORBIDDEN")]
    Forbidden {
        /// Acción que se intentó realizar
        action: String,
        /// Rol requerido
        required_role: Option<String>,
    },

    /// Recurso no encontrado.
    ///
    /// El frontend puede mostrar mensaje amigable o redirigir.
    #[serde(rename = "NOT_FOUND")]
    NotFound {
        /// Tipo de recurso (ej: "contratista", "empresa")
        resource_type: String,
        /// ID del recurso (opcional, no exponer si es sensible)
        resource_id: Option<String>,
    },

    /// Error de validación de datos de entrada.
    ///
    /// El frontend debe resaltar el campo con error.
    #[serde(rename = "VALIDATION_ERROR")]
    ValidationError {
        /// Campo específico con error (opcional)
        field: Option<String>,
        /// Mensaje descriptivo del error
        message: String,
    },

    /// Conflicto (ej: recurso ya existe, ingreso duplicado).
    ///
    /// El frontend debe informar al usuario y no permitir retry automático.
    #[serde(rename = "CONFLICT")]
    Conflict {
        /// Descripción del conflicto
        message: String,
    },

    /// Operación bloqueada por reglas de negocio.
    ///
    /// Ejemplo: Intento de ingreso de persona en lista negra.
    #[serde(rename = "BUSINESS_RULE_VIOLATION")]
    BusinessRuleViolation {
        /// Regla que se violó
        rule: String,
        /// Explicación para el usuario
        message: String,
    },

    /// Error interno del servidor.
    ///
    /// El frontend debe mostrar mensaje genérico y permitir reintentar.
    /// NO exponer detalles técnicos al usuario.
    #[serde(rename = "SERVER_ERROR")]
    ServerError {
        /// Mensaje genérico para el usuario
        message: String,
        /// ID de correlación para logs (opcional)
        #[serde(skip_serializing_if = "Option::is_none")]
        correlation_id: Option<String>,
    },
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unauthorized { message } => write!(f, "No autorizado: {}", message),
            Self::Forbidden { action, required_role } => {
                if let Some(role) = required_role {
                    write!(f, "Acceso denegado: '{}' requiere rol '{}'", action, role)
                } else {
                    write!(f, "Acceso denegado para: {}", action)
                }
            }
            Self::NotFound { resource_type, resource_id } => {
                if let Some(id) = resource_id {
                    write!(f, "{} no encontrado: {}", resource_type, id)
                } else {
                    write!(f, "{} no encontrado", resource_type)
                }
            }
            Self::ValidationError { field, message } => {
                if let Some(field) = field {
                    write!(f, "Error en '{}': {}", field, message)
                } else {
                    write!(f, "Error de validación: {}", message)
                }
            }
            Self::Conflict { message } => write!(f, "Conflicto: {}", message),
            Self::BusinessRuleViolation { rule, message } => {
                write!(f, "Regla violada ({}): {}", rule, message)
            }
            Self::ServerError { message, .. } => write!(f, "Error del servidor: {}", message),
        }
    }
}

// --------------------------------------------------------------------------
// CONVERSIONES DESDE ERRORES DE DOMINIO
// --------------------------------------------------------------------------

impl From<ContratistaError> for CommandError {
    fn from(err: ContratistaError) -> Self {
        match err {
            ContratistaError::CedulaDuplicada(cedula) => Self::Conflict {
                message: format!("Ya existe un contratista con la cédula: {}", cedula),
            },
            ContratistaError::CedulaInvalida(cedula) => Self::ValidationError {
                field: Some("cedula".to_string()),
                message: format!("Cédula inválida: {}. Formato esperado: X-XXXX-XXXX", cedula),
            },
            ContratistaError::CampoObligatorio { campo } => Self::ValidationError {
                field: Some(campo.clone()),
                message: format!("El campo '{}' es obligatorio", campo),
            },
            ContratistaError::PraindVencido(fecha) => Self::BusinessRuleViolation {
                rule: "praind_vigente".to_string(),
                message: format!("Certificación PRAIND vencida desde: {}", fecha),
            },
            ContratistaError::EmpresaNoEncontrada(id) => Self::NotFound {
                resource_type: "empresa".to_string(),
                resource_id: Some(id),
            },
            ContratistaError::ContratistaNoEncontrado(id) => Self::NotFound {
                resource_type: "contratista".to_string(),
                resource_id: Some(id),
            },
            ContratistaError::Validacion(msg) => Self::ValidationError {
                field: None,
                message: msg,
            },
            ContratistaError::Database(_) => Self::ServerError {
                message: "Error al procesar la solicitud. Intente nuevamente.".to_string(),
                correlation_id: None, // Aquí se podría agregar un ID único
            },
            _ => Self::ServerError {
                message: "Error inesperado del servidor".to_string(),
                correlation_id: None,
            },
        }
    }
}

impl From<IngresoError> for CommandError {
    fn from(err: IngresoError) -> Self {
        match err {
            IngresoError::PersonaBloqueada { motivo } => Self::BusinessRuleViolation {
                rule: "lista_negra".to_string(),
                message: format!("Ingreso bloqueado: {}", motivo),
            },
            IngresoError::IngresoActivoDuplicado(cedula) => Self::Conflict {
                message: format!(
                    "Ya existe un ingreso activo para la cédula: {}. Registre salida primero.",
                    cedula
                ),
            },
            IngresoError::GafeteNoDisponible => Self::Conflict {
                message: "No hay gafetes disponibles. Libere gafetes antes de continuar.".to_string(),
            },
            IngresoError::SalidaAnteriorAIngreso => Self::ValidationError {
                field: Some("fecha_salida".to_string()),
                message: "La fecha de salida no puede ser anterior a la de ingreso".to_string(),
            },
            IngresoError::IngresoNoEncontrado(id) => Self::NotFound {
                resource_type: "ingreso".to_string(),
                resource_id: Some(id),
            },
            IngresoError::Database(_) => Self::ServerError {
                message: "Error al procesar el ingreso. Intente nuevamente.".to_string(),
                correlation_id: None,
            },
            _ => Self::ServerError {
                message: "Error inesperado del servidor".to_string(),
                correlation_id: None,
            },
        }
    }
}

impl From<AlertaError> for CommandError {
    fn from(err: AlertaError) -> Self {
        match err {
            AlertaError::AlertaYaResuelta { usuario } => Self::Conflict {
                message: format!("La alerta ya fue resuelta previamente por: {}", usuario),
            },
            AlertaError::AlertaNoEncontrada(id) => Self::NotFound {
                resource_type: "alerta".to_string(),
                resource_id: Some(id),
            },
            AlertaError::Validacion(msg) => Self::ValidationError {
                field: None,
                message: msg,
            },
            AlertaError::Database(_) => Self::ServerError {
                message: "Error al procesar la alerta".to_string(),
                correlation_id: None,
            },
        }
    }
}
````

---

### 4. [ ] Uso en Commands

**Ejemplo de uso**:

```rust
use crate::commands::errors::CommandError;
use crate::services::contratista_service;

/// Crea un nuevo contratista.
///
/// ## Errores
/// Retorna `CommandError` serializable que el frontend puede manejar.
#[tauri::command]
pub async fn create_contratista(
    session: State<'_, SessionState>,
    input: CreateContratistaInput,
) -> Result<ContratistaResponse, CommandError> {
    // Validar sesión
    let _user = session
        .get_user()
        .ok_or(CommandError::Unauthorized {
            message: "Sesión no válida o expirada".to_string(),
        })?;

    // Llamar servicio - la conversión es automática gracias a From<ContratistaError>
    contratista_service::create_contratista(&search_service, input)
        .await
        .map_err(|e| e.into()) // Conversión automática
}
```

**En TypeScript**:

```typescript
interface CommandError {
	type:
		| 'UNAUTHORIZED'
		| 'FORBIDDEN'
		| 'NOT_FOUND'
		| 'VALIDATION_ERROR'
		| 'CONFLICT'
		| 'BUSINESS_RULE_VIOLATION'
		| 'SERVER_ERROR';
	details: {
		message: string;
		field?: string;
		resource_type?: string;
		action?: string;
		rule?: string;
	};
}

try {
	const contratista = await invoke('create_contratista', { input });
	toast.success('Contratista registrado exitosamente');
} catch (error: CommandError) {
	switch (error.type) {
		case 'CONFLICT':
			toast.error(error.details.message);
			break;
		case 'VALIDATION_ERROR':
			if (error.details.field) {
				setFieldError(error.details.field, error.details.message);
			} else {
				toast.error(error.details.message);
			}
			break;
		case 'BUSINESS_RULE_VIOLATION':
			showAlert({
				title: 'Operación bloqueada',
				message: error.details.message,
				type: 'warning'
			});
			break;
		case 'UNAUTHORIZED':
			router.push('/login');
			break;
		default:
			toast.error('Error inesperado. Intente nuevamente.');
	}
}
```

---

### 5. [ ] Testing de Errores

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cedula_duplicada_se_convierte_a_conflict() {
        let domain_error = ContratistaError::CedulaDuplicada("1-2345-6789".to_string());
        let command_error: CommandError = domain_error.into();

        assert!(matches!(command_error, CommandError::Conflict { .. }));
    }

    #[test]
    fn test_error_serializable_a_json() {
        let error = CommandError::ValidationError {
            field: Some("cedula".to_string()),
            message: "Formato inválido".to_string(),
        };

        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("VALIDATION_ERROR"));
        assert!(json.contains("cedula"));
    }
}
```

---

### 6. [ ] Verificación Final

**Checklist de Errors**:

- [ ] Todos los módulos usan `thiserror`
- [ ] Jerarquía clara: DB → Domain → Command
- [ ] Conversiones automáticas con `From`
- [ ] Mensajes en español
- [ ] CommandError es serializable
- [ ] No se exponen detalles técnicos en CommandError
- [ ] Tests para conversiones críticas

---

## Plantilla de Commit

```
refactor(errors): implementar jerarquía idiomática con thiserror

- Crear jerarquía: SurrealDbError → DomainError → CommandError
- Implementar conversiones automáticas con From trait
- CommandError serializable para manejo en frontend TypeScript
- Mensajes en español orientados al usuario
- Tests para conversiones críticas

Closes #{numero_issue}
```

---

**Fin del Workflow H - Errors Hierarchy**
