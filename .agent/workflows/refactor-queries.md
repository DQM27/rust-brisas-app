# Workflow D: Auditoría y Refactorización de Queries SurrealDB

**Versión**: 3.0  
**Idioma**: Español  
**Aplicación**: Brisas APP  

---

## Objetivo

Elevar la calidad de los módulos de queries de SurrealDB garantizando documentación exhaustiva, manejo robusto de errores, queries optimizados y separación estricta de responsabilidades.

---

## Principios Fundamentales

1. **Solo Acceso a Datos**: Queries NO deben contener lógica de negocio ni validaciones
2. **Documentación de Queries**: Cada query debe explicar QUÉ hace y POR QUÉ se construye así
3. **Manejo de Errores Específico**: Errores descriptivos que faciliten debugging
4. **Queries Optimizados**: Usar índices, FETCH cuando aplique, evitar N+1
5. **Testing Obligatorio**: Tests de integración para queries críticos

---

## FASE 0: ANÁLISIS PREVIO (⚠️ OBLIGATORIO)

### [ ] 0.1 Auditoría de Responsabilidad

**Plantilla de análisis**:

```markdown
**Archivo analizado**: `src/db/surrealdb_{modulo}_queries.rs`
**LOC**: {número de líneas}
**Número de funciones**: {N}

## ❌ VIOLACIONES DE RESPONSABILIDAD

### Lógica de Negocio (mover a domain/ o services/)
- [ ] Línea XX: Validación de formato → Mover a `domain::`
- [ ] Línea YY: Cálculo de negocio → Mover a `domain::`
- [ ] Línea ZZ: Decisión de bloqueo → Mover a `services::`

### Acceso a Otros Módulos (refactorizar)
- [ ] Línea XX: Llama a otro módulo de queries → Evaluar si es correcto
- [ ] Línea YY: Acceso a capa de servicios → ❌ CRÍTICO, invertir dependencia

### Queries Sin Optimizar
| Función | Query | Problema | Acción |
|---------|-------|----------|--------|
| `find_all()` | `SELECT *` sin límite | N registros sin paginación | Agregar `LIMIT` |
| `find_by_empresa()` | Sin índice | Scan completo de tabla | Crear índice en `empresa` |
| `get_related()` | Sin `FETCH` | Query adicional por relacionado | Usar `FETCH` |
```

### [ ] 0.2 Auditoría de Documentación

```markdown
## DOCUMENTACIÓN

| Función | Tiene `///`? | Explica query? | Explica `FETCH`? | Idioma |
|---------|--------------|----------------|------------------|--------|
| `create()` | ❌ | N/A | N/A | - |
| `find_by_id()` | ✅ | ❌ | ✅ | Español |
| `find_by_cedula()` | ❌ | N/A | N/A | - |
| `update()` | ✅ | ✅ | ❌ | Inglés |

**Cobertura**: X/Y funciones documentadas (Z%)
```

### [ ] 0.3 Auditoría de Manejo de Errores

```markdown
## MANEJO DE ERRORES

### Propagación Genérica (mejorar contexto)
- [ ] Línea XX: `map_err(|e| SurrealDbError::Query("Error genérico".into()))`
  - **Acción**: Agregar contexto específico del query

### Uso de `.unwrap()` (eliminar)
- [ ] Línea YY: `.unwrap()` en producción → Usar propagación `?`

### Errores Sin Contexto
- [ ] Línea ZZ: Error sin indicar qué query falló → Agregar información

**Sugerencia**: Crear enum de errores específico del módulo
```

### [ ] 0.4 Auditoría de Patrones de Query

```markdown
## PATRONES DE QUERY

### Queries que retornan entidad "fetched"
| Función | Usa `FETCH`? | Campos relacionados | Optimizado? |
|---------|--------------|---------------------|-------------|
| `find_by_id_fetched()` | ✅ | `empresa` | ✅ |
| `find_all_fetched()` | ✅ | `empresa` | ⚠️ Sin `LIMIT` |
| `create()` | ❌ | `empresa` | ❌ Requiere 2 queries |

