// src/lib/stores/contratistaStore.svelte.ts
import {
    fetchAllContratistas,
    fetchActiveContratistas,
    getArchivedContratistas
} from '$lib/logic/contratista/contratistaService';
import type {
    ContratistaResponse,
    ContratistaListResponse
} from '$lib/types/contratista';

class ContratistaStore {
    contratistas = $state<ContratistaResponse[]>([]);
    archived = $state<ContratistaResponse[]>([]);
    loading = $state(false);
    error = $state('');
    initialized = $state(false);

    // Initial load
    async init() {
        if (this.initialized) return;
        await this.refresh();
        this.initialized = true;
    }

    // Refresh active/all list
    async refresh(includeArchived = false) {
        this.loading = true;
        this.error = '';
        console.log('[ContratistaStore] Refreshing...');

        try {
            // Main list
            const result = await fetchAllContratistas();
            if (result.ok) {
                // Normalize response
                const rawList = 'contratistas' in result.data
                    ? (result.data as ContratistaListResponse).contratistas
                    : (result.data as ContratistaResponse[]);

                this.contratistas = rawList;
            } else {
                this.error = result.error;
            }

            // Archived list if requested
            if (includeArchived) {
                const archivedRes = await getArchivedContratistas();
                if (archivedRes.ok) {
                    this.archived = archivedRes.data;
                }
            }

        } catch (e: any) {
            console.error('[ContratistaStore] Error:', e);
            this.error = e.message || 'Error desconocido';
        } finally {
            this.loading = false;
        }
    }

    // Helpers to update local state without full reload
    add(contratista: ContratistaResponse) {
        this.contratistas = [contratista, ...this.contratistas];
    }

    update(id: string, partial: Partial<ContratistaResponse>) {
        const index = this.contratistas.findIndex(c => c.id === id);
        if (index !== -1) {
            this.contratistas[index] = { ...this.contratistas[index], ...partial };
        }
    }

    remove(id: string) {
        this.contratistas = this.contratistas.filter(c => c.id !== id);
    }

    // Getters helpers
    getById(id: string) {
        return this.contratistas.find(c => c.id === id);
    }
}

export const contratistaStore = new ContratistaStore();
