# Workflow B: Auditoría y Refactorización de Dominio

**Versión**: 3.0  
**Idioma**: Español  
**Aplicación**: Brisas APP  

---

## Objetivo

Garantizar que la capa de dominio contenga **únicamente lógica pura de negocio**, sin dependencias de servicios, base de datos o estructuras de datos (DTOs).

---

## Principios Fundamentales

1. **Lógica Pura**: El dominio NO debe contener structs de datos (van en `models/`), ni llamadas a servicios o BD
2. **Sin Efectos Secundarios**: Las funciones deben ser determinísticas (misma entrada = misma salida)
3. **Validaciones y Reglas**: El dominio define QUÉ reglas aplican, no CÓMO se persisten
4. **Testing Obligatorio**: Toda función de dominio debe tener al menos un test unitario
5. **Documentación Estándar**: Seguir estándares de Brisas APP para docstrings

---

## FASE 0: ANÁLISIS PREVIO (⚠️ OBLIGATORIO)

### [ ] 0.1 Auditoría de Pureza del Dominio

**Plantilla de análisis**:

```markdown
**Archivo analizado**: `src/domain/{modulo}.rs`
**LOC**: {número de líneas}

## ❌ VIOLACIONES DE DOMINIO PURO

### Structs de Datos (deben ir a models/)
- [ ] Línea XX: `struct {Nombre}` → Mover a `models/{modulo}.rs`
- [ ] Línea YY: `struct {Otro}` → Mover a `models/{modulo}.rs`

### Imports Impuros (eliminar)
- [ ] `use crate::services::`
- [ ] `use crate::db::`
- [ ] `use surrealdb::`
- [ ] `use tauri::`

### Efectos Secundarios (refactorizar)
- [ ] Línea XX: Acceso a DB → Debe estar en servicio
- [ ] Línea YY: Logging excesivo → Solo para errores críticos
- [ ] Línea ZZ: Mutación de estado global → Eliminar

### Código Obsoleto
- [ ] N bloques de código comentado → Eliminar
- [ ] M `TODO` obsoletos → Resolver o eliminar

## ✅ VALIDACIONES Y REGLAS (Correcto - mantener)

| Función | Tipo | Estado |
|---------|------|--------|
| `validar_cedula()` | Validator | ✅ Pura |
| `normalizar_placa()` | Normalizer | ✅ Pura |
| `calcular_descuento()` | Business Rule | ✅ Pura |

## 📚 DOCUMENTACIÓN

| Elemento | Estado | Acción |
|----------|--------|--------|
| Encabezado módulo | ✅/❌ | Agregar si falta |
| Funciones públicas | X/Y (Z%) | Documentar faltantes |
| Idioma español | ✅/❌ | Traducir si necesario |

## 🧪 TESTING

| Función | Test Presente | Cobertura | Acción |
|---------|---------------|-----------|--------|
| `validar_*()` | ❌ | 0% | Crear test |
| `normalizar_*()` | ✅ | 100% | - |
| `calcular_*()` | ❌ | 0% | Crear test |

**Cobertura total estimada**: X%
```

### [ ] 0.2 Auditoría de Constantes y Valores Mágicos

```markdown
## VALORES MÁGICOS DETECTADOS

| Línea | Código | Acción |
|-------|--------|--------|
| XX | `if valor > 100` | Crear constante `MAX_VALOR = 100` |
| YY | `"ACTIVO"` literal | Crear constante `ESTADO_ACTIVO` |
| ZZ | `0.16` (IVA) | Crear constante `TASA_IVA = 0.16` |
```

### [ ] 0.3 Auditoría de Errores (thiserror)

```markdown
## MANEJO DE ERRORES

### Estado Actual
- [ ] Usa `#[derive(thiserror::Error)]`: Sí/No
- [ ] Mensajes descriptivos: Sí/No
- [ ] Idioma español: Sí/No

### Errores a Definir/Mejorar
| Error | Estado | Acción |
|-------|--------|--------|
| `{Modulo}Error::CampoVacio` | ❌ Ausente | Crear |
| `{Modulo}Error::FormatoInvalido` | ✅ Presente | Mejorar mensaje |
```

### [ ] 0.4 Conformidad con Estándares de Fechas

```markdown
## VALIDACIONES DE FECHAS