### Queries con filtros
| Función | Usa índices? | Tiene `LIMIT`? | Maneja paginación? |
|---------|--------------|----------------|--------------------|
| `find_by_cedula()` | ✅ | N/A | N/A |
| `find_by_empresa()` | ⚠️ No verificado | ❌ | ❌ |
| `find_archived()` | ✅ | ❌ | ❌ |

### Soft Delete
- [ ] ¿Usa `deleted_at IS NONE` en queries de lectura? ✅/❌
- [ ] ¿`delete()` marca como borrado lógico? ✅/❌
- [ ] ¿`restore()` limpia `deleted_at`? ✅/❌
```

### [ ] 0.5 Auditoría de Transacciones

```markdown
## TRANSACCIONES

### Funciones que deberían usar transacciones
- [ ] `{funcion}()`: Modifica múltiples tablas → Necesita transacción
- [ ] `{funcion}()`: Operación atómica requerida → Necesita transacción

### Funciones existentes con transacciones
- [ ] `{funcion}()`: ✅ Usa transacción correctamente
```

### [ ] 0.6 Tests de Integración

```markdown
## TESTING

### Cobertura de Tests
- [ ] Tests de integración presentes: Sí/No
- [ ] Funciones críticas con tests: X/Y (Z%)

### Funciones sin tests (críticas)
1. `create()` - CRÍTICO: Crea datos en BD
2. `update()` - ALTO: Modifica datos
3. `delete()` - ALTO: Elimina (soft delete)
4. `find_by_{criterio_seguridad}()` - CRÍTICO
```

---

### 📋 Reporte Final de Análisis

```markdown
# Reporte de Análisis FASE 0 - Queries

**Archivo**: src/db/surrealdb_{modulo}_queries.rs
**LOC**: {número}
**Funciones**: {N}

## PROBLEMAS CRÍTICOS
1. [CRÍTICO] N funciones con lógica de negocio → Mover a domain/
2. [CRÍTICO] M queries sin optimizar (sin LIMIT, sin FETCH)

## PROBLEMAS MAYORES
3. [ALTO] K funciones sin documentar (X%)
4. [ALTO] P queries con errores genéricos → Agregar contexto

## MEJORAS RECOMENDADAS
5. [MEDIO] Q funciones sin tests de integración
6. [BAJO] R queries duplicados → Refactorizar

## ESTIMACIÓN
- Documentación: X horas
- Optimización de queries: Y horas
- Manejo de errores: Z horas
- Tests: W horas
- **TOTAL**: T horas

## ¿Proceder?
Esperar aprobación del usuario.
```

---

## FASE 1-8: EJECUCIÓN (Solo después de aprobar Fase 0)

### 1. [ ] Eliminar Lógica de Negocio

**Objetivo**: Queries solo acceden a datos, no toman decisiones.

**Acción**:

```rust
// ❌ ANTES - Lógica de negocio en queries
pub async fn find_by_cedula(cedula: &str) -> Result<Option<Contratista>, SurrealDbError> {
    // ❌ Validación de formato en queries
    if cedula.is_empty() || cedula.len() != 11 {
        return Err(SurrealDbError::Query("Cédula inválida".into()));
    }
    
    let db = get_db().await?;
    // ... query
}

