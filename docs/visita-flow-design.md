# Diseño: Flujo de Visitas

## Modelo de Datos

```mermaid
erDiagram
    VISITANTE ||--o{ PRE_REGISTRO : "puede tener"
    VISITANTE ||--o{ INGRESO_VISITA : "genera"
    PRE_REGISTRO ||--o| INGRESO_VISITA : "se completa en"
    EMPRESA ||--o{ VISITANTE : "trabaja para"
    VEHICULO ||--o{ INGRESO_VISITA : "usa"
    USER ||--o{ PRE_REGISTRO : "registra"
    USER ||--o{ INGRESO_VISITA : "registra ingreso/salida"

    VISITANTE {
        id string PK
        cedula string UK
        nombre string
        apellido string
        segundo_nombre string
        segundo_apellido string
        empresa RecordId FK
        telefono string
        email string
        created_at datetime
        updated_at datetime
    }

    PRE_REGISTRO {
        id string PK
        visitante RecordId FK
        cedula string
        nombre string
        apellido string
        empresa_nombre string
        fecha_esperada date
        anfitrion string
        area_visitada string
        motivo string
        modo_ingreso string
        observaciones string
        estado enum "PENDIENTE|COMPLETADO|CANCELADO|NO_SHOW"
        registrado_por RecordId FK
        created_at datetime
    }

    INGRESO_VISITA {
        id string PK
        visitante RecordId FK
        pre_registro RecordId FK
        cedula string
        nombre string
        apellido string
        empresa_nombre string
        anfitrion string
        area_visitada string
        motivo string
        modo_ingreso string
        placa_vehiculo string
        vehiculo RecordId FK
        gafete_numero int
        fecha_hora_ingreso datetime
        fecha_hora_salida datetime
        usuario_ingreso RecordId FK
        usuario_salida RecordId FK
        observaciones string
        created_at datetime
    }
```

---

## Diagrama de Flujo: Ingreso de Visita

```mermaid
flowchart TD
    START([🚶 Visitante llega]) --> INPUT[/Guardia escribe cédula/]
    INPUT --> CHECK_PREREG{¿Tiene pre-registro<br/>PENDIENTE?}

    CHECK_PREREG -->|SI| PREREG_FOUND[📋 Cargar datos del pre-registro]
    PREREG_FOUND --> QUICK_VIEW[🚀 Vista Rápida<br/>Solo asignar gafete]
    QUICK_VIEW --> SAVE_WITH_PREREG[Guardar]
    SAVE_WITH_PREREG --> CREATE_INGRESO_P[✅ Crear INGRESO_VISITA<br/>con pre_registro_id]
    CREATE_INGRESO_P --> UPDATE_PREREG[📝 Pre-registro → COMPLETADO]
    UPDATE_PREREG --> END_SUCCESS([✅ Ingreso exitoso])

    CHECK_PREREG -->|NO| CHECK_CATALOG{¿Está en catálogo<br/>VISITANTE?}

    CHECK_CATALOG -->|SI| PARTIAL_FORM[📝 Formulario Parcial<br/>Datos personales pre-llenados<br/>Solo: anfitrión, área, motivo, gafete]
    PARTIAL_FORM --> SAVE_PARTIAL[Guardar]
    SAVE_PARTIAL --> CREATE_INGRESO_C[✅ Crear INGRESO_VISITA]
    CREATE_INGRESO_C --> END_SUCCESS

    CHECK_CATALOG -->|NO| FULL_FORM[📝 Formulario Completo<br/>Datos personales + visita + gafete]
    FULL_FORM --> SAVE_FULL[Guardar]
    SAVE_FULL --> CREATE_VISITANTE[👤 Crear VISITANTE en catálogo]
    CREATE_VISITANTE --> CREATE_INGRESO_N[✅ Crear INGRESO_VISITA]
    CREATE_INGRESO_N --> END_SUCCESS
```

---

## Diagrama de Flujo: Pre-Registro

