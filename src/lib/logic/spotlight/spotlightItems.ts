/**
 * Spotlight Search Logic
 * Funciones para construir, filtrar y ejecutar items del Command Palette
 */

import { openTab, hasTab, activeTabId, getAllTabs } from '$lib/stores/tabs';
import { currentUser } from '$lib/stores/auth';
import { can } from '$lib/logic/permissions';
import { get } from 'svelte/store';
import type { SpotlightItem, SpotlightItemDefinition, SpotlightGroups, SpotlightCategory } from '$lib/types/spotlight';
import { MODULE_DEFINITIONS, ACTION_DEFINITIONS, MODULE_COMPONENT_MAP, TAB_ICON } from '$lib/logic/spotlight/spotlightDefinitions';
import type { Action } from '$lib/logic/permissions';

// Re-export types for convenience
export type { SpotlightItem, SpotlightItemDefinition, SpotlightGroups, SpotlightCategory };

// ============================================
// PERMISSION HELPERS
// ============================================

/**
 * Verifica si el usuario tiene permiso para ver un item
 */
export function hasPermission(definition: SpotlightItemDefinition): boolean {
    const user = get(currentUser);
    if (!user) return false;

    if (definition.permission && !can(user, definition.permission as Action)) {
        return false;
    }

    if (definition.roleId && !definition.roleId.includes(user.roleId)) {
        return false;
    }

    return true;
}

// ============================================
// ACTION EXECUTORS
// ============================================

/**
 * Ejecuta la acción de un módulo
 */
export function executeModuleAction(moduleId: string, onClose: () => void): void {
    const mapping = MODULE_COMPONENT_MAP[moduleId];
    if (mapping) {
        openTab({
            componentKey: mapping.componentKey as any,
            title: mapping.title,
            id: moduleId,
            focusOnOpen: true
        });
    }
    onClose();
}

/**
 * Ejecuta una acción rápida
 */
export function executeQuickAction(actionId: string, onClose: () => void): void {
    switch (actionId) {
        case 'create-contratista':
            openTab({
                componentKey: 'ingreso-list',
                title: 'Ingresos Contratista',
                id: 'ingreso-list',
                focusOnOpen: true,
                data: { openCreateModal: true }
            });
            break;
        case 'create-proveedor':
            openTab({
                componentKey: 'proveedor-ingreso-list',
                title: 'Ingresos Proveedor',
                id: 'proveedor-ingreso-list',
                focusOnOpen: true,
                data: { openCreateModal: true }
            });
            break;
        case 'create-visita':
            openTab({
                componentKey: 'visitas-list',
                title: 'Ingreso Visitas',
                id: 'visitas-list',
                focusOnOpen: true,
                data: { openCreateModal: true }
            });
            break;
        case 'open-settings':
            openTab({
                componentKey: 'settings-view' as any,
                title: 'Configuración',
                id: 'settings',
                focusOnOpen: true
            });
            break;
    }
    onClose();
}

/**
 * Navega a un tab existente
 */
export function navigateToTab(tabId: string, onClose: () => void): void {
    activeTabId.set(tabId);
    onClose();
}

// ============================================
// ITEM BUILDERS
// ============================================

/**
 * Construye la lista completa de items del Spotlight
 */
export function buildSpotlightItems(onClose: () => void): SpotlightItem[] {
    const items: SpotlightItem[] = [];
    const openTabs = getAllTabs();

    // 1. Módulos principales (filtrados por permisos)
    for (const mod of MODULE_DEFINITIONS) {
        if (!hasPermission(mod)) continue;

        const isTabOpen = hasTab(mod.id);

        items.push({
            ...mod,
            isOpen: isTabOpen,
            action: () => executeModuleAction(mod.id, onClose)
        });
    }

    // 2. Acciones rápidas (filtradas por permisos)
    for (const action of ACTION_DEFINITIONS) {
        if (!hasPermission(action)) continue;

        items.push({
            ...action,
            action: () => executeQuickAction(action.id, onClose)
        });
    }

    // 3. Tabs abiertos (para navegación rápida)
    for (const tab of openTabs) {
        // No duplicar si ya está en módulos
        if (MODULE_DEFINITIONS.some((m: SpotlightItemDefinition) => m.id === tab.id)) continue;

        items.push({
            id: `tab-${tab.id}`,
            label: tab.title,
            description: 'Tab abierto',
            icon: TAB_ICON,
            category: 'tab',
            isOpen: true,
            action: () => navigateToTab(tab.id, onClose)
        });
    }

    return items;
}

// ============================================
// FILTER & GROUP HELPERS
// ============================================

/**
 * Filtra items por query de búsqueda
 */
export function filterItems(items: SpotlightItem[], query: string): SpotlightItem[] {
    const q = query.trim().toLowerCase();
    if (!q) return items;

    return items.filter((item) => {
        const labelMatch = item.label.toLowerCase().includes(q);
        const descMatch = item.description?.toLowerCase().includes(q);
        const keywordMatch = item.keywords?.some((k) => k.toLowerCase().includes(q));
        return labelMatch || descMatch || keywordMatch;
    });
}

/**
 * Agrupa items por categoría
 */
export function groupItems(items: SpotlightItem[]): SpotlightGroups {
    return {
        modules: items.filter((i) => i.category === 'module'),
        actions: items.filter((i) => i.category === 'action'),
        tabs: items.filter((i) => i.category === 'tab')
    };
}

/**
 * Aplana los grupos en una lista ordenada
 */
export function flattenGroups(groups: SpotlightGroups): SpotlightItem[] {
    return [...groups.modules, ...groups.actions, ...groups.tabs];
}

/**
 * Obtiene el label de categoría para mostrar
 */
export function getCategoryLabel(category: SpotlightCategory): string {
    switch (category) {
        case 'module':
            return 'Módulos';
        case 'action':
            return 'Acciones Rápidas';
        case 'tab':
            return 'Tabs Abiertos';
    }
}
