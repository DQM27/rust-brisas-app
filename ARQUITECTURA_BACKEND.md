# Arquitectura Backend - Brisas App
## Guía de Implementación Strict Mode SQLx + Typed Errors

---

## 📁 ESTRUCTURA DE CAPAS

```
src/
├── domain/        # Modelos de dominio y errores tipados
│   ├── errors.rs  # TODOS los XxxError van aquí
│   └── xxx.rs     # Structs del dominio (ej: Visitante, IngresoVisita)
│
├── models/        # Modelos con derives SQLx
│   └── xxx.rs     # #[derive(FromRow, Serialize)] para mapeo DB
│
├── db/            # Capa de acceso a datos (SOLO queries)
│   └── xxx_queries.rs
│
├── services/      # Lógica de negocio
│   └── xxx_service.rs
│
└── commands/      # API Tauri (handlers)
    └── xxx_commands.rs
```

---

## ✅ PATRÓN STRICT MODE EN QUERIES

### 1. IMPORTS NECESARIOS
```rust
use crate::domain::xxx::TuStruct;
use sqlx::SqlitePool;
use chrono::Utc;
use uuid::Uuid;
```

### 2. QUERY SIMPLE (SELECT)
```rust
pub async fn find_by_id(pool: &SqlitePool, id: &str) -> sqlx::Result<Option<TuStruct>> {
    sqlx::query_as!(
        TuStruct,
        r#"
        SELECT 
            id as "id!",                              -- ! = garantiza non-null
            nombre as "nombre!",
            campo_opcional,                           -- sin ! = Option<T>
            CAST(created_at AS TEXT) as "created_at!" -- DATETIME -> String
        FROM tu_tabla
        WHERE id = ?
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}
```

### 3. TYPE OVERRIDES IMPORTANTES
```sql
-- Campo non-null garantizado:
columna as "columna!"

-- Campo con tipo específico:
has_vehicle as "has_vehicle!: bool"
estado as "estado!: EstadoProveedor"

-- DATETIME a String:
CAST(created_at AS TEXT) as "created_at!"
CAST(fecha_salida AS TEXT) as fecha_salida  -- nullable
```

### 4. INSERT (usar query! no query_as!)
```rust
pub async fn create(pool: &SqlitePool, input: CreateInput) -> sqlx::Result<TuStruct> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    sqlx::query!(
        r#"
        INSERT INTO tu_tabla (id, campo1, campo2, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?)
        "#,
        id,
        input.campo1,
        input.campo2,
        now,
        now
    )
    .execute(pool)
    .await?;

    // Retornar el struct manualmente construido
    Ok(TuStruct {
        id,
        campo1: input.campo1,
        campo2: input.campo2,
        created_at: now.clone(),
        updated_at: now,
    })
}
```

### 5. UPDATE
```rust
pub async fn update(pool: &SqlitePool, id: &str, input: UpdateInput) -> sqlx::Result<()> {
    let now = Utc::now().to_rfc3339();
    
    sqlx::query!(
        r#"
        UPDATE tu_tabla 
        SET campo = COALESCE(?, campo),
            updated_at = ?
        WHERE id = ?
        "#,
        input.campo,
        now,
        id
    )
    .execute(pool)
    .await?;
    
    Ok(())
}
```

---

## ✅ PATRÓN DE ERRORES TIPADOS

### 1. DEFINIR EN errors.rs
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TuModuloError {
    #[error("Registro no encontrado")]
    NotFound,
    
    #[error("Ya existe un registro con esa clave")]
    AlreadyExists,
    
    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),  // Conversión automática
    
    #[error("Error de validación: {0}")]
    Validation(String),
}
```

### 2. USAR EN SERVICE
```rust
use crate::domain::errors::TuModuloError;
use crate::db::xxx_queries;

pub async fn crear(pool: &SqlitePool, input: Input) -> Result<Output, TuModuloError> {
    // Validación
    if input.nombre.is_empty() {
        return Err(TuModuloError::Validation("Nombre requerido".into()));
    }
    
    // Query (el ? convierte sqlx::Error a TuModuloError automáticamente)
    let resultado = xxx_queries::create(pool, input).await?;
    
    Ok(resultado)
}
```

### 3. MAPEAR EN COMMAND
```rust
#[tauri::command]
pub async fn crear_xxx(
    pool: State<'_, SqlitePool>,
    input: Input,
) -> Result<Output, String> {
    xxx_service::crear(&pool, input)
        .await
        .map_err(|e| e.to_string())  // Convierte error a String para frontend
}
```

---

## 🔴 ERRORES COMUNES Y SOLUCIONES

### Error: "unsupported type DATETIME"
```sql
-- SOLUCIÓN: Usar CAST
CAST(fecha_campo AS TEXT) as "fecha_campo!"
```

### Error: "Option<bool> from Option<i64>"
```sql
-- SOLUCIÓN: Type override explícito
has_vehicle as "has_vehicle!: bool"
```

### Error: "String from Option<String>"
```sql
-- SOLUCIÓN: Agregar ! para garantizar non-null
columna as "columna!"
```

### Error: "`?` couldn't convert sqlx::Error to String"
```rust
// SOLUCIÓN: Agregar .map_err()
xxx_queries::funcion(pool).await.map_err(|e| e.to_string())?
```

---

## 📋 CHECKLIST PARA MIGRAR UN MÓDULO

1. [ ] Definir `XxxError` en `domain/errors.rs`
2. [ ] Verificar que el struct de dominio tenga `#[derive(FromRow)]`
3. [ ] Refactorizar `xxx_queries.rs`:
   - [ ] Cambiar `sqlx::query(` → `sqlx::query!(`
   - [ ] Cambiar `sqlx::query_as::<_, Type>` → `sqlx::query_as!(Type, ...)`
   - [ ] Agregar type overrides para campos non-null
   - [ ] Agregar `CAST(... AS TEXT)` para campos DATETIME
4. [ ] Refactorizar `xxx_service.rs`:
   - [ ] Cambiar struct con `impl` → funciones standalone
   - [ ] Retornar `Result<T, XxxError>` 
5. [ ] Actualizar `xxx_commands.rs`:
   - [ ] Llamar funciones de service directamente
   - [ ] Mapear errores con `.map_err(|e| e.to_string())`
6. [ ] `cargo check` sin errores
7. [ ] Commit

---

## 📂 ARCHIVOS PENDIENTES DE MIGRAR

- [ ] `ingreso_contratista_queries.rs` (~257 líneas)
- [ ] `ingreso_general_queries.rs` (~220 líneas)
- [ ] `ingreso_queries.rs` (~486 líneas)

Estos archivos tienen helpers manuales `row_to_ingreso` y `extract_details` que mapean con `Row`. 
Requieren mayor trabajo de refactorización.

---

## 🎯 BENEFICIO PRINCIPAL

**ANTES (runtime):**
```rust
sqlx::query("SELECT * FROM usuarios")  // Errores en producción
    .bind(id)
    .fetch_one(pool)
```

**AHORA (compile-time):**
```rust
sqlx::query!("SELECT * FROM usuarios WHERE id = ?", id)  // Error si DB no coincide
    .fetch_one(pool)
```

Los errores de schema se detectan en COMPILACIÓN, no en producción.
