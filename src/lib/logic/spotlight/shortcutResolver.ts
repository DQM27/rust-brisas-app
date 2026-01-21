
import { ALL_SHORTCUTS, shortcutRegistry } from '$lib/shortcuts';

// Mapeo manual de IDs de Spotlight a IDs de Shortcuts si difieren
// Si son iguales, no es necesario incluirlos aquí
const SPOTLIGHT_TO_SHORTCUT_MAP: Record<string, string> = {
    'create-contratista': 'create-new', // Asumiendo que Ctrl+N es genérico, o específico de lista
    // 'toggle-theme': 'toggle-theme' // Son iguales
    // 'show-shortcuts': 'show-shortcuts-help' // Este difiere
};

export function getShortcutDisplay(spotlightId: string, defaultShortcut?: string, definitions?: any[]): string | undefined {
    // 1. Intentar el default si no hay mapeo específico
    let shortcutId = SPOTLIGHT_TO_SHORTCUT_MAP[spotlightId] || spotlightId;

    // Casos especiales
    if (spotlightId === 'show-shortcuts') shortcutId = 'show-shortcuts-help';


    // 2. Buscar definición
    // Si pasamos definitions (desde $activeShortcuts), buscamos ahí primero para reactividad
    let shortcut = definitions?.find((s: any) => s.id === shortcutId);

    if (!shortcut) {
        // Buscar en registro global (no reactivo pero fallback seguro)
        shortcut = shortcutRegistry.getShortcut(shortcutId);
    }

    if (shortcut) {
        return formatKeys(shortcut.keys);
    }

    // 4. Fallback al default hardcodeado en spotlightDefinitions si existe
    return defaultShortcut;
}

function formatKeys(keys: string): string {
    return keys
        .replace(/\+/g, '+')
        .replace(/ctrl/gi, 'Ctrl')
        .replace(/shift/gi, 'Shift')
        .replace(/alt/gi, 'Alt')
        .replace(/meta/gi, 'Super')
        .replace(/escape/gi, 'Esc')
        .replace(/enter/gi, 'Enter')
        .replace(/backspace/gi, 'Backspace')
        .replace(/delete/gi, 'Del')
        .split('+')
        .map(k => k.charAt(0).toUpperCase() + k.slice(1)) // Capitalize each part if simpler
        .join('+');
}
