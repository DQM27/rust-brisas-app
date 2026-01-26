/**
 * Atajos de Ingresos
 *
 * Atajos específicos para el módulo de control de ingresos
 */

import type { ShortcutDefinition } from '../types';
import { emitCommand } from '../commands';

export const ingresoShortcuts: ShortcutDefinition[] = [
	{
		id: 'quick-entry',
		keys: 'alt+i',
		label: 'Entrada Rápida',
		description: 'Registra una entrada rápida',
		category: 'ingresos',
		scope: 'list',
		icon: 'LogIn',
		handler: (e) => {
			e.preventDefault();
			emitCommand('quick-entry');
		}
	},
	{
		id: 'quick-exit',
		keys: 'alt+o',
		label: 'Salida Rápida',
		description: 'Registra una salida rápida',
		category: 'ingresos',
		scope: 'list',
		icon: 'LogOut',
		handler: (e) => {
			e.preventDefault();
			emitCommand('quick-exit');
		}
	},
	{
		id: 'scan-badge',
		keys: 'alt+p',
		label: 'Escanear Gafete',
		description: 'Activa el modo de escaneo de gafete',
		category: 'ingresos',
		scope: 'list',
		icon: 'CreditCard',
		handler: (e) => {
			e.preventDefault();
			emitCommand('scan-badge');
		}
	}
];
