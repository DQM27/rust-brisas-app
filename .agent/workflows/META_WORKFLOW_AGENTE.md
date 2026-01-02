---
description: 
---



## ⚠️ REGLAS DE ORO (NUNCA OLVIDAR)

### 1. SIEMPRE EJECUTAR FASE 0 PRIMERO
```
❌ MAL: "Voy a refactorizar este servicio" → [empieza a modificar código]

✅ BIEN: 
1. Leer el workflow correspondiente COMPLETAMENTE
2. Ejecutar análisis FASE 0 (view del archivo)
3. Generar reporte de hallazgos
4. ESPERAR aprobación del usuario
5. Solo entonces modificar código
```

**Por qué**: Sin análisis previo, se pierden problemas críticos y se hacen refactors incompletos.

---

### 2. UN WORKFLOW A LA VEZ
```
❌ MAL: "Voy a aplicar Workflow A, B y D al mismo tiempo"

✅ BIEN: "Aplicaré solo Workflow A. Cuando termine, esperaré siguiente instrucción"
```

**Por qué**: Mezclar workflows satura el contexto y genera confusión.

---

### 3. NO INVENTAR, SEGUIR EL WORKFLOW
```
❌ MAL: "Veo que este código necesita X, voy a agregarlo"

✅ BIEN: "Según Workflow A sección 3.2, debo verificar X. ¿Está en el checklist?"
```

**Por qué**: Los workflows son el resultado de trial & error. No improvisar.

---

### 4. REPORTAR ANTES DE ACTUAR
```
❌ MAL: [Hace cambios] "Listo, refactoricé el servicio"

✅ BIEN: 
"ANÁLISIS FASE 0:
- Problema 1: ...
- Problema 2: ...
¿Deseas que proceda con la refactorización?"
```

**Por qué**: El usuario debe aprobar los cambios antes de ejecutarlos.

---

### 5. UN ARCHIVO A LA VEZ
```
❌ MAL: "Voy a refactorizar contratista_service.rs y vehiculo_service.rs"

✅ BIEN: "Refactoraré solo contratista_service.rs. Siguiente archivo en próxima sesión."
```

**Por qué**: Un archivo grande puede tomar 30+ llamadas a tools. Dos archivos = saturación.

---

## 📋 PROTOCOLO DE EJECUCIÓN ESTÁNDAR

### Paso 1: Identificar el Workflow Correcto

```markdown
Usuario dice: "Analiza este archivo: src/services/contratista_service.rs"

Agente DEBE:
1. ✅ Identificar capa: services/ → **Workflow A**
2. ✅ Confirmar: "Usaré Workflow A - Servicios"
3. ✅ Leer workflow completo: view /mnt/skills/... (SI HAY SKILL RELEVANTE)
4. ✅ Leer workflow: view /mnt/user-data/outputs/workflow_a_servicios.md
```

**Mapeo rápido**:
- `services/*.rs` → Workflow A
- `domain/*.rs` → Workflow B
- `models/*.rs` → Workflow C
- `db/surrealdb_*_queries.rs` → Workflow D
- `commands/*_commands.rs` → Workflow E
- `common.rs` → Workflow G
- `domain/errors.rs` o errores en general → Workflow H
- `main.rs` o configuración → Workflow I

---

### Paso 2: Ejecutar FASE 0 (OBLIGATORIO)

```markdown
Agente DEBE ejecutar TODAS estas secciones del workflow:

✅ 0.1 Auditoría de [Responsabilidad/Pureza/etc según workflow]
✅ 0.2 Auditoría de [Documentación/Seguridad/etc]
✅ 0.3 Auditoría de [Manejo de Errores/...]
✅ 0.4 Auditoría de [Validación/Optimización/...]
✅ 0.5 Auditoría de [Testing/...]
✅ 0.6 [Si aplica según workflow]

Resultado: Generar archivo markdown con análisis completo
```

**Plantilla de análisis**:
```markdown
# ANÁLISIS FASE 0 - {nombre_archivo}

## PROBLEMAS CRÍTICOS (bloquean refactor)
1. [CRÍTICO] Descripción del problema
   - Impacto: ...
   - Líneas afectadas: ...
   - Esfuerzo estimado: X horas

## PROBLEMAS MAYORES
2. [ALTO] ...

## MEJORAS RECOMENDADAS
3. [MEDIO] ...

## ESTIMACIÓN TOTAL
- Críticos: X horas
- Mayores: Y horas
- **TOTAL**: Z horas

## ¿PROCEDER?
Esperando aprobación del usuario.
```

---

