// ============================================
// src/lib/stores/gridStateStore.svelte.ts
// ============================================
// Grid state management + Column persistence using Svelte 5 runes

import type { GridApi, ColumnState } from "@ag-grid-community/core";
import { browser } from "$app/environment";

// ============================================
// STORAGE HELPERS
// ============================================

async function loadColumnState(key: string): Promise<ColumnState[] | null> {
    if (!browser) return null;

    // Try Tauri Store first
    try {
        const { getSetting } = await import('$lib/services/storeService');
        const stored = await getSetting<ColumnState[] | null>(key, null);
        if (stored) return stored;
    } catch {
        // Fallback below
    }

    // Fallback to localStorage
    const stored = localStorage.getItem(key);
    if (stored) {
        try {
            return JSON.parse(stored);
        } catch {
            return null;
        }
    }
    return null;
}

async function saveColumnState(key: string, state: ColumnState[]): Promise<void> {
    if (!browser) return;

    // Sync save to localStorage
    localStorage.setItem(key, JSON.stringify(state));

    // Async save to Tauri Store
    try {
        const { setSetting } = await import('$lib/services/storeService');
        await setSetting(key, state);
    } catch {
        // localStorage already saved
    }
}

// ============================================
// STORE CLASS
// ============================================

class GridStateStore {
    activeGridId = $state<string | null>(null);
    activeGridApi = $state<GridApi | null>(null);

    private restoring = new Map<string, boolean>();
    private canSave = new Map<string, boolean>();

    // ============================================
    // GRID REGISTRATION
    // ============================================

    registerGrid(id: string, api: GridApi) {
        this.activeGridId = id;
        this.activeGridApi = api;
    }

    unregisterGrid(id: string) {
        if (this.activeGridId === id) {
            this.activeGridId = null;
            this.activeGridApi = null;
        }
        this.restoring.delete(id);
        this.canSave.delete(id);
    }

    // ============================================
    // COLUMN STATE PERSISTENCE
    // ============================================

    private getKey(persistenceKey: string): string {
        return `ag-grid-state-${persistenceKey}`;
    }

    async restoreColumnState(api: GridApi, persistenceKey: string): Promise<boolean> {
        if (!api || this.restoring.get(persistenceKey)) return false;

        const state = await loadColumnState(this.getKey(persistenceKey));
        if (!state) return false;

        try {
            this.restoring.set(persistenceKey, true);
            api.applyColumnState({
                state,
                applyOrder: true,
            });

            // Allow saving after restore settles
            setTimeout(() => {
                this.restoring.set(persistenceKey, false);
                this.canSave.set(persistenceKey, true);
            }, 500);

            return true;
        } catch (e) {
            console.warn("Error restoring grid state:", e);
            this.restoring.set(persistenceKey, false);
            return false;
        }
    }

    saveColumnState(api: GridApi, persistenceKey: string): void {
        if (!api || !persistenceKey) return;
        if (this.restoring.get(persistenceKey)) return;
        if (!this.canSave.get(persistenceKey)) return;

        try {
            const state = api.getColumnState();
            saveColumnState(this.getKey(persistenceKey), state);
        } catch (e) {
            console.warn("Error saving grid state:", e);
        }
    }

    enableSaving(persistenceKey: string): void {
        this.canSave.set(persistenceKey, true);
    }
}

// ============================================
// SINGLETON EXPORT
// ============================================

export const gridState = new GridStateStore();
