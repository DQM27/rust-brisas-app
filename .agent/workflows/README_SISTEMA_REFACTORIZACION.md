# 📚 SISTEMA DE REFACTORIZACIÓN BRISAS APP

**Versión**: 1.0  
**Fecha**: 2026-01-02  
**Propósito**: Documentación completa para refactorizar Brisas APP a estándares Enterprise con Rust idiomático + Tauri v2  

---

## 🎯 ¿Qué es esto?

Un **sistema completo de workflows** para refactorizar tu aplicación Tauri (Brisas APP) con:
- ✅ Estándares profesionales
- ✅ Patrones idiomáticos de Rust (no OOP)
- ✅ Clean Architecture adaptada a Rust
- ✅ Testing obligatorio
- ✅ Documentación en español

---

## 📦 Contenido del Paquete

### 🎯 DOCUMENTOS DE ORQUESTACIÓN (LEER PRIMERO)

1. **📖 README.md** ← Estás aquí
   - Punto de entrada de toda la documentación

2. **🎯 META_WORKFLOW_AGENTE.md** ⚠️ CRÍTICO
   - Instrucciones estrictas para el agente IA
   - Reglas de oro que NUNCA debe olvidar
   - Protocolo de ejecución paso a paso
   - Auto-checklist para el agente

3. **📝 PLANTILLAS_INICIO_SESION.md** ⚠️ USAR SIEMPRE
   - Plantillas listas para copiar-pegar
   - Una plantilla por cada workflow (A-I)
   - Contexto fresco en cada sesión

4. **📊 GUIA_GESTION_SESIONES.md**
   - Estrategia de sesiones modulares
   - Cómo evitar saturación de contexto
   - Troubleshooting común

5. **📋 INDICE_WORKFLOWS_COMPLETO.md**
   - Catálogo de los 8 workflows
   - Matriz de prioridades
   - Roadmap sugerido
   - Casos de uso rápidos

---

### 🏗️ WORKFLOWS TÉCNICOS (8 workflows)

| ID | Workflow | Cuándo Usar | Prioridad |
|----|----------|-------------|-----------|
| **A** | `workflow_a_servicios.md` | Archivos en `services/` | ⭐⭐⭐⭐⭐ |
| **B** | `workflow_b_dominio.md` | Archivos en `domain/` | ⭐⭐⭐⭐ |
| **C** | `workflow_c_modelos.md` | Archivos en `models/` | ⭐⭐ |
| **D** | `workflow_d_queries_surrealdb.md` | Archivos en `db/` | ⭐⭐⭐ |
| **E** | `workflow_e_commands_tauri.md` | Archivos en `commands/` | ⭐⭐⭐⭐⭐ SEGURIDAD |
| **G** | `workflow_g_common_utils.md` | `common.rs` | ⭐⭐⭐ |
| **H** | `workflow_h_errors_hierarchy.md` | Errores en general | ⭐⭐⭐⭐ |
| **I** | `workflow_i_configuration_setup.md` | `main.rs`, config | ⭐⭐ |

---

### 📊 ANÁLISIS PREVIOS (3 archivos ya auditados)

1. **analisis_contratista_service.md** - Servicio con problemas críticos (16-20h)
2. **analisis_contratista_queries.md** - Queries sin documentación (5-7h)
3. **analisis_ingreso_general_commands.md** - 🔴 URGENTE: 8/9 sin autenticación

---

## 🚀 INICIO RÁPIDO

### Para el Usuario (Tú):

#### 1️⃣ **Leer documentos de orquestación** (30 min)
```
1. Este README (estás aquí) ✅
2. GUIA_GESTION_SESIONES.md → Estrategia
3. PLANTILLAS_INICIO_SESION.md → Plantillas listas
4. META_WORKFLOW_AGENTE.md → Qué esperar del agente
```

#### 2️⃣ **Identificar primer archivo a refactorizar**
```bash
# Ejemplo: Servicio de contratistas
src/services/contratista_service.rs → Workflow A
```