// ✅ DESPUÉS - Solo acceso a datos
/// Busca un contratista por su número de cédula.
///
/// ## Precondición
/// La cédula debe estar previamente validada en `domain::contratista::validar_cedula()`.
/// Este query NO valida formato, solo busca en la base de datos.
///
/// ## Query Ejecutado
/// ```sql
/// SELECT * FROM contratista 
/// WHERE cedula = $cedula AND deleted_at IS NONE 
/// FETCH empresa
/// ```
///
/// ## Parámetros
/// * `cedula` - Cédula normalizada (ej: "1-2345-6789")
///
/// ## Retorno
/// * `Ok(Some(Contratista))` - Contratista encontrado
/// * `Ok(None)` - No existe contratista con esa cédula
///
/// ## Errores
/// * `SurrealDbError::Connection` - Fallo de conexión a BD
/// * `SurrealDbError::Query` - Error en ejecución del query
pub async fn find_by_cedula(cedula: &str) -> Result<Option<ContratistaFetched>, SurrealDbError> {
    let db = get_db().await?;
    
    let mut result = db
        .query(
            "SELECT * FROM contratista WHERE cedula = $cedula AND deleted_at IS NONE FETCH empresa"
        )
        .bind(("cedula", cedula.to_string()))
        .await
        .map_err(|e| SurrealDbError::Query(format!(
            "Error al buscar contratista por cédula '{}': {}",
            cedula, e
        )))?;
    
    let contratista: Option<ContratistaFetched> = result.take(0).map_err(|e| {
        SurrealDbError::Deserialization(format!(
            "Error al deserializar contratista: {}",
            e
        ))
    })?;
    
    Ok(contratista)
}
```

---

### 2. [ ] Documentación Exhaustiva de Queries

**Objetivo**: Explicar QUÉ hace el query, POR QUÉ se construye así, y QUÉ espera recibir.

**Plantilla de documentación**:

```rust
/// {Descripción breve de la operación}.
///
/// ## Precondiciones
/// {Qué debe haberse validado ANTES de llamar esta función}
///
/// ## Query Ejecutado
/// ```sql
/// {Query SQL de SurrealDB exacto}
/// ```
///
/// ## Uso de FETCH
/// {Explicar qué campos relacionados se populan y por qué}
/// - `FETCH empresa`: Popula la empresa empleadora para evitar query adicional
///
/// ## Optimizaciones
/// {Explicar índices, límites, o decisiones de performance}
/// - Usa índice en `cedula` para búsqueda O(log n)
/// - Limita resultados a 100 para evitar saturación de memoria
///
/// ## Soft Delete
/// {Si aplica, explicar el filtro de deleted_at}
/// - Filtra `deleted_at IS NONE` para excluir registros eliminados
///
/// ## Parámetros
/// * `{param}` - {Descripción y ejemplo}
///
/// ## Retorno
/// * `Ok({Tipo})` - {Descripción del caso exitoso}
/// * `Ok(None)` - {Cuándo retorna None}
///
/// ## Errores
/// * `SurrealDbError::{Tipo}` - {Cuándo ocurre}
///
/// ## Ejemplo de Uso
/// ```rust
/// let contratista = find_by_cedula("1-2345-6789").await?;
/// if let Some(c) = contratista {
///     println!("Encontrado: {}", c.nombre);
/// }
/// ```
pub async fn funcion(...) -> Result<...> {
    // implementación
}
```

---

### 3. [ ] Optimización de Queries

**Objetivo**: Queries eficientes y escalables.

#### 3.1 Usar FETCH para Relaciones

```rust
// ❌ ANTES - N+1 Problem
pub async fn find_all() -> Result<Vec<ContratistaFetched>, SurrealDbError> {
    let db = get_db().await?;
    
    // Query 1: Obtener todos los contratistas
    let contratistas: Vec<Contratista> = db
        .query("SELECT * FROM contratista WHERE deleted_at IS NONE")
        .await?
        .take(0)?;
    
    // Query 2, 3, 4, ... N: Una query por cada contratista para obtener empresa
    // ❌ Si hay 100 contratistas, son 101 queries!
    let mut resultado = Vec::new();
    for c in contratistas {
        let empresa = db.select(&c.empresa).await?;  // ❌ N queries adicionales
        // ... construir fetched
    }
    
    Ok(resultado)
}

