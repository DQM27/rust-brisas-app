# Guía de Diseño UI - GitHub Dark Theme

Este documento define el sistema de diseño para formularios modales en la aplicación Brisas.

## Paleta de Colores

### Colores Base (Dark Mode)
| Uso | Código Hex | Variable |
|-----|-----------|----------|
| Fondo Modal | `#0d1117` | `dark:bg-[#0d1117]` |
| Fondo Header/Footer | `#161b22` | `dark:bg-[#161b22]` |
| Fondo Hover | `#21262d` | `dark:hover:bg-[#21262d]` |
| Bordes | `#30363d` | `dark:border-[#30363d]` |

### Colores de Acento
| Uso | Código Hex | Hover |
|-----|-----------|-------|
| Botón Primario (Azul) | `#2563eb` | `#1d4ed8` |
| Error | `red-500` | - |

### Colores de Texto
| Uso | Clase Tailwind |
|-----|----------------|
| Texto Principal | `dark:text-gray-100` |
| Labels | `dark:text-gray-400` |
| Placeholder | `dark:placeholder-gray-500` |
| Texto Secundario | `dark:text-gray-400` |

---

## Clases CSS Reutilizables

### Variables de Estilo (definir al inicio del script)

```typescript
// GitHub-style styles with blue accent
const labelClass = "block text-sm font-medium text-gray-500 dark:text-gray-400 mb-1.5";

const inputClass =
  "w-full rounded-md border border-gray-300 dark:border-[#30363d] bg-white dark:bg-[#0d1117] px-3 py-2.5 text-sm text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2563eb] focus:border-[#2563eb] focus:outline-none transition-all placeholder-gray-400 dark:placeholder-gray-500 disabled:opacity-50 disabled:bg-gray-100 dark:disabled:bg-[#161b22]";

const errorInputClass = "border-red-500 dark:border-red-500 focus:ring-red-500 focus:border-red-500";
```

---

## Estructura del Modal

### Contenedor Principal
```html
<div class="relative z-10 w-full max-w-md rounded-lg bg-white dark:bg-[#0d1117] shadow-2xl border border-gray-200 dark:border-gray-700">
```

### Header
```html
<div class="flex items-center justify-between px-6 py-4 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-[#161b22]">
  <div>
    <h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100">Título</h2>
    <p class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">Descripción</p>
  </div>
  <button class="p-1.5 rounded-md text-gray-400 hover:text-gray-500 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors">
    <X class="w-5 h-5" />
  </button>
</div>
```

### Contenido (Form)
```html
<div class="p-6 space-y-4">
  <!-- Campos aquí -->
</div>
```

### Footer
```html
<div class="sticky bottom-0 z-20 flex justify-end gap-3 px-6 py-4 border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-[#161b22]">
  <!-- Botones -->
</div>
```

---

## Componentes

### Label
```html
<label for="campo" class={labelClass}>
  Nombre del Campo <span class="text-red-500">*</span>
</label>
```

### Input de Texto
```html
<input
  id="campo"
  name="campo"
  type="text"
  bind:value={$form.campo}
  placeholder="Placeholder"
  disabled={loading || readonly}
  class="{inputClass} {$errors.campo ? errorInputClass : ''}"
  {...$constraints.campo}
/>
```

### Select/Dropdown
```html
<select
  id="campo"
  name="campo"
  bind:value={$form.campo}
  disabled={loading}
  class="{inputClass} {$errors.campo ? errorInputClass : ''}"
>
  <option value="" disabled>Seleccione opción</option>
  {#each opciones as opt}
    <option value={opt.id}>{opt.nombre}</option>
  {/each}
</select>
```

### Mensaje de Error
```html
{#if $errors.campo}
  <p class="text-xs text-red-500 mt-1">{$errors.campo}</p>
{/if}
```

---

## Botones

### Botón Primario (Azul)
```html
<button
  type="submit"
  disabled={loading}
  class="px-4 py-2.5 text-sm font-medium rounded-md bg-[#2563eb] text-white hover:bg-[#1d4ed8] disabled:opacity-50 disabled:cursor-not-allowed transition-colors flex items-center gap-2"
>
  {isEditMode ? "Guardar Cambios" : "Crear"}
</button>
```

### Botón Secundario (Cancelar)
```html
<button
  type="button"
  onclick={onClose}
  disabled={loading}
  class="px-4 py-2 text-sm font-medium rounded-md border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-[#21262d] transition-colors"
>
  Cancelar
</button>
```

### Botón Pequeño (Outline)
```html
<button
  type="button"
  onclick={handleAction}
  class="flex items-center gap-2 px-3 py-1.5 text-xs font-medium rounded-md border border-gray-300 dark:border-[#30363d] text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-[#21262d] transition-colors"
>
  <Icon class="w-3.5 h-3.5" />
  Texto
</button>
```

---

## Layouts

### Grid de 2 Columnas
```html
<div class="grid grid-cols-2 gap-4">
  <div class="space-y-1"><!-- Campo 1 --></div>
  <div class="space-y-1"><!-- Campo 2 --></div>
</div>
```

### Fila con Botón Inline
```html
<div class="flex gap-2">
  <div class="relative flex-1">
    <select>...</select>
  </div>
  <button class="px-3 py-2 ...">
    <Plus class="w-4 h-4" />
  </button>
</div>
```

---

## Formato de Fecha

### Configuración
- **Formato Visual**: `DD/MM/YYYY`
- **Separador**: `/` (barra)
- **Placeholder**: `DD/MM/YYYY`

### Máscara de Entrada (oninput)
```typescript
oninput={(e) => {
  const input = e.target as HTMLInputElement;
  let value = input.value.replace(/[^\d/]/g, "");
  if (value.length >= 3 && value[2] !== "/") {
    value = value.slice(0, 2) + "/" + value.slice(2);
  }
  if (value.length >= 6 && value[5] !== "/") {
    value = value.slice(0, 5) + "/" + value.slice(5);
  }
  value = value.slice(0, 10);
  $form.fecha = value;
  input.value = value;
}}
```

### Validación Zod
```typescript
fechaVencimiento: z
  .string()
  .min(10, "Fecha inválida (DD/MM/YYYY)")
  .refine((val) => {
    const regex = /^\d{2}\/\d{2}\/\d{4}$/;
    if (!regex.test(val)) return false;
    const [day, month, year] = val.split('/').map(Number);
    const date = new Date(year, month - 1, day);
    return date.getDate() === day && date.getMonth() === month - 1 && date.getFullYear() === year;
  }, "Fecha inválida"),
```

---

## Archivos de Referencia

- `src/lib/components/contratista/ContratistaFormModal.svelte` - Implementación completa
- `src/lib/components/vehiculo/VehiculoManagerModal.svelte` - Modal con lista + formulario
- `src/lib/schemas/contratistaSchema.ts` - Esquema Zod con validación de fecha