#### 3️⃣ **Abrir NUEVO chat con Claude**
```markdown
[Copiar plantilla A de PLANTILLAS_INICIO_SESION.md]
[Reemplazar {variables}]
[Adjuntar contratista_service.rs]

**⚠️ Seguir META_WORKFLOW_AGENTE.md estrictamente**
```

#### 4️⃣ **Esperar análisis FASE 0**
```
Claude ejecuta análisis → Genera reporte → Espera tu aprobación
```

#### 5️⃣ **Aprobar y ejecutar refactor**
```
Tú: "Procede con refactor completo"
Claude: [Refactoriza siguiendo workflow]
```

#### 6️⃣ **Verificar y commitear**
```bash
cargo check --package mega-brisas
cargo test --package mega-brisas
git commit -m "refactor(services): mensaje generado por Claude"
```

#### 7️⃣ **Cerrar sesión y repetir**
```
[Cerrar chat actual]
[Abrir NUEVO chat para siguiente archivo]
```

---

### Para el Agente (Claude):

#### Protocolo Estricto:

```markdown
1. Leer workflow correspondiente COMPLETO ✅
2. Ejecutar FASE 0 (análisis sin modificar código) ✅
3. Generar reporte de hallazgos ✅
4. ESPERAR aprobación del usuario ✅
5. Ejecutar fases 1-N linealmente ✅
6. Verificar compilación ✅
7. Entregar archivo refactorizado ✅
```

**Reglas de Oro** (ver META_WORKFLOW_AGENTE.md):
- ⚠️ SIEMPRE FASE 0 primero
- ⚠️ UN workflow a la vez
- ⚠️ NO improvisar
- ⚠️ Reportar antes de actuar
- ⚠️ UN archivo a la vez

---

## 📋 ESTRATEGIA RECOMENDADA

### Fase 1: Seguridad (URGENTE - Semana 1)

🔴 **Prioridad CRÍTICA**
```
1. Revisar analisis_ingreso_general_commands.md
2. Aplicar Workflow E a TODOS los commands
3. Agregar validación de sesión
```

**Estimación**: 2-3 días  
**Riesgo actual**: Datos sensibles expuestos sin autenticación

---

### Fase 2: Arquitectura Base (Semanas 2-3)

```
Módulo por módulo (ej: Contratista):

1. Servicio (Workflow A) → 1 sesión → commit
2. Queries (Workflow D) → 1 sesión → commit
3. Commands (Workflow E) → 1 sesión → commit
4. Dominio (Workflow B) → 1 sesión → commit
5. Modelos (Workflow C) → 1 sesión → commit
```

**Estimación por módulo**: 5-7 sesiones (1-2 semanas)

---

### Fase 3: Infraestructura (Semana 4)

```
1. Common/Utils (Workflow G) → 1 sesión
2. Errors Hierarchy (Workflow H) → 2 sesiones
3. Configuration (Workflow I) → 1 sesión
```

---

## 🎯 REGLA DE ORO

```
1 Sesión = 1 Archivo = 1 Commit = Chat Nuevo
```

### ¿Por qué?
- ✅ Contexto fresco (agente no se pierde)
- ✅ Commits atómicos (fácil de revertir)
- ✅ Historial limpio (fácil de buscar)
- ✅ Sin saturación de memoria

---

## 📊 EJEMPLO DE ROADMAP (Módulo Contratista)

### Semana 1: Contratista

```
📅 Lunes AM:
  Chat 1: contratista_service.rs (Workflow A)
  ├── FASE 0 → Análisis
  ├── Aprobación
  ├── Refactor (Fases 1-9)
  ├── Commit: abc123
  └── FIN

📅 Lunes PM:
  Chat 2: surrealdb_contratista_queries.rs (Workflow D)
  ├── FASE 0 → Análisis
  ├── Aprobación
  ├── Refactor (Fases 1-8)
  ├── Commit: def456
  └── FIN

📅 Martes AM:
  Chat 3: contratista_commands.rs (Workflow E)
  ├── FASE 0 → ⚠️ CRÍTICO: Sin auth
  ├── Aprobación → "URGENTE: Seguridad primero"
  ├── Refactor (Fases 1-7)
  ├── Commit: ghi789
  └── FIN

📅 Martes PM:
  Chat 4: domain/contratista.rs (Workflow B)
  └── [Mismo proceso]

📅 Miércoles AM:
  Chat 5: models/contratista.rs (Workflow C)
  └── [Mismo proceso]
```

