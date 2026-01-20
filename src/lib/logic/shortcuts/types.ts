/**
 * Tipos para el sistema de atajos de teclado
 */

export type ShortcutScope = 'global' | 'list' | 'detail' | 'modal';

export interface ShortcutContext {
    scope: ShortcutScope;
    moduleId?: string; // ID del módulo activo (ej: 'users-list')
}

export interface ShortcutAction {
    type: string;
    payload?: any;
}

export interface ShortcutDefinition {
    id: string;
    keys: string; // Ej: '$mod+k', 'Shift+?'
    label: string;
    description?: string;
    scope: ShortcutScope; // Dónde está activo este atajo
    action: (e: KeyboardEvent) => void;
    condition?: () => boolean; // Predicado opcional para verificar si debe ejecutarse
    category?: 'system' | 'navigation' | 'action' | 'edit';
}

export interface ShortcutGroup {
    category: string;
    items: ShortcutDefinition[];
}
