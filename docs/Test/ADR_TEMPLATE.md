# ADR-000: [Título de la Decisión]

**Estado**: [Propuesto | Aceptado | Rechazado | Deprecado | Reemplazado por ADR-XXX]  
**Fecha**: YYYY-MM-DD  
**Autores**: [Nombre(s)]  
**Stakeholders**: [Product Owner, Tech Lead, etc.]

---

## Contexto y Problema

**¿Qué problema estamos intentando resolver?**

Describe el contexto técnico y de negocio que motiva esta decisión. Incluye:

- Qué está pasando en el proyecto/sistema
- Por qué se necesita tomar una decisión ahora
- Qué limitaciones o restricciones existen (técnicas, presupuesto, tiempo, equipo)

**Ejemplo**:

> Actualmente, Mega Brisas almacena los datos de contratistas en SurrealDB embebido, pero el cliente solicita sincronización multi-dispositivo. Necesitamos decidir si migramos a un backend centralizado (Firebase/Supabase) o implementamos sincronización P2P con la DB embebida.

---

## Drivers de Decisión (Factores Clave)

Lista los factores que influyen en la decisión, ordenados por prioridad:

1. **[Factor 1]**: Descripción
2. **[Factor 2]**: Descripción
3. **[Factor 3]**: Descripción

**Ejemplo**:

1. **Costo de Operación**: La solución debe tener costos operativos ≤ $50/mes
2. **Complejidad de Implementación**: El equipo tiene 2 semanas de desarrollo
3. **Experiencia del Equipo**: Rust + Tauri (alto), Firebase (bajo)
4. **Escalabilidad**: Soportar hasta 100 dispositivos concurrentes

---

## Opciones Consideradas

### Opción 1: [Nombre de la Opción]

**Descripción**: Breve explicación de la solución.

**Pros**:

- ✅ Ventaja 1
- ✅ Ventaja 2
- ✅ Ventaja 3

**Contras**:

- ❌ Desventaja 1
- ❌ Desventaja 2

**Impacto en Drivers**:
| Driver | Evaluación | Nota |
|--------|-----------|------|
| Costo | ⭐⭐⭐⭐⭐ | $0/mes (self-hosted) |
| Complejidad | ⭐⭐⭐ | Requiere 3 semanas |
| Experiencia | ⭐⭐⭐⭐⭐ | Stack conocido |
| Escalabilidad | ⭐⭐⭐ | Limitado por hardware |

**Ejemplo**:

> **Opción 1: Supabase (PostgreSQL + Realtime)**
>
> Migrar la DB a Supabase Cloud, usar subscriptions para sincronización en tiempo real.
>
> **Pros**:
>
> - ✅ PostgreSQL bien conocido
> - ✅ Realtime subscriptions out-of-the-box
> - ✅ SDKs para Rust disponibles
>
> **Contras**:
>
> - ❌ Requiere conexión a internet permanente
> - ❌ Costos de ~$25/mes (tier Pro)
> - ❌ Lock-in a proveedor cloud

---

### Opción 2: [Nombre de la Opción]

_(Repetir la estructura de Opción 1)_

---

### Opción 3: [Nombre de la Opción]

_(Repetir la estructura de Opción 1)_

---

## Decisión

**Opción elegida**: [Opción X - Nombre]

**Justificación**:

Explica por qué esta opción es la mejor considerando:

1. Los drivers de decisión priorizados
2. Trade-offs aceptables
3. Contexto específico del proyecto

**Ejemplo**:

> Elegimos **Opción 2: CRDT con Automerge** porque:
>
> 1. **Prioriza offline-first**: Alineado con la arquitectura actual de Mega Brisas
> 2. **Sin costos operativos**: No requiere backend centralizado ($0/mes)
> 3. **Stack conocido**: Librería Rust nativa (automerge-rs), el equipo puede implementarlo en 2 semanas
> 4. **Trade-off aceptable**: La latencia de sincronización (P2P) es aceptable para el caso de uso (cambios no son críticos en tiempo real)

---

## Consecuencias

### Positivas (Beneficios)

- ✅ [Consecuencia positiva 1]
- ✅ [Consecuencia positiva 2]

**Ejemplo**:

- ✅ Eliminamos dependencia de conexión a internet
- ✅ Cero costos recurrentes de infraestructura
- ✅ Datos del usuario permanecen en su dispositivo (privacidad)

### Negativas (Riesgos/Limitaciones)

- ⚠️ [Consecuencia negativa 1]
- ⚠️ [Consecuencia negativa 2]

**Ejemplo**:

- ⚠️ Conflictos de merge en datos concurrentes (mitigado con estrategia LWW - Last Write Wins)
- ⚠️ Complejidad adicional en debugging de sincronización
- ⚠️ Limitación a 10 dispositivos por archivo (restricción de Automerge)

### Neutral (Cambios Estructurales)

- 🔄 [Cambio neutral 1]
- 🔄 [Cambio neutral 2]

**Ejemplo**:

- 🔄 Migración de `SurrealDB` a `automerge::Automerge` (2 semanas de trabajo)
- 🔄 Nuevos módulos: `sync/` y `crdt/`

