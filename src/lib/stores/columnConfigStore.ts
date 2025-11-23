import { persisted } from 'svelte-persisted-store';
import { derived } from 'svelte/store';
import type { Readable } from 'svelte/store';

/**
 * Configuración de visibilidad de columnas para Lista Negra
 */
export interface ColumnVisibility {
    cedula: boolean;
    nombreCompleto: boolean;
    empresaNombre: boolean;
    motivoBloqueo: boolean;
    isActive: boolean;
    esBloqueoPermanente: boolean;
    bloqueadoPor: boolean;
    fechaInicioBloqueo: boolean;
    diasTranscurridos: boolean;
    actions: boolean;
}

/**
 * Configuración por defecto de columnas visibles
 */
const DEFAULT_COLUMN_VISIBILITY: ColumnVisibility = {
    cedula: true,
    nombreCompleto: true,
    empresaNombre: true,
    motivoBloqueo: true,
    isActive: true,
    esBloqueoPermanente: true,
    bloqueadoPor: true,
    fechaInicioBloqueo: true,
    diasTranscurridos: true,
    actions: true,
};

/**
 * Store persistido con la configuración de columnas
 */
export const columnVisibilityStore = persisted<ColumnVisibility>(
    'lista-negra-column-visibility',
    DEFAULT_COLUMN_VISIBILITY
);

/**
 * Labels legibles para cada columna
 */
export const COLUMN_LABELS: Record<keyof ColumnVisibility, string> = {
    cedula: 'Cédula',
    nombreCompleto: 'Nombre Completo',
    empresaNombre: 'Empresa',
    motivoBloqueo: 'Motivo',
    isActive: 'Estado',
    esBloqueoPermanente: 'Tipo',
    bloqueadoPor: 'Bloqueado por',
    fechaInicioBloqueo: 'Fecha de bloqueo',
    diasTranscurridos: 'Días transcurridos',
    actions: 'Acciones',
};

/**
 * Store derivado que indica cuántas columnas están visibles
 */
export const visibleColumnCount: Readable<number> = derived(
    columnVisibilityStore,
    ($config) => Object.values($config).filter(Boolean).length
);

/**
 * Alterna la visibilidad de una columna
 * 
 * @param columnKey - Clave de la columna a alternar
 */
export function toggleColumnVisibility(columnKey: keyof ColumnVisibility): void {
    columnVisibilityStore.update(config => ({
        ...config,
        [columnKey]: !config[columnKey]
    }));
}

/**
 * Muestra una columna específica
 * 
 * @param columnKey - Clave de la columna a mostrar
 */
export function showColumn(columnKey: keyof ColumnVisibility): void {
    columnVisibilityStore.update(config => ({
        ...config,
        [columnKey]: true
    }));
}

/**
 * Oculta una columna específica
 * 
 * @param columnKey - Clave de la columna a ocultar
 */
export function hideColumn(columnKey: keyof ColumnVisibility): void {
    columnVisibilityStore.update(config => ({
        ...config,
        [columnKey]: false
    }));
}

/**
 * Muestra todas las columnas
 */
export function showAllColumns(): void {
    columnVisibilityStore.set(DEFAULT_COLUMN_VISIBILITY);
}

/**
 * Oculta todas las columnas excepto las esenciales
 */
export function showMinimalColumns(): void {
    columnVisibilityStore.set({
        cedula: true,
        nombreCompleto: true,
        empresaNombre: false,
        motivoBloqueo: false,
        isActive: true,
        esBloqueoPermanente: false,
        bloqueadoPor: false,
        fechaInicioBloqueo: false,
        diasTranscurridos: false,
        actions: true,
    });
}

/**
 * Restaura la configuración por defecto
 */
export function resetColumnVisibility(): void {
    columnVisibilityStore.set(DEFAULT_COLUMN_VISIBILITY);
}

/**
 * Actualiza múltiples columnas a la vez
 * 
 * @param updates - Objeto con las columnas a actualizar
 */
export function updateColumnVisibility(updates: Partial<ColumnVisibility>): void {
    columnVisibilityStore.update(config => ({
        ...config,
        ...updates
    }));
}

/**
 * Verifica si una columna está visible
 * 
 * @param columnKey - Clave de la columna
 * @param config - Configuración actual
 * @returns true si la columna está visible
 */
export function isColumnVisible(
    columnKey: keyof ColumnVisibility,
    config: ColumnVisibility
): boolean {
    return config[columnKey];
}

/**
 * Obtiene lista de columnas visibles
 * 
 * @param config - Configuración actual
 * @returns Array con las claves de columnas visibles
 */
export function getVisibleColumns(config: ColumnVisibility): (keyof ColumnVisibility)[] {
    return Object.entries(config)
        .filter(([_, visible]) => visible)
        .map(([key]) => key as keyof ColumnVisibility);
}

/**
 * Obtiene lista de columnas ocultas
 * 
 * @param config - Configuración actual
 * @returns Array con las claves de columnas ocultas
 */
export function getHiddenColumns(config: ColumnVisibility): (keyof ColumnVisibility)[] {
    return Object.entries(config)
        .filter(([_, visible]) => !visible)
        .map(([key]) => key as keyof ColumnVisibility);
}

/**
 * Configuración adicional de la tabla
 */
export interface TablePreferences {
    columnVisibility: ColumnVisibility;
    pageSize: number;
    quickFilterText: string;
}

/**
 * Store persistido con todas las preferencias de la tabla
 */
export const tablePreferencesStore = persisted<TablePreferences>(
    'lista-negra-table-preferences',
    {
        columnVisibility: DEFAULT_COLUMN_VISIBILITY,
        pageSize: 20,
        quickFilterText: '',
    }
);

/**
 * Actualiza el tamaño de página
 */
export function updatePageSize(pageSize: number): void {
    tablePreferencesStore.update(prefs => ({
        ...prefs,
        pageSize
    }));
}

/**
 * Actualiza el texto del filtro rápido
 */
export function updateQuickFilter(text: string): void {
    tablePreferencesStore.update(prefs => ({
        ...prefs,
        quickFilterText: text
    }));
}