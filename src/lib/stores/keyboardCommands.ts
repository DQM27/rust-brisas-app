// src/lib/stores/keyboardCommands.ts
import { writable } from 'svelte/store';

/**
 * Tipos de comandos que puede emitir el sistema de atajos
 */
export type KeyboardCommand = 'create-new' | 'search' | 'refresh';

/**
 * Estructura del evento de comando
 */
interface CommandEvent {
    command: KeyboardCommand;
    timestamp: number;
}

/**
 * Store que emite comandos de teclado
 * Los componentes pueden suscribirse para reaccionar a estos comandos
 */
export const keyboardCommand = writable<CommandEvent | null>(null);

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