**Resultado**: Módulo Contratista completamente refactorizado en 2.5 días

---

## 🛡️ CHECKLIST DE CALIDAD

Después de refactorizar cada archivo:

### Compilación ✅
```bash
cargo check --package mega-brisas
cargo clippy --package mega-brisas -- -D warnings
```

### Tests ✅
```bash
cargo test --package mega-brisas -- {modulo}
```

### Documentación ✅
- [ ] Todas las funciones públicas tienen `///`
- [ ] Idioma español
- [ ] Explica el "por qué"
- [ ] Ejemplos de uso

### Estándares ✅
- [ ] Fechas: RFC 3339 o YYYY-MM-DD según corresponda
- [ ] Logging: `log::info!`, `log::warn!`, `log::error!`
- [ ] Errores: `thiserror` con mensajes descriptivos
- [ ] Separadores visuales: `// ----------`

---

## 🆘 TROUBLESHOOTING

### "El agente se está perdiendo / omitiendo pasos"

**Solución INMEDIATA**:
```markdown
Tú en el chat: "ALTO. Lee META_WORKFLOW_AGENTE.md y reinicia desde FASE 0"
```

**Solución DEFINITIVA**:
1. Cerrar chat actual
2. Abrir NUEVO chat
3. Copiar plantilla fresca de PLANTILLAS_INICIO_SESION.md
4. Agregar explícitamente: "Seguir META_WORKFLOW_AGENTE.md estrictamente"

---

### "No sé qué workflow usar"

**Referencia rápida**:
```
src/services/*.rs         → Workflow A
src/domain/*.rs          → Workflow B
src/models/*.rs          → Workflow C
src/db/surrealdb_*.rs    → Workflow D
src/commands/*.rs        → Workflow E
src/common.rs            → Workflow G
src/domain/errors.rs     → Workflow H
src/main.rs              → Workflow I
```

Ver INDICE_WORKFLOWS_COMPLETO.md para más detalles.

---

### "El agente modificó código sin mi aprobación"

**Problema**: El agente se saltó FASE 0.

**Solución**:
```markdown
Tú: "No aprobé cambios. Revierte y ejecuta FASE 0 primero."
```

O mejor: **Reiniciar sesión** con plantilla que enfatice:
```markdown
**🚨 OBLIGATORIO: FASE 0 PRIMERO 🚨**
NO modificar código hasta mi "Procede" explícito.
```

---

### "¿Cómo trackeo el progreso?"

**Solución**: Crea `REFACTOR_PROGRESS.md` en tu repo:

```markdown
# Progreso de Refactorización

## Módulo: Contratista
- [x] services/contratista_service.rs (abc123)
- [x] db/surrealdb_contratista_queries.rs (def456)
- [ ] commands/contratista_commands.rs
- [ ] domain/contratista.rs
- [ ] models/contratista.rs

## Módulo: Usuario
- [ ] ...
```

---

## 📚 DOCUMENTOS POR ROL

### Si eres el que EJECUTA el refactor:
```
1. GUIA_GESTION_SESIONES.md (LEER PRIMERO)
2. PLANTILLAS_INICIO_SESION.md (USAR EN CADA SESIÓN)
3. INDICE_WORKFLOWS_COMPLETO.md (REFERENCIA)
```

### Si eres el AGENTE IA:
```
1. META_WORKFLOW_AGENTE.md (INSTRUCCIONES ESTRICTAS)
2. Workflow específico (A/B/C/D/E/G/H/I según archivo)
```

