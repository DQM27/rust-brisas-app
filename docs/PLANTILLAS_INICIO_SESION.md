# 📝 PLANTILLAS DE INICIO DE SESIÓN

**Propósito**: Copiar y pegar al inicio de cada nueva sesión para dar contexto fresco al agente  
**Versión**: 1.0

---

## 🎯 Cómo Usar Estas Plantillas

1. **Identifica qué archivo vas a refactorizar**
2. **Elige la plantilla según la capa** (Servicio, Dominio, Queries, etc.)
3. **Copia la plantilla completa**
4. **Reemplaza {variables} con tus valores**
5. **Pega al inicio de una NUEVA sesión**
6. **Adjunta el archivo a refactorizar**

---

## 📋 PLANTILLA A: SERVICIOS

```markdown
# 🔧 SESIÓN: Refactorización de Servicio

## Archivo

- **Módulo**: {nombre_modulo} (ej: Contratista, Usuario, Ingreso)
- **Ruta**: src/services/{nombre}\_service.rs
- **Workflow**: A - Servicios

## Objetivo

Refactorizar servicio según estándares de Clean Architecture idiomática de Rust.

## Estándares Brisas APP

- **Documentación**: Español, explicar "por qué"
- **Fechas**: RFC 3339 para timestamps, YYYY-MM-DD para fechas simples
- **Logging**: `log::info!`, `log::warn!`, `log::error!` (tauri-plugin-log)
- **Errores**: `thiserror` con mensajes en español
- **Transacciones**: Operaciones multi-entidad requieren atomicidad

## Instrucciones Estrictas

1. ✅ Leer `/mnt/user-data/outputs/workflow_a_servicios.md` COMPLETO
2. ✅ Ejecutar FASE 0 (análisis detallado sin modificar código)
3. ✅ Generar reporte con estimación de horas
4. ✅ ESPERAR mi aprobación explícita
5. ✅ Ejecutar fases 1-9 linealmente si apruebo
6. ✅ Verificar compilación antes de entregar:
   - `cargo check --package mega-brisas`
   - `cargo clippy --package mega-brisas -- -D warnings`
   - `cargo test --package mega-brisas -- services::{modulo}`

## Reglas de Oro

- ⚠️ NO modificar código hasta que yo apruebe
- ⚠️ UN ARCHIVO a la vez (solo el servicio, no tocar queries ni domain)
- ⚠️ Seguir workflow al pie de la letra (no improvisar)
- ⚠️ Si el archivo es >500 líneas, avisar y proceder en chunks

## Contexto del Proyecto

- **Proyecto**: Brisas APP - Sistema ERP de Control de Acceso
- **Stack**: Rust + Tauri v2 + SurrealDB
- **Arquitectura**: Clean Architecture idiomática (no OOP, no Repository Pattern)

## Archivo Adjunto

[Adjuntar: {nombre}_service.rs]

---

**¿Listo para comenzar? Confirma que leíste el workflow y procede con FASE 0.**
```

---

## 📋 PLANTILLA B: DOMINIO

```markdown
# 🧠 SESIÓN: Purificación de Dominio

## Archivo

- **Módulo**: {nombre_modulo}
- **Ruta**: src/domain/{nombre}.rs (o src/domain/errors.rs)
- **Workflow**: B - Dominio

## Objetivo

Purificar dominio para que contenga SOLO lógica de negocio pura (sin structs de datos, sin dependencias de infraestructura).

## Estándares Brisas APP

- **Pureza total**: Sin imports de services, db, commands
- **Funciones puras**: Determinísticas, sin efectos secundarios
- **Validaciones**: Usar funciones de `common.rs` para fechas
- **Errores**: `thiserror` con mensajes descriptivos en español
- **Tests**: Cobertura >80% obligatoria

## Instrucciones Estrictas

1. ✅ Leer `/mnt/user-data/outputs/workflow_b_dominio.md` COMPLETO
2. ✅ Ejecutar FASE 0 (identificar structs a mover, dependencias impuras, valores mágicos)
3. ✅ Reportar hallazgos con plan de acción
4. ✅ ESPERAR mi aprobación
5. ✅ Ejecutar fases 1-8 si apruebo
6. ✅ Verificar:
   - `cargo check --package mega-brisas`
   - `cargo test --package mega-brisas -- domain::{modulo}`
   - Sin imports impuros (services, db, commands)

## Reglas de Oro

- ⚠️ Si hay structs de datos → Mover a `models/` en OTRA sesión (no ahora)
- ⚠️ Solo funciones de validación, normalización, cálculos puros
- ⚠️ Constantes en SCREAMING_SNAKE_CASE
- ⚠️ Tests unitarios obligatorios para cada función pública

## Contexto

- **Paradigma**: Rust funcional (no OOP)
- **Crates permitidos**: `chrono`, `regex`, `once_cell`, tipos básicos
- **Crates prohibidos**: `surrealdb`, `tauri`, `crate::services`, `crate::db`

## Archivo Adjunto

[Adjuntar: {nombre}.rs]

---

**Comienza con FASE 0. Identifica qué es lógica pura y qué debe moverse a models/.**
```