```mermaid
flowchart TD
    START([📧 Anfitrión manda correo]) --> OPEN[Guardia abre módulo Pre-Registro]
    OPEN --> INPUT[/Escribe cédula de la lista/]
    INPUT --> CHECK{¿Existe en catálogo?}

    CHECK -->|SI| LOAD[Cargar datos existentes]
    LOAD --> PARTIAL[Completar campos faltantes]
    PARTIAL --> VISIT_DATA[Llenar: fecha, anfitrión, área, motivo]

    CHECK -->|NO| FULL[Llenar datos personales completos]
    FULL --> CREATE_V[👤 Crear VISITANTE]
    CREATE_V --> VISIT_DATA

    VISIT_DATA --> SAVE[Guardar]
    SAVE --> CREATE_PREREG[📋 Crear PRE_REGISTRO<br/>Estado: PENDIENTE]
    CREATE_PREREG --> MORE{¿Más personas<br/>en el correo?}

    MORE -->|SI| INPUT
    MORE -->|NO| END([✅ Pre-registros creados])
```

---

## Diagrama de Estados: Pre-Registro

```mermaid
stateDiagram-v2
    [*] --> PENDIENTE: Guardia crea pre-registro

    PENDIENTE --> COMPLETADO: Visitante llega y entra
    PENDIENTE --> NO_SHOW: No llegó al final del día
    PENDIENTE --> NO_SHOW: Expiró (72 horas)
    PENDIENTE --> CANCELADO: Guardia cancela manualmente

    COMPLETADO --> [*]
    NO_SHOW --> [*]
    CANCELADO --> [*]
```

---

## Flujos de Operación

### Flujo 1: Walk-in SIN registro previo

```
Visitante llega → No tiene pre-registro NI está en catálogo
───────────────────────────────────────────────────────────
1. Guardia escribe cédula
2. Sistema busca: ❌ No en catálogo, ❌ No pre-registro
3. Formulario COMPLETO (datos personales + datos de visita + gafete)
4. Al guardar:
   • Crea registro en VISITANTE (catálogo)
   • Crea registro en INGRESO_VISITA
```

### Flujo 2: Walk-in CON registro en catálogo

```
Visitante llega → Está en catálogo pero sin pre-registro
───────────────────────────────────────────────────────────
1. Guardia escribe cédula
2. Sistema busca: ✅ En catálogo, ❌ No pre-registro
3. Datos personales PRE-LLENADOS (nombre, empresa)
4. Formulario PARCIAL (anfitrión, área, motivo, gafete)
5. Al guardar: Solo crea INGRESO_VISITA
```

### Flujo 3: Pre-registro (guardia recibe correo)

```
Anfitrión manda correo con lista de visitantes esperados
───────────────────────────────────────────────────────────
1. Guardia abre módulo de Pre-Registro
2. Por cada persona del correo:
   a. Escribe cédula
   b. ¿Está en catálogo?
      • SI → Cargar datos, completar campos faltantes
      • NO → Llenar datos personales completos
   c. Llenar: fecha esperada, anfitrión, área, motivo
   d. Al guardar:
      • Si no existía → Crear VISITANTE
      • Crear PRE_REGISTRO (estado: PENDIENTE)
```

### Flujo 4: Ingreso CON pre-registro

```
Visitante llega y tiene pre-registro pendiente
───────────────────────────────────────────────────────────
1. Guardia escribe cédula
2. Sistema busca: ✅ En catálogo, ✅ Pre-registro PENDIENTE
3. Vista SIMPLIFICADA (datos ya cargados, solo pedir gafete)
4. Al guardar:
   • Crea INGRESO_VISITA (con pre_registro_id)
   • Actualiza PRE_REGISTRO → estado: COMPLETADO
```

---

## Políticas Definidas

| Política                    | Decisión                                                                            |
| --------------------------- | ----------------------------------------------------------------------------------- |
| **Fecha diferente**         | Si llega otro día: marcar pre-registro como NO_SHOW y crear nuevo ingreso (walk-in) |
| **Múltiples pre-registros** | ✅ Puede tener varios pendientes simultáneos                                        |
| **Expiración**              | Automática después de **72 horas** → Estado: NO_SHOW                                |
| **No-Show**                 | Automático: al final del día si no llegó                                            |
| **Vehículo**                | Igual que contratista: si no tiene registrado se crea, si tiene lista se selecciona |

---

## UI Sugerida

### Módulo Pre-Registro

- Vista principal: Lista de pre-registros pendientes
- Acciones: Crear nuevo, Dar ingreso, Cancelar, No-Show
- Filtros: Por fecha esperada, por estado

### Flujo de Ingreso (decisiones)

```
🔍 Buscar por cédula
        ↓
   ¿Tiene pre-reg?
    SI     NO
    ↓       ↓
  Vista   ¿En catálogo?
  Rápida   SI    NO
           ↓      ↓
         Vista  Vista
        Parcial  Full
```
