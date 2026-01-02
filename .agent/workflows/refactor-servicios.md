# Workflow A: Auditoría y Refactorización de Servicios

**Versión**: 3.0  
**Idioma**: Español  
**Aplicación**: Brisas APP  

---

## Objetivo

Elevar los servicios de aplicación a estándares Enterprise mediante desacoplamiento estricto, validación centralizada, documentación exhaustiva en español y cumplimiento de estándares de Brisas APP.

---

## FASE 0: ANÁLISIS PREVIO (⚠️ OBLIGATORIO - NO EJECUTAR CAMBIOS AÚN)

> **IMPORTANTE**: Antes de modificar cualquier código, el agente DEBE completar esta fase y presentar un reporte de hallazgos para aprobación del usuario.

### [ ] 0.1 Análisis Arquitectural

**Plantilla de análisis**:

```markdown
**Archivo analizado**: `src/services/{nombre}_service.rs`
**LOC**: {número de líneas}
**Complejidad ciclomática**: {estimación}

**Dependencias actuales**:
- ✅ Permitidas: 
  - `crate::domain::*` (X referencias)
  - `crate::models::*` (Y referencias)
  - `crate::common::*` (Z referencias)
  
- ⚠️ Sospechosas:
  - `crate::db::*` (N referencias) → ❌ Acceso directo a queries
  - `tauri::State` (M referencias) → ⚠️ Acoplamiento a infraestructura
  - `surrealdb::sql::Thing` → ❌ Fuga de abstracción

**Lógica de negocio inline detectada**:
1. Línea XX: `if campo.is_empty()` → Mover a `domain::validators`
2. Línea YY: Cálculo de negocio → Mover a `domain::rules`
3. Línea ZZ: Validación de fecha → Usar `common::validar_fecha_rfc3339`

**Responsabilidades del servicio**:
- [ ] ¿Actúa como orquestador puro? (Sí/No)
- [ ] ¿Tiene >3 niveles de dependencias anidadas? (Sí/No)
- [ ] ¿Accede directamente a queries de DB? (Sí/No) ❌ CRÍTICO
```

### [ ] 0.2 Auditoría de Validaciones

**Plantilla**:

```markdown
**Validaciones encontradas**: {N} total

| Línea | Código | Destino sugerido | Prioridad |
|-------|--------|------------------|-----------|
| 34 | `if campo.is_empty()` | `domain::validators::validate_campo()` | Media |
| 56 | `fecha.parse::<DateTime>()` | `common::validar_fecha_rfc3339()` | Alta |
| 89 | Cálculo de tiempo | `common::calcular_tiempo_permanencia()` | Alta |

**Validaciones críticas de seguridad** (prioridad CRÍTICA):
- Línea XX: Verificación de Lista Negra → Requiere test unitario + logging
- Línea YY: Validación de permisos → Requiere auditoría
```

### [ ] 0.3 Análisis de DTOs

**Plantilla**:

```markdown
**Inputs actuales**:
- ❌ `funcion(arg1: String, arg2: String, ...)` → N parámetros sueltos
  - **Acción**: Crear `{Nombre}Command` en `models/{modulo}/commands.rs`

**Outputs actuales**:
- ❌ `Result<Thing, surrealdb::Error>` → Tipo de infraestructura expuesto
  - **Acción**: Crear `{Nombre}Response` en `models/{modulo}/responses.rs`

**DTOs a crear**:
1. `{Accion}Command` (input)
2. `{Entidad}Response` (output)
```

### [ ] 0.4 Análisis de Transacciones

**Plantilla**:

```markdown
**Operaciones multi-entidad detectadas**:
- Línea XX-YY: Creación de {entidad} + {relacionada} + log
  - **Riesgo**: Sin transacción explícita → Inconsistencia posible
  - **Acción**: Envolver en `BEGIN ... COMMIT`

**Manejo de errores**:
- ⚠️ N lugares usan `.unwrap()` → Reemplazar con propagación `?`
- ⚠️ M lugares ignoran errores con `let _ =` → Evaluar si es correcto
```

### [ ] 0.5 Análisis de Logging

**Plantilla**:

```markdown
**Estado actual del logging**:
- ✅ Importa `log` crate: Sí/No
- ❌ Logs estructurados: X/Y operaciones cubiertas

**Eventos críticos sin log**:
1. Línea XX: Creación exitosa → Necesita `info!`
2. Línea YY: Bloqueo de seguridad → Necesita `warn!`
3. Línea ZZ: Error de DB → Necesita `error!` con contexto

**Plan de logging**:
- N `info!` para operaciones exitosas
- M `warn!` para validaciones fallidas recuperables
- K `error!` para fallos críticos de infraestructura
```