---

## 📋 PLANTILLA C: MODELOS

```markdown
# 📦 SESIÓN: Type-Driven Design en Modelos

## Archivo

- **Módulo**: {nombre_modulo}
- **Ruta**: src/models/{nombre}.rs
- **Workflow**: C - Modelos

## Objetivo

Aplicar Type-Driven Design: enums sobre strings, Option explícito, sin lógica de negocio.

## Estándares Brisas APP

- **Enums sobre strings**: Estados, tipos, categorías finitas
- **Option<T> explícito**: No valores centinela ("", 0 para null)
- **Sin lógica**: Models solo define datos, lógica va a `domain/`
- **Documentación**: Propósito de negocio de cada struct/enum

## Instrucciones Estrictas

1. ✅ Leer `/mnt/user-data/outputs/workflow_c_modelos.md` COMPLETO
2. ✅ Ejecutar FASE 0 (detectar "stringly typed", valores centinela, lógica de negocio)
3. ✅ Reportar enums a crear, campos a cambiar a Option
4. ✅ ESPERAR aprobación
5. ✅ Ejecutar fases 1-7 si apruebo
6. ✅ Verificar compilación

## Reglas de Oro

- ⚠️ Si hay métodos con `if`/`match`/cálculos → Mover a `domain/` (no ahora, otra sesión)
- ⚠️ Crear enums para campos con valores fijos (ej: "ACTIVO"/"INACTIVO" → enum Estado)
- ⚠️ Cambiar "" y 0 a Option<T> explícito
- ⚠️ Value Objects solo si aporta valor (no over-engineering)

## Archivo Adjunto

[Adjuntar: {nombre}.rs]

---

**FASE 0: Identifica campos "stringly typed" y lógica de negocio a extraer.**
```

---

## 📋 PLANTILLA D: QUERIES SURREALDB

```markdown
# 🗄️ SESIÓN: Optimización de Queries SurrealDB

## Archivo

- **Módulo**: {nombre_modulo}
- **Ruta**: src/db/surrealdb\_{nombre}\_queries.rs
- **Workflow**: D - Queries SurrealDB

## Objetivo

Queries optimizados, documentados y sin lógica de negocio.

## Estándares Brisas APP

- **Documentación exhaustiva**: Explicar cada query SQL, uso de FETCH, índices
- **Optimización**: LIMIT en listados, FETCH para N+1, índices documentados
- **Soft delete**: `deleted_at IS NONE` en queries de lectura
- **Sin lógica**: Queries solo acceden a datos, validaciones van a `domain/`
- **Logging**: `log::debug!` en lugar de `println!`

## Instrucciones Estrictas

1. ✅ Leer `/mnt/user-data/outputs/workflow_d_queries_surrealdb.md` COMPLETO
2. ✅ Ejecutar FASE 0 (detectar queries sin LIMIT, sin docs, sin optimizar)
3. ✅ Reportar queries críticos a optimizar
4. ✅ ESPERAR aprobación
5. ✅ Ejecutar fases 1-8 si apruebo
6. ✅ Verificar compilación y tests de integración

## Reglas de Oro

- ⚠️ TODOS los listados (find_all) deben tener LIMIT
- ⚠️ Documentar queries SQL en comentarios
- ⚠️ Explicar uso de FETCH (por qué y qué campos)
- ⚠️ Errores con contexto específico (no genéricos)

## Archivo Adjunto

[Adjuntar: surrealdb_{nombre}_queries.rs]

---

**FASE 0: Identifica queries sin LIMIT, sin FETCH donde aplique, y sin documentación.**
```