### Paso 3: ESPERAR Aprobación

```markdown
❌ MAL: [Después del análisis] "Procedo a refactorizar..."

✅ BIEN: "Análisis completado. ¿Deseas que proceda con:
- [ ] Refactor completo (Z horas)
- [ ] Solo críticos (X horas)
- [ ] Ajustar plan"
```

**Usuario dirá**:
- "Procede con refactor completo" → Ejecutar Fases 1-N
- "Solo críticos" → Ejecutar solo secciones marcadas CRÍTICO
- "Ajusta el plan" → Esperar nuevas instrucciones

---

### Paso 4: Ejecutar Fases de Refactorización

```markdown
Agente DEBE seguir el workflow LINEALMENTE:

Fase 1: [Según workflow]
✅ Leer sección completa del workflow
✅ Aplicar cambios siguiendo ejemplos
✅ Verificar checklist de la fase
✅ [Si es archivo grande] Mostrar progreso: "Completado 1/8 fases"

Fase 2: [Según workflow]
✅ ...

[Y así sucesivamente]
```

**IMPORTANTE**: 
- Si el archivo es >500 LOC, el agente DEBE avisar: "Este archivo es grande. Haré el refactor en chunks."
- Usar `str_replace` para cambios localizados
- Crear archivo nuevo si el refactor es >80% del código

---

### Paso 5: Verificación Final

```markdown
Agente DEBE ejecutar el checklist de "Verificación Final" del workflow:

✅ Compilación: cargo check --package mega-brisas
✅ Tests: cargo test --package mega-brisas -- {modulo}
✅ Linting: cargo clippy --package mega-brisas -- -D warnings
✅ Formato: cargo fmt

Si hay errores:
- ❌ NO entregar el archivo
- ✅ Corregir errores
- ✅ Verificar nuevamente
```

---

### Paso 6: Entrega y Commit

```markdown
Agente DEBE:
1. ✅ Mover archivo refactorizado a /mnt/user-data/outputs/
2. ✅ Usar present_files para mostrarlo
3. ✅ Generar mensaje de commit usando plantilla del workflow
4. ✅ Listar archivos modificados:
   - src/services/contratista_service.rs (refactorizado)
   - [Otros si aplica]
```

**Plantilla de mensaje de commit** (según workflow usado):
```
refactor(services): refactorizar contratista_service según Workflow A

- [Lista de cambios del checklist de verificación final]

Closes #{numero_issue}
```

---

## 🧠 GESTIÓN DE MEMORIA Y CONTEXTO

### Problema: Saturación de Contexto

Cuando el chat es muy largo (>50K tokens), el agente puede:
- ❌ Olvidar instrucciones iniciales
- ❌ Mezclar información de diferentes módulos
- ❌ Omitir pasos del workflow
- ❌ Perder el hilo de la tarea actual

### Solución: Estrategia de Sesiones Modulares

```markdown
📅 SESIÓN 1: Contratista (Servicio)
├── Aplicar Workflow A a contratista_service.rs
├── Commit
└── FIN DE SESIÓN ✅

🔄 [Usuario inicia NUEVA sesión]

📅 SESIÓN 2: Contratista (Queries)  
├── Aplicar Workflow D a surrealdb_contratista_queries.rs
├── Commit
└── FIN DE SESIÓN ✅

🔄 [Usuario inicia NUEVA sesión]

📅 SESIÓN 3: Contratista (Commands)
├── Aplicar Workflow E a contratista_commands.rs
├── Commit  
└── FIN DE SESIÓN ✅
```

**Beneficios**:
- ✅ Contexto fresco en cada sesión
- ✅ Agente enfocado en una sola tarea
- ✅ Historial más limpio
- ✅ Menos errores por sobrecarga

---

## 📝 PLANTILLA DE INICIO DE SESIÓN

### Para el Usuario (copiar y pegar al inicio de cada sesión)

```markdown
# NUEVA SESIÓN: Refactorización de {Módulo}

## Contexto
- Proyecto: Brisas APP (Rust + Tauri v2)
- Módulo: {nombre del módulo, ej: Contratista}
- Archivo a refactorizar: {ruta completa}
- Workflow a usar: {A/B/C/D/E/G/H/I}

## Archivos Adjuntos
1. {archivo_a_refactorizar.rs}
2. [Workflows relevantes ya están en /mnt/user-data/outputs/]

## Instrucciones para el Agente
1. Leer workflow correspondiente desde /mnt/user-data/outputs/
2. Ejecutar FASE 0 (análisis sin modificar código)
3. Generar reporte de hallazgos
4. ESPERAR mi aprobación antes de modificar código
5. Seguir protocolo de ejecución estándar

## Reglas Estrictas
- ⚠️ NO modificar código hasta que yo apruebe
- ⚠️ UN ARCHIVO a la vez
- ⚠️ Seguir workflow al pie de la letra
- ⚠️ Verificar compilación antes de entregar

## Estándares de Brisas APP
- Documentación: Español, explicar "por qué"
- Fechas: RFC 3339 para timestamps, YYYY-MM-DD para fechas simples
- Logging: tauri-plugin-log con info!/warn!/error!
- Errores: thiserror con mensajes en español

¿Listo para comenzar?
```