| Función | Formato Esperado | Usa `common::`? | Acción |
|---------|------------------|-----------------|--------|
| `validar_fecha_ingreso()` | RFC 3339 | ❌ | Usar `common::validar_fecha_rfc3339()` |
| `validar_fecha_vencimiento()` | YYYY-MM-DD | ❌ | Usar `common::validar_fecha_simple()` |
```

---

### 📋 Reporte Final de Análisis

```markdown
# Reporte de Análisis FASE 0 - Dominio

**Archivo**: src/domain/{modulo}.rs
**LOC**: {número}

## PROBLEMAS CRÍTICOS
1. [CRÍTICO] N structs de datos en dominio → Mover a models/
2. [CRÍTICO] Imports impuros: `use crate::db::` → Eliminar

## PROBLEMAS MAYORES
3. [ALTO] M funciones sin tests → Crear tests unitarios
4. [MEDIO] K funciones sin documentar → Agregar docstrings

## MEJORAS RECOMENDADAS
5. [BAJO] P valores mágicos → Crear constantes
6. [BAJO] Q errores sin thiserror → Refactorizar

## ESTIMACIÓN
- Separación dominio/models: X horas
- Tests unitarios: Y horas
- Documentación: Z horas
- **TOTAL**: T horas

## ¿Proceder?
Esperar aprobación del usuario.
```

---

## FASE 1-9: EJECUCIÓN (Solo después de aprobar Fase 0)

### 1. [ ] Separación Dominio/Models

**Objetivo**: Mover structs de datos al lugar correcto.

**Acción**:

```rust
// ❌ ANTES - src/domain/contratista.rs
pub struct ContratistaData {
    pub id: String,
    pub cedula: String,
    // ... campos de datos
}

// ✅ DESPUÉS - src/models/contratista.rs
pub struct Contratista {
    pub id: String,
    pub cedula: String,
    // ... campos de datos
}

// ✅ DOMINIO LIMPIO - src/domain/contratista.rs
// Solo validaciones y reglas, sin structs
```

**Proceso**:
1. Identificar structs de datos en dominio
2. Moverlas a `models/{modulo}.rs` o crear archivo si no existe
3. Actualizar imports en servicios y otros módulos
4. Verificar compilación: `cargo check --package mega-brisas`

---

### 2. [ ] Eliminar Dependencias Impuras

**Objetivo**: Limpiar imports que violan pureza del dominio.

**Acción**:

```rust
// ❌ ELIMINAR
use crate::services::*;
use crate::db::*;
use surrealdb::*;
use tauri::*;

// ✅ PERMITIDO
use crate::domain::errors::*;
use crate::common::{validar_fecha_rfc3339, validar_fecha_simple};
use chrono::NaiveDate;
use regex::Regex;
```

**Criterio**:
- [ ] Sin imports de capas superiores (servicios, comandos)
- [ ] Sin imports de infraestructura (DB, Tauri)
- [ ] Solo lógica pura y tipos básicos

---

### 3. [ ] Extraer Constantes y Eliminar Valores Mágicos

**Objetivo**: Hacer el código auto-documentado y mantenible.

**Acción**:

```rust
// ❌ ANTES - Valores mágicos
pub fn validar_edad(edad: u8) -> Result<(), ValidationError> {
    if edad < 18 {
        return Err(ValidationError::EdadInsuficiente);
    }
    if edad > 120 {
        return Err(ValidationError::EdadInvalida);
    }
    Ok(())
}

// ✅ DESPUÉS - Constantes nombradas
/// Edad mínima legal para contratación en Costa Rica
pub const EDAD_MINIMA_LEGAL: u8 = 18;

/// Edad máxima razonable (límite de validación)
pub const EDAD_MAXIMA_VALIDA: u8 = 120;

/// Longitud exacta de cédula costarricense (formato: X-XXXX-XXXX)
pub const LONGITUD_CEDULA_CR: usize = 9;