// ✅ DESPUÉS - Single Query con FETCH
/// Obtiene todos los contratistas activos con sus empresas.
///
/// ## Query Ejecutado
/// ```sql
/// SELECT * FROM contratista 
/// WHERE deleted_at IS NONE 
/// FETCH empresa
/// LIMIT 1000
/// ```
///
/// ## Optimización con FETCH
/// Usa `FETCH empresa` para popular la relación en un solo query, evitando
/// el problema N+1. Sin FETCH, serían 1 + N queries (N = cantidad de contratistas).
///
/// ## Límite de Resultados
/// Limita a 1000 registros para protección de memoria. Para cantidades mayores,
/// usar paginación con `find_paginated()`.
pub async fn find_all_fetched() -> Result<Vec<ContratistaFetched>, SurrealDbError> {
    let db = get_db().await?;
    
    let mut result = db
        .query("SELECT * FROM contratista WHERE deleted_at IS NONE FETCH empresa LIMIT 1000")
        .await?;
    
    let contratistas: Vec<ContratistaFetched> = result.take(0)?;
    Ok(contratistas)
}
```

#### 3.2 Agregar LIMIT a Queries de Listado

```rust
// ❌ ANTES - Sin límite (peligroso)
pub async fn find_all() -> Result<Vec<Contratista>, SurrealDbError> {
    let db = get_db().await?;
    let result: Vec<Contratista> = db
        .query("SELECT * FROM contratista WHERE deleted_at IS NONE")
        .await?
        .take(0)?;
    Ok(result)
}
// Problema: Si hay 1,000,000 registros, intenta cargar todos en memoria

// ✅ DESPUÉS - Con límite razonable
/// Obtiene contratistas con paginación.
///
/// ## Límite de Seguridad
/// Retorna máximo 100 registros por defecto. Para más registros,
/// usar paginación con `offset` o implementar cursor-based pagination.
///
/// ## Query
/// ```sql
/// SELECT * FROM contratista 
/// WHERE deleted_at IS NONE 
/// LIMIT 100
/// ```
pub async fn find_all() -> Result<Vec<Contratista>, SurrealDbError> {
    const MAX_RESULTS: usize = 100;
    
    let db = get_db().await?;
    let result: Vec<Contratista> = db
        .query("SELECT * FROM contratista WHERE deleted_at IS NONE LIMIT $limit")
        .bind(("limit", MAX_RESULTS))
        .await?
        .take(0)?;
    Ok(result)
}

/// Obtiene contratistas paginados.
///
/// ## Parámetros de Paginación
/// * `page` - Número de página (comienza en 1)
/// * `page_size` - Cantidad de registros por página (máximo 100)
///
/// ## Query
/// ```sql
/// SELECT * FROM contratista 
/// WHERE deleted_at IS NONE 
/// ORDER BY created_at DESC
/// LIMIT $limit START $offset
/// ```
pub async fn find_paginated(
    page: usize,
    page_size: usize
) -> Result<Vec<Contratista>, SurrealDbError> {
    const MAX_PAGE_SIZE: usize = 100;
    let page_size = page_size.min(MAX_PAGE_SIZE);
    let offset = (page.saturating_sub(1)) * page_size;
    
    let db = get_db().await?;
    let result: Vec<Contratista> = db
        .query(
            "SELECT * FROM contratista 
             WHERE deleted_at IS NONE 
             ORDER BY created_at DESC
             LIMIT $limit START $offset"
        )
        .bind(("limit", page_size))
        .bind(("offset", offset))
        .await?
        .take(0)?;
    
    Ok(result)
}
```

#### 3.3 Índices (Documentar en Comentario)

```rust
/// Busca contratistas por ID de empresa.
///
/// ## Índice Requerido
/// Este query requiere un índice en el campo `empresa` para performance óptima:
/// ```sql
/// DEFINE INDEX idx_contratista_empresa ON contratista FIELDS empresa;
/// ```
///
/// Sin el índice, el query hace un scan completo de la tabla (O(n)).
/// Con el índice, búsqueda es O(log n).
///
/// ## Query
/// ```sql
/// SELECT * FROM contratista 
/// WHERE empresa = $empresa_id AND deleted_at IS NONE
/// FETCH empresa
/// LIMIT 500
/// ```
pub async fn find_by_empresa(
    empresa_id: &RecordId
) -> Result<Vec<Contratista>, SurrealDbError> {
    // implementación
}
```

---

### 4. [ ] Manejo de Errores Específico

**Objetivo**: Errores que faciliten debugging.

**Acción**:

```rust
// ❌ ANTES - Errores genéricos
pub async fn create(dto: ContratistaCreateDTO) -> Result<Contratista, SurrealDbError> {
    let db = get_db().await?;
    let created: Option<Contratista> = db
        .query("CREATE contratista CONTENT $dto")
        .bind(("dto", dto))
        .await?
        .take(0)?;
    
    created.ok_or(SurrealDbError::Query("No se pudo crear".to_string()))
    // ❌ Mensaje inútil: "No se pudo crear" - ¿Por qué? ¿Qué falló?
}

