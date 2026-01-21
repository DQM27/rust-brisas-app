/**
 * Exportación de todas las definiciones de atajos
 */

export { systemShortcuts } from './system';
export { moduleShortcuts } from './modules';
export { modalShortcuts, createModalCloseHandler, createModalSaveHandler } from './modals';
export { gridShortcuts } from './grids';
export { ingresoShortcuts } from './ingresos';
export { ingressAccessShortcuts } from './ingressAccess';

import { systemShortcuts } from './system';
import { moduleShortcuts } from './modules';
import { modalShortcuts } from './modals';
import { gridShortcuts } from './grids';
import { ingresoShortcuts } from './ingresos';
import { ingressAccessShortcuts } from './ingressAccess';
import type { ShortcutDefinition } from '../types';

/**
 * Todos los atajos por defecto
 */
export const ALL_SHORTCUTS: ShortcutDefinition[] = [
    ...systemShortcuts,
    ...moduleShortcuts,
    ...modalShortcuts,
    ...gridShortcuts,
    ...ingresoShortcuts,
    ...ingressAccessShortcuts
];

/**
 * Obtener atajos por categoría
 */
export function getShortcutsByCategory(category: string): ShortcutDefinition[] {
    return ALL_SHORTCUTS.filter(s => s.category === category);
}

/**
 * Obtener atajos por scope
 */
export function getShortcutsByScope(scope: string): ShortcutDefinition[] {
    return ALL_SHORTCUTS.filter(s => s.scope === scope || s.scope === 'all');
}
