# Workflow G: Auditoría y Refactorización de Common/Utils

**Versión**: 3.0  
**Idioma**: Español  
**Aplicación**: Brisas APP  
**Paradigma**: Rust idiomático (funcional, no OOP)

---

## Objetivo

Garantizar que el módulo `common.rs` (o módulos de utilidades) contenga **funciones puras compartidas** sin dependencias de capas superiores, con testing exhaustivo y documentación clara.

---

## Principios Fundamentales

1. **Funciones Puras**: Sin efectos secundarios, determinísticas
2. **Zero Dependencies**: No importar servicios, DB, ni commands
3. **Composabilidad**: Funciones pequeñas que se combinan
4. **Testing Obligatorio**: 100% cobertura para utilidades críticas
5. **Lazy Statics**: Usar `once_cell::sync::Lazy` para regex, configuración estática

---

## FASE 0: ANÁLISIS PREVIO (⚠️ OBLIGATORIO)

### [ ] 0.1 Auditoría de Pureza

**Plantilla de análisis**:

```markdown
**Archivo analizado**: `src/common.rs`
**LOC**: {número de líneas}
**Número de funciones**: {N}

## ❌ VIOLACIONES DE PUREZA

### Funciones con Efectos Secundarios

- [ ] Línea XX: Función con I/O (lectura de archivos, logs)
- [ ] Línea YY: Función que muta estado global
- [ ] Línea ZZ: Función con dependencias externas (API calls)

### Imports Impuros (eliminar)

- [ ] `use crate::services::`
- [ ] `use crate::db::`
- [ ] `use crate::commands::`
- [ ] `use tauri::` (excepto para types si es necesario)

### Funciones que deberían estar en otro lugar

- [ ] Línea XX: Lógica de negocio específica → Mover a `domain::`
- [ ] Línea YY: Lógica de formateo UI → Mover a frontend
```

### [ ] 0.2 Auditoría de Categorización

```markdown
## CATEGORÍAS DE FUNCIONES

### Validaciones (deben ser puras)

| Función                   | Es Pura? | Tests? | Acción        |
| ------------------------- | -------- | ------ | ------------- |
| `validar_fecha_rfc3339()` | ✅       | ✅     | -             |
| `validar_cedula()`        | ⚠️       | ❌     | Agregar tests |

### Conversiones/Parsing

| Función                  | Es Pura? | Maneja Errores? | Tests? |
| ------------------------ | -------- | --------------- | ------ |
| `parsear_fecha_simple()` | ✅       | ✅              | ✅     |

### Formateo

| Función              | Es Pura? | Tests? |
| -------------------- | -------- | ------ |
| `formatear_cedula()` | ✅       | ❌     |

### Cálculos

| Función                         | Es Pura? | Tests? |
| ------------------------------- | -------- | ------ |
| `calcular_tiempo_permanencia()` | ✅       | ⚠️     |

### Constantes y Configuración

- [ ] ¿Usa `Lazy<Regex>` para regex?
- [ ] ¿Constantes en SCREAMING_SNAKE_CASE?
- [ ] ¿Valores mágicos extraídos?
```

### [ ] 0.3 Auditoría de Documentación

```markdown
## DOCUMENTACIÓN

| Función                   | Tiene `///`? | Explica "por qué"? | Ejemplos? | Tests docs? |
| ------------------------- | ------------ | ------------------ | --------- | ----------- |
| `validar_fecha_rfc3339()` | ✅           | ⚠️                 | ❌        | ❌          |

**Cobertura**: X/Y funciones documentadas (Z%)

### Elementos faltantes:

- [ ] Ejemplos de uso en docstrings
- [ ] Casos límite documentados
- [ ] Performance considerations (si aplica)
```

### [ ] 0.4 Auditoría de Testing

```markdown
## COBERTURA DE TESTS

### Por Categoría

| Categoría    | Funciones | Con Tests | Cobertura |
| ------------ | --------- | --------- | --------- |
| Validaciones | 5         | 3         | 60%       |
| Conversiones | 3         | 2         | 66%       |
| Formateo     | 2         | 0         | 0%        |
| Cálculos     | 4         | 2         | 50%       |

**Cobertura total estimada**: X%

### Funciones críticas sin tests

1. `validar_fecha_rfc3339()` - **CRÍTICO**: Usada en toda la app
2. `calcular_tiempo_permanencia()` - **ALTO**: Lógica de negocio
3. `normalizar_cedula()` - **MEDIO**: Datos sensibles
```

### [ ] 0.5 Auditoría de Performance

```markdown
## OPTIMIZACIONES