### Si quieres ENTENDER el sistema:
```
1. Este README (overview general)
2. INDICE_WORKFLOWS_COMPLETO.md (catálogo completo)
3. META_WORKFLOW_AGENTE.md (reglas del juego)
```

---

## ✅ RESULTADO ESPERADO

Después de aplicar todos los workflows a Brisas APP:

### Arquitectura ✅
- Clean Architecture idiomática en Rust
- Separación clara de responsabilidades
- Testing completo (unitarios + integración)

### Código ✅
- Type-safe (enums sobre strings)
- Funciones puras en dominio
- Errores descriptivos con jerarquía
- Documentación exhaustiva en español

### Seguridad ✅
- Validación de sesión en commands críticos
- Validación de inputs
- Secrets en keyring (no hardcoded)
- Logging de auditoría

### Performance ✅
- Queries optimizados (LIMIT, FETCH, índices)
- Regex compilados con `Lazy<>`
- Transacciones donde sea necesario

---

## 🎓 PRÓXIMOS PASOS

### Ahora mismo:
```
1. ✅ Leer GUIA_GESTION_SESIONES.md
2. ✅ Identificar primer archivo (sugiero: commands con análisis CRÍTICO)
3. ✅ Abrir NUEVO chat
4. ✅ Copiar plantilla E
5. ✅ Comenzar refactor
```

### Esta semana:
```
- Refactorizar todos los commands (seguridad)
- Módulo Contratista completo
- Actualizar REFACTOR_PROGRESS.md
```

### Este mes:
```
- 3-4 módulos principales
- Common/Utils optimizado
- Jerarquía de errores implementada
```

---

## 📞 SOPORTE

Si tienes dudas:

1. **Busca en documentación**: La respuesta probablemente está en alguno de los 15 documentos
2. **Revisa análisis previos**: Los 3 análisis FASE 0 tienen ejemplos reales
3. **Verifica checklist**: Cada workflow tiene checklist de verificación final

---

## 📦 INVENTARIO COMPLETO

```
📁 SISTEMA DE REFACTORIZACIÓN BRISAS APP/
│
├── 📖 README.md (este archivo) ← INICIO
│
├── 🎯 ORQUESTACIÓN (4 docs)
│   ├── META_WORKFLOW_AGENTE.md
│   ├── PLANTILLAS_INICIO_SESION.md
│   ├── GUIA_GESTION_SESIONES.md
│   └── INDICE_WORKFLOWS_COMPLETO.md
│
├── 🏗️ WORKFLOWS TÉCNICOS (8 docs)
│   ├── workflow_a_servicios.md
│   ├── workflow_b_dominio.md
│   ├── workflow_c_modelos.md
│   ├── workflow_d_queries_surrealdb.md
│   ├── workflow_e_commands_tauri.md
│   ├── workflow_g_common_utils.md
│   ├── workflow_h_errors_hierarchy.md
│   └── workflow_i_configuration_setup.md
│
└── 📊 ANÁLISIS PREVIOS (3 docs)
    ├── analisis_contratista_service.md
    ├── analisis_contratista_queries.md
    └── analisis_ingreso_general_commands.md
```

**Total**: 15 documentos listos para usar

---

## 🚀 EMPECEMOS

```markdown
# Copia esto y pégalo en un NUEVO chat:

# 🔧 SESIÓN: Refactorización de Commands (URGENTE - Seguridad)

## Archivo
- **Módulo**: Ingreso General
- **Ruta**: src/commands/ingreso_general_commands.rs
- **Workflow**: E - Commands Tauri

[... resto de plantilla E ...]

**⚠️ CRÍTICO: 8/9 commands sin autenticación (ver análisis previo)**
**⚠️ Seguir META_WORKFLOW_AGENTE.md estrictamente**

[Adjunta: ingreso_general_commands.rs]
```

---

**¡Sistema completo entregado!** 🎉  
**Todo listo para refactorizar Brisas APP a nivel Enterprise** ⭐⭐⭐⭐⭐
