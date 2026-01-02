# 🎯 GUÍA RÁPIDA: Gestión de Sesiones y Contexto

**Para**: Usuario de Brisas APP  
**Propósito**: Cómo trabajar con el agente sin que se pierda u olvide instrucciones  
**Versión**: 1.0  

---

## ⚡ TL;DR (Resumen Ultra-Rápido)

```
1️⃣ UN archivo por sesión
2️⃣ NUEVA sesión después de cada commit
3️⃣ Copiar plantilla al inicio
4️⃣ Adjuntar solo el archivo a refactorizar
5️⃣ Decir "Seguir meta-workflow estrictamente"
```

---

## 🚨 Problema: ¿Por qué el agente se pierde?

### Síntomas comunes:
- ❌ Omite pasos del workflow
- ❌ Mezcla información de diferentes módulos
- ❌ Olvida hacer FASE 0
- ❌ Modifica código sin esperar aprobación
- ❌ Improvisa en lugar de seguir el workflow

### Causa raíz:
**Saturación de contexto** - Cuando el chat es muy largo (>50K tokens), el agente pierde el hilo.

---

## ✅ Solución: Estrategia de Sesiones Modulares

### Concepto Clave
> **1 Sesión = 1 Archivo = 1 Commit**

```
📅 SESIÓN 1
├── Archivo: contratista_service.rs
├── Workflow: A
├── Análisis + Refactor
├── Commit ✅
└── FIN → CERRAR CHAT

🆕 NUEVA SESIÓN (Chat limpio)
├── Archivo: contratista_queries.rs
├── Workflow: D
├── Análisis + Refactor
├── Commit ✅
└── FIN → CERRAR CHAT

🆕 NUEVA SESIÓN
├── ...
```

### Beneficios:
- ✅ Contexto fresco cada vez
- ✅ Agente enfocado en UNA tarea
- ✅ Sin saturación de memoria
- ✅ Historial limpio y buscable
- ✅ Fácil de retomar si se interrumpe

---

## 📋 PROTOCOLO SESIÓN POR SESIÓN

### ANTES de iniciar sesión:

```bash
# 1. Identifica el archivo
src/services/contratista_service.rs

# 2. Identifica el workflow
services/ → Workflow A

# 3. ¿El archivo anterior ya está commiteado?
git status  # Debe estar limpio

# 4. Prepara la plantilla
# Copia PLANTILLAS_INICIO_SESION.md > Plantilla A
```

---

### INICIO de sesión (Chat nuevo):

```markdown
[Copiar plantilla completa de PLANTILLAS_INICIO_SESION.md]

# Agregar al final:
**⚠️ Instrucción adicional: Seguir META_WORKFLOW_AGENTE.md estrictamente**

[Adjuntar archivo a refactorizar]
```

**Ejemplo concreto**:
```markdown
# 🔧 SESIÓN: Refactorización de Servicio

## Archivo
- **Módulo**: Contratista
- **Ruta**: src/services/contratista_service.rs
- **Workflow**: A - Servicios

[... resto de plantilla ...]

**⚠️ Seguir META_WORKFLOW_AGENTE.md estrictamente**

[Adjunta: contratista_service.rs]
```

---

### DURANTE la sesión:

#### Fase 1: Análisis (FASE 0)
```
Agente ejecuta análisis →
Genera reporte de hallazgos →
TÚ REVISAS →
Decides: "Procede" o "Solo críticos" o "Ajusta plan"
```

#### Fase 2: Refactorización
```
Agente ejecuta fases 1-N del workflow →
Muestra progreso si es archivo grande →
Entrega archivo refactorizado →
TÚ VERIFICAS localmente
```

#### Fase 3: Commit
```
git add src/services/contratista_service.rs
git commit -m "refactor(services): mensaje del agente"
git push
```

---

### FIN de sesión:

```markdown
TÚ dices: "Perfecto, commit hecho. Siguiente archivo: contratista_queries.rs"

Agente genera resumen:
---
## RESUMEN DE SESIÓN
✅ Archivo: contratista_service.rs
✅ Workflow: A
✅ Commit: Hecho

## PRÓXIMA SESIÓN SUGERIDA
📌 Archivo: src/db/surrealdb_contratista_queries.rs
📌 Workflow: D - Queries
---

TÚ: "Gracias, cerrando esta sesión"
[CERRAR EL CHAT]
```