pub fn validar_edad(edad: u8) -> Result<(), ValidationError> {
    if edad < EDAD_MINIMA_LEGAL {
        return Err(ValidationError::EdadInsuficiente);
    }
    if edad > EDAD_MAXIMA_VALIDA {
        return Err(ValidationError::EdadInvalida);
    }
    Ok(())
}
```

**Tipos de constantes a crear**:
- Límites numéricos (MIN, MAX)
- Formatos y patrones (REGEX)
- Estados y valores de enumeración como strings (ESTADO_ACTIVO)
- Tasas y porcentajes (TASA_IVA)

---

### 4. [ ] Implementar Manejo de Errores con thiserror

**Objetivo**: Errores descriptivos y tipados.

**Acción**:

```rust
// src/domain/errors.rs o src/domain/{modulo}.rs

use thiserror::Error;

/// Errores específicos del dominio de {Módulo}.
#[derive(Debug, Error)]
pub enum {Modulo}Error {
    /// La cédula proporcionada está vacía o no cumple el formato costarricense.
    #[error("Cédula inválida: {0}")]
    CedulaInvalida(String),
    
    /// El campo obligatorio '{campo}' está vacío.
    #[error("El campo '{campo}' es obligatorio y no puede estar vacío")]
    CampoVacio { campo: String },
    
    /// La fecha no cumple el formato esperado (RFC 3339 o YYYY-MM-DD).
    #[error("Formato de fecha inválido: {0}")]
    FormatoFechaInvalido(String),
    
    /// Violación de regla de negocio: {motivo}.
    #[error("Regla de negocio violada: {motivo}")]
    ReglaNegocioViolada { motivo: String },
}
```

**Criterio de calidad**:
- [ ] Todos los errores tienen `#[error(...)]` descriptivo
- [ ] Mensajes en español
- [ ] Contexto suficiente (qué falló, por qué)
- [ ] Sin errores genéricos (`Error::Other`)

---

### 5. [ ] Aplicar Estándares de Fechas

**Objetivo**: Usar funciones centralizadas de `common.rs`.

**Acción**:

```rust
// ❌ ANTES - Validación manual de fechas
pub fn validar_fecha_ingreso(fecha: &str) -> Result<NaiveDate, DomainError> {
    let parsed = NaiveDate::parse_from_str(fecha, "%Y-%m-%d")
        .map_err(|_| DomainError::FormatoInvalido)?;
    Ok(parsed)
}

// ✅ DESPUÉS - Usar funciones de common.rs
use crate::common::{validar_fecha_rfc3339, validar_fecha_simple, parsear_fecha_simple};

/// Valida que la fecha de ingreso cumpla formato RFC 3339.
///
/// # Formato Esperado
/// RFC 3339: "2026-01-15T08:30:00Z"
///
/// # Errores
/// * `DomainError::FormatoFechaInvalido` - Si no cumple el formato
pub fn validar_fecha_hora_ingreso(fecha: &str) -> Result<(), DomainError> {
    validar_fecha_rfc3339(fecha)
        .map_err(|e| DomainError::FormatoFechaInvalido(e.to_string()))
}

/// Valida que la fecha de vencimiento cumpla formato YYYY-MM-DD.
///
/// # Formato Esperado
/// YYYY-MM-DD: "2026-12-31"
///
/// # Errores
/// * `DomainError::FormatoFechaInvalido` - Si no cumple el formato
pub fn validar_fecha_vencimiento(fecha: &str) -> Result<NaiveDate, DomainError> {
    parsear_fecha_simple(fecha)
        .map_err(|e| DomainError::FormatoFechaInvalido(e.to_string()))
}
```

**Reglas de aplicación**:
- **Timestamps con hora** → `validar_fecha_rfc3339()`
- **Fechas simples** → `validar_fecha_simple()` / `parsear_fecha_simple()`
- **Cálculos de tiempo** → `calcular_tiempo_permanencia()`

---

### 6. [ ] Documentación Estándar

**Objetivo**: Documentar en español según estándares de Brisas APP.

**Encabezado del Módulo**:

