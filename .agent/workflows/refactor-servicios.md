---
description: Workflow estricto para auditar, documentar y refactorizar Servicios bajo Clean Architecture
---

Este workflow guía el proceso de elevar la calidad de los servicios de aplicación a un estándar "Enterprise", asegurando desacoplamiento, validación centralizada y documentación exhaustiva.

1. [ ] **Análisis de Dependencias y Responsabilidad**:
    - Verificar que el servicio o archivo actúe únicamente como **Orquestador**.
    - Eliminar lógica de negocio "inline" (ifs de validación, reglas de negocio).
    - Asegurar que SOLO dependa de:
        - `crate::domain` (Reglas, Errores).
        - `crate::models` (Tipos de datos).
        - `crate::repositories` (Acceso a datos) o módulos de queries.
    - **Prohibido**: Dependencias directas a `tauri::State` o tipos de UI dentro de la lógica pura (separar en capas si es necesario).

2. [ ] **Auditoría de Validación (Dominio Puro)**:
    - Identificar todas las validaciones de entrada.
    - Mover reglas a `src/domain/` (crear módulo si no existe).
    - Reemplazarlas por llamadas a `domain::validaciones::...`.

3. [ ] **Estandarización de I/O (DTOs)**:
    - **Input**: El servicio debe recibir un `Struct` de input (DTO), no listas de argumentos sueltos.
    - **Output**: El servicio debe retornar `Result<ResponseDTO, ServiceError/DomainError>`.
    - **Prohibido**: Retornar `RecordId` sueltos, Tuplas sin nombre, o `surrealdb::Generic`.
    - Crear los DTOs en `models/{modulo}/response.rs` o `input.rs` si no existen.

4. [ ] **Manejo Transaccional y Errores**:
    - Si una operación modifica >1 tabla/entidad, ENVOLVER en transacción explícita (`BEGIN` ... `COMMIT`).
    - Mapear errores de bajo nivel a errores de dominio/servicio significativos.

5. [ ] **Logging Obligatorio**:
    - Importar `use log::{error, info, warn};` en todos los servicios.
    - Usar `info!()` para eventos importantes (creaciones, actualizaciones exitosas).
    - Usar `error!()` para fallos críticos de DB o infraestructura.
    - Usar `warn!()` para situaciones recuperables que merecen atención.
    - El crate `log` es zero-cost cuando no hay logger configurado.

6. [ ] **Documentación Estándar (Estricta)**:
    - **Encabezado del Archivo**:
      ```rust
      /// Servicio: [Nombre del Servicio]
      ///
      /// Orquestador de la lógica de negocio para [Dominio].
      /// Responsabilidades:
      /// - [Responsabilidad 1]
      /// - [Responsabilidad 2]
      ```
    - **Funciones Públicas**: Documentar cada función pública con:
      - Descripción breve.
      - `/// # Arguments` (si aplica, aunque preferible referenciar al DTO).
      - `/// # Returns`.
      - `/// # Errors` (lista de posibles errores retornados).

7. [ ] **Análisis de Mejoras**:
    - Buscar oportunidades para introducir Traits (`trait XService`) si facilita el testing.
    - Identificar código repetitivo que pueda moverse a un *Helper* o *Macro*.
    - Sugerir estas mejoras al usuario antes de cerrar la tarea.

8. [ ] **Estrategia de Testing (Unitario/Integración)**:
    - **Lógica Pura**: Si el servicio tiene métodos sin DB (ej: cálculos, crypto), AGREGAR `mod tests` al final del archivo con tests unitarios.
    - **Dependencia de DB**: Si el servicio depende de la DB, evaluar si se puede desacoplar fácil o dejar marcado como `// TODO: Integration Tests needed`.

9. [ ] **Verificación Final**:
    - Ejecutar `cargo check` y asegurar 0 warnings.
    - Confirmar que la compilación es limpia.
