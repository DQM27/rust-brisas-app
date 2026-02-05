/**
 * Hook para activar scope de shortcuts automáticamente
 *
 * Uso en ListViews:
 * ```svelte
 * <script>
 *   import { useListScope } from '$lib/shortcuts/hooks';
 *   useListScope('my-list-context');
 * </script>
 * ```
 *
 * Uso en Modales:
 * ```svelte
 * <script>
 *   import { useModalScope } from '$lib/shortcuts/hooks';
 *   let { show } = $props();
 *   useModalScope(() => show);
 * </script>
 * ```
 */

import { onDestroy } from 'svelte';
import { get } from 'svelte/store';
import { activeTabId } from '$lib/stores/tabs';
import { shortcutRegistry, setActiveContext, type ShortcutScope } from '$lib/shortcuts';

/**
 * Activa el scope 'list' cuando la tab está activa
 * @param contextId Identificador único del contexto (ej: 'contratista-list')
 * @param tabId ID de la tab (opcional, usa el que viene como prop por defecto)
 */
export function useListScope(contextId: string, tabId?: string): void {
    $effect(() => {
        const currentTabId = tabId || get(activeTabId);
        const activeTab = get(activeTabId);

        if (activeTab === currentTabId) {
            shortcutRegistry.setScope('list');
            setActiveContext(contextId);
        }
    });

    onDestroy(() => {
        // Restaurar scope global al destruir
        if (get(activeTabId) === tabId) {
            shortcutRegistry.setScope('all');
            setActiveContext(null);
        }
    });
}

/**
 * Activa el scope 'modal' cuando un modal está abierto
 * Restaura el scope anterior al cerrar
 * @param isOpenGetter Función que retorna si el modal está abierto
 */
export function useModalScope(isOpenGetter: () => boolean): void {
    let previousScope: ShortcutScope = 'all';

    $effect(() => {
        const isOpen = isOpenGetter();

        if (isOpen) {
            // Guardar scope actual y cambiar a modal
            previousScope = shortcutRegistry.getScope() as ShortcutScope;
            shortcutRegistry.pushScope('modal');
        }
    });

    $effect(() => {
        const isOpen = isOpenGetter();

        // Cuando se cierra, restaurar scope anterior
        return () => {
            if (!isOpen && shortcutRegistry.getScope() === 'modal') {
                shortcutRegistry.popScope();
            }
        };
    });
}

/**
 * Activar scope genérico (para grids, ingreso, etc.)
 */
export function useScope(scope: ShortcutScope, contextId?: string): void {
    $effect(() => {
        shortcutRegistry.setScope(scope);
        if (contextId) setActiveContext(contextId);
    });

    onDestroy(() => {
        shortcutRegistry.setScope('all');
        if (contextId) setActiveContext(null);
    });
}
