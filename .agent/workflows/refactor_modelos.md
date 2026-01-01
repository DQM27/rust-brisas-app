---
description: Workflow para auditar y refactorizar modelos bajo estándares estrictos y DDD.
---

# Workflow: Refactorización Estricta de Modelos (DDD)

Este flujo de trabajo guía la transformación de archivos en `src/models` para adherirse a prácticas estrictas de Rust y Domain-Driven Design (DDD).

## 1. Auditoría de Lógica y Responsabilidad
- [ ] **Verificar Pureza**: El archivo debe contener **solo** definiciones de datos (`struct`, `enum`) y conversiones simples (`From`, `Into`, `DTOs`).
- [ ] **Extraer Lógica**: Si encuentra métodos con lógica de negocio (cálculos, validaciones complejas, acceso a BD), muévalos a:
    - `domain/` (reglas de negocio puras)
    - `services/` (orquestación y casos de uso)
- [ ] **Revisar Dependencias**: El modelo no debe depender de capas superiores (como controladores o servicios).

## 2. Aplicación de Tipos Estrictos (Type-Driven Design)
- [ ] **Eliminar "Stringly Typed"**: Identificar campos `String` que representan un conjunto finito de valores.
    - **Acción**: Crear un `enum` público (ej. `EstadoCita` en lugar de `String`).
- [ ] **Value Objects**: Para campos con reglas de validación (ej. Email, RUT, Placa), usar el patrón NewType o Value Objects.
- [ ] **Manejo de Nulos**: Usar `Option<T>` explícitamente. Evitar valores centinela (strings vacíos para nulos).

## 3. Documentación y Estándares (Pragmático)
- [ ] **Encabezado de Módulo**: Agregar bloque `///` al inicio del archivo describiendo qué datos maneja el módulo (sin lógica).
- [ ] **Estilo Visual**: Usar separadores estandarizados `// ---` para dividir secciones (Enums, Structs, DTOs).
- [ ] **Documentación Pragmática**: Usar `///` solo para aportar valor de negocio. Evitar comentarios obvios (ej. "Campo ID").
- [ ] **Referencia**: Seguir `docs/guia-documentacion-codigo.md`.
- [ ] **Nombres Expresivos**: El código debe auto-documentarse. Renombrar campos crípticos.

## Ejemplo de Transformación

**Antes (Débil):**
```rust
struct Cita {
    estado: String, // "PROGRAMADA", "CANCELADA"
}
```

**Después (Estricto):**
```rust
/// Representa el ciclo de vida de una cita.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EstadoCita {
    Programada,
    Cancelada,
    // ...
}

/// Entidad principal de Cita.
pub struct Cita {
    /// Estado actual del flujo.
    pub estado: EstadoCita,
}
```
