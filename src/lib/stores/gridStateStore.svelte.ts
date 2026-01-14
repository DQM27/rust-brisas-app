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
    private restorePromises = new Map<string, Promise<boolean>>();

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
        // NOTE: Do NOT clear canSave, restoring, or restorePromises here!
        // These are keyed by persistenceKey, not gridId, and should persist
        // across component mount/unmount cycles to maintain state.
    }

    /**
     * Call this at the START of onGridReady to block effects during restore
     */
    prepareForRestore(persistenceKey: string): void {
        this.canSave.set(persistenceKey, false);
        this.restoring.set(persistenceKey, true);
    }

    // ============================================
    // COLUMN STATE PERSISTENCE
    // ============================================

    private getKey(persistenceKey: string): string {
        return `ag-grid-state-${persistenceKey}`;
    }

    async restoreColumnState(api: GridApi, persistenceKey: string): Promise<boolean> {
        if (!api) return false;

        // If a restore is already in progress, return the existing promise
        if (this.restorePromises.has(persistenceKey)) {
            console.log(`[GridState] Joining existing restore for ${persistenceKey}`);
            return this.restorePromises.get(persistenceKey)!;
        }

        const restorePromise = (async () => {
            // Set restoring flag IMMEDIATELY to block any saves during async load
            this.restoring.set(persistenceKey, true);
            console.log(`[GridState] Starting restore for ${persistenceKey} (Blocking saves)`);

            try {
                const state = await loadColumnState(this.getKey(persistenceKey));

                if (!state) {
                    console.log(`[GridState] No saved state found for ${persistenceKey}, unblocking saves`);
                    this.restoring.set(persistenceKey, false);
                    // Enable saving immediately for new grids
                    this.canSave.set(persistenceKey, true);
                    return false;
                }

                console.log(`[GridState] Applying state for ${persistenceKey}`, state);
                api.applyColumnState({
                    state,
                    applyOrder: true,
                });

                // Allow saving after restore settles
                setTimeout(() => {
                    this.restoring.set(persistenceKey, false);
                    this.canSave.set(persistenceKey, true);
                    console.log(`[GridState] Restore complete for ${persistenceKey}, saves unblocked`);
                }, 500);

                return true;
            } catch (e) {
                console.warn("Error restoring grid state:", e);
                this.restoring.set(persistenceKey, false);
                return false;
            } finally {
                // Clear the promise so future calls can start a new restore if needed
                this.restorePromises.delete(persistenceKey);
            }
        })();

        this.restorePromises.set(persistenceKey, restorePromise);
        return restorePromise;
    }

    saveColumnState(api: GridApi, persistenceKey: string): void {
        if (!api || !persistenceKey) return;

        // Don't save if restoring OR if there is an active restore promise
        if (this.restoring.get(persistenceKey) || this.restorePromises.has(persistenceKey)) {
            console.log(`[GridState] Skipping save for ${persistenceKey} (Restoring)`);
            return;
        }
        if (!this.canSave.get(persistenceKey)) {
            console.log(`[GridState] Skipping save for ${persistenceKey} (Not allowed yet)`);
            return;
        }

        try {
            console.log(`[GridState] Saving state for ${persistenceKey}`);
            const state = api.getColumnState();
            saveColumnState(this.getKey(persistenceKey), state);
        } catch (e) {
            console.warn("Error saving grid state:", e);
        }
    }

    enableSaving(persistenceKey: string): void {
        this.canSave.set(persistenceKey, true);
    }

    /**
     * Check if the grid is ready for modifications (after initial restore)
     */
    isReady(persistenceKey: string): boolean {
        return this.canSave.get(persistenceKey) ?? false;
    }
}

// ============================================
// SINGLETON EXPORT
// ============================================

export const gridState = new GridStateStore();
