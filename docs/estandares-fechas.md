# Estándares de Fechas y Tiempo

## Principio General

| Capa | Formato | Responsabilidad |
|------|---------|-----------------|
| **Backend/BD** | ISO 8601 / RFC 3339 | Almacenar, validar, transmitir |
| **Frontend** | Cualquier formato local | Mostrar al usuario |

---

## Formatos Estándar

### RFC 3339 - Para timestamps con hora
- **Formato**: `YYYY-MM-DDThh:mm:ssZ`
- **Ejemplo**: `2026-01-15T08:30:00Z`
- **Uso**: `created_at`, `updated_at`, `fecha_hora_ingreso`, `fecha_hora_salida`
- **Función**: `validar_fecha_rfc3339()`

### YYYY-MM-DD - Para fechas sin hora
- **Formato**: `YYYY-MM-DD`
- **Ejemplo**: `2026-12-31`
- **Uso**: `fecha_vencimiento_praind`, cumpleaños, fechas de documentos
- **Funciones**: `validar_fecha_simple()`, `parsear_fecha_simple()`

---

## Frontend

El frontend **puede** mostrar fechas en cualquier formato local (ej: `DD/MM/YYYY`).

**Pero debe**:
1. **Enviar** al backend en formato estándar
2. **Recibir** del backend en formato estándar
3. **Convertir** para mostrar al usuario

```javascript
// Ejemplo en JavaScript
const fechaBackend = "2026-01-15T08:30:00Z";
const fechaLocal = new Intl.DateTimeFormat('es-CR', {
  dateStyle: 'short',
  timeStyle: 'short'
}).format(new Date(fechaBackend));
// Resultado: "15/1/2026, 2:30"
```

---

## Funciones Centralizadas (common.rs)

| Función | Propósito |
|---------|-----------|
| `validar_fecha_rfc3339(str)` | Valida formato RFC 3339 (datetime) |
| `validar_fecha_simple(str)` | Valida formato YYYY-MM-DD (date-only) |
| `parsear_fecha_simple(str)` | Parsea y retorna `NaiveDate` |
| `validar_tiempo_salida(ing, sal)` | Valida salida > ingreso |
| `calcular_tiempo_permanencia(ing, sal)` | Calcula minutos de estancia |
