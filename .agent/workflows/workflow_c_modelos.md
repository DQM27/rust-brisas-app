# Workflow C: Auditoría y Refactorización de Modelos (DDD)

**Versión**: 3.0  
**Idioma**: Español  
**Aplicación**: Brisas APP

---

## Objetivo

Transformar archivos en `src/models` para adherirse a prácticas estrictas de Rust y Domain-Driven Design (DDD), eliminando "stringly typed" code y aplicando Type-Driven Design.

---

## Principios Fundamentales

1. **Solo Estructuras de Datos**: Models contiene `struct`, `enum` y conversiones simples (`From`, `Into`, DTOs)
2. **Type-Driven Design**: Usar tipos específicos en lugar de tipos primitivos genéricos
3. **Sin Lógica de Negocio**: Validaciones y cálculos van a `domain/`
4. **Enums sobre Strings**: Eliminar "stringly typed" code
5. **Option Explícito**: No usar valores centinela (strings vacíos para nulos)

---

## FASE 0: ANÁLISIS PREVIO (⚠️ OBLIGATORIO)

### [ ] 0.1 Auditoría de Pureza del Modelo

**Plantilla de análisis**:

```markdown
**Archivo analizado**: `src/models/{modulo}.rs`
**LOC**: {número de líneas}

## ❌ VIOLACIONES DE PUREZA DE MODELS

### Lógica de Negocio (mover a domain/)

- [ ] Línea XX: Método `validar_*()` → Mover a `domain/{modulo}`
- [ ] Línea YY: Método `calcular_*()` → Mover a `domain/{modulo}`
- [ ] Línea ZZ: Acceso a DB → Mover a `services/` o `repositories/`

### "Stringly Typed" Code (crear enums)

| Campo           | Tipo Actual | Valores Posibles                   | Acción                         |
| --------------- | ----------- | ---------------------------------- | ------------------------------ |
| `estado`        | `String`    | "ACTIVO", "INACTIVO", "SUSPENDIDO" | Crear `enum EstadoContratista` |
| `tipo_vehiculo` | `String`    | "AUTO", "MOTO", "CAMION"           | Crear `enum TipoVehiculo`      |
| `prioridad`     | `String`    | "ALTA", "MEDIA", "BAJA"            | Crear `enum Prioridad`         |

### Tipos Primitivos Débiles (value objects)

| Campo    | Tipo Actual | Validación Implícita | Acción                                      |
| -------- | ----------- | -------------------- | ------------------------------------------- |
| `email`  | `String`    | Formato RFC 5322     | Crear `struct Email(String)` con validación |
| `cedula` | `String`    | Formato X-XXXX-XXXX  | Crear `struct Cedula(String)`               |
| `placa`  | `String`    | Formato ABC-123      | Crear `struct Placa(String)`                |

### Nulabilidad Implícita (usar Option)

- [ ] Línea XX: Campo que usa `""` para indicar null → Cambiar a `Option<String>`
- [ ] Línea YY: Campo que usa `0` para indicar null → Cambiar a `Option<i32>`

### Dependencias Impuras

- [ ] Imports de `services/`
- [ ] Imports de `db/`
- [ ] Lógica de persistencia en el modelo
```

### [ ] 0.2 Auditoría de Documentación

```markdown
## DOCUMENTACIÓN

| Elemento          | Estado   | Acción                               |
| ----------------- | -------- | ------------------------------------ |
| Encabezado módulo | ✅/❌    | Agregar descripción                  |
| Structs públicas  | X/Y (Z%) | Documentar propósito de negocio      |
| Enums públicas    | X/Y (Z%) | Documentar casos de uso              |
| Campos de negocio | X/Y (Z%) | Explicar significado para el negocio |

**Idioma**: ✅/❌ Español
```

### [ ] 0.3 Auditoría de Separadores y Organización

```markdown
## ORGANIZACIÓN

- [ ] ¿Usa separadores visuales? (`// ----------`)
- [ ] ¿Estructura clara?: Enums → Structs → DTOs → Conversiones
- [ ] ¿Código comentado obsoleto?: N bloques encontrados
```

### [ ] 0.4 Conformidad con Estándares de Fechas

```markdown
## CAMPOS DE FECHA

| Campo               | Tipo Actual             | Tipo Esperado                  | Acción             |
| ------------------- | ----------------------- | ------------------------------ | ------------------ |
| `fecha_creacion`    | `String`                | `String` (RFC 3339 validado)   | Documentar formato |
| `fecha_vencimiento` | `String`                | `String` (YYYY-MM-DD validado) | Documentar formato |
| `updated_at`        | `chrono::DateTime<Utc>` | ✅ Correcto                    | -                  |

