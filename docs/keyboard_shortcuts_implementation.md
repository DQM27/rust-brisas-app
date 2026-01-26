# Plan de Implementación: Sistema de Atajos de Teclado

> **Fecha**: 2026-01-20  
> **Estado**: ✅ Implementado Completamente  
> **Prioridad**: Alta
> **Librería**: hotkeys-js (migrado desde tinykeys)

## Resumen Ejecutivo

Refactorización completa del sistema de atajos de teclado para hacerlo más robusto, categorizado y personalizable por usuario.

**Decisión clave**: Migrar de `tinykeys` a `hotkeys-js` por sus capacidades superiores.

---

## ✅ Implementación Completada

### Backend (Rust)

| Archivo                                            | Descripción                     |
| -------------------------------------------------- | ------------------------------- |
| `src-tauri/src/db/surrealdb_schema.surql`          | Tabla `user_shortcuts` agregada |
| `src-tauri/src/models/user_shortcuts.rs`           | Tipos de respuesta              |
| `src-tauri/src/services/user_shortcuts_service.rs` | CRUD en SurrealDB               |
| `src-tauri/src/commands/shortcuts_commands.rs`     | Comandos Tauri                  |

### Frontend (Svelte)

| Archivo                                                     | Descripción         |
| ----------------------------------------------------------- | ------------------- |
| `src/lib/shortcuts/`                                        | Sistema completo    |
| `src/lib/shortcuts/userShortcutsService.ts`                 | Cliente del backend |
| `src/lib/components/settings/ShortcutsSettingsPanel.svelte` | UI de configuración |
| `src/lib/components/modals/ShortcutConfigModal.svelte`      | Modal alternativo   |

### Acceso

- **Menu Configuración** → "Atajos de Teclado" → Abre pestaña de configuración
- Visible para **todos los usuarios**

---

## Características Implementadas

| Característica                 | Estado |
| ------------------------------ | ------ |
| Migración a hotkeys-js         | ✅     |
| Sistema de categorías          | ✅     |
| Detección de colisiones        | ✅     |
| Persistencia en SurrealDB      | ✅     |
| UI de configuración tipo Roles | ✅     |
| Recording de teclas en vivo    | ✅     |
| Restaurar atajos individuales  | ✅     |
| Restaurar todos los atajos     | ✅     |

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

## Estructura de Archivos

```
src/lib/
├── shortcuts/
│   ├── index.ts              # Exports públicos
│   ├── types.ts              # Tipos centralizados
│   ├── categories.ts         # Definición de categorías
│   ├── commands.ts           # Stores de comandos
│   ├── registry.ts           # Registro central (hotkeys-js)
│   ├── userShortcutsService.ts # Cliente backend
│   └── definitions/
│       ├── index.ts          # Barrel export
│       ├── system.ts         # Atajos de sistema
│       ├── spotlight.ts      # Atajos de spotlight
│       ├── modules.ts        # Atajos de listas/módulos
│       ├── modals.ts         # Atajos de modales
│       ├── grids.ts          # Atajos de grids
│       └── ingresos.ts       # Atajos específicos

src-tauri/src/
├── models/user_shortcuts.rs
├── services/user_shortcuts_service.rs
└── commands/shortcuts_commands.rs
```

---

## Comandos Tauri

| Comando                | Descripción                                      |
| ---------------------- | ------------------------------------------------ |
| `get_user_shortcuts`   | Obtiene atajos personalizados del usuario actual |
| `save_user_shortcut`   | Guarda/actualiza un atajo personalizado          |
| `delete_user_shortcut` | Elimina un atajo (vuelve al default)             |
| `reset_user_shortcuts` | Resetea todos los atajos del usuario             |
