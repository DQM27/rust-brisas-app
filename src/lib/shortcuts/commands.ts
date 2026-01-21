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

/**
 * Emitir un comando de atajo
 */
export function emitCommand(command: ShortcutCommand, context?: string): void {
    shortcutCommand.set({
        command,
        timestamp: Date.now(),
        context
    });
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