---

## Métricas de Validación

**¿Cómo sabremos si esta decisión fue correcta?**

Define métricas medibles para evaluar el éxito de la decisión después de implementarla.

**Ejemplo**:

- **Tiempo de sincronización**: < 5 segundos entre dispositivos en la misma red
- **Conflictos de merge**: < 1% de operaciones (monitorear con telemetría)
- **Adopción de usuarios**: > 80% de usuarios usan multi-dispositivo después de 3 meses
- **Estabilidad**: 0 crashes relacionados con sincronización en primeros 30 días

---

## Alternativas Descartadas (Resumen)

Breve resumen de por qué otras opciones fueron rechazadas.

**Ejemplo**:

- **Firebase Firestore**: Rechazado por costos ($100/mes en tier Blaze) y lock-in
- **Sync via Dropbox**: Rechazado por complejidad de manejo de conflictos manual
- **WebRTC P2P**: Rechazado por problemas de NAT traversal en redes corporativas

---

## Plan de Implementación

### Fase 1: Prototipo (Semana 1-2)

- [ ] Integrar biblioteca `automerge-rs`
- [ ] Migrar modelo de datos de `Contractor` a CRDT
- [ ] Implementar sincronización básica (2 dispositivos en LAN)

### Fase 2: Producción (Semana 3-4)

- [ ] Manejo de conflictos con estrategia LWW
- [ ] UI para estado de sincronización
- [ ] Tests de integración (multi-dispositivo)

### Fase 3: Monitoreo (Semana 5+)

- [ ] Telemetría de métricas de validación
- [ ] Documentación para usuarios
- [ ] Plan de rollback si falla

---

## Revisión Futura

**Fecha de revisión**: [YYYY-MM-DD + 6 meses]

**Trigger para revisión anticipada**:

- Si las métricas de validación no se cumplen después de 3 meses
- Si aparece una nueva tecnología que resuelva las limitaciones (ej: WASM P2P nativo en Tauri)
- Si el contexto de negocio cambia (ej: cliente solicita sincronización en tiempo real crítica)

---

## Referencias

### Documentación Técnica

- [Automerge Documentation](https://automerge.org/docs/)
- [Blog post: CRDTs in Rust](https://example.com)

### Discusiones Internas

- Issue #123: "Multi-device sync requirements"
- Slack thread: #tech-decisions (2026-01-08)

### ADRs Relacionados

- ADR-001: "Elección de SurrealDB como base de datos embebida" (contexto histórico)

---

## Notas Adicionales

Cualquier información relevante que no encaje en las secciones anteriores.

**Ejemplo**:

> El cliente expresó que la sincronización en tiempo real no es crítica (pueden tolerar 1-2 minutos de delay), lo cual valida la elección de P2P sobre soluciones cloud en tiempo real.

---

## Aprobaciones

| Rol                  | Nombre   | Fecha      | Firma/OK |
| -------------------- | -------- | ---------- | -------- |
| Tech Lead            | [Nombre] | YYYY-MM-DD | ✓        |
| Product Owner        | [Nombre] | YYYY-MM-DD | ✓        |
| Desarrollador Senior | [Nombre] | YYYY-MM-DD | ✓        |

---

## Changelog del ADR

| Fecha      | Cambio                                                    | Autor    |
| ---------- | --------------------------------------------------------- | -------- |
| YYYY-MM-DD | Creación del ADR                                          | [Nombre] |
| YYYY-MM-DD | Actualización post-implementación (sección Consecuencias) | [Nombre] |

---

## Plantilla Version

**ADR Template Version**: 1.0  
**Basado en**: [Michael Nygard's ADR format](https://github.com/joelparkerhenderson/architecture-decision-record)  
**Adaptado para**: Mega Brisas / Tauri Projects

---

## Instrucciones de Uso

1. **Copia este template** a `docs/adr/ADR-NNN-titulo-decision.md` (donde NNN es el número consecutivo)
2. **Reemplaza todos los placeholders** `[...]` con información real
3. **Elimina las secciones de "Ejemplo"** (son solo para guía)
4. **No elimines secciones vacías** - déjalas con "TBD" si aún no tienes la info
5. **Actualiza el ADR** después de implementar (sección Consecuencias y Métricas)
6. **Versiona el ADR** en Git junto con el código

---

## Buenas Prácticas

✅ **Hazlo**:

- Escribe ADRs para decisiones que afectan más de 1 módulo
- Incluye trade-offs honestos (no solo beneficios)
- Actualiza el ADR si la implementación difiere del plan
- Enlaza el ADR en PRs relacionados

❌ **No hagas esto**:

- Escribir ADRs para decisiones triviales (ej: "usar snake_case en Rust")
- Ocultar desventajas de la opción elegida
- Dejar ADRs "Propuestos" sin resolver por >2 semanas
- Crear ADRs después de implementar (debe ser antes o durante)

---

**¿Preguntas sobre cómo usar este template?**  
Consulta: [docs/adr/README.md](./README.md) o pregunta en #tech-decisions (Slack)
