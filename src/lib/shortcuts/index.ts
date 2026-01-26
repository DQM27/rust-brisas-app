/**
 * Sistema de Atajos de Teclado
 *
 * Exportaciones públicas del sistema de shortcuts
 */

// Tipos
export type {
	ShortcutCategory,
	ShortcutScope,
	ShortcutDefinition,
	ShortcutCommand,
	ShortcutEvent,
	UserShortcutCustomization,
	HotkeysEvent,
	CategoryMetadata,
	CollisionResult
} from './types';

// Categorías
export { SHORTCUT_CATEGORIES, getOrderedCategories, getCategoryMetadata } from './categories';

// Definiciones
export {
	ALL_SHORTCUTS,
	systemShortcuts,
	moduleShortcuts,
	modalShortcuts,
	gridShortcuts,
	ingresoShortcuts,
	getShortcutsByCategory,
	getShortcutsByScope,
	createModalCloseHandler,
	createModalSaveHandler
} from './definitions';

// Registro y Stores
export {
	shortcutRegistry,
	activeShortcuts,
	shortcutCommand,
	activeScope,
	activeContext,
	emitCommand,
	clearCommand,
	setActiveContext,
	isContextActive
} from './registry';
