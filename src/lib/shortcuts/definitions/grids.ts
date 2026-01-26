/**
 * Atajos de Grids/Tablas
 *
 * Atajos para navegación en grids Tabulator
 */

import type { ShortcutDefinition } from '../types';
import { emitCommand } from '../commands';

export const gridShortcuts: ShortcutDefinition[] = [
	{
		id: 'grid-next-page',
		keys: 'pagedown',
		label: 'Siguiente Página',
		description: 'Navega a la siguiente página del grid',
		category: 'grids',
		scope: 'grid',
		icon: 'ChevronDown',
		handler: (e) => {
			e.preventDefault();
			emitCommand('next-page');
		}
	},
	{
		id: 'grid-prev-page',
		keys: 'pageup',
		label: 'Página Anterior',
		description: 'Navega a la página anterior del grid',
		category: 'grids',
		scope: 'grid',
		icon: 'ChevronUp',
		handler: (e) => {
			e.preventDefault();
			emitCommand('prev-page');
		}
	},
	{
		id: 'grid-first-page',
		keys: 'home',
		label: 'Primera Página',
		description: 'Navega a la primera página del grid',
		category: 'grids',
		scope: 'grid',
		icon: 'ChevronsUp',
		handler: (e) => {
			e.preventDefault();
			emitCommand('first-page');
		}
	}
];
