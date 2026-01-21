/**
 * Sistema de Atajos de Teclado - Tipos
 * 
 * Tipos centralizados para el sistema de shortcuts usando hotkeys-js
 */

// ============================================
// CATEGORÍAS
// ============================================

export type ShortcutCategory =
    | 'system'      // Globales del sistema (tema, spotlight, logout)
    | 'modules'     // En vistas de lista/módulos
    | 'modals'      // En modales abiertos
    | 'grids'       // En grids/tablas
    | 'ingresos'    // Específicos del módulo de ingresos
    | 'ingress-access'; // Accesos rápidos a módulos de ingreso

export interface CategoryMetadata {
    id: ShortcutCategory;
    label: string;
    description: string;
    icon: string;
    order: number;
}

// ============================================
// SCOPES (hotkeys-js native)
// ============================================

export type ShortcutScope =
    | 'all'         // Siempre activo (hotkeys-js default)
    | 'global'      // Alias para 'all'
    | 'modal'       // Cuando un modal está abierto
    | 'list'        // En vistas de lista
    | 'grid'        // En grids
    | 'ingreso';    // En módulo de ingresos

// ============================================
// DEFINICIONES
// ============================================

export interface ShortcutDefinition {
    /** Identificador único del atajo */
    id: string;

    /** Combinación de teclas (formato hotkeys-js: 'ctrl+n', 'escape', etc.) */
    keys: string;

    /** Etiqueta para mostrar en UI */
    label: string;

    /** Descripción detallada */
    description?: string;

    /** Categoría para agrupar en UI de ayuda */
    category: ShortcutCategory;

    /** Scope de hotkeys-js donde está activo */
    scope: ShortcutScope;

    /** Handler del atajo */
    handler: (event: KeyboardEvent, hotkeysEvent: HotkeysEvent) => void;

    /** Función opcional para verificar si el atajo debe ejecutarse */
    enabled?: () => boolean;

    /** Si true, no se puede personalizar */
    readonly?: boolean;

    /** Ícono para UI (nombre de Lucide icon) */
    icon?: string;
}

// ============================================
// PERSONALIZACIÓN POR USUARIO
// ============================================

export interface UserShortcutCustomization {
    /** ID del usuario */
    userId: string;

    /** ID del atajo que se personaliza */
    shortcutId: string;

    /** Nueva combinación de teclas */
    customKeys: string;

    /** Si está habilitado */
    enabled: boolean;
}

// ============================================
// EVENTOS Y COMANDOS
// ============================================

export type ShortcutCommand =
    // CRUD
    | 'create'
    | 'edit'
    | 'delete'
    | 'save'
    | 'cancel'
    | 'refresh'
    // Navegación
    | 'search'
    | 'select-all'
    | 'escape'
    // Sistema
    | 'toggle-spotlight'
    | 'toggle-theme'
    | 'show-help'
    | 'logout'
    // Grid
    | 'next-page'
    | 'prev-page'
    | 'first-page'
    // Ingresos
    | 'quick-entry'
    | 'quick-exit'
    | 'scan-badge';

export interface ShortcutEvent {
    command: ShortcutCommand;
    timestamp: number;
    context?: string;
}

// ============================================
// HOTKEYS-JS TYPES (para mejor tipado)
// ============================================

export interface HotkeysEvent {
    key: string;
    scope: string;
    method: any; // Compatible con hotkeys-js internal types
    mods: number[];
    shortcut: string;
}

// ============================================
// REGISTRO
// ============================================

export interface ShortcutRegistryState {
    /** Todos los atajos registrados */
    shortcuts: ShortcutDefinition[];

    /** Scope activo actual */
    activeScope: ShortcutScope;

    /** Stack de scopes (para modales anidados) */
    scopeStack: ShortcutScope[];

    /** Personalizaciones del usuario actual */
    customizations: UserShortcutCustomization[];
}

export interface CollisionResult {
    hasCollision: boolean;
    conflictingShortcut?: ShortcutDefinition;
    scope: ShortcutScope;
}
