/**
 * Registro Central de Atajos de Teclado
 *
 * Usa hotkeys-js para manejar los keybindings con soporte nativo para:
 * - Scopes (contextos)
 * - Unbind dinámico
 * - Detección de colisiones
 */

import hotkeys from 'hotkeys-js';
import { writable, get } from 'svelte/store';
import type {
	ShortcutDefinition,
	ShortcutScope,
	CollisionResult,
	UserShortcutCustomization
} from './types';
import { ALL_SHORTCUTS } from './definitions';
import {
	shortcutCommand,
	activeScope,
	activeContext,
	emitCommand,
	clearCommand,
	setActiveContext,
	isContextActive
} from './commands';

// Re-export from commands for backwards compatibility
export {
	shortcutCommand,
	activeScope,
	activeContext,
	emitCommand,
	clearCommand,
	setActiveContext,
	isContextActive
};

// ============================================
// STORES
// ============================================

/** Store reactivo con los atajos activos */
export const activeShortcuts = writable<ShortcutDefinition[]>([]);

// ============================================
// REGISTRO
// ============================================

class ShortcutRegistry {
	private shortcuts: Map<string, ShortcutDefinition> = new Map();
	private scopeStack: ShortcutScope[] = ['all'];
	private customizations: Map<string, UserShortcutCustomization> = new Map();
	private initialized = false;

	constructor() {
		// Configurar hotkeys-js para permitir atajos en inputs si queremos
		hotkeys.filter = (event) => {
			const target = event.target as HTMLElement;
			const tagName = target.tagName;

			// Siempre permitir Escape
			if (event.key === 'Escape') return true;

			// Bloquear otros atajos en inputs/textareas/contenteditable
			if (target.isContentEditable) return false;
			if (tagName === 'INPUT' || tagName === 'TEXTAREA' || tagName === 'SELECT') {
				return false;
			}

			return true;
		};
	}

	/**
	 * Inicializar el registro con los atajos por defecto
	 */
	public init(customizations?: UserShortcutCustomization[]): void {
		if (this.initialized) {
			this.destroy();
		}

		// Cargar personalizaciones si existen
		if (customizations) {
			customizations.forEach((c) => {
				this.customizations.set(c.shortcutId, c);
			});
		}

		// Registrar todos los atajos
		ALL_SHORTCUTS.forEach((shortcut) => {
			this.register(shortcut);
		});

		this.initialized = true;
		this.updateActiveShortcuts();
	}

	/**
	 * Destruir todos los atajos registrados
	 */
	public destroy(): void {
		hotkeys.unbind();
		this.shortcuts.clear();
		this.initialized = false;
	}

	/**
	 * Registrar un atajo
	 */
	public register(definition: ShortcutDefinition): void {
		// Verificar personalización del usuario
		const customization = this.customizations.get(definition.id);
		const keys = customization?.customKeys || definition.keys;
		const enabled = customization?.enabled ?? true;

		if (!enabled) return;

		// Detectar colisiones
		const collision = this.checkCollision(keys, definition.scope);
		if (collision.hasCollision) {
			console.warn(
				`[Shortcuts] Colisión detectada: "${keys}" ya está registrado en scope "${collision.scope}"`,
				collision.conflictingShortcut
			);
		}

		// Guardar definición
		const finalDefinition = { ...definition, keys };
		this.shortcuts.set(definition.id, finalDefinition);

		// Registrar en hotkeys-js
		const scope =
			definition.scope === 'all' || definition.scope === 'global' ? 'all' : definition.scope;

		hotkeys(keys, scope, (event, handler) => {
			// Verificar condición enabled si existe
			if (definition.enabled && !definition.enabled()) {
				return;
			}

			definition.handler(event, handler);
		});
	}

	/**
	 * Desregistrar un atajo
	 */
	public unregister(shortcutId: string): void {
		const definition = this.shortcuts.get(shortcutId);
		if (!definition) return;

		hotkeys.unbind(definition.keys, definition.scope);
		this.shortcuts.delete(shortcutId);
		this.updateActiveShortcuts();
	}

