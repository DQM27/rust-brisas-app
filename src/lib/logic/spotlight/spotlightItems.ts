/**
 * Spotlight Search Logic
 * Funciones para construir, filtrar y ejecutar items del Command Palette
 */

import { openTab, hasTab, activeTabId, getAllTabs } from '$lib/stores/tabs';
import { currentUser, logout } from '$lib/stores/auth';
import { themeStore, toggleTheme } from '$lib/stores/themeStore';
import { can } from '$lib/logic/permissions';
import { get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { spotlightSettings, recentSpotlightItems } from '$lib/stores/spotlightStore';
import { showShortcutsHelp, personaQuickView, quickSwitchTarget } from '$lib/stores/ui';

import type { SpotlightItem, SpotlightItemDefinition, SpotlightGroups, SpotlightCategory, SpotlightSubCategory } from '$lib/types/spotlight';
import { MODULE_DEFINITIONS, ACTION_DEFINITIONS, MODULE_COMPONENT_MAP, TAB_ICON } from '$lib/logic/spotlight/spotlightDefinitions';
import { User, Building2, ShieldCheck, Mail, IdCard, Search } from 'lucide-svelte';
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
        recentSpotlightItems.add(moduleId);
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
    if (actionId !== 'toggle-theme') {
        recentSpotlightItems.add(actionId);
    }

    // --- ACCIONES ESPECIALES DE SISTEMA ---
    if (actionId === 'toggle-theme') {
        toggleTheme();
        // No cerramos el buscador para que vea el cambio? 
        // O lo cerramos? Mejor cerrarlo para evitar parpadeo si es pesado.
        onClose();
        return;

    }

    if (actionId === 'show-shortcuts') {
        showShortcutsHelp.set(true);
        onClose();
        return;
    }

    if (actionId === 'logout') {
        import('@tauri-apps/plugin-dialog').then(async ({ ask }) => {
            const confirmed = await ask('¿Cerrar sesión ahora?', {
                title: 'Cerrar Sesión',
                kind: 'info'
            });
            if (confirmed) {
                logout();
                onClose();
            }
        }).catch(() => {
            // Fallback
            if (confirm('¿Cerrar sesión ahora?')) {
                logout();
                onClose();
            }
        });
        return;
    }

    if (actionId === 'app-reload') {
        window.location.reload();
        return;
    }

    // Verificar si la acción tiene un mapeo de componente (para configuraciones)
    if (MODULE_COMPONENT_MAP[actionId]) {
        executeModuleAction(actionId, onClose);
        return;
    }

    switch (actionId) {
        case 'create-contratista':
            openTab({
                componentKey: 'ingreso-list',
                title: 'Ingresos Contratistas',
                id: 'ingreso-list',
                focusOnOpen: true,
                data: { openCreateModal: Date.now() }
            });
            break;
        case 'create-proveedor':
            openTab({
                componentKey: 'proveedor-ingreso-list',
                title: 'Ingresos Proveedores',
                id: 'proveedor-ingreso-list',
                focusOnOpen: true,
                data: { openCreateModal: Date.now() }
            });
            break;
        case 'master-contratista':
            openTab({
                componentKey: 'contratista-list',
                title: 'Lista Contratistas',
                id: 'contratista-list',
                focusOnOpen: true,
                data: { openCreateModal: Date.now() }
            });
            break;
        case 'master-proveedor':
            openTab({
                componentKey: 'proveedor-list',
                title: 'Lista Proveedores',
                id: 'proveedor-list',
                focusOnOpen: true,
                data: { openCreateModal: Date.now() }
            });
            break;
        case 'master-visitante':
            openTab({
                componentKey: 'visitante-list',
                title: 'Lista Visitantes',
                id: 'visitante-list',
                focusOnOpen: true,
                data: { openCreateModal: Date.now() }
            });
            break;
        case 'create-user':
            openTab({
                componentKey: 'user-list',
                title: 'Lista Usuarios',
                id: 'users-list',
                focusOnOpen: true,
                data: { openCreateModal: Date.now() }
            });
            break;
        case 'create-visita':
            openTab({
                componentKey: 'visitas-list',
                title: 'Ingresos Visitas',
                id: 'visitas-list',
                focusOnOpen: true,
                data: { openCreateModal: Date.now() }
            });
            break;
        case 'create-blacklist':
            openTab({
                componentKey: 'lista-negra-list',
                title: 'Lista Negra',
                id: 'lista-negra-list',
                focusOnOpen: true,
                data: { openCreateModal: Date.now() }
            });
            break;
        case 'create-gafete':
            openTab({
                componentKey: 'gafete-list',
                title: 'Lista Gafetes',
                id: 'gafete-list',
                focusOnOpen: true,
                data: { openCreateModal: Date.now() }
            });
            break;
        case 'create-gafete-batch':
            openTab({
                componentKey: 'gafete-list',
                title: 'Lista Gafetes',
                id: 'gafete-list',
                focusOnOpen: true,
                data: { openCreateBatchModal: Date.now() }
            });
            break;
        case 'action-reindex':
            // TODO: Implementar llamada global a reindexado si es accesible
            console.log('Solicitud de reindexado desde Spotlight');
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
export function buildSpotlightItems(tabs: any[], settings: any, onClose: () => void): SpotlightItem[] {
    const items: SpotlightItem[] = [];

    // Helper para verificar si un tab está abierto
    const hasTabId = (id: string) => tabs.some(t => t.id === id);

    // 1. Módulos principales (filtrados por permisos y configuración)
    if (settings.showModules) {
        for (const mod of MODULE_DEFINITIONS) {
            if (!hasPermission(mod)) continue;

            const isTabOpen = hasTabId(mod.id);

            items.push({
                ...mod,
                isOpen: isTabOpen,
                action: () => executeModuleAction(mod.id, onClose)
            });
        }
    }

    // 2. Acciones rápidas (filtradas por permisos y configuración)
    if (settings.showActions) {
        for (const action of ACTION_DEFINITIONS) {
            if (!hasPermission(action)) continue;

            items.push({
                ...action,
                action: () => executeQuickAction(action.id, onClose)
            });
        }
    }

    // 3. Recientes
    if (settings.showRecent) {
        const recentIds = get(recentSpotlightItems);
        for (const id of recentIds) {
            const def = [...MODULE_DEFINITIONS, ...ACTION_DEFINITIONS].find(d => d.id === id);
            if (def && hasPermission(def)) {
                // Respetar visibilidad de la categoría original
                if (def.category === 'module' && !settings.showModules) continue;
                if (def.category === 'action' && !settings.showActions) continue;

                items.push({
                    ...def,
                    category: 'recent',
                    action: () => executeQuickAction(def.id, onClose)
                });
            }
        }
    }

    // 4. Tabs abiertos (para navegación rápida)
    for (const tab of tabs) {
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

    // Trigger para mostrar todo (estilo VS Code / Raycast)
    if (q === '>' || q === 'todo' || q === 'all') return items;

    return items.filter((item) => {
        // Excluir recientes de los resultados de búsqueda para evitar duplicados
        if (item.category === 'recent') return false;

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
        recent: items.filter((i) => i.category === 'recent'),
        modules: items.filter((i) => i.category === 'module'),
        actions: items.filter((i) => i.category === 'action'),
        tabs: items.filter((i) => i.category === 'tab'),
        data: items.filter((i) => i.category === 'data')
    };
}

/**
 * Aplana los grupos en una lista ordenada
 */
export function flattenGroups(groups: SpotlightGroups): SpotlightItem[] {
    return [...groups.recent, ...groups.modules, ...groups.actions, ...groups.tabs, ...groups.data];
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
        case 'data':
            return 'Registros del Sistema';
        case 'recent':
            return 'Recientes';
    }
}

// ============================================
// TANTIVY INTEGRATION
// ============================================

/**
 * Realiza una búsqueda profunda en el índice de Tantivy
 */
export async function searchDeep(query: string, settings: any, onClose: () => void): Promise<SpotlightItem[]> {
    if (!settings.enableTantivySearch || !query || query.length < 2) return [];

    try {
        const results = await invoke<any[]>('search_global', {
            query,
            limit: settings.tantivyLimit
        });

        return results.map(res => {
            let icon = User;
            let subCategory: SpotlightSubCategory = 'link';
            let label = res.nombreCompleto || res.id;
            let description = res.tipo.toUpperCase();

            // Lógica de iconos y labels según tipo
            if (res.tipo === 'contratista') {
                icon = IdCard;
                subCategory = 'master';
                if (res.empresaNombre) {
                    description += ` • ${res.empresaNombre}`;
                }
            } else if (res.tipo === 'proveedor') {
                icon = Building2;
                subCategory = 'master';
            } else if (res.tipo === 'user') {
                icon = ShieldCheck;
                subCategory = 'settings';
            }

            if (res.cedula) {
                description = `${res.cedula} • ${description}`;
            }

            return {
                id: ` tantivy-${res.id}`,
                label,
                description,
                icon,
                category: 'data',
                subCategory,
                action: () => {
                    // Acción: Disparar cambio rápido de usuario
                    quickSwitchTarget.set({
                        id: res.id,
                        email: res.email, // Importante para el login
                        nombreCompleto: res.nombreCompleto || res.id,
                        tipo: res.tipo
                    });
                    onClose();
                }

            };
        });
    } catch (e) {
        console.error('Error en búsqueda profunda:', e);
        return [];
    }
}