// ✅ DESPUÉS - Errores descriptivos con contexto
/// Crea un nuevo contratista en la base de datos.
///
/// ## Query Ejecutado
/// ```sql
/// CREATE contratista CONTENT $dto
/// ```
///
/// ## Validaciones Previas Requeridas
/// Este query NO valida los datos. Debe llamarse solo después de:
/// - `domain::contratista::validar_create_input()`
/// - `domain::contratista::normalizar_cedula()`
///
/// ## Parámetros
/// * `dto` - DTO con datos ya validados
///
/// ## Retorno
/// * `Ok(Contratista)` - Contratista creado exitosamente
///
/// ## Errores
/// * `SurrealDbError::Connection` - No se pudo conectar a la BD
/// * `SurrealDbError::Query` - Error al ejecutar CREATE (ej: violación de constraint)
/// * `SurrealDbError::Deserialization` - Error al parsear resultado
/// * `SurrealDbError::NotFound` - CREATE no retornó el registro (muy raro)
pub async fn create(dto: ContratistaCreateDTO) -> Result<Contratista, SurrealDbError> {
    let db = get_db().await.map_err(|e| {
        SurrealDbError::Connection(format!(
            "Error al conectar para crear contratista: {}",
            e
        ))
    })?;
    
    let created: Option<Contratista> = db
        .query("CREATE contratista CONTENT $dto")
        .bind(("dto", &dto))
        .await
        .map_err(|e| SurrealDbError::Query(format!(
            "Error al ejecutar CREATE contratista con cédula '{}': {}",
            dto.cedula, e
        )))?
        .take(0)
        .map_err(|e| SurrealDbError::Deserialization(format!(
            "Error al deserializar contratista creado: {}",
            e
        )))?;
    
    created.ok_or_else(|| SurrealDbError::NotFound(format!(
        "CREATE contratista no retornó registro para cédula '{}'",
        dto.cedula
    )))
}
```

**Enum de Errores Mejorado**:

```rust
/// Errores específicos de operaciones de base de datos SurrealDB.
#[derive(Debug, thiserror::Error)]
pub enum SurrealDbError {
    /// Error al conectar a la base de datos.
    #[error("Error de conexión a SurrealDB: {0}")]
    Connection(String),
    
    /// Error al ejecutar un query.
    #[error("Error en query de SurrealDB: {0}")]
    Query(String),
    
    /// Error al deserializar resultado de query.
    #[error("Error de deserialización: {0}")]
    Deserialization(String),
    
    /// Registro no encontrado (cuando se esperaba uno).
    #[error("Registro no encontrado: {0}")]
    NotFound(String),
    
    /// Error de transacción.
    #[error("Error en transacción: {0}")]
    Transaction(String),
    
