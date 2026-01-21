/**
 * Atajos de Spotlight
 * 
 * Atajos para navegación dentro del Spotlight
 */

import type { ShortcutDefinition } from '../types';

// Nota: Los atajos de navegación del Spotlight (↑/↓, Enter, Escape)
// se manejan internamente en el componente SpotlightSearch.svelte
// porque requieren acceso al estado interno del componente.
// 
// Este archivo existe para documentar los atajos y mostrarlos
// en el modal de ayuda.

export const spotlightShortcuts: ShortcutDefinition[] = [
    {
        id: 'spotlight-nav-up',
        keys: 'up',
        label: 'Resultado Anterior',
        description: 'Navega al resultado anterior',
        category: 'spotlight',
        scope: 'spotlight',
        icon: 'ArrowUp',
        readonly: true,
        handler: () => {
            // Manejado internamente por SpotlightSearch
        }
    },
    {
        id: 'spotlight-nav-down',
        keys: 'down',
        label: 'Siguiente Resultado',
        description: 'Navega al siguiente resultado',
        category: 'spotlight',
        scope: 'spotlight',
        icon: 'ArrowDown',
        readonly: true,
        handler: () => {
            // Manejado internamente por SpotlightSearch
        }
    },
    {
        id: 'spotlight-select',
        keys: 'enter',
        label: 'Seleccionar',
        description: 'Ejecuta la acción seleccionada',
        category: 'spotlight',
        scope: 'spotlight',
        icon: 'CornerDownLeft',
        readonly: true,
        handler: () => {
            // Manejado internamente por SpotlightSearch
        }
    },
    {
        id: 'spotlight-close',
        keys: 'escape',
        label: 'Cerrar Spotlight',
        description: 'Cierra el buscador',
        category: 'spotlight',
        scope: 'spotlight',
        icon: 'X',
        readonly: true,
        handler: () => {
            // Manejado internamente por SpotlightSearch
        }
    }
];
