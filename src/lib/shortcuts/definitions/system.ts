/**
 * Atajos de Sistema
 * 
 * Atajos globales que funcionan en cualquier contexto
 */

import type { ShortcutDefinition } from '../types';
import { showSpotlight, showShortcutsHelp } from '$lib/stores/ui';
import { toggleTheme } from '$lib/stores/themeStore';
import { logout } from '$lib/stores/auth';

export const systemShortcuts: ShortcutDefinition[] = [
    {
        id: 'toggle-spotlight',
        keys: 'ctrl+k',
        label: 'Abrir Spotlight',
        description: 'Abre el buscador global',
        category: 'system',
        scope: 'all',
        icon: 'Search',
        handler: (e) => {
            e.preventDefault();
            showSpotlight.update(v => !v);
        }
    },
    {
        id: 'toggle-theme',
        keys: 'ctrl+t',
        label: 'Cambiar Tema',
        description: 'Alterna entre tema claro y oscuro',
        category: 'system',
        scope: 'all',
        icon: 'Sun',
        handler: (e) => {
            e.preventDefault();
            toggleTheme();
        }
    },
    {
        id: 'show-shortcuts-help',
        keys: 'shift+/',  // ? es shift+/ en teclado US
        label: 'Ayuda de Atajos',
        description: 'Muestra todos los atajos disponibles',
        category: 'system',
        scope: 'all',
        icon: 'Keyboard',
        handler: (e) => {
            e.preventDefault();
            showShortcutsHelp.update(v => !v);
        }
    },
    {
        id: 'logout',
        keys: 'ctrl+q',
        label: 'Cerrar Sesión',
        description: 'Cierra la sesión actual',
        category: 'system',
        scope: 'all',
        icon: 'LogOut',
        handler: async (e) => {
            e.preventDefault();
            try {
                const { ask } = await import('@tauri-apps/plugin-dialog');
                const confirmed = await ask('¿Cerrar sesión ahora?', {
                    title: 'Cerrar Sesión',
                    kind: 'info'
                });
                if (confirmed) logout();
            } catch {
                if (confirm('¿Cerrar sesión ahora?')) logout();
            }
        }
    }
];
