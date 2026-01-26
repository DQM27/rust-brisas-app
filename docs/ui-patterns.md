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

| Estado                    | Borde               | Texto            | Fondo                      |
| ------------------------- | ------------------- | ---------------- | -------------------------- |
| **Normal**                | `border-surface`    | `text-secondary` | Transparente               |
| **Hover Positivo**        | `border-success/50` | `text-success`   | Transparente               |
| **Hover Negativo**        | `border-error/50`   | `text-error`     | Transparente               |
| **Seleccionado Positivo** | `border-success`    | `text-success`   | `bg-success bg-opacity-10` |
| **Seleccionado Negativo** | `border-error`      | `text-error`     | `bg-error bg-opacity-10`   |

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

---

## 🌟 Estándar de Modal CRUD (UserFormModal)

Referencia: `UserFormModal.svelte`
Este es el **Molde Maestro** para todos los formularios complejos de la aplicación.

### 1. Estructura General y Dimensiones

El modal debe ocupar el máximo espacio disponible pero mantener márgenes y bordes redondeados.

| Propiedad      | Valor                 | HTML                                                       |
| -------------- | --------------------- | ---------------------------------------------------------- |
| **Contenedor** | Surface-2 + Sombra XL | `bg-surface-2 shadow-2xl border border-surface rounded-xl` |
| **Ancho**      | Dinámico (Max 700px)  | `w-full max-w-[700px]`                                     |
| **Alto**       | Max 95% viewport      | `max-h-[95vh] overflow-hidden`                             |
| **Layout**     | Flex Column           | `flex flex-col`                                            |
| **Backdrop**   | Blur + Oscuro         | `bg-black/60 backdrop-blur-sm`                             |

### 2. Header Estándar

Cabecera limpia con título a la izquierda y botón de cierre a la derecha.

```svelte
<div
	class="flex-none flex items-center justify-between px-3 py-3 bg-surface-2 border-b border-surface"
>
	<h2 class="text-xl font-semibold text-primary">
		{modalTitle}
	</h2>
	<button
		onclick={onClose}
		class="p-1.5 rounded-lg text-secondary hover:text-primary hover:bg-surface-3 transition-colors"
	>
		<X size={20} />
	</button>
</div>
```

### 3. Grid de Formulario

El contenido principal va dentro de un contenedor scrollable, pero los inputs se agrupan en un "card" visual distintivo.

- **Contenedor Principal**: `flex-1 p-6 space-y-4 overflow-y-auto`
- **Card de Inputs**: `bg-surface-1 rounded-lg border border-surface p-7`
- **Grid System**: `grid grid-cols-1 lg:grid-cols-2 gap-6`

### 4. Inputs y Selects (Definiciones CSS)

Usar estas constantes JS para mantener consistencia absoluta.

```typescript
// Input de texto estándar (34px altura)
const inputClass =
	'w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[34px] text-sm text-white placeholder:text-gray-500 focus:outline-none focus:!border-blue-500/50 focus:!ring-1 focus:!ring-blue-500/20 disabled:opacity-50 transition-all';

// Botón trigger para Selects Custom
const selectClass =
	'w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[34px] text-sm text-white focus:outline-none disabled:opacity-50 transition-all cursor-pointer appearance-none bg-no-repeat bg-right pr-8';

// Labels
const labelClass = 'block text-xs font-medium text-secondary mb-1';

// Mensajes de Error
const errorClass = 'text-xs text-red-500 mt-0.5';
```

### 5. Comportamiento y Estados de Validación

Los campos deben proporcionar feedback visual inmediato sobre su estado.

| Estado               | Indicador Visual                    | Clases CSS                                        |
| -------------------- | ----------------------------------- | ------------------------------------------------- |
| **Normal / Vacío**   | Borde sutil, fondo semitransparente | `border-white/10`                                 |
| **Foco (Focus)**     | Borde azul, sombra azul suave       | `focus:border-blue-500/50 focus:ring-blue-500/20` |
| **Error / Inválido** | Borde rojo, anillo rojo suave       | `!border-red-500/50 !ring-red-500/20`             |
| **Requerido**        | Asterisco rojo junto al label       | `<span class="text-red-500">*</span>`             |

**Helper de Validación (`getFieldStateClass`)**
Se utiliza una función helper para aplicar condicionalmente las clases de error si el validor (Superforms/Zod) reporta un fallo.

