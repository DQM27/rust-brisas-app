// src/lib/types/contratista.ts (LIMPIO)

// ======================================================
// TIPOS CENTRALES DEL DOMINIO CONTRATISTA
// ======================================================

export type EstadoContratista = "activo" | "inactivo" | "suspendido";

export interface ContratistaBase {
    id?: string;
    cedula: string;
    nombre: string;
    apellido: string;
    empresaId: string;
    empresaNombre?: string;
    fechaVencimientoPraind: string; // YYYY-MM-DD
    isActive?: boolean;
}

export interface ContratistaResponse {
    id: string;
    cedula: string;
    nombre: string;
    apellido: string;
    nombreCompleto: string;
    empresaId: string;
    empresaNombre: string;
    fechaVencimientoPraind: string;
    estado: EstadoContratista;
    puedeIngresar: boolean;
    praindVencido: boolean;
    diasHastaVencimiento: number;
    requiereAtencion: boolean;
    createdAt: string;
    updatedAt: string;
}

export interface CreateContratistaInput {
    cedula: string;
    nombre: string;
    apellido: string;
    empresaId: string;
    fechaVencimientoPraind: string;
}

export interface UpdateContratistaInput {
    id: string;
    cedula?: string;
    nombre?: string;
    apellido?: string;
    empresaId?: string;
    fechaVencimientoPraind?: string;
    isActive?: boolean;
}

export interface ContratistaListResponse {
    contratistas: ContratistaResponse[];
    total: number;
    activos: number;
    conPraindVencido: number;
    requierenAtencion: number;
}
// Las interfaces de Empresa han sido movidas a /types/empresa.ts