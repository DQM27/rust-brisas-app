/**
 * Categorías de Atajos de Teclado
 * 
 * Metadatos para mostrar en la UI de ayuda
 */

import type { CategoryMetadata, ShortcutCategory } from './types';

export const SHORTCUT_CATEGORIES: Record<ShortcutCategory, CategoryMetadata> = {
    system: {
        id: 'system',
        label: 'Sistema',
        description: 'Atajos globales del sistema',
        icon: 'Settings',
        order: 1
    },
    spotlight: {
        id: 'spotlight',
        label: 'Spotlight',
        description: 'Navegación en el buscador',
        icon: 'Search',
        order: 2
    },
    modules: {
        id: 'modules',
        label: 'Módulos',
        description: 'Acciones en vistas de lista',
        icon: 'LayoutList',
        order: 3
    },
    modals: {
        id: 'modals',
        label: 'Modales',
        description: 'Acciones en formularios',
        icon: 'Square',
        order: 4
    },
    grids: {
        id: 'grids',
        label: 'Tablas',
        description: 'Navegación en grids',
        icon: 'Table',
        order: 5
    },
    ingresos: {
        id: 'ingresos',
        label: 'Ingresos',
        description: 'Acciones rápidas de ingreso',
        icon: 'DoorOpen',
        order: 6
    }
};

/**
 * Obtener categorías ordenadas para UI
 */
export function getOrderedCategories(): CategoryMetadata[] {
    return Object.values(SHORTCUT_CATEGORIES).sort((a, b) => a.order - b.order);
}

/**
 * Obtener metadatos de una categoría
 */
export function getCategoryMetadata(category: ShortcutCategory): CategoryMetadata {
    return SHORTCUT_CATEGORIES[category];
}