---

## 🎯 Ejemplo Completo: 3 Sesiones para Módulo Contratista

### 🗓️ Lunes - Sesión 1: Servicio

**Chat nuevo** → Copiar plantilla A → Adjuntar contratista_service.rs

```
Agente: Análisis FASE 0
Tú: "Procede con refactor completo"
Agente: [Refactoriza]
Tú: [Verifica y hace commit]
→ CERRAR CHAT
```

---

### 🗓️ Martes - Sesión 2: Queries

**Chat nuevo** → Copiar plantilla D → Adjuntar surrealdb_contratista_queries.rs

```
Agente: Análisis FASE 0
Tú: "Solo críticos por ahora"
Agente: [Refactoriza solo críticos]
Tú: [Verifica y hace commit]
→ CERRAR CHAT
```

---

### 🗓️ Miércoles - Sesión 3: Commands

**Chat nuevo** → Copiar plantilla E → Adjuntar contratista_commands.rs

```
Agente: Análisis FASE 0
Tú: "Procede, pero URGENTE: validación de sesión"
Agente: [Refactoriza con foco en seguridad]
Tú: [Verifica y hace commit]
→ CERRAR CHAT
```

---

## 🛡️ Cómo Detectar que el Agente se Está Perdiendo

### Señales de alerta:

1. **Omite FASE 0**
```
❌ Agente: "Voy a refactorizar el servicio..."
✅ Debería: "Ejecutando FASE 0. Leyendo workflow..."
```

2. **Modifica código sin aprobación**
```
❌ Agente: [Presenta código modificado]
✅ Debería: "Análisis completo. ¿Deseas que proceda?"
```

3. **Mezcla workflows**
```
❌ Agente: "Voy a aplicar Workflow A y también veo que necesita Workflow B..."
✅ Debería: "Aplicaré solo Workflow A según instrucciones"
```

4. **Improvisa pasos**
```
❌ Agente: "Veo que también podríamos mejorar X..."
✅ Debería: "Siguiendo paso 3.2 del workflow..."
```

### Qué hacer si detectas señales:

```markdown
Tú: "ALTO. Estás omitiendo pasos del workflow. 

Por favor:
1. Lee META_WORKFLOW_AGENTE.md
2. Vuelve a FASE 0
3. Sigue el protocolo estrictamente"
```

O más simple: **REINICIA LA SESIÓN** (chat nuevo con plantilla).

---

## 📊 Tracking de Progreso Entre Sesiones

### Crea un checklist en tu repo:

```markdown
# REFACTOR_PROGRESS.md

## Módulo: Contratista

### Archivos
- [x] src/services/contratista_service.rs (Workflow A) - Commit: abc123
- [x] src/db/surrealdb_contratista_queries.rs (Workflow D) - Commit: def456
- [ ] src/commands/contratista_commands.rs (Workflow E) - Pendiente
- [ ] src/domain/contratista.rs (Workflow B) - Pendiente
- [ ] src/models/contratista.rs (Workflow C) - Pendiente

## Módulo: Usuario

### Archivos
- [ ] src/services/usuario_service.rs (Workflow A)
- [ ] ...

## Configuración Global

- [ ] src/main.rs (Workflow I)
- [ ] src/common.rs (Workflow G)
- [ ] src/domain/errors.rs (Workflow H)
```

---

## 🎓 Mejores Prácticas

### ✅ HACER:

1. **Nueva sesión por archivo**
```bash
# Después de cada commit
git commit -m "..."
# Cerrar chat actual
# Abrir nuevo chat con plantilla
```

2. **Plantillas siempre**
```markdown
# No improvises el prompt
# Usa las plantillas de PLANTILLAS_INICIO_SESION.md
```

3. **Mencionar meta-workflow**
```markdown
**⚠️ Seguir META_WORKFLOW_AGENTE.md estrictamente**
```

4. **Verificar localmente antes de commit**
```bash
cargo check --package mega-brisas
cargo clippy --package mega-brisas -- -D warnings
cargo test --package mega-brisas
```

5. **Commits atómicos**
```bash
# Un commit por archivo refactorizado
git add src/services/contratista_service.rs
git commit -m "refactor(services): refactorizar contratista_service según Workflow A"
```

---

### ❌ NO HACER:

1. **Múltiples archivos en una sesión**
```
❌ "Refactoriza contratista_service.rs y usuario_service.rs"
✅ "Refactoriza contratista_service.rs" → commit → nueva sesión para usuario
```

2. **Sesiones largas**
```
❌ Una sesión de 3 horas con 5 archivos
✅ 5 sesiones de 30-60 min cada una
```

3. **Mezclar workflows**
```
❌ "Aplica Workflow A y B al mismo tiempo"
✅ "Aplica solo Workflow A" → commit → "Ahora Workflow B"
```

4. **Improvisar prompts**
```
❌ "Mejora este código"
✅ [Usar plantilla con workflow específico]
```

5. **Continuar si el agente se pierde**
```
❌ Intentar corregir al agente en la misma sesión
✅ Cerrar chat, abrir nuevo, copiar plantilla
```

---

## 💾 Plantilla de Resumen de Sesión (Para trackear)

Al final de cada sesión, el agente genera esto. Tú lo copias a `REFACTOR_PROGRESS.md`:

```markdown
---
## SESIÓN: 2026-01-02 14:30

**Archivo**: src/services/contratista_service.rs  
**Workflow**: A - Servicios  
**Estado**: ✅ Completado  
**Commit**: abc1234  
**Duración**: 45 minutos  

### Problemas Resueltos
- ✅ Eliminado acceso directo a queries
- ✅ Agregadas transacciones
- ✅ Implementado logging estructurado

### Próximo Archivo Sugerido
📌 src/db/surrealdb_contratista_queries.rs (Workflow D)
---
```

---

## 🆘 Troubleshooting

### "El agente sigue olvidando hacer FASE 0"

**Solución**: Agrega al inicio de la plantilla:

```markdown
**🚨 OBLIGATORIO: FASE 0 PRIMERO 🚨**

NO modificar código hasta que yo apruebe explícitamente.

1. Leer workflow COMPLETO
2. Análisis FASE 0
3. Generar reporte
4. ESPERAR mi "Procede"
```

---

### "El agente mezcla información de sesiones anteriores"

**Causa**: Sesión demasiado larga o no cerraste el chat.

**Solución**:
1. Cerrar chat actual
2. Abrir NUEVO chat
3. Copiar plantilla fresca
4. NO mencionar archivos anteriores

---

### "El agente improvisa pasos que no están en el workflow"

**Solución inmediata**:
```markdown
Tú: "ALTO. No improvises.

Cita textualmente el paso del workflow que estás ejecutando.
Ejemplo: 'Ejecutando Workflow A, Fase 3, Sección 3.2: Validación de Inputs'"
```

**Solución preventiva**: Agregar a plantilla:
```markdown
**⚠️ NO improvisar. Citar secciones del workflow textualmente.**
```

---

### "No sé en qué sesión voy"

**Solución**: Mantén `REFACTOR_PROGRESS.md` actualizado después de cada commit.

---

## 📚 Documentos de Referencia

Tienes estos 3 documentos clave:

1. **META_WORKFLOW_AGENTE.md** → Para el agente (instrucciones estrictas)
2. **PLANTILLAS_INICIO_SESION.md** → Para copiar-pegar al inicio
3. **Este documento** → Para ti (estrategia de gestión)

---

## ✅ Checklist de Pre-Sesión

Antes de cada sesión nueva:

```markdown
- [ ] Commit anterior está hecho (git status limpio)
- [ ] Identifiqué el archivo a refactorizar
- [ ] Identifiqué el workflow correcto (A/B/C/D/E/G/H/I)
- [ ] Copié la plantilla correspondiente
- [ ] Reemplacé {variables} con valores reales
- [ ] Adjunté el archivo a refactorizar
- [ ] Agregué "Seguir META_WORKFLOW_AGENTE.md estrictamente"
- [ ] Abrí un chat NUEVO (no continuar el anterior)
```

---

## 🎯 Resultado Esperado

Siguiendo esta estrategia:

### ✅ Agente será:
- Enfocado en una tarea
- Consistente entre sesiones
- Menos propenso a olvidar pasos
- Más fácil de corregir si se desvía

### ✅ Tú tendrás:
- Commits atómicos y claros
- Historial de chat organizado
- Progreso trackeable
- Refactors de alta calidad

---

**¡Listo para refactorizar con estrategia!** 🚀

Recuerda: **1 Sesión = 1 Archivo = 1 Commit = Chat Nuevo**
