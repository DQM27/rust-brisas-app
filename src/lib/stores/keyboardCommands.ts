// src/lib/stores/keyboardCommands.ts
import { writable, get } from 'svelte/store';

/**
 * Tipos de comandos que puede emitir el sistema de atajos
 */
export type KeyboardCommand =
	| 'create-new'
	| 'edit'
	| 'delete'
	| 'search'
	| 'refresh'
	| 'save'
	| 'escape';

/**
 * Estructura del evento de comando
 */
interface CommandEvent {
	command: KeyboardCommand;
	timestamp: number;
}

/**
 * Store que mantiene el contexto activo (ej: 'users-list', 'ingreso-form')
 * Solo el componente en el contexto activo debería reaccionar a los comandos
 */
export const activeContext = writable<string | null>(null);

/**
 * Store que emite comandos de teclado
 * Los componentes pueden suscribirse para reaccionar a estos comandos
 */
export const keyboardCommand = writable<CommandEvent | null>(null);

/**
 * Registra un contexto como activo
 * @param contextId - Identificador del contexto (ej: 'users-list')
 */
export function setActiveContext(contextId: string | null): void {
	activeContext.set(contextId);
}

/**
 * Verifica si un contexto específico es el activo
 * @param contextId - Contexto a verificar
 */
export function isContextActive(contextId: string): boolean {
	return get(activeContext) === contextId;
}

/**
 * Emite un comando de teclado
 * @param command - El comando a emitir
 */
export function emitCommand(command: KeyboardCommand): void {
	keyboardCommand.set({ command, timestamp: Date.now() });
}

/**
 * Limpia el comando actual (útil después de procesarlo)
 */
export function clearCommand(): void {
	keyboardCommand.set(null);
}
