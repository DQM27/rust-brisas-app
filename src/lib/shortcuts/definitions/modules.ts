/**
 * Atajos de Módulos/Listas
 * 
 * Atajos para operaciones CRUD en vistas de lista
 */

import type { ShortcutDefinition } from '../types';
import { emitCommand } from '../commands';

export const moduleShortcuts: ShortcutDefinition[] = [
    {
        id: 'create-new',
        keys: 'ctrl+n',
        label: 'Crear Nuevo',
        description: 'Abre el formulario para crear un nuevo registro',
        category: 'modules',
        scope: 'list',
        icon: 'Plus',
        handler: (e) => {
            e.preventDefault();
            emitCommand('create');
        }
    },
    {
        id: 'edit-selected',
        keys: 'ctrl+e',
        label: 'Editar Seleccionado',
        description: 'Edita el registro seleccionado',
        category: 'modules',
        scope: 'list',
        icon: 'Pencil',
        handler: (e) => {
            e.preventDefault();
            emitCommand('edit');
        }
    },
    {
        id: 'delete-selected',
        keys: 'delete',
        label: 'Eliminar Seleccionado',
        description: 'Elimina el registro seleccionado',
        category: 'modules',
        scope: 'list',
        icon: 'Trash2',
        handler: () => {
            // No preventDefault para permitir delete en inputs
            emitCommand('delete');
        }
    },
    {
        id: 'refresh-data',
        keys: 'ctrl+r',
        label: 'Actualizar Datos',
        description: 'Recarga los datos de la vista',
        category: 'modules',
        scope: 'list',
        icon: 'RefreshCw',
        handler: (e) => {
            e.preventDefault();
            emitCommand('refresh');
        }
    },
    {
        id: 'search-focus',
        keys: 'ctrl+f',
        label: 'Buscar',
        description: 'Enfoca el campo de búsqueda',
        category: 'modules',
        scope: 'list',
        icon: 'Search',
        handler: (e) => {
            e.preventDefault();
            emitCommand('search');
        }
    },
    {
        id: 'select-all',
        keys: 'ctrl+a',
        label: 'Seleccionar Todo',
        description: 'Selecciona todos los registros visibles',
        category: 'modules',
        scope: 'list',
        icon: 'CheckSquare',
        handler: (e) => {
            e.preventDefault();
            emitCommand('select-all');
        }
    }
];