```typescript
function getFieldStateClass(field, value) {
	// Si hay error en el objeto $errors o un error custom (ej. duplicados)
	if ($errors[field] || customErrors[field]) {
		return '!border-red-500/50 !ring-1 !ring-red-500/20';
	}
	return ''; // Retorna string vacío si es válido
}
```

### 6. Input de Fecha (Texto con Autoformato)

Para mantener uniformidad visual y evitar los controles nativos del navegador (calendar pickers) que rompen el estilo, usamos `input[type="text"]` con formato `DD/MM/YYYY`.

**Características:**

- Auto-agrega barras `/` mientras el usuario escribe.
- Validación de solo números.
- Conversión transparente a formato backend (`YYYY-MM-DD`).

```typescript
// Helpers JS
function formatDateForDisplay(isoDate: string): string {
	if (!isoDate) return '';
	const [year, month, day] = isoDate.split('T')[0].split('-');
	return `${day}/${month}/${year}`;
}

function formatDateForBackend(displayDate: string): string {
	if (!displayDate || displayDate.length !== 10) return '';
	const [day, month, year] = displayDate.split('/');
	return `${year}-${month}-${day}`;
}
```

```svelte
<!-- Implementación en Template -->
<input
  type="text"
  placeholder="DD/MM/YYYY"
  maxlength="10"
  class={inputClass}
  oninput={(e) => {
    const input = e.target as HTMLInputElement;
    let value = input.value.replace(/[^\d/]/g, ""); // Solo números
    if (value.length >= 3 && value[2] !== "/") { // Auto / al día
        value = value.slice(0, 2) + "/" + value.slice(2);
    }
    if (value.length >= 6 && value[5] !== "/") { // Auto / al mes
        value = value.slice(0, 5) + "/" + value.slice(5);
    }
    value = value.slice(0, 10); // Max length
    $form.fecha = value;
    input.value = value;
  }}
/>
```

````

### 7. Textareas "Container Pattern"

Para textareas (dirección, observaciones), usamos un contenedor div que gestiona el borde para permitir que el textarea interno no tenga bordes y se vea premium.

```svelte
<div>
  <label class={labelClass}>Dirección</label>
  <div class="obs-container w-full bg-black/20 border border-white/10 rounded-lg transition-all outline-none focus-within:!border-blue-500/50 focus-within:!ring-1 focus-within:!ring-blue-500/20">
    <textarea
      class="w-full bg-transparent px-3 py-2 text-sm text-white placeholder:text-gray-500 resize-none focus:outline-none outline-none border-none appearance-none ring-0 h-[93px]"
      rows="4"
    ></textarea>
  </div>
</div>
````

````

### 8. Controles de Seguridad Integrados

Los controles especiales (como "Forzar cambio de clave") se integran directamente en el grid, ocupando ambas columnas si es necesario, sin secciones colapsables.

```svelte
<!-- Columna expandida (col-span-2) o alineada al final -->
<div class="flex items-center gap-3">
  <!-- Toggle Switch Estilo iOS -->
  <label class="relative inline-flex items-center cursor-pointer">
    <input type="checkbox" class="sr-only peer" />
    <div class="w-9 h-5 bg-surface-3 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:bg-blue-600 after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:rounded-full after:h-4 after:w-4 after:transition-all"></div>
  </label>
  <span class="text-xs font-medium text-gray-300">Forzar cambio de clave</span>
</div>
````

````

### 9. Footer de Acciones

El footer está siempre visible en la parte inferior (`flex-none`).

- **Fondo**: `bg-surface-1` (ligeramente más claro que el header para contraste visual sutil).
- **Borde Superior**: `border-t border-surface`.
- **Botones "Reactivos"**: Todos usan `border-2`. El color solo aparece en Hover.

| Botón | Estilo Base | Hover |
|-------|-------------|-------|
| **Cancelar** | `border-surface text-secondary` | `hover:border-white/60 hover:text-white/80` (Gris) |
| **Acción Secundaria** | `border-surface text-secondary` | `hover:border-accent hover:text-accent` (Azul/Cyan) |
| **Guardar (Primario)** | `border-surface text-secondary` | `hover:border-success hover:text-success` (Verde) |