### Regex no compilados (mover a Lazy)

- [ ] Línea XX: Regex inline → Usar `Lazy<Regex>`

### Allocations innecesarias

- [ ] Línea YY: Clone innecesario → Usar referencias
- [ ] Línea ZZ: String concatenation en loop → Usar `format!` o `push_str`

### Oportunidades de const fn

- [ ] Función XX puede ser `const fn`
```

---

### 📋 Reporte Final de Análisis

```markdown
# Reporte de Análisis FASE 0 - Common/Utils

**Archivo**: src/common.rs
**LOC**: {número}
**Funciones**: {N}

## PROBLEMAS CRÍTICOS

1. [CRÍTICO] N funciones sin tests (especialmente validaciones)
2. [CRÍTICO] M regex inline sin Lazy (performance)

## PROBLEMAS MAYORES

3. [ALTO] K funciones con dependencias impuras
4. [MEDIO] P funciones sin documentar

## MEJORAS RECOMENDADAS

5. [BAJO] Q allocations innecesarias
6. [BAJO] R funciones que pueden ser const fn

## ESTIMACIÓN

- Tests: X horas
- Optimización regex: Y horas
- Documentación: Z horas
- **TOTAL**: T horas

## ¿Proceder?

Esperar aprobación del usuario.
```

---

## FASE 1-7: EJECUCIÓN (Solo después de aprobar Fase 0)

### 1. [ ] Organización y Estructura

**Objetivo**: Código navegable por categorías.

**Estructura estándar**:

```rust
//! # Common: Utilidades Compartidas
//!
//! Funciones puras y constantes usadas en múltiples módulos de la aplicación.
//!
//! ## Categorías
//! - **Validaciones**: Verificación de formatos (fechas, cédulas, emails)
//! - **Conversiones**: Parsing y transformación de tipos
//! - **Formateo**: Normalización de strings para persistencia
//! - **Cálculos**: Operaciones matemáticas y de tiempo
//! - **Constantes**: Valores de configuración inmutables
//!
//! ## Principios
//! - **Funciones puras**: Sin efectos secundarios
//! - **Sin dependencias**: No importa servicios, DB, ni commands
//! - **Testing obligatorio**: 100% cobertura en funciones críticas
//!
//! ## Estándares de Fechas
//! - **RFC 3339** (timestamps): "2026-01-15T08:30:00Z"
//! - **YYYY-MM-DD** (fechas simples): "2026-12-31"

use chrono::{DateTime, NaiveDate, Utc};
use once_cell::sync::Lazy;
use regex::Regex;

// --------------------------------------------------------------------------
// CONSTANTES DE CONFIGURACIÓN
// --------------------------------------------------------------------------

/// Patrón regex para validar cédulas costarricenses (formato: X-XXXX-XXXX)
static CEDULA_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\d{1}-\d{4}-\d{4}$")
        .expect("CEDULA_REGEX es un patrón válido")
});

/// Patrón regex para validar formato RFC 3339
static RFC3339_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$")
        .expect("RFC3339_REGEX es un patrón válido")
});

// --------------------------------------------------------------------------
// VALIDACIONES DE FORMATO
// --------------------------------------------------------------------------

// --------------------------------------------------------------------------
// CONVERSIONES Y PARSING
// --------------------------------------------------------------------------

// --------------------------------------------------------------------------
// FORMATEO Y NORMALIZACIÓN
// --------------------------------------------------------------------------

// --------------------------------------------------------------------------
// CÁLCULOS Y TRANSFORMACIONES
// --------------------------------------------------------------------------

