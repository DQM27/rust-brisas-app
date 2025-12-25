import type { GridApi } from "@ag-grid-community/core";

class GridStateStore {
    activeGridId = $state<string | null>(null);
    activeGridApi = $state<GridApi | null>(null);

    registerGrid(id: string, api: GridApi) {
        this.activeGridId = id;
        this.activeGridApi = api;
    }

    unregisterGrid(id: string) {
        if (this.activeGridId === id) {
            this.activeGridId = null;
            this.activeGridApi = null;
        }
    }
}

export const gridState = new GridStateStore();