### [ ] 0.6 Análisis de Documentación

**Plantilla**:

```markdown
**Cobertura actual**:
- Documentación de módulo: ✅/❌
- Funciones públicas documentadas: X/Y (Z%)
- Comentarios obsoletos: N bloques

**Funciones sin documentar**:
1. `{nombre}()` → Falta descripción de validaciones críticas
2. `{nombre}()` → Falta explicación del "por qué"

**Idioma**: 
- ⚠️ N funciones con docs en inglés → Traducir a español
```

### [ ] 0.7 Análisis de Testing

**Plantilla**:

```markdown
**Cobertura estimada**: ~X%

**Lógica pura sin tests** (candidatos a tests unitarios):
- `calcular_*()` → Lógica de cálculo
- `formatear_*()` → Transformación de datos
- `validar_*()` → Reglas de negocio

**Dependencias de DB**: X/Y funciones
- **Estrategia**: Introducir `trait {Nombre}Repository` para mocking
```

### [ ] 0.8 Conformidad con Estándares Brisas APP

#### Estándar de Fechas

```markdown
| Campo | Formato Actual | Formato Esperado | Acción |
|-------|----------------|------------------|--------|
| `fecha_hora_*` | String sin validar | RFC 3339 | Usar `validar_fecha_rfc3339()` |
| `fecha_vencimiento_*` | `DD/MM/YYYY` | `YYYY-MM-DD` | Usar `validar_fecha_simple()` |
```

#### Estándar de Documentación

```markdown
- [ ] Idioma español: X% cumplimiento
- [ ] Tono profesional: ✅/❌
- [ ] Explicación del "por qué": ✅/❌
- [ ] Separadores visuales: ✅/❌
```

---

### 📋 Plantilla de Reporte Final

```markdown
# Reporte de Análisis FASE 0

**Archivo**: src/services/{nombre}_service.rs
**LOC**: {número}
**Complejidad**: {alta/media/baja}

## PROBLEMAS CRÍTICOS (Bloquean refactor)
1. [CRÍTICO] Descripción del problema
   - **Impacto**: Qué riesgos genera
   - **Solución**: Qué hacer
   - **Esfuerzo**: Estimación en horas

## PROBLEMAS MAYORES
2. [ALTO] Descripción
3. [MEDIO] Descripción

## MEJORAS RECOMENDADAS
4. [BAJO] Descripción

## ESTIMACIÓN DE ESFUERZO
- Refactor obligatorio: X-Y horas
- Testing: Z horas
- Documentación: W horas
- **TOTAL**: T horas

## ¿Proceder con el refactor?
Esperar aprobación del usuario.
```

---

## FASE 1-9: EJECUCIÓN (Solo después de aprobar Fase 0)

### 1. [ ] Refactorización de Dependencias

**Acciones**:

1. **Crear Traits de Repositorio** (si no existen):
```rust
// src/repositories/traits/{modulo}_repository.rs

use async_trait::async_trait;
use crate::models::{/*...*/};
use crate::domain::errors::{/*...*/};

#[async_trait]
pub trait {Nombre}Repository: Send + Sync {
    async fn create(&self, dto: /*...*/) -> Result</*...*/, RepositoryError>;
    async fn find_by_id(&self, id: &str) -> Result<Option</*...*/>, RepositoryError>;
    // ... resto de métodos
}
```

2. **Implementar Repositorio para SurrealDB**:
```rust
// src/repositories/surrealdb_{modulo}_repository.rs

use super::traits::{Nombre}Repository;
use crate::db::surrealdb_{modulo}_queries as queries;

pub struct SurrealDb{Nombre}Repository;

#[async_trait]
impl {Nombre}Repository for SurrealDb{Nombre}Repository {
    async fn create(&self, dto: /*...*/) -> Result</*...*/, RepositoryError> {
        queries::create(dto).await
            .map_err(|e| RepositoryError::Database(e.to_string()))
    }
    // ... resto de implementaciones
}
```