// --------------------------------------------------------------------------
// TESTS UNITARIOS
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    mod validaciones { /* ... */ }
    mod conversiones { /* ... */ }
    mod formateo { /* ... */ }
    mod calculos { /* ... */ }
}
```

---

### 2. [ ] Optimización de Regex con Lazy

**Objetivo**: Evitar recompilar regex en cada llamada.

**Acción**:

````rust
// ❌ ANTES - Regex compilado en cada llamada
pub fn validar_cedula(cedula: &str) -> Result<(), ValidationError> {
    let regex = Regex::new(r"^\d{1}-\d{4}-\d{4}$").unwrap();  // ❌ Compilado cada vez
    if !regex.is_match(cedula) {
        return Err(ValidationError::FormatoInvalido);
    }
    Ok(())
}

// ✅ DESPUÉS - Regex compilado una sola vez
use once_cell::sync::Lazy;
use regex::Regex;

/// Patrón regex para cédulas costarricenses.
///
/// Este regex se compila una sola vez al inicio de la aplicación
/// y se reutiliza en todas las validaciones subsecuentes.
///
/// Formato: X-XXXX-XXXX (1 dígito, 4 dígitos, 4 dígitos separados por guiones)
static CEDULA_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\d{1}-\d{4}-\d{4}$")
        .expect("CEDULA_REGEX es un patrón válido")
});

/// Valida el formato de una cédula costarricense.
///
/// ## Formato Esperado
/// - X-XXXX-XXXX (ej: "1-2345-6789")
/// - 1 dígito, 4 dígitos, 4 dígitos separados por guiones
///
/// ## Ejemplos
/// ```rust
/// use brisas_app_lib::common::validar_cedula;
///
/// assert!(validar_cedula("1-2345-6789").is_ok());
/// assert!(validar_cedula("123456789").is_err());  // Sin guiones
/// assert!(validar_cedula("12-345-6789").is_err()); // Formato incorrecto
/// ```
///
/// ## Performance
/// El regex se compila una sola vez usando `Lazy<Regex>`.
///
/// ## Errores
/// * `ValidationError::FormatoInvalido` - No cumple el patrón X-XXXX-XXXX
pub fn validar_cedula(cedula: &str) -> Result<(), ValidationError> {
    if !CEDULA_REGEX.is_match(cedula) {
        return Err(ValidationError::FormatoInvalido(
            "Cédula debe tener formato X-XXXX-XXXX".to_string()
        ));
    }
    Ok(())
}
````

**Beneficio**: Mejora performance ~100x en validaciones repetidas.

---

### 3. [ ] Validaciones de Fechas (Estándar Brisas APP)

**Objetivo**: Funciones centralizadas para todos los formatos de fecha.

**Implementación**:

````rust
use chrono::{DateTime, NaiveDate, Utc};

// --------------------------------------------------------------------------
// VALIDACIONES DE FECHAS
// --------------------------------------------------------------------------

/// Error de validación de fechas.
#[derive(Debug, thiserror::Error)]
pub enum DateValidationError {
    /// Formato de fecha inválido
    #[error("Formato de fecha inválido: {0}")]
    FormatoInvalido(String),

    /// Fecha de fin anterior a fecha de inicio
    #[error("La fecha de fin debe ser posterior a la fecha de inicio")]
    RangoInvalido,
}

/// Valida que una fecha cumpla el formato RFC 3339.
///
/// ## Formato Esperado
/// RFC 3339: "YYYY-MM-DDThh:mm:ssZ"
///
/// ## Uso en Brisas APP
/// Este formato se usa para timestamps con hora exacta:
/// - `fecha_hora_ingreso`
/// - `fecha_hora_salida`
/// - `created_at`, `updated_at`
///
/// ## Ejemplos
/// ```rust
/// use brisas_app_lib::common::validar_fecha_rfc3339;
///
/// // ✅ Válido
/// assert!(validar_fecha_rfc3339("2026-01-15T08:30:00Z").is_ok());
///
/// // ❌ Inválido
/// assert!(validar_fecha_rfc3339("2026-01-15").is_err());          // Sin hora
/// assert!(validar_fecha_rfc3339("15/01/2026 08:30").is_err());    // Formato local
/// assert!(validar_fecha_rfc3339("2026-13-01T00:00:00Z").is_err()); // Mes inválido
/// ```
///
/// ## Performance
/// Usa `Lazy<Regex>` para compilar el patrón una sola vez.
///
/// ## Errores
/// * `DateValidationError::FormatoInvalido` - No cumple RFC 3339
pub fn validar_fecha_rfc3339(fecha: &str) -> Result<(), DateValidationError> {
    // Validar formato con regex
    if !RFC3339_REGEX.is_match(fecha) {
        return Err(DateValidationError::FormatoInvalido(
            "Fecha debe estar en formato RFC 3339 (YYYY-MM-DDThh:mm:ssZ)".to_string()
        ));
    }

    // Validar que sea parseable (verifica fechas imposibles como 2026-13-01)
    DateTime::parse_from_rfc3339(fecha)
        .map_err(|e| DateValidationError::FormatoInvalido(
            format!("Fecha RFC 3339 inválida: {}", e)
        ))?;

    Ok(())
}