```rust
//! # Dominio: {Módulo}
//!
//! Contiene las reglas de negocio puras y validaciones para {dominio}.
//!
//! ## Responsabilidades
//! - Validar formatos de entrada (cédulas, placas, emails)
//! - Normalizar datos (mayúsculas, espacios, formatos)
//! - Aplicar reglas de negocio ({regla 1}, {regla 2})
//!
//! ## Principios
//! - **Sin efectos secundarios**: Todas las funciones son puras
//! - **Sin dependencias de infraestructura**: No accede a DB ni servicios
//! - **Testing obligatorio**: Cada función tiene al menos un test
//!
//! ## Estándares de Fechas
//! - Timestamps: RFC 3339 ("2026-01-15T08:30:00Z")
//! - Fechas simples: YYYY-MM-DD ("2026-12-31")
//!
//! Ver `common.rs` para funciones centralizadas de validación de fechas.

use crate::common::{validar_fecha_rfc3339, validar_fecha_simple};
use chrono::NaiveDate;
use regex::Regex;
use once_cell::sync::Lazy;

// --------------------------------------------------------------------------
// CONSTANTES
// --------------------------------------------------------------------------

// --------------------------------------------------------------------------
// VALIDACIONES
// --------------------------------------------------------------------------

// --------------------------------------------------------------------------
// NORMALIZACIONES
// --------------------------------------------------------------------------

// --------------------------------------------------------------------------
// REGLAS DE NEGOCIO
// --------------------------------------------------------------------------

// --------------------------------------------------------------------------
// TESTS UNITARIOS
// --------------------------------------------------------------------------
```

**Funciones Públicas**:

```rust
/// Valida el formato de una cédula costarricense.
///
/// La cédula debe cumplir el formato X-XXXX-XXXX (9 dígitos con guiones).
/// Esta validación es **crítica para la seguridad** del sistema ya que
/// la cédula es el identificador único de personas.
///
/// # Proceso
/// 1. Verifica que no esté vacía
/// 2. Valida formato con regex
/// 3. Verifica longitud correcta (9 dígitos)
///
/// # Argumentos
/// * `cedula` - Cédula en formato string (ej: "1-2345-6789")
///
/// # Retorno
/// * `Ok(())` - La cédula es válida
///
/// # Errores
/// * `DomainError::CedulaVacia` - La cédula está vacía
/// * `DomainError::CedulaFormatoInvalido` - No cumple el patrón X-XXXX-XXXX
///
/// # Ejemplo
/// ```rust
/// use brisas_app_lib::domain::contratista::validar_cedula;
///
/// assert!(validar_cedula("1-2345-6789").is_ok());
/// assert!(validar_cedula("123456789").is_err());
/// assert!(validar_cedula("").is_err());
/// ```
pub fn validar_cedula(cedula: &str) -> Result<(), DomainError> {
    if cedula.trim().is_empty() {
        return Err(DomainError::CedulaVacia);
    }
    
    static CEDULA_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^\d{1}-\d{4}-\d{4}$").unwrap()
    });
    
    if !CEDULA_REGEX.is_match(cedula) {
        return Err(DomainError::CedulaFormatoInvalido(
            cedula.to_string()
        ));
    }
    
    Ok(())
}
```

**Calidad de documentación**:
- [ ] Explicar el "por qué" (importancia de negocio)
- [ ] Describir el proceso paso a paso
- [ ] Ejemplos de uso con `assert!`
- [ ] Todos los errores posibles documentados
- [ ] Idioma español

---

### 7. [ ] Implementar Tests Unitarios

**Objetivo**: Cobertura >80% en lógica de dominio.

**Estructura de Tests**:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // --------------------------------------------------------------------------
    // TESTS DE VALIDACIÓN
    // --------------------------------------------------------------------------
    
    mod validar_cedula {
        use super::*;
        
        #[test]
        fn acepta_cedula_valida() {
            let resultado = validar_cedula("1-2345-6789");
            assert!(resultado.is_ok());
        }
        
        #[test]
        fn rechaza_cedula_vacia() {
            let resultado = validar_cedula("");
            assert!(resultado.is_err());
            assert!(matches!(resultado.unwrap_err(), DomainError::CedulaVacia));
        }
        
        #[test]
        fn rechaza_cedula_sin_guiones() {
            let resultado = validar_cedula("123456789");
            assert!(resultado.is_err());
            assert!(matches!(
                resultado.unwrap_err(),
                DomainError::CedulaFormatoInvalido(_)
            ));
        }
        
        #[test]
        fn rechaza_formato_incorrecto() {
            let casos = vec![
                "1-234-5678",     // Muy corto
                "12-3456-7890",   // Demasiados dígitos al inicio
                "A-2345-6789",    // Letra en lugar de número
            ];
            
            for caso in casos {
                let resultado = validar_cedula(caso);
                assert!(resultado.is_err(), "Debería rechazar: {}", caso);
            }
        }
    }
    
    // --------------------------------------------------------------------------
    // TESTS DE NORMALIZACIÓN
    // --------------------------------------------------------------------------
    
    mod normalizar_cedula {
        use super::*;
        
        #[test]
        fn elimina_espacios() {
            let resultado = normalizar_cedula(" 1-2345-6789 ");
            assert_eq!(resultado, "1-2345-6789");
        }
    }
    
    // --------------------------------------------------------------------------
    // TESTS DE REGLAS DE NEGOCIO
    // --------------------------------------------------------------------------
    
    mod reglas_negocio {
        use super::*;
        
        #[test]
        fn test_regla_especifica() {
            // Test de lógica de negocio compleja
        }
    }
    
    // --------------------------------------------------------------------------
    // TESTS DE FECHAS
    // --------------------------------------------------------------------------
    
    mod validaciones_fechas {
        use super::*;
        
        #[test]
        fn acepta_fecha_rfc3339_valida() {
            let resultado = validar_fecha_hora_ingreso("2026-01-15T08:30:00Z");
            assert!(resultado.is_ok());
        }
        
        #[test]
        fn rechaza_fecha_formato_incorrecto() {
            let resultado = validar_fecha_hora_ingreso("15/01/2026");
            assert!(resultado.is_err());
        }
        
        #[test]
        fn acepta_fecha_simple_valida() {
            let resultado = validar_fecha_vencimiento("2026-12-31");
            assert!(resultado.is_ok());
        }
    }
}
```

