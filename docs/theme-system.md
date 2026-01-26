# Sistema de Temas - Guía de Colores y Arquitectura

> **Última actualización:** Enero 2026  
> **Estado:** En implementación

---

## Resumen

La aplicación soporta tres modos de visualización:

| Modo            | Descripción                              | Uso Principal                    |
| --------------- | ---------------------------------------- | -------------------------------- |
| **Light**       | Tema claro corporativo, azul como acento | Oficinas con luz natural         |
| **Dark**        | Tema oscuro VS Code style                | Uso general, menor fatiga visual |
| **Tokyo Night** | Tema oscuro premium (solo grids)         | Opcional, estética "hacker"      |

---

## Paleta de Colores

### 🌞 Tema Light (Azul Corporativo)

```
SUPERFICIES
├── Primary:     #ffffff (blanco puro)
├── Secondary:   #f5f7fa (gris azulado muy claro)
├── Tertiary:    #e8ecf0 (gris azulado claro)
├── Hover:       #dce3eb (hover sutil)
└── Active:      #cfd8e3 (pressed state)

TEXTO
├── Primary:     #1a1f2e (casi negro azulado)
├── Secondary:   #4a5568 (gris oscuro)
├── Tertiary:    #718096 (gris medio)
└── Disabled:    #a0aec0 (gris claro)

BORDES
├── Subtle:      #e2e8f0 (casi invisible)
├── Emphasis:    #cbd5e1 (visible)
└── Strong:      #94a3b8 (prominente)

ACENTO (Azul Corporativo)
├── Primary:     #0066cc (azul principal)
├── Hover:       #0052a3 (más oscuro)
├── Active:      #004080 (presionado)
├── Background:  rgba(0, 102, 204, 0.08)
└── Bg-Hover:    rgba(0, 102, 204, 0.12)

SEMÁNTICOS
├── Success:     #059669 (verde esmeralda)
├── Error:       #dc2626 (rojo)
├── Warning:     #d97706 (ámbar)
└── Info:        #2563eb (azul info)
```

### 🌙 Tema Dark (VS Code Style)

```
SUPERFICIES
├── Primary:     #1e1e1e (fondo principal)
├── Secondary:   #252526 (paneles)
├── Tertiary:    #2d2d30 (elevado)
├── Hover:       #37373d (hover)
└── Active:      #404045 (pressed)

TEXTO
├── Primary:     #e8e8e8 (texto principal)
├── Secondary:   #c8c8c8 (texto secundario)
├── Tertiary:    #9d9d9d (texto terciario)
└── Disabled:    #6e6e6e (deshabilitado)

BORDES
├── Subtle:      #303030 (sutil)
├── Emphasis:    #454545 (énfasis)
└── Strong:      #5a5a5a (fuerte)

ACENTO (Azul Brillante)
├── Primary:     #4da6ff (azul claro)
├── Hover:       #66b3ff (más claro)
├── Active:      #80bfff (activo)
├── Background:  rgba(77, 166, 255, 0.15)
└── Bg-Hover:    rgba(77, 166, 255, 0.25)

SEMÁNTICOS
├── Success:     #73c991 (verde menta)
├── Error:       #f57a7a (rojo coral)
├── Warning:     #e5b567 (dorado)
└── Info:        #75bfff (azul cielo)
```

### 🌃 Tokyo Night (Solo Grids - Opcional)

```
SUPERFICIES
├── Primary:     #1a1b26 (azul muy oscuro)
├── Secondary:   #1f2335 (headers)
├── Tertiary:    #24283b (elevado)
└── Hover:       #292e42 (hover)

TEXTO
├── Primary:     #c0caf5 (lavanda claro)
├── Secondary:   #a9b1d6 (lavanda medio)
└── Accent:      #7aa2f7 (azul neón)

BORDES
└── All:         rgba(122, 162, 247, 0.2) (azul translúcido)
```

---

## Arquitectura CSS

### Variables en `theme.css`

```css
/* Light mode (default en @theme) */
@theme {
	--color-surface-primary: #ffffff;
	--color-text-primary: #1a1f2e;
	/* ... */
}

/* Dark mode (clase .dark) */
.dark {
	--color-surface-primary: #1e1e1e;
	--color-text-primary: #e8e8e8;
	/* ... */
}
```

### Nuevas Variables para Grids

```css
/* Grid-specific (hereda del tema activo) */
:root {
	--grid-bg: var(--color-surface-primary);
	--grid-header-bg: var(--color-surface-secondary);
	--grid-row-hover: var(--color-surface-hover);
	--grid-text: var(--color-text-primary);
	--grid-border: var(--color-border-subtle);
	--grid-selection: var(--color-accent-bg);
}

/* Tokyo Night override (clase opcional) */
.grid-tokyo {
	--grid-bg: #1a1b26;
	--grid-header-bg: #1f2335;
	--grid-row-hover: #24283b;
	--grid-text: #c0caf5;
	--grid-border: rgba(122, 162, 247, 0.2);
	--grid-selection: rgba(122, 162, 247, 0.2);
}
```

---

## Clases Utilitarias

| Clase             | Uso                            |
| ----------------- | ------------------------------ |
| `.text-primary`   | Texto principal (respeta tema) |
| `.text-secondary` | Texto secundario               |
| `.bg-surface-1`   | Fondo principal                |
| `.bg-surface-2`   | Fondo secundario               |
| `.border-surface` | Borde sutil                    |
| `.bg-accent`      | Fondo acento                   |
| `.text-accent`    | Texto acento                   |

---

## Migración de `text-white`

### ❌ Evitar

```html
<span class="text-white">Nombre</span> <input class="text-white bg-black/20" />
```

### ✅ Usar

```html
<span class="text-primary">Nombre</span> <input class="input-base" />
```

---

## Archivos a Modificar

| Archivo                      | Cambio                                          |
| ---------------------------- | ----------------------------------------------- |
| `TabulatorWrapper.svelte`    | Usar variables `--grid-*`, hacer Tokyo opcional |
| `ingresoColumns.ts`          | `text-white` → `text-primary`                   |
| `ingresoProveedorColumns.ts` | `text-white` → `text-primary`                   |
| `gridBadge.ts`               | Agregar variantes `dark:`                       |
| Modales diversos             | Usar clases `input-base`, `modal-input`         |

---

## Toggle de Tema

- **Atajo:** `Ctrl+L`
- **Spotlight:** "Cambiar tema"
- **Almacenamiento:** Tauri Store + localStorage fallback
