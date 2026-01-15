# Instrucciones: Agente de Escritura de Tests

**Objetivo**: Escribir una suite de tests completa basada en comportamiento (behavior-driven) que valide las reglas de negocio documentadas en `ARCHITECTURE.md`.

**Contexto**: Los tests removidos estaban acoplados a implementación. Los nuevos tests deben enfocarse en validar QUÉ hace el sistema (comportamiento), no CÓMO lo hace (implementación).

---

## 1. Principios Fundamentales

### 🎯 Testing de Comportamiento vs Implementación

```rust
// ❌ MAL: Testa implementación (frágil)
#[test]
fn test_servicio_usa_query_correcta() {
    let mock = MockRepo::new();
    let service = IngresoService::new(mock);
    
    service.registrar("123");
    
    // Esto falla si optimizas la query, aunque el resultado sea correcto
    assert_eq!(mock.last_query(), "SELECT * FROM contratistas WHERE cedula = ?");
}

// ✅ BIEN: Testa comportamiento (robusto)
#[test]
fn test_rn_ing_001_contratista_inactivo_no_puede_ingresar() {
    // Given: Un sistema con un contratista inactivo
    let service = setup_service_with_inactive_contractor("123");
    
    // When: Se intenta registrar ingreso
    let resultado = service.registrar_ingreso("123");
    
    // Then: El sistema rechaza el ingreso
    assert!(matches!(resultado, Err(IngresoError::ContratistaInactivo)));
}
```

### 📋 Regla de Oro

**Cada test debe mapear a UNA regla de negocio del documento `ARCHITECTURE.md`.**

Usa el formato de nombres: `test_rn_{modulo}_{numero}_{descripcion}`

Ejemplo:
- `test_rn_cont_003_inactivo_no_puede_ingresar`
- `test_rn_ing_002_no_duplicar_ingreso_activo`

---

## 2. Estructura de Archivos

### Layout de Tests

```
src-tauri/
└── tests/                          ← Integration tests (CREAR)
    ├── common/
    │   └── mod.rs                 ← Test utilities compartidas
    ├── contratista_tests.rs       ← Tests de bounded context Contratista
    ├── ingreso_tests.rs           ← Tests de bounded context Ingreso
    └── reportes_tests.rs          ← Tests de bounded context Reportes
```

**NO crear** tests unitarios dentro de `src/`. Solo integration tests en `tests/`.

---

## 3. Instrucciones por Módulo

### 3.1 Contratista Tests (`tests/contratista_tests.rs`)

**Reglas de Negocio a Validar** (del documento `ARCHITECTURE.md`):

| Regla | Descripción | Test a Crear |
|-------|-------------|--------------|
| RN-CONT-001 | Estado binario (activo/inactivo) | `test_rn_cont_001_estado_solo_activo_o_inactivo` |
| RN-CONT-002 | Cédula única | `test_rn_cont_002_cedula_debe_ser_unica` |
| RN-CONT-003 | Inactivo no ingresa | `test_rn_cont_003_inactivo_no_registra_ingreso` |
| RN-CONT-004 | Solo admin cambia estado | `test_rn_cont_004_solo_admin_cambia_estado` |

**Ejemplo de Implementación**:

```rust
use mega_brisas::contratista::{ContratistaService, Contractor, ContratistaError};

#[test]
fn test_rn_cont_002_cedula_debe_ser_unica() {
    // Given: Un servicio con un contratista existente
    let service = ContratistaService::new_with_test_db();
    let contratista1 = Contractor {
        cedula: "123456789".to_string(),
        nombre: "Juan".to_string(),
        activo: true,
    };
    service.crear(contratista1.clone()).expect("Primer insert debe pasar");
    
    // When: Se intenta crear otro con la misma cédula
    let contratista2 = Contractor {
        cedula: "123456789".to_string(), // Misma cédula
        nombre: "Pedro".to_string(),
        activo: true,
    };
    let resultado = service.crear(contratista2);
    
    // Then: El sistema rechaza la operación
    assert!(matches!(resultado, Err(ContratistaError::CedulaDuplicada)));
}
```

### 3.2 Ingreso Tests (`tests/ingreso_tests.rs`)

**Reglas de Negocio a Validar**:

| Regla | Test a Crear |
|-------|--------------|
| RN-ING-001 | `test_rn_ing_001_requiere_contratista_activo` |
| RN-ING-002 | `test_rn_ing_002_no_duplicar_ingreso_activo` |
| RN-ING-003 | `test_rn_ing_003_salida_requiere_entrada_previa` |
| RN-ING-004 | `test_rn_ing_004_salida_posterior_a_entrada` |
| RN-ING-005 | `test_rn_ing_005_timestamp_del_servidor` |

**Ejemplo de Implementación**:

```rust
use mega_brisas::ingreso::{IngresoService, IngresoError};
use mega_brisas::contratista::Contractor;

#[test]
fn test_rn_ing_002_no_duplicar_ingreso_activo() {
    // Given: Un contratista con ingreso activo (sin salida)
    let service = IngresoService::new_with_test_db();
    let contratista = crear_contratista_activo("123456789");
    service.registrar_entrada(contratista.cedula.clone())
        .expect("Primera entrada debe pasar");
    
    // When: Se intenta registrar otra entrada sin haber registrado salida
    let resultado = service.registrar_entrada(contratista.cedula.clone());
    
    // Then: El sistema rechaza la operación
    assert!(matches!(resultado, Err(IngresoError::IngresoDuplicado)));
}
```

### 3.3 Reportes Tests (`tests/reportes_tests.rs`)

**Reglas de Negocio a Validar**:

| Regla | Test a Crear |
|-------|--------------|
| RN-REP-001 | `test_rn_rep_001_datos_desde_database_no_cache` |
| RN-REP-002 | `test_rn_rep_002_exportacion_incluye_timestamp` |
| RN-REP-003 | `test_rn_rep_003_solo_rol_reportes_exporta` |

---

## 4. Test Utilities (Helpers Compartidos)

### Archivo: `tests/common/mod.rs`

```rust
use mega_brisas::contratista::{Contractor, ContratistaService};
use mega_brisas::ingreso::IngresoService;
use surrealdb::{Surreal, engine::local::Mem};

/// Crea una base de datos en memoria para tests
pub async fn setup_test_db() -> Surreal<Mem> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    
    // Ejecutar migraciones/schema
    setup_schema(&db).await;
    
    db
}

/// Crea un contratista activo de prueba
pub fn crear_contratista_activo(cedula: &str) -> Contractor {
    Contractor {
        cedula: cedula.to_string(),
        nombre: format!("Test Contractor {}", cedula),
        activo: true,
    }
}

/// Crea un contratista inactivo de prueba
pub fn crear_contratista_inactivo(cedula: &str) -> Contractor {
    Contractor {
        cedula: cedula.to_string(),
        nombre: format!("Test Contractor {}", cedula),
        activo: false,
    }
}

// Más helpers según necesites...
```

---

## 5. Estrategia de Dependency Injection

### Problema: Evitar mocks complejos

En lugar de crear 20 líneas de mocks, usa **Repositories con implementaciones reales en memoria**.

**Estructura Recomendada**:

```rust
// En producción: src/contratista/repository.rs
pub trait ContratistaRepository {
    fn crear(&self, c: Contractor) -> Result<()>;
    fn buscar_por_cedula(&self, cedula: &str) -> Result<Contractor>;
}

pub struct SurrealDBContratistaRepo {
    db: Surreal<Any>,
}

// En tests: Usa la misma implementación con DB en memoria
impl SurrealDBContratistaRepo {
    pub fn new_test() -> Self {
        let db = Surreal::new::<Mem>(()).await.unwrap();
        // ... setup
        Self { db }
    }
}
```

**NO hagas esto** (mock complejo):

```rust
// ❌ Evitar (frágil y acoplado)
struct MockRepo {
    called_with: RefCell<Vec<String>>,
    return_value: Result<Contractor>,
}

impl ContratistaRepository for MockRepo {
    fn buscar_por_cedula(&self, cedula: &str) -> Result<Contractor> {
        self.called_with.borrow_mut().push(cedula.to_string());
        self.return_value.clone()
    }
}
```

---

## 6. Casos de Prueba Mínimos (Checklist)

### Por Cada Regla de Negocio, Crear:

- ✅ **Happy Path**: El caso donde la regla se cumple y todo funciona
- ✅ **Sad Path**: El caso donde la regla se viola y debe fallar
- ✅ **Edge Cases**: Límites (ej: timestamp exactamente igual, cédula vacía)

**Ejemplo para RN-ING-001**:

```rust
// Happy Path
#[test]
fn test_rn_ing_001_contratista_activo_puede_ingresar() {
    let service = setup_service();
    let contratista = crear_contratista_activo("123");
    
    let resultado = service.registrar_entrada(contratista.cedula);
    
    assert!(resultado.is_ok());
}

// Sad Path
#[test]
fn test_rn_ing_001_contratista_inactivo_no_puede_ingresar() {
    let service = setup_service();
    let contratista = crear_contratista_inactivo("123");
    
    let resultado = service.registrar_entrada(contratista.cedula);
    
    assert!(matches!(resultado, Err(IngresoError::ContratistaInactivo)));
}

// Edge Case
#[test]
fn test_rn_ing_001_contratista_sin_estado_definido_falla() {
    // Si el campo `activo` es Option<bool>, testear caso None
}
```

---

## 7. Nomenclatura y Estilo

### Nombres de Tests

```rust
// ✅ BIEN: Descriptivo y mapeado a regla de negocio
#[test]
fn test_rn_ing_002_no_duplicar_ingreso_activo() { }

// ❌ MAL: Genérico
#[test]
fn test_ingreso() { }

// ❌ MAL: Testa implementación
#[test]
fn test_repository_returns_correct_sql() { }
```

### Estructura AAA (Arrange-Act-Assert)

```rust
#[test]
fn test_ejemplo() {
    // Arrange (Given): Preparar el estado inicial
    let service = setup_service();
    let contratista = crear_contratista_activo("123");
    
    // Act (When): Ejecutar la acción
    let resultado = service.registrar_entrada(contratista.cedula);
    
    // Assert (Then): Verificar el resultado
    assert!(resultado.is_ok());
}
```

**Usa comentarios `// Given`, `// When`, `// Then`** para claridad.

---

## 8. Validación y Entrega

### Criterios de Aceptación

Antes de marcar como "completo", verificar:

```bash
# 1. Todos los tests pasan
cargo test --no-fail-fast

# 2. Cobertura de reglas de negocio
# Cada regla RN-XXX-### del ARCHITECTURE.md debe tener al menos 1 test

# 3. No hay warnings de clippy en tests
cargo clippy --tests -- -D warnings

# 4. Tests son independientes (orden de ejecución no importa)
cargo test -- --test-threads=1  # Debe pasar igual que en paralelo
```

### Métricas Esperadas