/// Valida que una fecha cumpla el formato YYYY-MM-DD.
///
/// ## Formato Esperado
/// YYYY-MM-DD: "2026-12-31"
///
/// ## Uso en Brisas APP
/// Este formato se usa para fechas simples sin hora:
/// - `fecha_vencimiento_praind`
/// - `fecha_nacimiento`
/// - Reportes por día
///
/// ## Ejemplos
/// ```rust
/// use brisas_app_lib::common::validar_fecha_simple;
///
/// // ✅ Válido
/// assert!(validar_fecha_simple("2026-12-31").is_ok());
///
/// // ❌ Inválido
/// assert!(validar_fecha_simple("31/12/2026").is_err());     // Formato local
/// assert!(validar_fecha_simple("2026-13-01").is_err());     // Mes inválido
/// assert!(validar_fecha_simple("2026-02-30").is_err());     // Día inválido
/// ```
///
/// ## Errores
/// * `DateValidationError::FormatoInvalido` - No cumple YYYY-MM-DD
pub fn validar_fecha_simple(fecha: &str) -> Result<(), DateValidationError> {
    NaiveDate::parse_from_str(fecha, "%Y-%m-%d")
        .map_err(|e| DateValidationError::FormatoInvalido(
            format!("Fecha debe estar en formato YYYY-MM-DD: {}", e)
        ))?;

    Ok(())
}

/// Parsea una fecha en formato YYYY-MM-DD a `NaiveDate`.
///
/// ## Diferencia con `validar_fecha_simple`
/// - `validar_fecha_simple()`: Solo valida (retorna `()`)
/// - `parsear_fecha_simple()`: Valida Y retorna el objeto `NaiveDate`
///
/// ## Uso
/// Usar cuando necesites el objeto `NaiveDate` para cálculos.
///
/// ## Ejemplos
/// ```rust
/// use brisas_app_lib::common::parsear_fecha_simple;
///
/// let fecha = parsear_fecha_simple("2026-12-31").unwrap();
/// assert_eq!(fecha.year(), 2026);
/// assert_eq!(fecha.month(), 12);
/// assert_eq!(fecha.day(), 31);
/// ```
///
/// ## Errores
/// * `DateValidationError::FormatoInvalido` - No cumple YYYY-MM-DD
pub fn parsear_fecha_simple(fecha: &str) -> Result<NaiveDate, DateValidationError> {
    NaiveDate::parse_from_str(fecha, "%Y-%m-%d")
        .map_err(|e| DateValidationError::FormatoInvalido(
            format!("Error al parsear fecha: {}", e)
        ))
}

/// Valida que la fecha de salida sea posterior a la de ingreso.
///
/// ## Uso en Brisas APP
/// Previene registros de salida antes del ingreso (físicamente imposible).
///
/// ## Parámetros
/// * `fecha_ingreso` - Fecha/hora de ingreso (RFC 3339)
/// * `fecha_salida` - Fecha/hora de salida (RFC 3339)
///
/// ## Ejemplos
/// ```rust
/// use brisas_app_lib::common::validar_tiempo_salida;
///
/// // ✅ Válido (salida después de ingreso)
/// assert!(validar_tiempo_salida(
///     "2026-01-15T08:00:00Z",
///     "2026-01-15T17:00:00Z"
/// ).is_ok());
///
/// // ❌ Inválido (salida antes de ingreso)
/// assert!(validar_tiempo_salida(
///     "2026-01-15T17:00:00Z",
///     "2026-01-15T08:00:00Z"
/// ).is_err());
/// ```
///
/// ## Errores
/// * `DateValidationError::RangoInvalido` - Salida anterior a ingreso
/// * `DateValidationError::FormatoInvalido` - Fechas no son RFC 3339 válidas
pub fn validar_tiempo_salida(
    fecha_ingreso: &str,
    fecha_salida: &str,
) -> Result<(), DateValidationError> {
    let ingreso = DateTime::parse_from_rfc3339(fecha_ingreso)
        .map_err(|e| DateValidationError::FormatoInvalido(
            format!("Fecha de ingreso inválida: {}", e)
        ))?;

    let salida = DateTime::parse_from_rfc3339(fecha_salida)
        .map_err(|e| DateValidationError::FormatoInvalido(
            format!("Fecha de salida inválida: {}", e)
        ))?;

    if salida <= ingreso {
        return Err(DateValidationError::RangoInvalido);
    }

    Ok(())
}

