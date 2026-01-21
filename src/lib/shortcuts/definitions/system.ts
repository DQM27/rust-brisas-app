/**
 * Atajos de Sistema
 * 
 * Atajos globales que funcionan en cualquier contexto
 */

import type { ShortcutDefinition } from '../types';
import { showSpotlight, showShortcutsHelp, showQuickSwitch } from '$lib/stores/ui';
import { toggleTheme } from '$lib/stores/themeStore';
import { logout } from '$lib/stores/auth';
import { activeTabId, closeTab } from '$lib/stores/tabs';
import { get } from 'svelte/store';

export const systemShortcuts: ShortcutDefinition[] = [
    {
        id: 'toggle-quick-switch',
        keys: 'ctrl+shift+u',
        label: 'Cambio de Usuario',
        description: 'Búsqueda rápida para cambiar de usuario',
        category: 'system',
        scope: 'all',
        icon: 'UserRoundPen',
        handler: (e) => {
            e.preventDefault();
            showQuickSwitch.update(v => !v);
        }
    },
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
        id: 'close-active-tab',
        keys: 'alt+q',
        label: 'Cerrar Pestaña',
        description: 'Cierra la pestaña activa actual',
        category: 'system',
        scope: 'all',
        icon: 'Tabs',
        handler: (e) => {
            e.preventDefault();
            const currentId = get(activeTabId);
            if (currentId) {
                closeTab(currentId);
            }
        }
    },
    {
        id: 'show-shortcuts-help',
        keys: 'shift+a',
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