    /// Error genérico de SurrealDB.
    #[error("Error de SurrealDB: {0}")]
    Database(#[from] surrealdb::Error),
}
```

---

### 5. [ ] Patrón CREATE con FETCH

**Objetivo**: Retornar entidad "fetched" después de crearla.

**Problema**: SurrealDB no soporta `CREATE ... FETCH` en un solo query.

**Solución**: Query en 2 pasos con comentarios explicativos.

```rust
/// Crea un contratista y retorna la entidad con empresa populated.
///
/// ## Limitación de SurrealDB
/// SurrealDB no soporta `CREATE ... FETCH` en un solo query, por lo que
/// esta función ejecuta 2 queries:
/// 1. `CREATE contratista CONTENT $dto` - Crea el registro
/// 2. `SELECT * FROM $id FETCH empresa` - Obtiene el registro con relaciones
///
/// ## Query 1: CREATE
/// ```sql
/// CREATE contratista CONTENT $dto
/// ```
///
/// ## Query 2: FETCH
/// ```sql
/// SELECT * FROM $id FETCH empresa
/// ```
///
/// ## Parámetros
/// * `dto` - DTO con datos validados
///
/// ## Retorno
/// * `Ok(ContratistaFetched)` - Contratista creado con empresa populated
///
/// ## Errores
/// * `SurrealDbError::Query` - Si algún query falla
/// * `SurrealDbError::NotFound` - Si no se puede recuperar el registro creado
pub async fn create(dto: ContratistaCreateDTO) -> Result<ContratistaFetched, SurrealDbError> {
    let db = get_db().await?;
    
    // Paso 1: Crear el registro
    let created: Option<Contratista> = db
        .query("CREATE contratista CONTENT $dto")
        .bind(("dto", &dto))
        .await
        .map_err(|e| SurrealDbError::Query(format!(
            "Error al crear contratista: {}",
            e
        )))?
        .take(0)?;
    
    let contratista = created.ok_or_else(|| {
        SurrealDbError::NotFound("CREATE no retornó registro".to_string())
    })?;
    
    // Paso 2: Fetch con empresa populated
    let mut result = db
        .query("SELECT * FROM $id FETCH empresa")
        .bind(("id", &contratista.id))
        .await
        .map_err(|e| SurrealDbError::Query(format!(
            "Error al fetch contratista creado: {}",
            e
        )))?;
    
    let fetched: Option<ContratistaFetched> = result.take(0)?;
    fetched.ok_or_else(|| {
        SurrealDbError::NotFound(format!(
            "No se pudo recuperar contratista creado con ID: {}",
            contratista.id
        ))
    })
}
```

---

### 6. [ ] Patrón Soft Delete

**Objetivo**: Consistencia en borrado lógico.

**Acción**:

```rust
/// Marca un contratista como eliminado (soft delete).
///
/// ## Soft Delete
/// Esta función NO elimina el registro físicamente de la base de datos.
/// En su lugar, marca el campo `deleted_at` con el timestamp actual.
///
/// Los registros eliminados son filtrados automáticamente por otros queries
/// mediante la condición `WHERE deleted_at IS NONE`.
///
/// ## Query
/// ```sql
/// UPDATE $id SET deleted_at = time::now()
/// ```
///
/// ## Parámetros
/// * `id` - ID del contratista a eliminar
///
/// ## Retorno
/// * `Ok(())` - Eliminado exitosamente
///
/// ## Errores
/// * `SurrealDbError::Query` - Si el query falla
/// * `SurrealDbError::NotFound` - Si el ID no existe
///
/// ## Restauración
/// Para restaurar un registro eliminado, usar `restore()`.
pub async fn delete(id: &RecordId) -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    
    let result: Option<Contratista> = db
        .query("UPDATE $id SET deleted_at = time::now()")
        .bind(("id", id))
        .await
        .map_err(|e| SurrealDbError::Query(format!(
            "Error al eliminar contratista {}: {}",
            id, e
        )))?
        .take(0)?;
    
