# Patrones de UI - Rust Brisas App

Este documento contiene los patrones de diseño estándar utilizados en la aplicación.

---

## 🎨 Botones de Selección Binaria (Sí/No)

Referencia: `SalidaModal.svelte` (líneas 188-209)

### Descripción
Botones de selección mutuamente excluyentes que cambian de color al hacer hover y al ser seleccionados. Ideales para preguntas de tipo Sí/No con significado semántico (éxito/error).

### Características
- **Transición suave**: `transition-all`
- **Borde grueso**: `border-2`
- **Bordes redondeados**: `rounded-lg`
- **Padding consistente**: `p-3`
- **Iconos integrados**: Lucide icons (`CheckCircle`, `XCircle`)

### Estados

| Estado | Borde | Texto | Fondo |
|--------|-------|-------|-------|
| **Normal** | `border-surface` | `text-secondary` | Transparente |
| **Hover Positivo** | `border-success/50` | `text-success` | Transparente |
| **Hover Negativo** | `border-error/50` | `text-error` | Transparente |
| **Seleccionado Positivo** | `border-success` | `text-success` | `bg-success bg-opacity-10` |
| **Seleccionado Negativo** | `border-error` | `text-error` | `bg-error bg-opacity-10` |

### Código de Referencia

```svelte
<!-- Botón Positivo (Sí, lo devolvió) -->
<button
  type="button"
  onclick={() => (devolvioGafete = true)}
  class="flex items-center justify-center gap-2 p-3 rounded-lg border-2 transition-all 
    {devolvioGafete === true
      ? 'border-success bg-success bg-opacity-10 text-success'
      : 'border-surface hover:border-success/50 text-secondary hover:text-success'}"
>
  <CheckCircle size={20} />
  <span class="font-medium">Sí, lo devolvió</span>
</button>

<!-- Botón Negativo (No lo devolvió) -->
<button
  type="button"
  onclick={() => (devolvioGafete = false)}
  class="flex items-center justify-center gap-2 p-3 rounded-lg border-2 transition-all 
    {devolvioGafete === false
      ? 'border-error bg-error bg-opacity-10 text-error'
      : 'border-surface hover:border-error/50 text-secondary hover:text-error'}"
>
  <XCircle size={20} />
  <span class="font-medium">No lo devolvió</span>
</button>
```

### Estructura HTML

```svelte
<div class="grid grid-cols-2 gap-3">
  <!-- Botón Positivo -->
  <!-- Botón Negativo -->
</div>
```

---

---

## 🔘 Botones de Acción (Footer de Modales)

Referencia: `SalidaModal.svelte` (líneas 265-285)

### Descripción
Botones de acción para confirmar o cancelar operaciones en modales. Siguen convenciones UX estándar donde la acción primaria es prominente y la secundaria es sutil.

### Convención de Colores UX

| Tipo de Acción | Color Hover | Razón |
|----------------|-------------|-------|
| **Primaria (Confirmar)** | Verde (`success`) | Acción positiva, completa la tarea |
| **Secundaria (Cancelar)** | Gris sutil (`white/60`) | Menos prominente, no distrae |
| **Destructiva (Eliminar)** | Rojo (`error`) | Indica peligro o eliminación |

### Características
- **Sin efecto de escala**: Evitar `hover:scale` y `active:scale` para prevenir glitches visuales
- **Transición suave**: `transition-all duration-200`
- **Borde grueso**: `border-2`
- **Bordes redondeados**: `rounded-lg`

### Estados

| Botón | Estado Normal | Hover |
|-------|---------------|-------|
| **Cancelar** | `border-surface` + `text-secondary` | `border-white/60` + `text-white/80` |
| **Confirmar** | `border-surface` + `text-secondary` | `border-success` + `text-success` |

### Código de Referencia

```svelte
<!-- Botón Cancelar (Secundario) -->
<button
  onclick={handleClose}
  disabled={loading}
  class="flex items-center justify-center gap-2 px-4 py-2.5 rounded-lg border-2 
    transition-all duration-200 border-surface text-secondary 
    hover:border-white/60 hover:text-white/80 
    focus:outline-none disabled:opacity-50"
>
  Cancelar
</button>

<!-- Botón Confirmar (Primario) -->
<button
  onclick={handleConfirm}
  disabled={loading}
  class="flex items-center justify-center gap-2 px-4 py-2.5 rounded-lg border-2 
    transition-all duration-200 border-surface text-secondary 
    hover:border-success hover:text-success 
    focus:outline-none disabled:opacity-50"
>
  <LogOut size={18} />
  Confirmar Salida
</button>
```

---

## 📝 Inputs y Textareas Minimalistas

Referencia: `GafeteInput.svelte`, `IngresoFormModal.svelte`, `SalidaModal.svelte`