3. **Refactorizar Servicio con Inyección de Dependencias**:
```rust
// src/services/{modulo}_service.rs

pub struct {Nombre}Service {
    repo: Arc<dyn {Nombre}Repository>,
    // ... otros repositorios
}

impl {Nombre}Service {
    pub fn new(
        repo: Arc<dyn {Nombre}Repository>,
        // ...
    ) -> Self {
        Self { repo }
    }
    
    pub async fn crear(&self, cmd: /*...*/) -> Result</*...*/, DomainError> {
        // Validaciones de dominio
        domain::validar_input(&cmd)?;
        
        // Delegación al repositorio
        let entidad = self.repo.create(dto).await?;
        
        info!("Entidad creada exitosamente: id={}", entidad.id);
        Ok(entidad)
    }
}
```

**Criterio de éxito**:
- [ ] Servicio NO importa `crate::db::`
- [ ] Servicio recibe repositorios por constructor
- [ ] Funciones son testables con mocks

---

### 2. [ ] Validación y Reglas de Negocio

**Acciones**:

1. **Identificar validaciones inline**:
```rust
// ❌ ANTES (en servicio)
if input.campo.is_empty() {
    return Err(ServiceError::InvalidField);
}

// ✅ DESPUÉS (delegar a dominio)
domain::validators::validate_campo(&input.campo)?;
```

2. **Mover a capa de dominio**:
```rust
// src/domain/{modulo}/validators.rs

/// Valida que el campo no esté vacío y cumpla formato.
///
/// # Errores
/// * `DomainError::CampoVacio` - El campo está vacío
/// * `DomainError::FormatoInvalido` - No cumple el patrón esperado
pub fn validate_campo(valor: &str) -> Result<(), DomainError> {
    if valor.trim().is_empty() {
        return Err(DomainError::CampoVacio);
    }
    
    // Validación de formato...
    
    Ok(())
}
```

3. **Aplicar estándares de fechas**:
```rust
// ✅ Usar funciones centralizadas de common.rs
use crate::common::{validar_fecha_rfc3339, validar_fecha_simple};

// Para timestamps (con hora)
validar_fecha_rfc3339(&input.fecha_hora_ingreso)?;

// Para fechas simples (sin hora)
validar_fecha_simple(&input.fecha_vencimiento)?;
```

---

### 3. [ ] Estandarización de Contratos (DTOs)

**Acciones**:

1. **Crear Commands (Input)**:
```rust
// src/models/{modulo}/commands.rs

/// Comando para crear {entidad}.
///
/// ## Formato de Fechas (Estándar Brisas APP)
/// - `fecha_hora_*`: RFC 3339 (ej: "2026-01-15T08:30:00Z")
/// - `fecha_vencimiento_*`: YYYY-MM-DD (ej: "2026-12-31")
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Create{Entidad}Command {
    pub campo1: String,
    pub campo2: String,
    /// Fecha y hora en formato RFC 3339
    pub fecha_hora_creacion: String,
}
```

2. **Crear Responses (Output)**:
```rust
// src/models/{modulo}/responses.rs

/// Respuesta al crear {entidad} exitosamente.
///
/// Las fechas se retornan en formato RFC 3339 para que el frontend
/// las convierta al formato local del usuario.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct {Entidad}CreatedResponse {
    pub id: String,
    /// Fecha en formato RFC 3339
    pub fecha_hora_creacion: String,
}
```

---

### 4. [ ] Gestión Transaccional

**Acciones**:

```rust
/// Crea {entidad} y sus relaciones de forma atómica.
///
/// Esta operación es **transaccional** porque modifica:
/// 1. {Entidad principal}
/// 2. {Relación 1}
/// 3. {Log de auditoría}
///
/// Si cualquier paso falla, se revierten TODOS los cambios.
pub async fn crear_con_relaciones(
    &self,
    cmd: Create{Entidad}Command
) -> Result<{Entidad}Response, DomainError> {
    // Inicio de transacción
    let tx = self.begin_transaction().await?;
    
    // Paso 1: Crear entidad principal
    let entidad = tx.repo.create(&cmd).await.map_err(|e| {
        error!("Error al crear {entidad}: {}", e);
        e
    })?;
    
    // Paso 2: Crear relaciones
    tx.relacion_repo.create_relacionada(&entidad.id).await?;
    
    // Paso 3: Auditar
    tx.audit_repo.log_creacion(&entidad.id).await?;
    
    // Commit
    tx.commit().await.map_err(|e| {
        error!("Error al confirmar transacción: {}", e);
        DomainError::TransactionFailed(e.to_string())
    })?;
    
    info!("{Entidad} creada exitosamente: id={}", entidad.id);
    
    Ok(entidad.into())
}
```

