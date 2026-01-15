# Mega Brisas - Arquitectura y Visión de Dominio

**Última actualización**: 2026-01-08  
**Estado**: Living Document  
**Tipo de Aplicación**: Desktop Application (Tauri + Svelte + Rust)

---

## 1. Visión General

**Mega Brisas** es un sistema de control de acceso y gestión de personal para empresas constructoras. Permite registrar ingresos/egresos de contratistas, gestionar su información, generar reportes y mantener un historial completo de actividad.

### Objetivo de Negocio
Reemplazar procesos manuales de registro de acceso con un sistema digital robusto que garantice:
- **Trazabilidad completa** de ingreso/egreso de personal
- **Validación automática** de permisos y estado de contratistas
- **Reportería precisa** para auditorías y control operativo
- **Desempeño offline-first** (no depende de conexión a internet)

---

## 2. Bounded Contexts (Dominios de Negocio)

### 2.1 Gestión de Contratistas (`contratista` module)

**Responsabilidad**: Administración del ciclo de vida de contratistas (CRUD + validaciones).

**Entidades Principales**:
- `Contractor`: Representa a un trabajador externo con datos personales y estado laboral

**Reglas de Negocio**:
1. **RN-CONT-001**: Un contratista solo puede estar `activo` o `inactivo`
2. **RN-CONT-002**: La cédula debe ser única en el sistema (constraint de base de datos)
3. **RN-CONT-003**: Un contratista inactivo NO puede registrar ingreso (validación en capa de servicio)
4. **RN-CONT-004**: Solo administradores pueden cambiar el estado de un contratista

**Eventos de Dominio**:
- `ContratistaCreadoEvent`
- `ContratistaActivadoEvent`
- `ContratistaDesactivadoEvent`

**Dependencias Externas**: Ninguna (dominio raíz)

---

### 2.2 Control de Acceso (`ingreso` module)

**Responsabilidad**: Registro y validación de entradas/salidas de contratistas.

**Entidades Principales**:
- `Ingreso`: Registro de un evento de acceso (entrada o salida)

**Reglas de Negocio**:
1. **RN-ING-001**: Un contratista DEBE estar activo para registrar ingreso
2. **RN-ING-002**: No se permite ingreso duplicado (mismo contratista con entrada activa sin salida)
3. **RN-ING-003**: Para registrar salida, debe existir una entrada previa sin salida
4. **RN-ING-004**: La hora de salida DEBE ser posterior a la hora de entrada
5. **RN-ING-005**: Se registra automáticamente timestamp del servidor (no se confía en cliente)

**Eventos de Dominio**:
- `IngresoRegistradoEvent`
- `SalidaRegistradaEvent`

**Dependencias**: 
- `contratista` module (lee estado del contratista, no lo modifica)

---

### 2.3 Reportería (`reportes` module)

**Responsabilidad**: Generación de reportes analíticos y exportación de datos.

**Casos de Uso**:
- Reporte diario de asistencia
- Historial por contratista
- Exportación a Excel/PDF
- Estadísticas de acceso

**Reglas de Negocio**:
1. **RN-REP-001**: Los reportes SIEMPRE muestran datos desde la base de datos (no cache)
2. **RN-REP-002**: Las exportaciones incluyen marca de agua con fecha de generación
3. **RN-REP-003**: Solo usuarios con rol `reportes` o superior pueden exportar datos

**Dependencias**:
- `ingreso` module (lectura de datos históricos)
- `contratista` module (información de contratistas)

---

## 3. Arquitectura Técnica

### 3.1 Capas de la Aplicación

```
┌─────────────────────────────────────────┐
│         Frontend (Svelte)               │  ← UI Components
├─────────────────────────────────────────┤
│      Tauri Commands (Rust)              │  ← API Bridge
├─────────────────────────────────────────┤
│   Services (Business Logic)             │  ← Domain Rules (AQUÍ SE TESTEA)
├─────────────────────────────────────────┤
│   Repositories (Data Access)            │  ← Database Abstraction
├─────────────────────────────────────────┤
│      SurrealDB (Embedded)               │  ← Persistent Storage
└─────────────────────────────────────────┘
```

### 3.2 Principios de Diseño

1. **Modularización por Bounded Context**: Cada módulo (`contratista`, `ingreso`, `reportes`) es autónomo
2. **Dependency Inversion**: Los servicios dependen de traits, no de implementaciones concretas
3. **Separation of Concerns**: La lógica de negocio (services) está separada de la infraestructura (repositories)
4. **Fail-Fast**: Las validaciones de negocio se ejecutan ANTES de tocar la base de datos

### 3.3 Ejemplo de Flujo (Registro de Ingreso)

```rust
// 1. Frontend llama al command
invoke('registrar_ingreso', { cedula: "12345678" })

// 2. Tauri Command delega al servicio
#[tauri::command]
fn registrar_ingreso(cedula: String) -> Result<Ingreso> {
    ingreso_service.registrar(cedula)  // ← NO lógica aquí, solo delegación
}

// 3. Servicio aplica reglas de negocio (AQUÍ TESTEAS)
impl IngresoService {
    fn registrar(&self, cedula: String) -> Result<Ingreso> {
        // Validación RN-ING-001
        let contratista = self.contratista_repo.buscar_por_cedula(cedula)?;
        if !contratista.activo {
            return Err(Error::ContratistaInactivo);
        }
        
        // Validación RN-ING-002
        if self.ingreso_repo.tiene_ingreso_activo(cedula)? {
            return Err(Error::IngresoDuplicado);
        }
        
        // Persistencia
        self.ingreso_repo.guardar(Ingreso::nuevo(contratista))
    }
}
```

