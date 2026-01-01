---
description: Refactorización de Capa de Dominio - Workflow de auditoría y purificación
---

# Workflow: Refactorización de Capa de Dominio

Este workflow guía la auditoría y refactorización de cualquier módulo dentro de `src/domain/`. El objetivo es garantizar que el dominio contenga **únicamente lógica pura de negocio**, sin dependencias de servicios, base de datos o estructuras de datos (DTOs).

## Principios Fundamentales

1. **Lógica Pura**: El dominio NO debe contener structs de datos (van en `models/`), ni llamadas a servicios o BD.
2. **Sin Efectos Secundarios**: Las funciones de dominio deben ser determinísticas (misma entrada = misma salida).
3. **Validaciones y Reglas**: El dominio define QUÉ reglas aplican, no CÓMO se persisten.
4. **Testing Obligatorio**: Toda función de dominio debe tener al menos un test unitario.
5. **Documentación Estándar**: Seguir la [Guía de Documentación](file:///c:/Users/femprobrisas/proyecto-brisas/Rust/brisas-app/docs/guia-documentacion-codigo.md) para docstrings y estructura.

---

## Paso 1: Auditoría Inicial

**Objetivo**: Identificar violaciones de los principios de dominio puro.

1. Revisar el archivo de dominio objetivo (`src/domain/<modulo>.rs`).
2. Identificar y documentar:
   - [ ] Structs de datos que deberían estar en `models/`
   - [ ] Imports de `services/`, `db/`, o crates de persistencia (ej. `surrealdb`)
   - [ ] Funciones con efectos secundarios (I/O, logging excesivo, mutación de estado global)
   - [ ] Código comentado o TODOs obsoletos
   - [ ] Funciones sin documentación (`///`)
   - [ ] Funciones sin tests
   - [ ] **Uso correcto de `thiserror`**: Verificar que los errores del módulo usen `#[derive(thiserror::Error)]` y tengan mensajes descriptivos
   - [ ] **Valores mágicos**: Identificar números literales que deberían ser constantes nombradas

3. Revisar el archivo de modelos correspondiente (`src/models/<modulo>.rs`) para entender la separación actual.

---

## Paso 2: Propuesta de Plan

**Objetivo**: Presentar un plan detallado al usuario ANTES de ejecutar cambios.

1. Crear un plan de implementación que incluya:
   - **Cambios de Separación**: Qué structs o funciones se moverán entre archivos.
   - **Refactorizaciones**: Funciones a renombrar, simplificar o eliminar.
   - **Tests Faltantes**: Qué funciones necesitan tests.
   - **Sugerencias de Mejora**: Oportunidades de optimización, simplificación o estandarización.

2. **PAUSA OBLIGATORIA**: Usar `notify_user` para presentar el plan y esperar aprobación.
   - NO ejecutar cambios sin confirmación explícita del usuario.

---

## Paso 3: Ejecución de Refactorización

**Objetivo**: Implementar los cambios aprobados de forma ordenada.

1. **Mover Structs a `models/`**: Si hay structs de datos en el dominio, moverlos al archivo de modelos correspondiente.
2. **Eliminar Dependencias Impuras**: Remover imports de servicios o BD del dominio.
3. **Añadir Documentación**: Asegurar que cada función tenga docstrings según la Guía de Estándares.
4. **Añadir Separadores Visuales**: Organizar el archivo con secciones claras (`// ----------`).
5. **Implementar Tests Unitarios**: Crear tests para funciones sin cobertura.

---

## Paso 4: Verificación

**Objetivo**: Confirmar que el código compila y los tests pasan.

// turbo
1. Ejecutar `cargo check --package mega-brisas` para verificar compilación.

// turbo
2. Ejecutar `cargo test --package mega-brisas -- <modulo>` para correr los tests del módulo.

3. Revisar logs y resultados.

---

## Paso 5: Documentación Final

**Objetivo**: Registrar los cambios realizados.

1. Actualizar `task.md` con el progreso.
2. Actualizar `walkthrough.md` con un resumen de los cambios.
3. Proponer commit con mensaje descriptivo.

---

## Checklist de Dominio Puro

Antes de cerrar la refactorización, verificar:

- [ ] El archivo NO importa nada de `services/`, `db/`, `commands/`
- [ ] El archivo NO importa `surrealdb`, `tauri`, u otros crates de infraestructura
- [ ] Todas las funciones públicas tienen documentación `///`
- [ ] Existen tests unitarios para las funciones de validación
- [ ] El archivo sigue la estructura: Imports → Constantes → Validaciones → Normalización → Tests
- [ ] No hay código comentado ni TODOs obsoletos