---

### 5. [ ] Logging Estructurado (Tauri)

**Configuración en main.rs**:

```rust
use tauri_plugin_log::{LogTarget, RotationStrategy};

tauri::Builder::default()
    .plugin(
        tauri_plugin_log::Builder::default()
            .targets([
                LogTarget::LogDir,      // Guarda en disco
                LogTarget::Stdout,      // Consola (desarrollo)
                LogTarget::Webview,     // DevTools del frontend
            ])
            .level(log::LevelFilter::Info)
            .level_for("brisas_app", log::LevelFilter::Debug)
            .rotation_strategy(RotationStrategy::KeepAll)
            .build()
    )
    .run(tauri::generate_context!())
    .expect("Error al iniciar Brisas APP");
```

**Niveles de Log en Servicios**:

```rust
use log::{trace, debug, info, warn, error};

// ✅ INFO: Eventos de negocio exitosos
info!(
    "{Entidad} creada: id={}, usuario={}, duration_ms={}",
    id, usuario, duration.as_millis()
);

// ✅ WARN: Situaciones anómalas pero recuperables
warn!(
    "Intento de operación sobre {entidad} bloqueada: id={}, motivo={}",
    id, motivo
);

// ✅ ERROR: Fallos críticos de infraestructura
error!(
    "Fallo en transacción: operacion={}, error={}, rollback=true",
    operacion, err
);

// ✅ DEBUG: Troubleshooting (off en producción)
debug!("Validando campos: campo1={}, campo2={}", c1, c2);

// ✅ TRACE: Detalles exhaustivos (solo desarrollo)
trace!("Query ejecutado: {:?}", query);
```

**Contexto Enriquecido**:

```rust
// ❌ Log inútil
info!("Operación completada");

// ✅ Log accionable
info!(
    target: "services::{modulo}",
    "{Entidad} creada: id={}, tipo={}, usuario={}, hora_local={}, duracion_ms={}",
    id, tipo, usuario, chrono::Local::now().format("%H:%M:%S"), duration.as_millis()
);
```

---

### 6. [ ] Documentación Técnica

**Encabezado del Módulo**:

```rust
//! # Servicio de Gestión de {Entidades}
//!
//! Orquestador de operaciones relacionadas con el dominio de {entidades}.
//!
//! ## Responsabilidades
//! - Coordinar la creación y actualización de {entidades}
//! - Validar reglas de negocio mediante `domain::{modulo}`
//! - Gestionar transacciones multi-entidad
//!
//! ## Validaciones Críticas de Seguridad
//! - Verificación de {criterio} antes de cualquier operación
//! - Prevención de {escenario peligroso}
//!
//! ## Dependencias
//! - `domain::{modulo}::validators`: Validación de datos
//! - `repositories::{Nombre}Repository`: Acceso a datos
//! - `common`: Funciones centralizadas (fechas, validaciones)
//!
//! ## Estándares de Fechas
//! - **RFC 3339** (con hora): `fecha_hora_*`
//! - **YYYY-MM-DD** (solo fecha): `fecha_vencimiento_*`
//!
//! Ver `docs/estandares-fechas.md` para más detalles.

use crate::common::{validar_fecha_rfc3339, calcular_tiempo_permanencia};
use log::{info, warn, error};

// --------------------------------------------------------------------------
// VALIDACIONES DE NEGOCIO
// --------------------------------------------------------------------------

// --------------------------------------------------------------------------
// LÓGICA DE ORQUESTACIÓN
// --------------------------------------------------------------------------

// --------------------------------------------------------------------------
// TESTS UNITARIOS
// --------------------------------------------------------------------------
```

**Funciones Públicas**:

