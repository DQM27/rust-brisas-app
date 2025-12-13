import { invoke } from "@tauri-apps/api/core";

// Types matching backend models/proveedor.rs
export interface CreateProveedorInput {
    cedula: string;
    nombre: string;
    segundo_nombre?: string;
    apellido: string;
    segundo_apellido?: string;
    empresa_id: string;
    // Vehicle (optional)
    tiene_vehiculo?: boolean;
    tipo_vehiculo?: string;
    placa?: string;
    marca?: string;
    modelo?: string;
    color?: string;
}

export interface ProveedorResponse {
    id: string;
    cedula: string;
    nombre: string;
    segundoNombre?: string;
    apellido: string;
    segundoApellido?: string;
    empresaId: string;
    empresaNombre: string;
    estado: string;
    puedeIngresar: boolean;
    // Vehicle
    vehiculoTipo?: string;
    vehiculoPlaca?: string;
    vehiculoMarca?: string;
    vehiculoModelo?: string;
    vehiculoColor?: string;
    // Timestamps
    createdAt: string;
    updatedAt: string;
}

export const proveedorService = {
    async create(input: CreateProveedorInput): Promise<ProveedorResponse> {
        return await invoke("create_proveedor", { input });
    },

    async search(query: string): Promise<ProveedorResponse[]> {
        return await invoke("search_proveedores_catalog", { query });
    },

    async getByCedula(cedula: string): Promise<ProveedorResponse | null> {
        return await invoke("get_proveedor_by_cedula", { cedula });
    },

    async getAll(): Promise<ProveedorResponse[]> {
        // Using search with empty string to get all
        return await invoke("search_proveedores_catalog", { query: "" });
    }
};