---

## 4. Estrategia de Testing

### 4.1 Pirámide de Testing (Adaptada a Brisas)

```
        ┌──────────┐
        │  E2E     │  ← 5% (Manual/Smoke tests)
        │  Tests   │
        ├──────────┤
        │Integration│ ← 30% (Servicios + DB real/in-memory)
        │  Tests    │
        ├──────────┤
        │   Unit    │  ← 65% (Reglas de negocio puras)
        │  Tests    │
        └──────────┘
```

### 4.2 Qué Testear en Cada Capa

**Unit Tests** (Lógica pura, sin dependencias):
- ✅ Validaciones de negocio (ej: `contratista.puede_ingresar()`)
- ✅ Transformaciones de datos
- ✅ Cálculos de reportes

**Integration Tests** (Servicio + Repository + DB):
- ✅ Flujos completos (ej: `registrar_ingreso` end-to-end)
- ✅ Interacción entre bounded contexts
- ✅ Validaciones que requieren estado de DB

**E2E Tests** (Frontend + Backend):
- ✅ Smoke tests en releases (flujo crítico: login → ingreso → reporte)
- ❌ NO automatizar todo el UI (demasiado frágil)

### 4.3 Ejemplo de Test Correcto (Behavior-Driven)

```rust
// ✅ CORRECTO: Testa COMPORTAMIENTO, no implementación
#[test]
fn contratista_inactivo_no_puede_registrar_ingreso() {
    // Arrange
    let service = IngresoService::new_with_mocks();
    let contratista_inactivo = Contractor { activo: false, ..Default::default() };
    
    // Act
    let resultado = service.registrar_ingreso(contratista_inactivo.cedula);
    
    // Assert
    assert!(matches!(resultado, Err(IngresoError::ContratistaInactivo)));
}

// ❌ INCORRECTO: Testa implementación (frágil)
#[test]
fn servicio_llama_a_repositorio_con_parametros_correctos() {
    let mock_repo = MockRepository::new();
    let service = IngresoService::new(mock_repo);
    
    service.registrar_ingreso("12345678");
    
    // Esto falla si refactorizas el método interno, aunque el comportamiento sea correcto
    assert_eq!(mock_repo.called_with(), "SELECT * FROM ...");
}
```

---

## 5. Reglas de Negocio Centralizadas

### Contratistas
| ID | Regla | Validación |
|----|-------|-----------|
| RN-CONT-001 | Estado binario (activo/inactivo) | Enum en modelo |
| RN-CONT-002 | Cédula única | Constraint DB + validación pre-insert |
| RN-CONT-003 | Inactivo no ingresa | `IngresoService::validar_estado()` |
| RN-CONT-004 | Solo admin cambia estado | Guard en Tauri command |

### Ingresos
| ID | Regla | Validación |
|----|-------|-----------|
| RN-ING-001 | Requiere contratista activo | `IngresoService::validar_contratista()` |
| RN-ING-002 | No duplicar ingreso | `IngresoRepository::tiene_ingreso_activo()` |
| RN-ING-003 | Salida requiere entrada previa | `IngresoService::validar_salida()` |
| RN-ING-004 | Salida > Entrada (tiempo) | Validación en modelo `Ingreso` |
| RN-ING-005 | Timestamp del servidor | `Ingreso::new()` usa `SystemTime::now()` |

---

## 6. Decisiones Arquitectónicas Pendientes

1. **Autenticación/Autorización**: ¿JWT local o sesión en DB?
2. **Sincronización multi-dispositivo**: ¿Necesario o single-instance?
3. **Backup automático**: ¿Exportar DB a Google Drive/Dropbox?
4. **Logs de auditoría**: ¿Implementar event sourcing ligero?

---

## 7. Cómo Usar Este Documento

**Para Desarrolladores**:
- Consulta este doc ANTES de modificar reglas de negocio
- Actualiza las tablas de reglas cuando agregues validaciones
- Usa los códigos RN-XXX-### en mensajes de error

**Para QA/Testers**:
- Cada regla RN-XXX-### debe tener un test que la valide
- Los test names deben referenciar el código de regla (ej: `test_rn_ing_001_contratista_inactivo`)

**Para Agentes de IA**:
- Lee este documento COMPLETO antes de escribir tests
- No testees implementación, testea las reglas de negocio listadas aquí
- Mantén la separación de bounded contexts

---

## 8. Próximos Pasos

- [ ] Implementar ADRs para decisiones futuras (ver `ADR_TEMPLATE.md`)
- [ ] Documentar Context Map (relaciones entre bounded contexts)
- [ ] Agregar diagramas C4 (Context, Container, Component)
- [ ] Definir estrategia de migración de datos para futuras versiones