---

## 📋 PLANTILLA E: COMMANDS TAURI

```markdown
# 🎮 SESIÓN: Refactorización de Commands (Thin Layer)

## Archivo

- **Módulo**: {nombre_modulo}
- **Ruta**: src/commands/{nombre}\_commands.rs
- **Workflow**: E - Commands Tauri

## Objetivo

Commands como adaptadores puros: validar sesión, validar inputs, llamar servicio, mapear errores. Sin lógica de negocio.

## Estándares Brisas APP

- **Thin layer**: Máximo 5-10 líneas por command
- **Validación de sesión**: TODOS los commands de escritura (create, update, delete)
- **Validación de inputs**: Fechas, IDs, rangos ANTES de llamar servicio
- **Errores serializables**: `CommandError` para manejo en TypeScript
- **Documentación**: Incluir ejemplos TypeScript

## Instrucciones Estrictas

1. ✅ Leer `/mnt/user-data/outputs/workflow_e_commands_tauri.md` COMPLETO
2. ✅ Ejecutar FASE 0 (detectar commands sin auth, sin validación, con lógica)
3. ✅ Reportar commands CRÍTICOS sin sesión
4. ✅ ESPERAR aprobación
5. ✅ Ejecutar fases 1-7 si apruebo
6. ✅ Verificar que TODOS los commands de escritura validen sesión

## Reglas de Oro

- ⚠️ CRÍTICO: Commands de escritura (create/update/delete) DEBEN validar sesión
- ⚠️ Sin lógica de negocio (transformaciones van a servicios)
- ⚠️ Validar formato de fechas con `common::validar_fecha_*`
- ⚠️ Errores específicos (no .map_err(|e| e.to_string()) genérico)

## Archivo Adjunto

[Adjuntar: {nombre}_commands.rs]

---

**⚠️ URGENTE: Identifica commands sin validación de sesión en FASE 0.**
```

---

## 📋 PLANTILLA G: COMMON/UTILS

```markdown
# 🛠️ SESIÓN: Optimización de Common/Utils

## Archivo

- **Módulo**: Utilidades compartidas
- **Ruta**: src/common.rs
- **Workflow**: G - Common/Utils

## Objetivo

Funciones puras compartidas con tests exhaustivos, regex optimizados, sin dependencias de capas superiores.

## Estándares Brisas APP

- **Funciones puras**: Sin efectos secundarios, determinísticas
- **Lazy<Regex>**: Compilar regex una sola vez
- **Tests obligatorios**: >90% cobertura para utilidades críticas
- **Zero dependencies**: Sin imports de services, db, commands
- **Const fn**: Cuando sea posible

## Instrucciones Estrictas

1. ✅ Leer `/mnt/user-data/outputs/workflow_g_common_utils.md` COMPLETO
2. ✅ Ejecutar FASE 0 (detectar regex inline, funciones sin tests, dependencias impuras)
3. ✅ Reportar funciones críticas sin tests
4. ✅ ESPERAR aprobación
5. ✅ Ejecutar fases 1-7 si apruebo
6. ✅ Tests con >90% cobertura

## Reglas de Oro

- ⚠️ Regex DEBEN usar `Lazy<Regex>` (performance)
- ⚠️ Tests exhaustivos: happy path, errores, casos límite
- ⚠️ Funciones de fechas: centralizadas aquí, usadas en todo el proyecto
- ⚠️ Sin lógica de negocio específica de un módulo

## Archivo Adjunto

[Adjuntar: common.rs]

---

**FASE 0: Identifica regex sin Lazy, funciones sin tests, dependencias impuras.**
```

---

## 📋 PLANTILLA H: ERRORS HIERARCHY

