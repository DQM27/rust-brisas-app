/**
 * Atajos de Modales
 * 
 * Atajos para operaciones en modales y formularios
 */

import type { ShortcutDefinition } from '../types';
import { emitCommand } from '../commands';

export const modalShortcuts: ShortcutDefinition[] = [
    {
        id: 'save-form',
        keys: 'ctrl+s',
        label: 'Guardar',
        description: 'Guarda el formulario actual',
        category: 'modals',
        scope: 'modal',
        icon: 'Save',
        handler: (e) => {
            e.preventDefault();
            emitCommand('save');
        }
    },
    {
        id: 'close-modal',
        keys: 'escape',
        label: 'Cerrar',
        description: 'Cierra el modal actual',
        category: 'modals',
        scope: 'modal',
        icon: 'X',
        readonly: true, // No se puede personalizar
        handler: (e) => {
            e.preventDefault();
            emitCommand('cancel');
        }
    }
];

/**
 * Handler centralizado para cerrar modales
 * Exportado para uso directo en componentes que necesiten
 * comportamiento personalizado
 */
export function createModalCloseHandler(onClose: () => void) {
    return {
        keys: 'escape',
        handler: (e: KeyboardEvent) => {
            const target = e.target as HTMLElement;
            // No cerrar si estamos en un input (permitir escape para limpiar)
            if (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA') {
                return;
            }
            e.preventDefault();
            onClose();
        }
    };
}

/**
 * Handler centralizado para guardar formularios
 */
export function createModalSaveHandler(onSave: () => void) {
    return {
        keys: 'ctrl+s',
        handler: (e: KeyboardEvent) => {
            e.preventDefault();
            onSave();
        }
    };
}
