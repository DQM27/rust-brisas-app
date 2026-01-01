# Guía de Estándares de Documentación del Código

Esta guía establece las normas y convenciones para documentar el backend (Rust) de **Brisas APP**. El objetivo es mantener una base de código "ordenada", comprensible para no programadores y técnicamente robusta.

## 1. Idioma y Tono
- **Idioma**: Toda la documentación debe estar en **Español**.
- **Tono**: Profesional, explicativo y directo. Debe explicar no solo *qué* hace el código, sino *por qué* es importante para el negocio o la seguridad.

---

## 2. Documentación de Módulos (Archivos)
Cada archivo debe comenzar con un bloque de documentación que explique su propósito general y su lugar en la arquitectura.

```rust
/// Capa de Dominio: Gestión de Gafetes.
///
/// Este módulo centraliza las reglas de negocio para los gafetes físicos,
/// incluyendo validaciones de formato, estados de inventario y
/// normalización de datos.
```

---

## 3. Documentación de Funciones y Comandos
Las funciones deben usar triple barra (`///`) para permitir que las herramientas de Rust generen documentación automática.

### Componentes de una descripción:
1.  **Resumen**: Una línea corta que defina la acción principal.
2.  **Detalles (Opcional)**: Explicación de reglas de negocio complejas o implicaciones de seguridad.
3.  **Parámetros**: Descripción de lo que recibe.
4.  **Retorno**: Qué resultado entrega o qué error puede ocurrir.

**Ejemplo:**
```rust
/// Registra el ingreso de un nuevo visitante al sistema.
///
/// Realiza validaciones críticas:
/// - Verifica que el visitante no esté en la Lista Negra.
/// - Asegura que no tenga otro ingreso activo simultáneo.
///
/// # Argumentos
/// * `input` - Estructura con los datos del visitante y el área de destino.
///
/// # Retorno
/// Retorna `Ok(())` si el registro fue exitoso o un error descriptivo si falló.
pub async fn registrar_ingreso(input: CreateIngresoInput) -> Result<(), Error> { ... }
```

---

## 4. Organización y Limpieza
Para mantener el código "ordenado", seguimos estas reglas:

### Separadores Visuales
Usa encabezados claros para separar secciones lógicas dentro de un archivo (Validaciones, Lógica, Tests).

```rust
// --------------------------------------------------------------------------
// VALIDACIONES DE NEGOCIO
// --------------------------------------------------------------------------
```

### Eliminación de Comentarios Obsoletos
- Elimina comentarios de tipo "TODO" una vez resueltos.
- Borra bloques de código comentados (el historial está en Git).
- No uses comentarios evidentes (ej: `// suma 1 a i` sobre `i += 1;`).

---

## 5. Diferencia entre Dominio y Modelos
Es crucial documentar correctamente según la carpeta:

- **`src/models/`**: Define **CÓMO** se ven los datos (Estructuras para la base de datos, DTOs para el frontend).
- **`src/domain/`**: Define **QUÉ REGLAS** se aplican a esos datos (Validaciones, cálculos de tiempo, elegibilidad).

---

## 6. Etiquetas Especiales (Atributos)
En Tauri, es importante documentar cuándo una función es un comando accesible desde el frontend:

```rust
/// [Comando Tauri] Cierra la aplicación de forma segura.
#[tauri::command]
pub fn exit_app() { ... }
```

---

> [!TIP]
> Un código bien documentado es aquel que puede ser leído como un manual de operaciones de la empresa.