	/**
	 * Cambiar la tecla de un atajo
	 */
	public rebind(shortcutId: string, newKeys: string): boolean {
		const definition = this.shortcuts.get(shortcutId);
		if (!definition) return false;

		// Verificar si se puede personalizar
		if (definition.readonly) {
			console.warn(`[Shortcuts] El atajo "${shortcutId}" no se puede personalizar`);
			return false;
		}

		// Verificar colisiones
		const collision = this.checkCollision(newKeys, definition.scope);
		if (collision.hasCollision && collision.conflictingShortcut?.id !== shortcutId) {
			console.warn(`[Shortcuts] No se puede rebind: "${newKeys}" ya está en uso`);
			return false;
		}

		// Desregistrar viejo
		hotkeys.unbind(definition.keys, definition.scope);

		// Registrar nuevo
		const newDefinition = { ...definition, keys: newKeys };
		this.shortcuts.set(shortcutId, newDefinition);

		const scope =
			definition.scope === 'all' || definition.scope === 'global' ? 'all' : definition.scope;

		hotkeys(newKeys, scope, (event, handler) => {
			if (definition.enabled && !definition.enabled()) {
				return;
			}
			definition.handler(event, handler);
		});

		this.updateActiveShortcuts();
		return true;
	}

	/**
	 * Detectar colisiones
	 */
	public checkCollision(keys: string, scope: ShortcutScope): CollisionResult {
		const allCodes = hotkeys.getAllKeyCodes();

		for (const existing of allCodes) {
			if (existing.shortcut.toLowerCase() === keys.toLowerCase()) {
				// Encontrar la definición correspondiente
				for (const def of this.shortcuts.values()) {
					if (def.keys.toLowerCase() === keys.toLowerCase()) {
						// Es colisión si están en el mismo scope o uno es 'all'
						if (def.scope === scope || def.scope === 'all' || scope === 'all') {
							return {
								hasCollision: true,
								conflictingShortcut: def,
								scope: def.scope
							};
						}
					}
				}
			}
		}

		return { hasCollision: false, scope };
	}

	// ============================================
	// SCOPE MANAGEMENT
	// ============================================

	/**
	 * Cambiar el scope activo (reemplaza el scope actual del stack)
	 */
	public setScope(scope: ShortcutScope): void {
		// Actualizar el scopeStack para que popScope funcione correctamente
		if (this.scopeStack.length > 0) {
			this.scopeStack[this.scopeStack.length - 1] = scope;
		} else {
			this.scopeStack.push(scope);
		}
		hotkeys.setScope(scope);
		activeScope.set(scope);
		this.updateActiveShortcuts();
	}

	/**
	 * Obtener el scope activo
	 */
	public getScope(): string {
		return hotkeys.getScope();
	}

	/**
	 * Push un scope al stack (para modales anidados)
	 */
	public pushScope(scope: ShortcutScope): void {
		this.scopeStack.push(scope);
		this.setScope(scope);
	}

	/**
	 * Pop el scope del stack
	 */
	public popScope(): void {
		if (this.scopeStack.length > 1) {
			this.scopeStack.pop();
			this.setScope(this.scopeStack[this.scopeStack.length - 1]);
		}
	}

	// ============================================
	// HELPERS
	// ============================================

	/**
	 * Obtener todos los atajos registrados
	 */
	public getAllShortcuts(): ShortcutDefinition[] {
		return Array.from(this.shortcuts.values());
	}

	/**
	 * Obtener atajo por ID
	 */
	public getShortcut(id: string): ShortcutDefinition | undefined {
		return this.shortcuts.get(id);
	}

	/**
	 * Trigger programático de un atajo
	 */
	public trigger(keys: string, scope?: string): void {
		hotkeys.trigger(keys, scope);
	}

	/**
	 * Verificar si una tecla está presionada
	 */
	public isPressed(key: string): boolean {
		return hotkeys.isPressed(key);
	}

	/**
	 * Actualizar el store de atajos activos
	 */
	private updateActiveShortcuts(): void {
		const currentScope = this.getScope();
		const shortcuts = this.getAllShortcuts().filter(
			(s) => s.scope === 'all' || s.scope === currentScope
		);
		activeShortcuts.set(shortcuts);
	}
}

// Singleton
export const shortcutRegistry = new ShortcutRegistry();