```svelte
<div class="flex-none flex items-center justify-end gap-3 px-6 py-4 border-t border-surface bg-surface-1">
  <!-- Cancelar -->
  <button class="px-4 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-white/60 hover:text-white/80 text-sm">
    Cancelar
  </button>

  <!-- Guardar -->
  <button class="px-6 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-success hover:text-success text-sm">
    Guardar Cambios
  </button>
</div>
````

````

### 10. CSS Globales Requeridos

Agregar estos hacks CSS en el bloque `<style>` del componente para garantizar la estética.

```css
/* Autofill Fix (Evita fondo blanco de Chrome) */
input:-webkit-autofill,
textarea:-webkit-autofill {
  -webkit-text-fill-color: white !important;
  -webkit-box-shadow: 0 0 0px 1000px #1c2128 inset !important;
  transition: background-color 5000s ease-in-out 0s;
}

/* YA NO SE USA: Date Picker style (usamos text input) */
/*
input[type="date"] {
  color-scheme: dark;
}
*/

/* Focus Override Global */
input:focus, textarea:focus {
  border-color: rgba(59, 130, 246, 0.5) !important;
  box-shadow: 0 0 0 1px rgba(59, 130, 246, 0.2) !important;
  outline: none !important;
}
````

---

## 🛡️ Modales de Conformidad y Seguridad

Referencia: `AdminConfirmModal.svelte`, `ConfirmPasswordModal.svelte`

### Tema "High Security"

Usado para acciones destructivas o verificaciones de administrador.

- **Fondo**: `bg-[#0d1117]` (Casi negro, "GitHub Dark")
- **Borde**: `border border-white/10`
- **Sombra**: `shadow-2xl` con efecto Glow
- **Iconografía**: Escudos (`Shield`, `ShieldAlert`) con efectos de anillo y sombra.

### Alertas dentro del Modal

```svelte
<div class="bg-yellow-900/20 border border-yellow-700/30 rounded-lg p-4 flex gap-3">
	<TriangleAlert class="text-yellow-500" />
	<p class="text-yellow-200/80">Mensaje de advertencia...</p>
</div>
```

---

---

## 🔽 Custom Dropdown / Select (Standard)

Referencia: `UserFormModal.svelte`

### Descripción

Reemplazo visual para el `<select>` nativo que ofrece una experiencia consistente en estilo y comportamiento.

### Estructura

1.  **Trigger Button**: Botón que muestra el valor actual y un icono `ChevronDown`.
2.  **Backdrop**: `div` transparente fijo para cerrar el menú al hacer clic fuera.
3.  **Menú Flotante**: Lista absoluta con los items seleccionables.

### Implementación

```svelte
<div class="relative">
	<label class={labelClass}>Etiqueta</label>

	<!-- 1. Trigger -->
	<button
		type="button"
		onclick={() => (showDropdown = !showDropdown)}
		class="{inputClass} flex items-center justify-between cursor-pointer w-full text-left"
		class:!border-blue-500={showDropdown}
	>
		<span class="truncate">{currentValue ?? 'Seleccionar...'}</span>
		<ChevronDown size={16} class="text-secondary" />
	</button>

	{#if showDropdown}
		<!-- 2. Backdrop -->
		<div
			class="fixed inset-0 z-40"
			onclick={() => (showDropdown = false)}
			role="presentation"
		></div>

		<!-- 3. Menú -->
		<div
			class="absolute z-50 w-full mt-1 bg-[#1c2128] border border-white/10 rounded-lg shadow-xl overflow-hidden p-1 origin-top"
			transition:fly={{ y: -5, duration: 200 }}
		>
			{#each options as option}
				<button
					type="button"
					onclick={() => {
						currentValue = option.value;
						showDropdown = false;
					}}
					class="w-full text-left px-3 py-1.5 text-sm text-gray-300 hover:bg-white/10 rounded-md transition-colors flex items-center justify-between group"
				>
					<span>{option.label}</span>
					{#if currentValue === option.value}
						<Check size={14} class="text-white" />
					{/if}
				</button>
			{/each}
		</div>
	{/if}
</div>
```

### Características Visuales

- **Fondo Menú**: `bg-[#1c2128]` (Gris oscuro GitHub)
- **Borde**: `border border-white/10`
- **Hover Item**: `hover:bg-white/10`
- **Animación**: `transition:fly={{ y: -5, duration: 200 }}` para un efecto de despliegue suave.

---