### Descripción
Campos de entrada con estilo oscuro y minimalista. Usan un **DIV contenedor** que maneja el borde/focus, y el elemento interno (input/textarea) no tiene ningún estilo de borde.

### Estructura Clave
```
DIV contenedor (maneja borde + focus)
  └── INPUT/TEXTAREA interno (transparente, sin borde)
```

### Clases del Contenedor
```
.container {
  bg-black/20
  border border-white/10
  rounded-lg
  focus-within:border-blue-500/50
  focus-within:ring-1 focus-within:ring-blue-500/20
  transition-all
  outline-none
}
```

### Clases del Elemento Interno
```
input/textarea {
  w-full
  bg-transparent
  px-3 py-2
  text-sm text-white
  placeholder:text-gray-500
  resize-none (solo textarea)
  focus:outline-none outline-none
  border-none
  appearance-none
  ring-0
}
```

### CSS Requerido (¡IMPORTANTE!)
```css
/* Quitar outline del navegador */
.container,
.container *:focus {
  outline: none !important;
  box-shadow: none !important;
}

/* Re-aplicar ring redondeado via focus-within */
.container:focus-within {
  border-color: rgba(59, 130, 246, 0.5) !important;
  box-shadow: 0 0 0 1px rgba(59, 130, 246, 0.2) !important;
}
```

### Código de Referencia Completo

```svelte
<!-- Contenedor -->
<div class="obs-container w-full bg-black/20 border border-white/10 rounded-lg focus-within:border-blue-500/50 focus-within:ring-1 focus-within:ring-blue-500/20 transition-all outline-none">
  <!-- Input/Textarea interno -->
  <textarea
    class="w-full bg-transparent px-3 py-2 text-sm text-white placeholder:text-gray-500 resize-none focus:outline-none outline-none border-none appearance-none ring-0"
    rows="2"
    placeholder="Notas adicionales..."
  ></textarea>
</div>

<style>
  .obs-container,
  .obs-container *:focus {
    outline: none !important;
    box-shadow: none !important;
  }

  .obs-container:focus-within {
    border-color: rgba(59, 130, 246, 0.5) !important;
    box-shadow: 0 0 0 1px rgba(59, 130, 246, 0.2) !important;
  }
</style>
```

---

## 🪟 Estructura del Modal Estándar

Referencia: `IngresoFormModal.svelte`, `SalidaModal.svelte`

### Descripción
Estructura base para todos los modales de la aplicación. Diseño oscuro, minimalista y con bordes sutiles.

### Dimensiones y Colores

| Propiedad | Valor |
|-----------|-------|
| **Ancho máximo** | `max-w-md` (448px) |
| **Altura máxima** | `max-h-[90vh]` |
| **Fondo** | `bg-surface-2` (oscuro) |
| **Borde** | `border border-surface` |
| **Esquinas** | `rounded-lg` |
| **Sombra** | `shadow-surface-xl` |

### Overlay (Fondo)

```svelte
<div class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4">
```

### Estructura del Modal

```svelte
<div class="bg-surface-2 rounded-lg shadow-surface-xl border border-surface max-w-md w-full">
  <!-- Header -->
  <div class="flex items-center justify-between px-6 py-4 border-b border-surface">
    <div>
      <h2 class="text-xl font-semibold text-primary">Título</h2>
      <p class="text-sm text-secondary mt-1">Subtítulo descriptivo</p>
    </div>
    <button class="p-2 hover:bg-surface-hover rounded-md transition-colors">
      <X size={20} class="text-secondary" />
    </button>
  </div>

  <!-- Content -->
  <div class="p-6 space-y-6">
    <!-- Contenido del formulario -->
  </div>

  <!-- Footer -->
  <div class="flex items-center justify-end gap-3 px-6 py-4 border-t border-surface bg-surface-1">
    <!-- Botones de acción -->
  </div>
</div>
```

### Animaciones del Modal

| Elemento | Transición |
|----------|------------|
| **Overlay** | `transition:fade={{ duration: 150 }}` |
| **Modal** | `transition:scale={{ duration: 200, start: 0.95 }}` |

### Secciones

1. **Header** (`px-6 py-4 border-b border-surface`)
   - Título: `text-xl font-semibold text-primary`
   - Subtítulo: `text-sm text-secondary`
   - Botón cerrar: `p-2 hover:bg-surface-hover rounded-md`

2. **Content** (`p-6 space-y-6`)
   - Cards informativos: `p-4 bg-surface-1 rounded-lg border border-surface`
   - Labels: `text-[12px] font-bold uppercase tracking-wider text-gray-500`

3. **Footer** (`px-6 py-4 border-t border-surface bg-surface-1`)
   - Gap entre botones: `gap-3`
   - Alineación: `justify-end`

---

## 📝 Notas Adicionales

*Este documento se irá actualizando con más patrones de UI a medida que se estandaricen.*