**Nota**: Models puede usar `String` para fechas si la validación ocurre en `domain/`
```

---

### 📋 Reporte Final de Análisis

```markdown
# Reporte de Análisis FASE 0 - Models

**Archivo**: src/models/{modulo}.rs
**LOC**: {número}

## PROBLEMAS CRÍTICOS

1. [CRÍTICO] N campos "stringly typed" → Crear enums
2. [CRÍTICO] M métodos con lógica de negocio → Mover a domain/

## PROBLEMAS MAYORES

3. [ALTO] K campos sin `Option` (usan valores centinela) → Refactorizar
4. [MEDIO] P structs/enums sin documentar → Agregar docstrings

## MEJORAS RECOMENDADAS

5. [BAJO] Q campos primitivos débiles → Considerar value objects
6. [BAJO] R sin separadores visuales → Organizar

## ESTIMACIÓN

- Crear enums: X horas
- Mover lógica a domain: Y horas
- Refactorizar nulabilidad: Z horas
- Documentación: W horas
- **TOTAL**: T horas

## ¿Proceder?

Esperar aprobación del usuario.
```

---

## FASE 1-7: EJECUCIÓN (Solo después de aprobar Fase 0)

### 1. [ ] Eliminar "Stringly Typed" Code

**Objetivo**: Reemplazar `String` por `enum` cuando representen conjuntos finitos.

**Acción**:

```rust
// ❌ ANTES - Débil y propenso a errores
#[derive(Debug, Serialize, Deserialize)]
pub struct Contratista {
    pub estado: String,  // "ACTIVO", "INACTIVO", "SUSPENDIDO"
}

// ¿Qué pasa si alguien escribe "activo" en minúscula?
// ¿O "ACTIVE" en inglés?
// ¿O un typo como "ACITVO"?

// ✅ DESPUÉS - Fuerte y type-safe
/// Representa el ciclo de vida de un contratista en el sistema.
///
/// ## Estados Posibles
/// - `Activo`: Puede ingresar a las instalaciones
/// - `Inactivo`: Temporalmente deshabilitado (PRAIND vencido)
/// - `Suspendido`: Bloqueado por motivos administrativos o de seguridad
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EstadoContratista {
    /// Contratista habilitado para ingresar
    Activo,
    /// Certificaciones vencidas o documentación incompleta
    Inactivo,
    /// Bloqueado por decisión administrativa
    Suspendido,
}

impl EstadoContratista {
    /// Retorna el estado como string para persistencia.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Activo => "ACTIVO",
            Self::Inactivo => "INACTIVO",
            Self::Suspendido => "SUSPENDIDO",
        }
    }
}