```markdown
# ⚠️ SESIÓN: Jerarquía de Errores

## Archivos

- src/domain/errors.rs (o errores por módulo)
- src/commands/errors.rs
- src/db/errors.rs

## Workflow

H - Errors Hierarchy

## Objetivo

Crear jerarquía idiomática: DB → Domain → Command con conversiones automáticas (From trait).

## Estándares Brisas APP

- **thiserror**: Para todos los errores custom
- **Jerarquía clara**: SurrealDbError → DomainError → CommandError (serializable)
- **Conversiones automáticas**: Implementar From trait
- **Mensajes en español**: Orientados al usuario final
- **CommandError serializable**: Para manejo en frontend TypeScript

## Instrucciones Estrictas

1. ✅ Leer `/mnt/user-data/outputs/workflow_h_errors_hierarchy.md` COMPLETO
2. ✅ Ejecutar FASE 0 (auditar errores actuales, conversiones faltantes)
3. ✅ Reportar jerarquía a implementar
4. ✅ ESPERAR aprobación
5. ✅ Ejecutar fases 1-6 si apruebo
6. ✅ Tests para conversiones críticas

## Reglas de Oro

- ⚠️ Crear TRES capas: DB, Domain, Command
- ⚠️ Implementar From<Inferior> for Superior
- ⚠️ CommandError DEBE ser serializable (Serialize de serde)
- ⚠️ No exponer detalles técnicos en CommandError

## Archivos a Revisar

[Listar archivos de errores existentes]

---

**FASE 0: Audita jerarquía actual y conversiones faltantes.**
```

---

## 📋 PLANTILLA I: CONFIGURATION

```markdown
# ⚙️ SESIÓN: Setup y Configuración de Tauri v2

## Archivo

- **Principal**: src/main.rs
- **Relacionados**: tauri.conf.json, .env, Cargo.toml

## Workflow

I - Configuration & Setup

## Objetivo

main.rs limpio (<150 líneas), plugins configurados, logging con rotación, secrets en keyring.

## Estándares Brisas APP

- **main.rs**: Solo orquestación, lógica delegada a módulos
- **Plugins Tauri v2**: Configurados (no solo defaults)
- **Logging**: tauri-plugin-log con rotación y múltiples targets
- **Secrets**: keyring (no hardcoded)
- **Variables de entorno**: Documentadas en .env.example

## Instrucciones Estrictas

1. ✅ Leer `/mnt/user-data/outputs/workflow_i_configuration_setup.md` COMPLETO
2. ✅ Ejecutar FASE 0 (auditar main.rs, plugins, logging, secrets)
3. ✅ Reportar problemas de configuración
4. ✅ ESPERAR aprobación
5. ✅ Ejecutar fases 1-7 si apruebo
6. ✅ Verificar que app inicia correctamente

## Reglas de Oro

- ⚠️ main.rs debe ser <150 líneas
- ⚠️ Setup de BD delegado a módulo service
- ⚠️ Logging con LogTarget::LogDir, Stdout, Webview
- ⚠️ Secrets NUNCA hardcoded

## Archivos Adjuntos

[Adjuntar: main.rs, tauri.conf.json]

---

**FASE 0: Audita main.rs y configuración de plugins.**
```

---

## 🎯 PROMPT ULTRA-COMPACTO (Para sesiones rápidas)

Si necesitas algo más corto:

```markdown
# SESIÓN: {Módulo} - Workflow {LETRA}

**Archivo**: {ruta}  
**Workflow**: /mnt/user-data/outputs/workflow*{letra}*{nombre}.md

## Instrucciones

1. Leer workflow COMPLETO
2. FASE 0 (análisis, NO modificar código)
3. Esperar aprobación
4. Ejecutar refactor si apruebo
5. Verificar compilación

## Reglas

- UN archivo a la vez
- Seguir workflow linealmente
- No improvisar

[Adjunta archivo]

**Comienza con FASE 0.**
```

---

## 💡 TIPS

### Para evitar olvidar instrucciones:

1. ✅ Usa estas plantillas siempre
2. ✅ Nueva sesión por cada archivo
3. ✅ Menciona "Seguir meta-workflow estrictamente"
4. ✅ Si el agente omite pasos, reinicia sesión

### Para mantener contexto entre commits:

1. ✅ Genera resumen al final de cada sesión
2. ✅ Copia resumen al inicio de la siguiente
3. ✅ Usa plantilla para "próximo archivo sugerido"

---

**Fin de Plantillas de Inicio de Sesión**