```rust
/// {Descripción breve de la acción}.
///
/// {Explicación del "por qué" es importante esta función y qué validaciones críticas realiza}
///
/// ## Proceso
/// 1. Valida {criterio}
/// 2. Verifica {condición}
/// 3. Crea/actualiza {entidad}
///
/// ## Transaccionalidad
/// Esta operación {es/no es} transaccional porque {razón}.
///
/// ## Formato de Fechas
/// - `campo_fecha_hora`: RFC 3339 (ej: "2026-01-15T08:30:00Z")
/// - `campo_fecha`: YYYY-MM-DD (ej: "2026-12-31")
///
/// # Argumentos
/// * `command` - Estructura con los datos de entrada
///
/// # Retorno
/// * `Ok({Response})` - {Descripción del resultado exitoso}
///
/// # Errores
/// * `DomainError::{Tipo1}` - {Descripción de cuándo ocurre}
/// * `DomainError::{Tipo2}` - {Descripción de cuándo ocurre}
///
/// # Ejemplo
/// ```rust
/// let resultado = servicio.operacion(Command {
///     campo1: "valor".to_string(),
///     fecha_hora: "2026-01-15T08:30:00Z".to_string(),
/// }).await?;
/// ```
///
/// # Logging
/// - `INFO`: Cuando la operación se completa exitosamente
/// - `WARN`: Cuando {condición de advertencia}
/// - `ERROR`: Cuando falla la transacción o hay error de DB
pub async fn operacion(
    &self,
    command: {Command}
) -> Result<{Response}, DomainError> {
    // implementación
}
```

---

### 7. [ ] Estrategia de Testing

**Tests Unitarios (Lógica Pura)**:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};
    
    /// Test crítico: Verificar cálculo de {algo}.
    #[test]
    fn test_calculo_correcto() {
        let resultado = calcular(input);
        assert_eq!(resultado, esperado);
    }
    
    /// Test de seguridad: Verificar que {condición peligrosa} es rechazada.
    #[test]
    fn test_validacion_rechaza_caso_invalido() {
        let resultado = validar(input_invalido);
        assert!(resultado.is_err());
        assert!(matches!(resultado.unwrap_err(), DomainError::{Tipo}));
    }
}
```

**Tests de Integración con Mocks**:

```rust
#[cfg(test)]
mod integration_tests {
    use mockall::predicate::*;
    use crate::repositories::Mock{Nombre}Repository;
    
    #[tokio::test]
    async fn test_caso_critico() {
        let mut mock_repo = Mock{Nombre}Repository::new();
        mock_repo
            .expect_metodo()
            .with(eq("parametro"))
            .returning(|_| Ok(resultado_mock));
        
        let servicio = {Nombre}Service::new(Arc::new(mock_repo));
        let resultado = servicio.operacion(command).await;
        
        assert!(resultado.is_ok());
    }
}
```

---

### 8. [ ] Optimización

**Identificar N+1 Queries**:

```rust
// ❌ N+1 Queries
for item in items {
    let relacionado = repo.get_relacionado(item.id).await?;
}

// ✅ Batch Query
let ids: Vec<_> = items.iter().map(|i| i.id.clone()).collect();
let relacionados = repo.get_relacionados_batch(&ids).await?;
```

---

### 9. [ ] Verificación Final

**Pre-Commit**:

```bash
# Compilación sin warnings
cargo clippy -- -D warnings

# Formateo
cargo fmt -- --check

# Tests
cargo test --all-features

# Cobertura (opcional)
cargo tarpaulin --out Html
```

**Checklist**:

- [ ] Zero `cargo clippy` warnings
- [ ] Documentación en español (100% funciones públicas)
- [ ] Tests unitarios >80% en lógica pura
- [ ] Sin `TODO` ni `FIXME` en código crítico
- [ ] Fechas en formato estándar (RFC 3339 / YYYY-MM-DD)
- [ ] Logging en operaciones críticas
- [ ] Separadores visuales (`// ----------`)
- [ ] Sin código comentado

---

## Plantilla de Commit

```
refactor(services): migrar {nombre}_service a Clean Architecture

- Extraer lógica de negocio a capa de dominio
- Introducir Repository Pattern con traits
- Agregar gestión transaccional en operaciones multi-entidad
- Implementar logging estructurado con niveles apropiados
- Documentar en español según estándar de Brisas APP
- Aplicar estándares de fechas (RFC 3339 / YYYY-MM-DD)
- Agregar tests unitarios con cobertura >80%

Closes #{numero_issue}
```

---

## 📂 Ubicaciones de Logs en Tauri

| Sistema | Ruta |
|---------|------|
| **Windows** | `%APPDATA%\brisas-app\logs\brisas_app.log` |
| **macOS** | `~/Library/Application Support/brisas-app/logs/brisas_app.log` |
| **Linux** | `~/.local/share/brisas-app/logs/brisas_app.log` |

**Comando opcional para acceder desde UI**:

```rust
#[tauri::command]
pub async fn obtener_logs_recientes(lineas: usize) -> Result<String, String> {
    let lineas = lineas.min(1000);
    // Implementar lectura...
}
```