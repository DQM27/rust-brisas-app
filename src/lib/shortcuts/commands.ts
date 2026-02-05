/**
 * Store de Comandos de Atajos
 *
 * Separado del registry para evitar dependencias circulares
 */

import { writable, get } from 'svelte/store';
import type { ShortcutCommand, ShortcutEvent, ShortcutScope } from './types';

/** Store para eventos de comandos */
export const shortcutCommand = writable<ShortcutEvent | null>(null);

/** Store para el scope activo */
export const activeScope = writable<ShortcutScope>('all');

/** Store para el contexto activo (ej: 'contratista-list') */
export const activeContext = writable<string | null>(null);

/** Timer para auto-limpieza */
let autoClearTimer: ReturnType<typeof setTimeout> | null = null;

/** Tiempo de auto-limpieza en ms */
const AUTO_CLEAR_DELAY = 150;

/**
 * Emitir un comando de atajo
 * Auto-limpia después de 150ms si no es consumido
 */
export function emitCommand(command: ShortcutCommand, context?: string): void {
	// Cancelar timer anterior si existe
	if (autoClearTimer) {
		clearTimeout(autoClearTimer);
	}

	shortcutCommand.set({
		command,
		timestamp: Date.now(),
		context
	});

	// Auto-limpiar después de un corto delay
	autoClearTimer = setTimeout(() => {
		shortcutCommand.set(null);
		autoClearTimer = null;
	}, AUTO_CLEAR_DELAY);
}

/**
 * Limpiar el comando actual
 */
export function clearCommand(): void {
	shortcutCommand.set(null);
}

/**
 * Establecer el contexto activo
 */
export function setActiveContext(context: string | null): void {
	activeContext.set(context);
}

/**
 * Verificar si un contexto está activo
 */
export function isContextActive(context: string): boolean {
	return get(activeContext) === context;
}
