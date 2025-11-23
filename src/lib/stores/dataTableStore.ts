import { persisted } from 'svelte-persisted-store';
import { writable, derived } from 'svelte/store';
import type { Readable, Writable } from 'svelte/store';
import type { TablePreferences, ColumnVisibilityConfig } from '$lib/types/dataTable';

/**
 * Cache de stores por storageKey
 */
const storeCache = new Map<string, Writable<TablePreferences>>();

/**
 * Obtiene o crea un store de preferencias para una tabla específica
 * 
 * @param storageKey - Clave única para esta tabla
 * @param defaultVisibility - Visibilidad por defecto de columnas
 * @returns Store persistido de preferencias
 */
export function getTablePreferencesStore(
    storageKey: string,
    defaultVisibility: ColumnVisibilityConfig
): Writable<TablePreferences> {
    // Retornar del cache si ya existe
    if (storeCache.has(storageKey)) {
        return storeCache.get(storageKey)!;
    }

    // Crear nuevo store
    const store = persisted<TablePreferences>(storageKey, {
        columnVisibility: defaultVisibility,
        pageSize: 20,
    });

    // Cachear
    storeCache.set(storageKey, store);

    return store;
}

/**
 * Obtiene un store derivado con el conteo de columnas visibles
 */
export function getVisibleColumnCount(
    preferencesStore: Writable<TablePreferences>
): Readable<number> {
    return derived(preferencesStore, ($prefs) =>
        Object.values($prefs.columnVisibility).filter(Boolean).length
    );
}

/**
 * Alterna la visibilidad de una columna
 */
export function toggleColumnVisibility(
    preferencesStore: Writable<TablePreferences>,
    columnKey: string
): void {
    preferencesStore.update(prefs => ({
        ...prefs,
        columnVisibility: {
            ...prefs.columnVisibility,
            [columnKey]: !prefs.columnVisibility[columnKey]
        }
    }));
}

/**
 * Actualiza la visibilidad de múltiples columnas
 */
export function updateColumnVisibility(
    preferencesStore: Writable<TablePreferences>,
    updates: Partial<ColumnVisibilityConfig>
): void {
    preferencesStore.update(prefs => ({
        ...prefs,
        columnVisibility: {
            ...prefs.columnVisibility,
            ...(updates as ColumnVisibilityConfig)
        }
    }));
}

/**
 * Muestra todas las columnas
 */
export function showAllColumns(
    preferencesStore: Writable<TablePreferences>
): void {
    preferencesStore.update(prefs => {
        const allVisible = Object.keys(prefs.columnVisibility).reduce(
            (acc, key) => ({ ...acc, [key]: true }),
            {} as ColumnVisibilityConfig
        );
        return {
            ...prefs,
            columnVisibility: allVisible
        };
    });
}

/**
 * Oculta todas las columnas excepto las especificadas
 */
export function showOnlyColumns(
    preferencesStore: Writable<TablePreferences>,
    columnsToShow: string[]
): void {
    preferencesStore.update(prefs => {
        const visibility = Object.keys(prefs.columnVisibility).reduce(
            (acc, key) => ({ ...acc, [key]: columnsToShow.includes(key) }),
            {} as ColumnVisibilityConfig
        );
        return {
            ...prefs,
            columnVisibility: visibility
        };
    });
}

/**
 * Actualiza el tamaño de página
 */
export function updatePageSize(
    preferencesStore: Writable<TablePreferences>,
    pageSize: number
): void {
    preferencesStore.update(prefs => ({
        ...prefs,
        pageSize
    }));
}

/**
 * Resetea las preferencias a los valores por defecto
 */
export function resetTablePreferences(
    preferencesStore: Writable<TablePreferences>,
    defaultVisibility: ColumnVisibilityConfig
): void {
    preferencesStore.set({
        columnVisibility: defaultVisibility,
        pageSize: 20,
    });
}