/// Calcula el tiempo de permanencia en minutos entre dos timestamps.
///
/// ## Uso en Brisas APP
/// Calcula cuánto tiempo estuvo una persona en las instalaciones.
///
/// ## Parámetros
/// * `fecha_ingreso` - Fecha/hora de ingreso (RFC 3339)
/// * `fecha_salida` - Fecha/hora de salida (RFC 3339)
///
/// ## Retorno
/// Cantidad de minutos transcurridos (redondeado hacia abajo).
///
/// ## Ejemplos
/// ```rust
/// use brisas_app_lib::common::calcular_tiempo_permanencia;
///
/// // 3.5 horas = 210 minutos
/// let minutos = calcular_tiempo_permanencia(
///     "2026-01-15T08:00:00Z",
///     "2026-01-15T11:30:00Z"
/// ).unwrap();
/// assert_eq!(minutos, 210);
/// ```
///
/// ## Errores
/// * `DateValidationError::FormatoInvalido` - Fechas no son RFC 3339 válidas
/// * `DateValidationError::RangoInvalido` - Salida anterior a ingreso
pub fn calcular_tiempo_permanencia(
    fecha_ingreso: &str,
    fecha_salida: &str,
) -> Result<i64, DateValidationError> {
    // Validar que salida > ingreso
    validar_tiempo_salida(fecha_ingreso, fecha_salida)?;

    let ingreso = DateTime::parse_from_rfc3339(fecha_ingreso)
        .map_err(|e| DateValidationError::FormatoInvalido(format!("{}", e)))?;

    let salida = DateTime::parse_from_rfc3339(fecha_salida)
        .map_err(|e| DateValidationError::FormatoInvalido(format!("{}", e)))?;

    let duracion = salida.signed_duration_since(ingreso);
    Ok(duracion.num_minutes())
}
````

---

### 4. [ ] Tests Exhaustivos

**Objetivo**: 100% cobertura en funciones críticas.

**Estructura de tests**:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // --------------------------------------------------------------------------
    // TESTS DE VALIDACIÓN DE FECHAS RFC 3339
    // --------------------------------------------------------------------------

    mod validar_fecha_rfc3339 {
        use super::*;

        #[test]
        fn acepta_formato_valido() {
            assert!(validar_fecha_rfc3339("2026-01-15T08:30:00Z").is_ok());
            assert!(validar_fecha_rfc3339("2025-12-31T23:59:59Z").is_ok());
            assert!(validar_fecha_rfc3339("2026-01-01T00:00:00Z").is_ok());
        }

        #[test]
        fn rechaza_formato_sin_hora() {
            assert!(validar_fecha_rfc3339("2026-01-15").is_err());
        }

        #[test]
        fn rechaza_formato_local() {
            assert!(validar_fecha_rfc3339("15/01/2026 08:30").is_err());
            assert!(validar_fecha_rfc3339("01-15-2026T08:30:00Z").is_err());
        }

        #[test]
        fn rechaza_mes_invalido() {
            assert!(validar_fecha_rfc3339("2026-13-01T00:00:00Z").is_err());
            assert!(validar_fecha_rfc3339("2026-00-01T00:00:00Z").is_err());
        }

        #[test]
        fn rechaza_dia_invalido() {
            assert!(validar_fecha_rfc3339("2026-02-30T00:00:00Z").is_err());
            assert!(validar_fecha_rfc3339("2026-04-31T00:00:00Z").is_err());
        }

        #[test]
        fn rechaza_hora_invalida() {
            assert!(validar_fecha_rfc3339("2026-01-15T25:00:00Z").is_err());
            assert!(validar_fecha_rfc3339("2026-01-15T08:60:00Z").is_err());
        }
    }

    // --------------------------------------------------------------------------
    // TESTS DE VALIDACIÓN DE FECHAS SIMPLES
    // --------------------------------------------------------------------------

    mod validar_fecha_simple {
        use super::*;

        #[test]
        fn acepta_formato_valido() {
            assert!(validar_fecha_simple("2026-12-31").is_ok());
            assert!(validar_fecha_simple("2026-01-01").is_ok());
        }

        #[test]
        fn rechaza_formato_con_hora() {
            assert!(validar_fecha_simple("2026-01-15T08:30:00Z").is_err());
        }

        #[test]
        fn rechaza_formato_local() {
            assert!(validar_fecha_simple("31/12/2026").is_err());
            assert!(validar_fecha_simple("12-31-2026").is_err());
        }

        #[test]
        fn rechaza_fechas_imposibles() {
            assert!(validar_fecha_simple("2026-02-30").is_err());
            assert!(validar_fecha_simple("2026-13-01").is_err());
        }
    }

    // --------------------------------------------------------------------------
    // TESTS DE VALIDACIÓN DE RANGO
    // --------------------------------------------------------------------------

    mod validar_tiempo_salida {
        use super::*;

        #[test]
        fn acepta_salida_posterior_a_ingreso() {
            let resultado = validar_tiempo_salida(
                "2026-01-15T08:00:00Z",
                "2026-01-15T17:00:00Z"
            );
            assert!(resultado.is_ok());
        }

        #[test]
        fn rechaza_salida_anterior_a_ingreso() {
            let resultado = validar_tiempo_salida(
                "2026-01-15T17:00:00Z",
                "2026-01-15T08:00:00Z"
            );
            assert!(resultado.is_err());
            assert!(matches!(resultado.unwrap_err(), DateValidationError::RangoInvalido));
        }

        #[test]
        fn rechaza_salida_igual_a_ingreso() {
            let resultado = validar_tiempo_salida(
                "2026-01-15T08:00:00Z",
                "2026-01-15T08:00:00Z"
            );
            assert!(resultado.is_err());
        }
    }

    // --------------------------------------------------------------------------
    // TESTS DE CÁLCULO DE TIEMPO
    // --------------------------------------------------------------------------

    mod calcular_tiempo_permanencia {
        use super::*;

        #[test]
        fn calcula_minutos_correctamente() {
            let minutos = calcular_tiempo_permanencia(
                "2026-01-15T08:00:00Z",
                "2026-01-15T11:30:00Z"
            ).unwrap();
            assert_eq!(minutos, 210); // 3.5 horas
        }

        #[test]
        fn calcula_dias_completos() {
            let minutos = calcular_tiempo_permanencia(
                "2026-01-15T00:00:00Z",
                "2026-01-16T00:00:00Z"
            ).unwrap();
            assert_eq!(minutos, 1440); // 24 horas
        }

        #[test]
        fn falla_si_salida_anterior_a_ingreso() {
            let resultado = calcular_tiempo_permanencia(
                "2026-01-15T17:00:00Z",
                "2026-01-15T08:00:00Z"
            );
            assert!(resultado.is_err());
        }
    }
}
```

---

### 5. [ ] Funciones const cuando sea posible

**Objetivo**: Evaluación en compile-time cuando aplique.

```rust
/// Longitud de una cédula costarricense sin guiones (9 dígitos).
pub const CEDULA_LENGTH: usize = 9;

