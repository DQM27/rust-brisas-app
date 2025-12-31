// src/lib/stores/empresaStore.svelte.ts
import { fetchEmpresasActivas } from '$lib/api/empresa';
import type { EmpresaResponse } from '$lib/types/empresa';

class EmpresaStore {
    empresas = $state<EmpresaResponse[]>([]);
    loading = $state(false);
    initialized = $state(false);

    async init() {
        if (this.initialized) return;
        await this.refresh();
        this.initialized = true;
    }

    async refresh() {
        this.loading = true;
        try {
            const data = await fetchEmpresasActivas();
            this.empresas = data;
        } catch (error) {
            console.error("Error loading empresas:", error);
        } finally {
            this.loading = false;
        }
    }

    add(empresa: EmpresaResponse) {
        this.empresas = [...this.empresas, empresa];
    }
}

export const empresaStore = new EmpresaStore();