---

## 🎯 CHECKLIST DEL AGENTE (Auto-verificación)

Antes de cada respuesta, el agente DEBE preguntarse:

### ¿Estoy siguiendo el protocolo?
- [ ] ¿Leí el workflow completo antes de empezar?
- [ ] ¿Ejecuté FASE 0 completamente?
- [ ] ¿Esperé aprobación antes de modificar código?
- [ ] ¿Estoy trabajando solo en UN archivo?
- [ ] ¿Estoy siguiendo el workflow linealmente (Fase 1 → 2 → 3...)?

### ¿Estoy respetando las reglas de oro?
- [ ] ¿Estoy aplicando solo UN workflow?
- [ ] ¿Estoy reportando antes de actuar?
- [ ] ¿Estoy siguiendo el checklist del workflow?
- [ ] ¿NO estoy improvisando ni inventando pasos?

### ¿Voy a entregar calidad?
- [ ] ¿Ejecuté los comandos de verificación (cargo check, test, clippy)?
- [ ] ¿El código compila sin errores?
- [ ] ¿Generé mensaje de commit usando plantilla?
- [ ] ¿Moví archivo a /mnt/user-data/outputs/?

---

## 🚨 SEÑALES DE ALERTA (Agente debe detenerse)

Si el agente detecta alguna de estas situaciones, DEBE DETENERSE y reportar:

### 1. Archivo demasiado grande
```
"⚠️ ALERTA: Este archivo tiene {N} líneas. 
Recomiendo dividir el refactor en múltiples sesiones:
- Sesión 1: Fases 1-3
- Sesión 2: Fases 4-6
¿Cómo deseas proceder?"
```

### 2. Múltiples workflows aplicables
```
"⚠️ ALERTA: Este archivo parece necesitar Workflow A y Workflow B.
Recomiendo aplicarlos en sesiones separadas:
- Sesión 1: Workflow A
- Sesión 2: Workflow B
¿Cuál aplicamos primero?"
```

### 3. Dependencias circulares
```
"⚠️ ALERTA: Para refactorizar este servicio, primero necesito:
- Refactorizar {otro_archivo.rs}
- O ajustar {dependencia.rs}
¿Deseas que abordemos las dependencias primero?"
```

### 4. Errores de compilación
```
"⚠️ ALERTA: El código refactorizado no compila:
{error de compilación}

Necesito corregir esto antes de continuar. Dame un momento..."
```

---

## 📊 PROGRESO Y TRACKING

### Para mantener track entre sesiones

El agente DEBE generar al final de cada sesión:

```markdown
# RESUMEN DE SESIÓN - {Fecha}

## Completado
✅ Módulo: Contratista
✅ Archivo: src/services/contratista_service.rs
✅ Workflow: A - Servicios
✅ Fases ejecutadas: 1-8 (todas)
✅ Compilación: ✅ OK
✅ Tests: ✅ OK
✅ Commit: ✅ Listo

## Próxima Sesión Sugerida
📌 Módulo: Contratista
📌 Archivo: src/db/surrealdb_contratista_queries.rs
📌 Workflow: D - Queries SurrealDB
📌 Duración estimada: 5-7 horas

## Archivos Pendientes del Módulo Contratista
- [ ] src/db/surrealdb_contratista_queries.rs (Workflow D)
- [ ] src/commands/contratista_commands.rs (Workflow E)
- [ ] src/domain/contratista.rs (Workflow B)
- [ ] src/models/contratista.rs (Workflow C)
```

---

## 🎓 EJEMPLO COMPLETO: Sesión Ideal

### Usuario inicia sesión:
```
# NUEVA SESIÓN: Refactorización de Contratista (Servicio)

Archivo: src/services/contratista_service.rs
Workflow: A

[Adjunta archivo]

Aplica el protocolo estándar.
```

### Agente responde:

```markdown
Entendido. Usaré **Workflow A - Servicios**.

## Plan de Ejecución
1. Leer workflow completo ✅
2. Ejecutar FASE 0 (análisis)
3. Reportar hallazgos
4. Esperar aprobación
5. Ejecutar refactorización
6. Verificar compilación
7. Entregar código refactorizado

Comenzando con FASE 0...
```

[Ejecuta análisis]

```markdown
# ANÁLISIS FASE 0 - contratista_service.rs

## PROBLEMAS CRÍTICOS
1. [CRÍTICO] Acceso directo a queries (líneas 45, 67, 89)
   - Viola Clean Architecture
   - Impacto: Testing imposible
   - Esfuerzo: 3-4 horas

2. [CRÍTICO] Operaciones sin transacción (líneas 67-89)
   - Riesgo de inconsistencia
   - Esfuerzo: 2 horas

## ESTIMACIÓN TOTAL: 9-12 horas

¿Deseas que proceda con el refactor completo?
```

### Usuario aprueba:
```
Sí, procede con refactor completo.
```

### Agente ejecuta:
```markdown
Entendido. Ejecutando Fases 1-8...

[Progreso: Fase 1/8] Eliminando acceso directo a queries...
[Progreso: Fase 2/8] Implementando gestión transaccional...
...
[Progreso: Fase 8/8] Verificación final...

✅ Refactorización completada
✅ Compilación: OK
✅ Tests: OK

[present_files: contratista_service.rs]
```

---

## 💡 TIPS PARA EL USUARIO

### Cómo formular prompts claros

#### ✅ BUENOS PROMPTS:
```
"Analiza este servicio con Workflow A"
"Aplica Workflow D a las queries de contratista"
"FASE 0 solamente, no modifiques código"
"Procede con refactor completo"
"Solo fixes críticos por ahora"
```

#### ❌ PROMPTS AMBIGUOS:
```
"Mejora este código" (¿qué workflow? ¿qué criterios?)
"Hazlo mejor" (demasiado vago)
"Arregla todo" (demasiado amplio)
```

---

## 🔄 CUANDO REINICIAR SESIÓN

### Reiniciar cuando:
- ✅ Se completó refactor de 1 archivo (hacer commit y nueva sesión)
- ✅ El chat tiene >40K tokens
- ✅ El agente empieza a omitir pasos
- ✅ Se cambió de módulo (Contratista → Usuario)

### NO reiniciar cuando:
- ❌ Estás a mitad de un refactor
- ❌ El agente está en FASE 0 (esperar reporte)
- ❌ Hay errores de compilación por corregir

---

## 📚 DOCUMENTOS DE REFERENCIA

El agente tiene acceso a estos archivos:

### Workflows:
- `/mnt/user-data/outputs/workflow_a_servicios.md`
- `/mnt/user-data/outputs/workflow_b_dominio.md`
- `/mnt/user-data/outputs/workflow_c_modelos.md`
- `/mnt/user-data/outputs/workflow_d_queries_surrealdb.md`
- `/mnt/user-data/outputs/workflow_e_commands_tauri.md`
- `/mnt/user-data/outputs/workflow_g_common_utils.md`
- `/mnt/user-data/outputs/workflow_h_errors_hierarchy.md`
- `/mnt/user-data/outputs/workflow_i_configuration_setup.md`

### Análisis previos:
- `/mnt/user-data/outputs/analisis_contratista_service.md`
- `/mnt/user-data/outputs/analisis_contratista_queries.md`
- `/mnt/user-data/outputs/analisis_ingreso_general_commands.md`

### Índice:
- `/mnt/user-data/outputs/INDICE_WORKFLOWS_COMPLETO.md`

---

## ✅ RESUMEN FINAL

### El agente SIEMPRE debe:
1. ✅ Leer workflow completo antes de empezar
2. ✅ Ejecutar FASE 0 primero (análisis)
3. ✅ Esperar aprobación del usuario
4. ✅ Trabajar en UN archivo a la vez
5. ✅ Seguir workflow linealmente
6. ✅ Verificar compilación antes de entregar
7. ✅ Generar mensaje de commit con plantilla
8. ✅ Auto-verificarse con checklist

### El agente NUNCA debe:
1. ❌ Modificar código sin análisis previo
2. ❌ Improvisar o inventar pasos
3. ❌ Trabajar en múltiples archivos simultáneamente
4. ❌ Mezclar múltiples workflows
5. ❌ Entregar código que no compile
6. ❌ Omitir pasos del workflow
7. ❌ Continuar si detecta señales de alerta

---

**Fin del Meta-Workflow. Este documento debe ser compartido al inicio de cada sesión de refactorización.**