/// Longitud de una cédula costarricense con guiones (11 caracteres).
pub const CEDULA_LENGTH_WITH_HYPHENS: usize = 11;

/// Verifica si una longitud es válida para una cédula.
///
/// Esta función se puede evaluar en tiempo de compilación.
pub const fn is_valid_cedula_length(len: usize) -> bool {
    len == CEDULA_LENGTH || len == CEDULA_LENGTH_WITH_HYPHENS
}
```

---

### 6. [ ] Documentación Completa

Ya cubierto en los ejemplos anteriores. Cada función debe tener:

- [ ] Descripción clara
- [ ] Sección de ejemplos con `assert!`
- [ ] Documentación de errores
- [ ] Notas de performance si usa `Lazy<>`

---

### 7. [ ] Verificación Final

**Checklist de Common/Utils**:

- [ ] Sin dependencias de capas superiores
- [ ] Todas las funciones son puras
- [ ] Regex usan `Lazy<Regex>`
- [ ] Constantes en SCREAMING_SNAKE_CASE
- [ ] 100% funciones documentadas con ejemplos
- [ ] Tests exhaustivos (>90% cobertura)
- [ ] Separadores visuales entre categorías
- [ ] Sin código comentado

**Comandos**:

```bash
# Tests
cargo test --package mega-brisas -- common

# Coverage
cargo tarpaulin --package mega-brisas --lib -- common

# Benchmarks (opcional)
cargo bench --package mega-brisas -- common
```

---

## Plantilla de Commit

```
refactor(common): optimizar y documentar utilidades compartidas

- Optimizar regex con Lazy<Regex> para mejor performance
- Agregar tests exhaustivos (>90% cobertura)
- Documentar todas las funciones con ejemplos
- Extraer constantes mágicas
- Organizar por categorías con separadores visuales
- Validaciones de fechas según estándar Brisas APP

Closes #{numero_issue}
```

---

**Fin del Workflow G - Common/Utils**
