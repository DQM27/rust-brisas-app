# Plan de Implementación: Sistema de Atajos de Teclado

> **Fecha**: 2026-01-20  
> **Estado**: ✅ Implementado  
> **Prioridad**: Alta
> **Librería**: hotkeys-js (migrado desde tinykeys)

## Resumen Ejecutivo

Refactorización completa del sistema de atajos de teclado para hacerlo más robusto, categorizado y personalizable por usuario.

**Decisión clave**: Migrar de `tinykeys` a `hotkeys-js` por sus capacidades superiores.

---

## Problema Actual

- ❌ No permite cambiar atajos dinámicamente (tinykeys requiere reinicializar todo)
- ❌ No detecta colisiones entre atajos
- ❌ Scopes manuales (lógica que ya existe en otras librerías)
- ❌ Categorización básica
- ❌ Handler de modales no centralizado

---

## Análisis: Tinykeys vs Hotkeys-js

| Característica | Tinykeys (actual) | Hotkeys-js (propuesto) |
|----------------|-------------------|------------------------|
| Tamaño | ~650B | ~2.5KB |
| Cambiar atajos dinámicamente | ❌ Requiere reinicializar | ✅ `unbind()` + `hotkeys()` |
| Detectar colisiones | ❌ No soportado | ✅ `getAllKeyCodes()` |
| Scopes/Contexts | ❌ Manual | ✅ Built-in |
| Unbind individual | ❌ No soportado | ✅ Soportado |
| Filtrar inputs | ❌ Manual | ✅ `hotkeys.filter` |
| Trigger programático | ❌ No | ✅ `hotkeys.trigger()` |

**Recomendación**: Migrar a hotkeys-js.

---

## Personalización por Usuario

### Arquitectura

```
SQLite (user_shortcuts) → Rust Backend → Frontend → hotkeys-js
```

### Tabla en SQLite

```sql
CREATE TABLE user_shortcuts (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    shortcut_id TEXT NOT NULL,
    custom_keys TEXT NOT NULL,
    enabled BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id),
    UNIQUE(user_id, shortcut_id)
);
```

---

## Categorías de Atajos

```
📂 CATEGORÍAS DE ATAJOS
├── 🖥️ Sistema (Global)
│   ├── Tema claro/oscuro (Ctrl+T)
│   ├── Spotlight (Ctrl+K)  
│   ├── Ayuda de atajos (Shift+?)
│   └── Cerrar sesión (Ctrl+Q)
│
├── 🔍 Spotlight (contextual)
│   ├── Navegar resultados (↑/↓)
│   ├── Seleccionar (Enter)
│   └── Cerrar (Escape)
│
├── 📋 Módulos/Listas
│   ├── Crear nuevo (Ctrl+N)
│   ├── Editar seleccionado (Ctrl+E)
│   ├── Eliminar seleccionado (Delete)
│   ├── Buscar en lista (Ctrl+F)
│   ├── Actualizar datos (Ctrl+R)
│   └── Seleccionar todo (Ctrl+A)
│
├── 🚪 Ingresos
│   ├── Registrar entrada (F1)
│   ├── Registrar salida (F2)
│   └── Escanear gafete (F3)
│
├── 📝 Modales/Formularios
│   ├── Guardar (Ctrl+S)
│   ├── Cancelar/Cerrar (Escape)
│   └── Navegar campos (Tab/Shift+Tab)
│
└── 📊 Grid/Tablas
    ├── Siguiente página (PageDown)
    ├── Página anterior (PageUp)
    └── Ir al inicio (Home)
```

---

## Estructura de Archivos Propuesta

```
src/lib/
├── shortcuts/
│   ├── index.ts              # Exports públicos
│   ├── types.ts              # Tipos centralizados
│   ├── categories.ts         # Definición de categorías
│   ├── definitions/
│   │   ├── system.ts         # Atajos de sistema
│   │   ├── spotlight.ts      # Atajos de spotlight
│   │   ├── modules.ts        # Atajos de listas/módulos
│   │   ├── modals.ts         # Atajos de modales
│   │   ├── grids.ts          # Atajos de grids
│   │   └── ingresos.ts       # Atajos específicos de ingresos
│   ├── registry.ts           # Registro central
│   └── handlers/
│       ├── modalHandler.ts   # Handler centralizado para modales
│       └── gridHandler.ts    # Handler centralizado para grids
├── stores/
│   └── shortcuts.ts          # Store unificado
└── actions/
    └── shortcutScope.ts      # Svelte action para scopes
```

---

## Archivos a Modificar

| Acción | Archivo |
|--------|---------|
| [NEW] | `src/lib/shortcuts/index.ts` |
| [NEW] | `src/lib/shortcuts/types.ts` |
| [NEW] | `src/lib/shortcuts/categories.ts` |
| [NEW] | `src/lib/shortcuts/definitions/*.ts` |
| [NEW] | `src/lib/shortcuts/registry.ts` |
| [NEW] | `src/lib/shortcuts/handlers/*.ts` |
| [MODIFY] | `src/lib/components/layout/KeyboardShortcuts.svelte` |
| [MODIFY] | `src/lib/components/modals/ShortcutHelpModal.svelte` |
| [DELETE] | `src/lib/stores/keyboardCommands.ts` |
| [DELETE] | `src/lib/types/shortcuts.ts` |
| [DELETE] | `src/lib/logic/shortcuts/` |

---

## Verificación

1. **Atajos Globales**: Ctrl+K, Ctrl+T, Shift+?
2. **Atajos de Lista**: Ctrl+N, Ctrl+R, Delete
3. **Atajos de Modal**: Escape, Ctrl+S
4. **UI de Ayuda**: Verificar categorías y agrupación
5. **Sin Conflictos**: Escape en inputs, Ctrl+F en grids