    if result.is_none() {
        return Err(SurrealDbError::NotFound(format!(
            "Contratista no encontrado: {}",
            id
        )));
    }
    
    Ok(())
}

/// Restaura un contratista previamente eliminado.
///
/// ## Restauración
/// Limpia el campo `deleted_at` para que el registro vuelva a ser visible
/// en los queries normales.
///
/// ## Query
/// ```sql
/// UPDATE $id SET deleted_at = NONE
/// ```
///
/// ## Parámetros
/// * `id` - ID del contratista a restaurar
///
/// ## Retorno
/// * `Ok(())` - Restaurado exitosamente
///
/// ## Errores
/// * `SurrealDbError::Query` - Si el query falla
/// * `SurrealDbError::NotFound` - Si el ID no existe
pub async fn restore(id: &RecordId) -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    
    let result: Option<Contratista> = db
        .query("UPDATE $id SET deleted_at = NONE")
        .bind(("id", id))
        .await
        .map_err(|e| SurrealDbError::Query(format!(
            "Error al restaurar contratista {}: {}",
            id, e
        )))?
        .take(0)?;
    
    if result.is_none() {
        return Err(SurrealDbError::NotFound(format!(
            "Contratista no encontrado: {}",
            id
        )));
    }
    
    Ok(())
}

/// Obtiene contratistas eliminados (archivados).
///
/// ## Query
/// ```sql
/// SELECT * FROM contratista 
/// WHERE deleted_at IS NOT NONE 
/// ORDER BY deleted_at DESC 
/// FETCH empresa
/// ```
///
/// ## Ordenamiento
/// Ordena por `deleted_at DESC` para mostrar los eliminados más recientemente primero.
pub async fn find_archived() -> Result<Vec<ContratistaFetched>, SurrealDbError> {
    let db = get_db().await?;
    
    let mut result = db
        .query(
            "SELECT * FROM contratista 
             WHERE deleted_at IS NOT NONE 
             ORDER BY deleted_at DESC 
             FETCH empresa"
        )
        .await?;
    
    Ok(result.take(0)?)
}
```

**Recordatorio en queries de lectura**:

```rust
/// Obtiene todos los contratistas activos (no eliminados).
///
/// ## Filtro de Soft Delete
/// Usa `WHERE deleted_at IS NONE` para excluir registros eliminados.
/// Para ver registros eliminados, usar `find_archived()`.
pub async fn find_all() -> Result<Vec<Contratista>, SurrealDbError> {
    let db = get_db().await?;
    let result: Vec<Contratista> = db
        .query("SELECT * FROM contratista WHERE deleted_at IS NONE LIMIT 1000")
        .await?
        .take(0)?;
    Ok(result)
}
```

---

### 7. [ ] Tests de Integración

**Objetivo**: Verificar que queries funcionen correctamente contra BD real.

**Setup de tests**:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::surrealdb_service::init_db;
    
    async fn setup_test_db() {
        // Inicializar BD de test (en memoria o archivo temporal)
        init_db(":memory:").await.expect("Failed to init test DB");
    }
    
    async fn cleanup_test_db() {
        // Limpiar datos de test si es necesario
    }
    
    // --------------------------------------------------------------------------
    // TESTS DE CREATE
    // --------------------------------------------------------------------------
    
    mod create_tests {
        use super::*;
        
        #[tokio::test]
        async fn crea_contratista_exitosamente() {
            setup_test_db().await;
            
            let dto = ContratistaCreateDTO {
                cedula: "1-2345-6789".to_string(),
                nombre: "Juan".to_string(),
                // ... resto de campos
            };
            
            let resultado = create(dto).await;
            
            assert!(resultado.is_ok());
            let contratista = resultado.unwrap();
            assert_eq!(contratista.cedula, "1-2345-6789");
            assert_eq!(contratista.nombre, "Juan");
        }
        
        #[tokio::test]
        async fn create_retorna_con_empresa_populated() {
            setup_test_db().await;
            
            // ... crear empresa primero
            // ... crear contratista
            
            let contratista = create(dto).await.unwrap();
            
            // Verificar que empresa está populated
            assert_eq!(contratista.empresa.nombre, "Empresa Test");
        }
    }
    
    // --------------------------------------------------------------------------
    // TESTS DE READ
    // --------------------------------------------------------------------------
    
    mod read_tests {
        use super::*;
        
        #[tokio::test]
        async fn find_by_cedula_encuentra_existente() {
            setup_test_db().await;
            
            // ... crear contratista
            
            let resultado = find_by_cedula("1-2345-6789").await;
            
            assert!(resultado.is_ok());
            let contratista = resultado.unwrap();
            assert!(contratista.is_some());
        }
        
        #[tokio::test]
        async fn find_by_cedula_retorna_none_si_no_existe() {
            setup_test_db().await;
            
            let resultado = find_by_cedula("9-9999-9999").await;
            
            assert!(resultado.is_ok());
            assert!(resultado.unwrap().is_none());
        }
    }
    
    // --------------------------------------------------------------------------
    // TESTS DE SOFT DELETE
    // --------------------------------------------------------------------------
    
    mod soft_delete_tests {
        use super::*;
        
        #[tokio::test]
        async fn delete_marca_como_eliminado() {
            setup_test_db().await;
            
            let contratista = create(dto).await.unwrap();
            
            let resultado = delete(&contratista.id).await;
            assert!(resultado.is_ok());
            
            // Verificar que ya no aparece en find_all
            let todos = find_all().await.unwrap();
            assert!(!todos.iter().any(|c| c.id == contratista.id));
            
            // Verificar que SÍ aparece en find_archived
            let archivados = find_archived().await.unwrap();
            assert!(archivados.iter().any(|c| c.id == contratista.id));
        }
        
        #[tokio::test]
        async fn restore_recupera_eliminado() {
            setup_test_db().await;
            
            let contratista = create(dto).await.unwrap();
            delete(&contratista.id).await.unwrap();
            
            let resultado = restore(&contratista.id).await;
            assert!(resultado.is_ok());
            
            // Verificar que vuelve a aparecer en find_all
            let todos = find_all().await.unwrap();
            assert!(todos.iter().any(|c| c.id == contratista.id));
        }
    }
}
```

