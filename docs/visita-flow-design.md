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

| Política | Decisión |
|----------|----------|
| **Fecha diferente** | Si llega otro día: marcar pre-registro como NO_SHOW y crear nuevo ingreso (walk-in) |
| **Múltiples pre-registros** | ✅ Puede tener varios pendientes simultáneos |
| **Expiración** | Automática después de **72 horas** → Estado: NO_SHOW |
| **No-Show** | Automático: al final del día si no llegó |
| **Vehículo** | Igual que contratista: si no tiene registrado se crea, si tiene lista se selecciona |

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
