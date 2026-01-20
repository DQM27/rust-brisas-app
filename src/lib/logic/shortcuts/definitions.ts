import type { ShortcutDefinition } from './types';
import { emitCommand } from '$lib/stores/keyboardCommands';
import { showSpotlight, showShortcutsHelp } from '$lib/stores/ui';
import { logout } from '$lib/stores/auth';
import { toggleTheme } from '$lib/stores/themeStore';

export const DEFAULT_SHORTCUTS: ShortcutDefinition[] = [
    // --- SYSTEM ---
    {
        id: 'spotlight',
        keys: '$mod+k',
        label: 'Spotlight Search',
        category: 'system',
        scope: 'global',
        action: (e) => {
            e.preventDefault();
            showSpotlight.update(v => !v);
        }
    },
    {
        id: 'theme',
        keys: '$mod+t',
        label: 'Cambiar Tema',
        category: 'system',
        scope: 'global',
        action: (e) => {
            e.preventDefault();
            toggleTheme();
        }
    },
    {
        id: 'logout',
        keys: '$mod+q',
        label: 'Cerrar Sesión',
        category: 'system',
        scope: 'global',
        action: async (e) => {
            e.preventDefault();
            // Lógica de logout con confirmación (import dinámico para evitar deps circulares si fuera necesario, o uso directo)
            try {
                const { ask } = await import('@tauri-apps/plugin-dialog');
                const confirmed = await ask('¿Cerrar sesión ahora?', {
                    title: 'Cerrar Sesión',
                    kind: 'info'
                });
                if (confirmed) logout();
            } catch (error) {
                if (confirm('¿Cerrar sesión ahora?')) logout();
            }
        }
    },
    // --- NAVIGATION / UI ---
    {
        id: 'search-focus',
        keys: '$mod+f',
        label: 'Buscar en Lista',
        category: 'navigation',
        scope: 'list',
        action: (e) => {
            e.preventDefault();
            emitCommand('search');
        }
    },
    {
        id: 'refresh-data',
        keys: '$mod+r',
        label: 'Actualizar Datos',
        category: 'navigation',
        scope: 'list',
        action: (e) => {
            e.preventDefault();
            emitCommand('refresh');
        }
    },
    {
        id: 'escape',
        keys: 'Escape',
        label: 'Cancelar / Cerrar',
        category: 'navigation',
        scope: 'global', // Escape es especial, el registry lo maneja en inputs también
        action: (e) => {
            // No prevenimos default siempre porque puede ser para salir de fullscreen, etc.
            // Pero para nuestro caso de uso:
            emitCommand('escape');
        }
    },
    // --- CRUD ---
    {
        id: 'create-new',
        keys: '$mod+n',
        label: 'Crear Nuevo',
        category: 'action',
        scope: 'list',
        action: (e) => {
            e.preventDefault();
            emitCommand('create-new');
        }
    },
    {
        id: 'edit-item',
        keys: '$mod+e',
        label: 'Editar Seleccionado',
        category: 'action',
        scope: 'list',
        action: (e) => {
            e.preventDefault();
            emitCommand('edit');
        }
    },
    {
        id: 'delete-item',
        keys: 'Delete',
        label: 'Eliminar Seleccionado',
        category: 'action',
        scope: 'list',
        action: (e) => {
            // Delete no suele necesitar preventDefault salvo que navegue atrás en navegadores viejos
            emitCommand('delete');
        }
    },
    {
        id: 'save-form',
        keys: '$mod+s',
        label: 'Guardar',
        category: 'edit',
        scope: 'modal', // Asumiendo que Create/Edit ocurre en modales o vistas de detalle
        action: (e) => {
            e.preventDefault();
            emitCommand('save');
        }
    },
    {
        id: 'show-help',
        keys: 'Shift+?',
        label: 'Mostrar Atajos',
        category: 'system',
        scope: 'global',
        action: (e) => {
            e.preventDefault();
            showShortcutsHelp.update(v => !v);
        }
    }
];