---

### 8. [ ] Verificación Final

**Checklist de Queries**:

- [ ] Sin lógica de negocio (validaciones en `domain/`)
- [ ] Todas las funciones documentadas con `///`
- [ ] Queries explicados en comentarios SQL
- [ ] Uso de `FETCH` documentado
- [ ] Índices requeridos documentados
- [ ] Soft delete implementado consistentemente (`deleted_at`)
- [ ] Límites en queries de listado (`LIMIT`)
- [ ] Errores con contexto específico
- [ ] Tests de integración para funciones críticas
- [ ] Sin `unwrap()` en código de producción
- [ ] Sin `println!()` (usar `log::` o eliminar)

**Compilación y Tests**:

```bash
# Verificar compilación
cargo check --package mega-brisas

# Ejecutar tests de integración
cargo test --package mega-brisas --test integration_tests

# Verificar warnings
cargo clippy --package mega-brisas -- -D warnings
```

---

## Plantilla de Commit

```
refactor(db): mejorar queries de {modulo} con documentación y optimizaciones

- Documentar todos los queries con explicaciones SQL
- Explicar uso de FETCH para evitar N+1
- Optimizar queries de listado con LIMIT
- Agregar contexto específico a errores
- Implementar tests de integración para funciones críticas
- Documentar índices requeridos para performance
- Consistencia en patrón de soft delete

Closes #{numero_issue}
```

---

**Fin del Workflow D - Queries SurrealDB**