- **Mínimo 15 tests** (5 por módulo aproximadamente)
- **100% de reglas de negocio cubiertas** (todas las RN-XXX-### tienen test)
- **0 mocks complejos** (usar DB en memoria, no mocks de 20 líneas)

---

## 9. Errores Comunes a Evitar

### ❌ Anti-Patrón 1: Testear métodos privados

```rust
// ❌ NO HAGAS ESTO
#[test]
fn test_metodo_interno_privado() {
    // Si el método es privado, NO lo testees directamente
}
```

**Solución**: Testea el comportamiento público. Si un método privado tiene bugs, fallará un test público.

### ❌ Anti-Patrón 2: Tautologías

```rust
// ❌ NO HAGAS ESTO (replica la lógica)
#[test]
fn test_calcular_total() {
    let precio = 100;
    let cantidad = 2;
    let descuento = 10;
    
    let resultado = calcular_total(precio, cantidad, descuento);
    
    // Esto es tautología: estás replicando la fórmula del código
    assert_eq!(resultado, precio * cantidad - descuento);
}
```

**Solución**: Usa valores concretos esperados, no fórmulas:

```rust
// ✅ BIEN
#[test]
fn test_calcular_total_con_descuento() {
    let resultado = calcular_total(100, 2, 10);
    assert_eq!(resultado, 190); // Valor esperado concreto
}
```

### ❌ Anti-Patrón 3: Tests que dependen de orden

```rust
// ❌ NO HAGAS ESTO
static mut CONTADOR: i32 = 0;

#[test]
fn test_1_incrementa() {
    unsafe { CONTADOR += 1; }
}

#[test]
fn test_2_verifica() {
    unsafe { assert_eq!(CONTADOR, 1); } // Depende de test_1
}
```

**Solución**: Cada test debe ser independiente. Usa `setup()` y `teardown()` si necesitas estado.

---

## 10. Ejemplo Completo de Archivo de Test

```rust
// tests/ingreso_tests.rs

use mega_brisas::ingreso::{IngresoService, IngresoError};
use mega_brisas::contratista::Contractor;

mod common;
use common::{setup_test_db, crear_contratista_activo, crear_contratista_inactivo};

#[tokio::test]
async fn test_rn_ing_001_contratista_activo_puede_registrar_entrada() {
    // Given: Un servicio con un contratista activo
    let db = setup_test_db().await;
    let service = IngresoService::new(db);
    let contratista = crear_contratista_activo("123456789");
    service.contratista_repo.crear(contratista.clone()).await.unwrap();
    
    // When: Se registra una entrada
    let resultado = service.registrar_entrada(contratista.cedula).await;
    
    // Then: La operación es exitosa
    assert!(resultado.is_ok(), "Un contratista activo debe poder ingresar");
}

#[tokio::test]
async fn test_rn_ing_001_contratista_inactivo_no_puede_registrar_entrada() {
    // Given: Un servicio con un contratista inactivo
    let db = setup_test_db().await;
    let service = IngresoService::new(db);
    let contratista = crear_contratista_inactivo("987654321");
    service.contratista_repo.crear(contratista.clone()).await.unwrap();
    
    // When: Se intenta registrar una entrada
    let resultado = service.registrar_entrada(contratista.cedula).await;
    
    // Then: El sistema rechaza la operación
    assert!(
        matches!(resultado, Err(IngresoError::ContratistaInactivo)),
        "RN-ING-001: Un contratista inactivo NO debe poder ingresar"
    );
}

#[tokio::test]
async fn test_rn_ing_002_no_permite_ingreso_duplicado() {
    // Given: Un contratista con ingreso activo (sin salida)
    let db = setup_test_db().await;
    let service = IngresoService::new(db);
    let contratista = crear_contratista_activo("555555555");
    service.contratista_repo.crear(contratista.clone()).await.unwrap();
    service.registrar_entrada(contratista.cedula.clone()).await.unwrap();
    
    // When: Se intenta registrar otra entrada sin salida previa
    let resultado = service.registrar_entrada(contratista.cedula).await;
    
    // Then: El sistema rechaza la operación
    assert!(
        matches!(resultado, Err(IngresoError::IngresoDuplicado)),
        "RN-ING-002: No se permite duplicar ingreso activo"
    );
}

// ... más tests para RN-ING-003, RN-ING-004, RN-ING-005
```

---

## 11. Entregables Finales

Al completar la tarea, debes entregar:

1. ✅ Archivos de tests creados en `tests/`
2. ✅ `tests/common/mod.rs` con utilities compartidas
3. ✅ Output de `cargo test` mostrando todos los tests pasando
4. ✅ Reporte de cobertura:
   ```markdown
   # Cobertura de Reglas de Negocio
   
   ## Contratista
   - [x] RN-CONT-001 (test_rn_cont_001_...)
   - [x] RN-CONT-002 (test_rn_cont_002_...)
   - [x] RN-CONT-003 (test_rn_cont_003_...)
   - [x] RN-CONT-004 (test_rn_cont_004_...)
   
   ## Ingreso
   - [x] RN-ING-001 (test_rn_ing_001_...)
   ...
   ```

---

## 12. Comando de Verificación Final

```bash
# Ejecutar TODOS los checks
cargo test --no-fail-fast && \
cargo clippy --tests -- -D warnings && \
echo "✓ Suite de tests completada exitosamente"
```

---

## 📚 Referencias

- Lee `ARCHITECTURE.md` COMPLETO antes de empezar
- Consulta las reglas de negocio en la sección 5 del `ARCHITECTURE.md`
- Revisa el ejemplo de testing en la sección 4.3 del `ARCHITECTURE.md`

**¡Buena suerte! Recuerda: testa COMPORTAMIENTO, no implementación.**