impl std::str::FromStr for EstadoContratista {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ACTIVO" => Ok(Self::Activo),
            "INACTIVO" => Ok(Self::Inactivo),
            "SUSPENDIDO" => Ok(Self::Suspendido),
            _ => Err(format!("Estado inválido: {}", s)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contratista {
    pub estado: EstadoContratista,  // ✅ Type-safe, imposible tener valores inválidos
}
```

**Beneficios**:

- ✅ Imposible tener valores inválidos en tiempo de compilación
- ✅ IDE autocompletado
- ✅ Pattern matching exhaustivo
- ✅ Refactoring seguro

**Campos candidatos a enum**:

- Estados (activo/inactivo)
- Tipos (categorías finitas)
- Prioridades (alta/media/baja)
- Roles (admin/usuario/invitado)

---

### 2. [ ] Extraer Lógica de Negocio

**Objetivo**: Models no debe tener validaciones ni cálculos.

**Acción**:

```rust
// ❌ ANTES - Lógica en el modelo
#[derive(Debug, Serialize, Deserialize)]
pub struct Contratista {
    pub cedula: String,
    pub praind_vencido: bool,
}

impl Contratista {
    /// ❌ Esto es lógica de negocio, NO pertenece al modelo
    pub fn validar_cedula(&self) -> Result<(), String> {
        if self.cedula.is_empty() {
            return Err("Cédula vacía".to_string());
        }
        // ... más validaciones
        Ok(())
    }

    /// ❌ Esto es cálculo de negocio
    pub fn calcular_dias_vencimiento(&self) -> i64 {
        // ... lógica de cálculo
    }
}

// ✅ DESPUÉS - Modelo limpio
#[derive(Debug, Serialize, Deserialize)]
pub struct Contratista {
    pub cedula: String,
    pub fecha_vencimiento_praind: String,
}

// ✅ Lógica movida a domain/contratista.rs
pub fn validar_cedula(cedula: &str) -> Result<(), DomainError> {
    // ... validaciones
}

pub fn calcular_dias_vencimiento(fecha_vencimiento: &str) -> Result<i64, DomainError> {
    // ... cálculo
}
```

**Regla de oro**: Si un método tiene `if`, `match`, o cálculos, va a `domain/`.

---

### 3. [ ] Refactorizar Nulabilidad

**Objetivo**: Usar `Option<T>` explícitamente, eliminar valores centinela.

**Acción**:

```rust
// ❌ ANTES - Valores centinela ambiguos
#[derive(Debug, Serialize, Deserialize)]
pub struct Contratista {
    pub segundo_nombre: String,  // "" significa "no tiene"
    pub telefono: String,         // "" significa "no proporcionado"
    pub edad: i32,                // 0 significa "desconocido"
}

// Problema: ¿"" es válido o es null? ¿Alguien realmente se llama ""?
// Problema: ¿0 años es un bebé o es "no proporcionado"?

// ✅ DESPUÉS - Option explícito
/// Información de un contratista.
///
/// ## Campos Opcionales
/// - `segundo_nombre`: Null si el contratista no tiene segundo nombre
/// - `telefono`: Null si no proporcionó número de contacto
/// - `edad`: Null si no se registró en el sistema
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contratista {
    /// Primer nombre (obligatorio)
    pub nombre: String,

    /// Segundo nombre (opcional, null si no tiene)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_nombre: Option<String>,

    /// Teléfono de contacto (opcional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telefono: Option<String>,

    /// Edad (opcional, null si no se registró)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edad: Option<u8>,
}
```

**Criterio**:

- Campo puede no existir legítimamente → `Option<T>`
- Campo siempre debe tener valor → `T` sin Option

---

### 4. [ ] Value Objects (Opcional - Solo si Aporta Valor)

**Objetivo**: Encapsular validaciones en tipos específicos.

**Cuándo usar**:

- ✅ Campo con reglas de validación complejas
- ✅ Campo que se usa en muchos lugares
- ✅ Campo crítico para la seguridad

**Cuándo NO usar**:

- ❌ Campos simples sin validaciones
- ❌ Over-engineering (no todo necesita ser un tipo)

**Ejemplo**:

```rust
// ✅ Value Object para Email
/// Dirección de email validada según RFC 5322.
///
/// Este tipo garantiza que el email tiene formato válido en tiempo de construcción.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Email(String);

impl Email {
    /// Crea un Email validado.
    ///
    /// # Errores
    /// * `DomainError::EmailInvalido` - Si no cumple RFC 5322
    pub fn new(email: String) -> Result<Self, DomainError> {
        // La validación ocurre en domain/
        crate::domain::validar_email(&email)?;
        Ok(Self(email))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Email {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.0
    }
}
```

**Nota**: Value Objects agregan complejidad. Úsalos solo cuando el beneficio sea claro.

---

### 5. [ ] Documentación de Models

**Objetivo**: Explicar el propósito de negocio de cada estructura.

**Encabezado del Módulo**:

```rust
//! # Models: {Módulo}
//!
//! Estructuras de datos para el dominio de {módulo}.
//!
//! ## Entidades Principales
//! - `{Entidad}`: Representación completa de {concepto de negocio}
//! - `{Entidad}DTO`: Versión simplificada para transferencia
//! - `{Entidad}CreateDTO`: Input para creación
//! - `{Entidad}UpdateDTO`: Input para actualización
//!
//! ## Convenciones de Fechas
//! - Campos `*_at` (created_at, updated_at): Timestamps en RFC 3339
//! - Campos `fecha_vencimiento_*`: Fechas simples en YYYY-MM-DD
//! - La validación de formatos ocurre en `domain/{modulo}`
//!
//! ## Enums de Estado
//! Los enums como `EstadoContratista` usan `SCREAMING_SNAKE_CASE` para serialización
//! y son compatibles con SurrealDB.

use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

// --------------------------------------------------------------------------
// ENUMS Y TIPOS
// --------------------------------------------------------------------------

// --------------------------------------------------------------------------
// ENTIDADES PRINCIPALES
// --------------------------------------------------------------------------

// --------------------------------------------------------------------------
// DTOs DE ENTRADA (Commands)
// --------------------------------------------------------------------------

// --------------------------------------------------------------------------
// DTOs DE SALIDA (Responses)
// --------------------------------------------------------------------------

// --------------------------------------------------------------------------
// CONVERSIONES (From/Into)
// --------------------------------------------------------------------------
```

**Structs y Enums**:

```rust
/// Representa un contratista registrado en el sistema.
///
/// ## Ciclo de Vida
/// 1. Creado con estado `Activo` y PRAIND válido
/// 2. Cambia a `Inactivo` si PRAIND vence
/// 3. Puede ser `Suspendido` por decisión administrativa
///
/// ## Relaciones
/// - Pertenece a una `Empresa` (campo `empresa`)
/// - Puede tener `Vehículos` asociados
/// - Puede estar en `ListaNegra`
///
/// ## Campos Críticos para Seguridad
/// - `cedula`: Identificador único, validado en `domain::`
/// - `fecha_vencimiento_praind`: Determina si puede ingresar
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contratista {
    /// ID único en SurrealDB
    pub id: RecordId,

    /// Cédula costarricense (formato: X-XXXX-XXXX)
    pub cedula: String,

    /// Primer nombre del contratista
    pub nombre: String,

    /// Fecha de vencimiento de certificación PRAIND (formato: YYYY-MM-DD)
    pub fecha_vencimiento_praind: String,

    /// Estado actual del contratista
    pub estado: EstadoContratista,

    /// Relación con empresa empleadora
    pub empresa: RecordId,
}
```

**Calidad de documentación**:

- [ ] Explicar propósito de negocio
- [ ] Documentar campos críticos
- [ ] Explicar relaciones con otras entidades
- [ ] Formatos de fecha documentados
- [ ] Idioma español

---

### 6. [ ] Organización con Separadores

**Objetivo**: Código navegable fácilmente.

**Estructura estándar**:

```rust
//! Documentación del módulo

use statements...

// --------------------------------------------------------------------------
// ENUMS Y TIPOS AUXILIARES
// --------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EstadoContratista {
    // ...
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TipoVehiculo {
    // ...
}

// --------------------------------------------------------------------------
// ENTIDADES PRINCIPALES
// --------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contratista {
    // ...
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Empresa {
    // ...
}

// --------------------------------------------------------------------------
// DTOs DE ENTRADA (Commands)
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct CreateContratistaCommand {
    // ...
}

#[derive(Debug, Deserialize)]
pub struct UpdateContratistaCommand {
    // ...
}

// --------------------------------------------------------------------------
// DTOs DE SALIDA (Responses)
// --------------------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct ContratistaResponse {
    // ...
}

// --------------------------------------------------------------------------
// CONVERSIONES Y UTILIDADES
// --------------------------------------------------------------------------

impl From<Contratista> for ContratistaResponse {
    fn from(c: Contratista) -> Self {
        // ...
    }
}
```

---

### 7. [ ] Verificación Final

**Checklist de Pureza de Models**:

- [ ] Sin lógica de negocio (validaciones, cálculos)
- [ ] Sin acceso a DB o servicios
- [ ] Enums en lugar de Strings para conjuntos finitos
- [ ] `Option<T>` en lugar de valores centinela
- [ ] Todas las structs/enums públicas documentadas
- [ ] Separadores visuales entre secciones
- [ ] Sin código comentado
- [ ] Formatos de fecha documentados

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
refactor(models): aplicar Type-Driven Design en {modulo}

- Crear enums para eliminar "stringly typed" code
- Mover lógica de negocio a domain/{modulo}
- Refactorizar nulabilidad con Option<T> explícito
- Documentar en español según estándares de Brisas APP
- Organizar con separadores visuales
- Eliminar código comentado

Closes #{numero_issue}
```

---

## Ejemplo de Transformación Completa

**ANTES** (Débil):

```rust
pub struct Cita {
    pub estado: String,               // "PROGRAMADA", "CANCELADA"
    pub prioridad: String,            // "ALTA", "MEDIA", "BAJA"
    pub paciente_nombre: String,      // "" significa no tiene
    pub telefono: String,             // "" significa no proporcionó
}

impl Cita {
    pub fn esta_activa(&self) -> bool {
        self.estado == "PROGRAMADA" || self.estado == "EN_CURSO"
    }
}
```

**DESPUÉS** (Fuerte):

```rust
/// Estado del ciclo de vida de una cita médica.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EstadoCita {
    Programada,
    EnCurso,
    Completada,
    Cancelada,
}

/// Nivel de urgencia de la cita.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PrioridadCita {
    Alta,
    Media,
    Baja,
}

/// Representa una cita médica en el sistema.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cita {
    pub estado: EstadoCita,
    pub prioridad: PrioridadCita,
    pub paciente_nombre: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telefono: Option<String>,
}

// Lógica movida a domain/cita.rs
pub fn esta_activa(estado: EstadoCita) -> bool {
    matches!(estado, EstadoCita::Programada | EstadoCita::EnCurso)
}
```

---

**Fin del Workflow C - Models**