**Criterios de cobertura**:
- [ ] Casos válidos (happy path)
- [ ] Casos inválidos (errores esperados)
- [ ] Casos límite (edge cases)
- [ ] Casos de formato incorrecto
- [ ] Al menos 1 test por error posible

**Ejecutar tests**:
```bash
# Tests del módulo específico
cargo test --package mega-brisas -- domain::{modulo}

# Con output detallado
cargo test --package mega-brisas -- domain::{modulo} --nocapture
```

---

### 8. [ ] Verificación Final

**Checklist de Dominio Puro**:

- [ ] Sin structs de datos (movidos a `models/`)
- [ ] Sin imports de `services/`, `db/`, `commands/`
- [ ] Sin imports de `surrealdb`, `tauri` u otros crates de infraestructura
- [ ] Todas las funciones públicas tienen `///` documentación
- [ ] Tests unitarios para todas las validaciones
- [ ] Constantes nombradas (sin valores mágicos)
- [ ] Errores con `thiserror` y mensajes descriptivos
- [ ] Separadores visuales entre secciones
- [ ] Sin código comentado ni `TODO` obsoletos
- [ ] Usa funciones centralizadas de `common.rs` para fechas

**Compilación y Tests**:

```bash
# Verificar compilación
cargo check --package mega-brisas

# Ejecutar tests del dominio
cargo test --package mega-brisas -- domain

# Verificar warnings
cargo clippy --package mega-brisas -- -D warnings

# Formatear código
cargo fmt
```

---

## Plantilla de Commit

```
refactor(domain): purificar {modulo} según Clean Architecture

- Mover structs de datos a models/{modulo}
- Eliminar dependencias de infraestructura (DB, servicios)
- Extraer constantes para eliminar valores mágicos
- Implementar errores con thiserror en español
- Agregar tests unitarios con cobertura >80%
- Documentar en español según estándares de Brisas APP
- Aplicar convenciones de fechas (common.rs)
- Organizar con separadores visuales

Closes #{numero_issue}
```

---

**Fin del Workflow B - Dominio**