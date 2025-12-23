# Reglas de Negocio y Casos de Uso - brisas-app

> 📋 **Documento generado desde código.** Pendiente validación del usuario.

---

## 1. Módulo: Ingreso Contratista

### Caso de Uso: Registrar Entrada

**Flujo:**
1. Usuario escanea/ingresa cédula
2. Sistema busca contratista
3. Se evalúa elegibilidad
4. Se asigna gafete (opcional)
5. Se registra ingreso con `fecha_ingreso = now()`

**Reglas Bloqueantes (no puede entrar si):**

| # | Regla | Código |
|---|-------|--------|
| 1 | Está en lista negra | `esta_bloqueado == true` |
| 2 | Ya tiene ingreso activo | `tiene_ingreso_abierto == true` |
| 3 | Estado ≠ "activo" | `estado_contratista != "activo"` |
| 4 | PRAIND vencido | `fecha_vencimiento_praind < hoy` |

**Reglas No Bloqueantes (warnings):**

| # | Regla | Acción |
|---|-------|--------|
| 1 | Tiene alertas de gafete | Mostrar advertencia, permitir entrada |

**Constantes:**
```rust
TIEMPO_MAXIMO_HORAS = 14
TIEMPO_ALERTA_TEMPRANA_MINUTOS = 810 // 13h 30min
```

### Caso de Uso: Registrar Salida

**Reglas:**
- `fecha_salida > fecha_ingreso` (no puede ser anterior)
- Si tenía gafete y no lo devolvió → generar reporte
- Si devolvió gafete incorrecto → generar reporte

**Estados de Permanencia:**
| Estado | Condición |
|--------|-----------|
| Normal | tiempo < 13h 30min |
| AlertaTemprana | 13h 30min ≤ tiempo < 14h |
| TiempoExcedido | tiempo ≥ 14h |

### 🔍 Casos Borde (PENDIENTE VALIDAR)

- [ ] ¿Qué pasa si PRAIND vence HOY a las 23:59?
- [ ] ¿Puede entrar un contratista "suspendido" temporalmente?
- [ ] ¿Se puede forzar entrada con autorización especial?
- [ ] ¿Qué pasa si el contratista pierde el gafete adentro?

---

## 2. Módulo: Lista Negra

### Caso de Uso: Agregar a Lista Negra

**Dos flujos:**

**A) Con contratista_id existente:**
- Solo requiere: `motivo_bloqueo`, `bloqueado_por`
- `fecha_fin_bloqueo` opcional (temporal/permanente)

**B) Sin contratista_id (persona externa):**
- Requiere: `cedula`, `nombre`, `apellido`, `motivo_bloqueo`, `bloqueado_por`

**Validaciones:**
- Cédula: 7-20 chars, solo números y guiones
- Nombre: max 100 chars
- Motivo: max 500 chars

### 🔍 Casos Borde

- [ ] ¿Se puede bloquear al mismo contratista dos veces?
- [ ] ¿Qué pasa cuando vence `fecha_fin_bloqueo`? ¿Auto-desbloqueo?
- [ ] ¿Se puede bloquear a un usuario del sistema?

---

## 3. Módulo: Usuarios

### Caso de Uso: Crear Usuario

**Campos obligatorios:**
- Email (único, con @)
- Nombre, Apellido
- Cédula (7-20 chars, números y guiones)
- Role

**Campos opcionales:**
- Segundo nombre, Segundo apellido
- Teléfono (max 20), Dirección (max 200)
- Número de gafete

**Validaciones Password:**
- Mínimo 6 caracteres

### Caso de Uso: Login

**Flujo inferido del código:**
1. Buscar usuario por email
2. Verificar password con Argon2
3. Crear sesión

### 🔍 Casos Borde

- [ ] ¿Intentos máximos de login fallido?
- [ ] ¿Bloqueo temporal por intentos fallidos?
- [ ] ¿Expiración de sesión?
- [ ] ¿Password temporal al crear usuario?

---

## 4. Módulo: Gafetes

### Caso de Uso: Crear Gafete

**Tipos disponibles:** `contratista`, `visita`, `proveedor`

**Validaciones:**
- Número: max 20 chars, no vacío
- `"S/G"` es reservado (Sin Gafete)

### Caso de Uso: Asignar Gafete

**Reglas inferidas:**
- Un gafete solo puede estar asignado a un ingreso activo
- Al registrar salida, se marca como devuelto

### 🔍 Casos Borde

- [ ] ¿Qué pasa si se pierde un gafete?
- [ ] ¿Hay límite de reasignaciones por gafete?
- [ ] ¿Se puede desactivar un gafete?

---

## 5. Módulo: Ingreso Visita

Similar a Ingreso Contratista pero más simple:

**Diferencias:**
- No hay validación de PRAIND
- No hay validación de lista negra (¿debería haber?)
- Requiere: `anfitrion`, `area_visitada`, `motivo`

### 🔍 Casos Borde

- [ ] ¿Visitante puede estar en lista negra?
- [ ] ¿Límite de tiempo para visitas?
- [ ] ¿Cita previa obligatoria o opcional?

---

## 6. Módulo: Contratista

### Caso de Uso: Crear Contratista

**Campos obligatorios:**
- Cédula, Nombre, Apellido
- Empresa ID
- Fecha vencimiento PRAIND

**Validaciones:**
- Cédula: 7-20 chars
- Nombres: max 50 chars
- Fecha PRAIND: formato YYYY-MM-DD

### Estados

| Estado | Descripción |
|--------|-------------|
| activo | Puede ingresar |
| inactivo | No puede ingresar |
| (otros?) | ¿Suspendido, pendiente? |

---

## Preguntas Globales

1. **Auditoría:** ¿Se registra quién hizo cada cambio?
2. **Horarios:** ¿Hay restricciones de horario de entrada?
3. **Roles:** ¿Qué puede hacer cada rol?
4. **Reportes:** ¿Se envían automáticamente o solo se generan?
5. **Multi-terminal:** ¿Importa desde qué terminal se registra?

---

*Documento para debate y validación. Por favor, marca como ✅ o ❌ cada punto.*
