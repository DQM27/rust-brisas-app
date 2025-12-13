import { invoke } from "@tauri-apps/api/core";
import type { ProveedorSnapshot } from "$lib/types/ingreso-nuevos";

// Definición de tipos para la respuesta del catálogo (puede ser igual o más rica que el snapshot)
export interface ProveedorCatalogResponse extends ProveedorSnapshot {
    id: string; // El snapshot no tenía ID explicito, el modelo de dominio sí
    estado: string;
    puedeIngresar: boolean;
}

export const proveedorService = {
    async create(input: {
        cedula: string;
        nombre: string;
        apellido: string;
        empresa_id: string;
    }): Promise<ProveedorCatalogResponse> {
        return await invoke("create_proveedor", { input });
    },

    async search(query: string): Promise<ProveedorCatalogResponse[]> {
        return await invoke("search_proveedores_catalog", { query });
    },

    async getByCedula(cedula: string): Promise<ProveedorCatalogResponse | null> {
        return await invoke("get_proveedor_by_cedula", { cedula });
    }
};
