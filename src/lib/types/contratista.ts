// src/lib/types/contratista.ts

import type { TipoVehiculo } from './vehiculo';

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
    estaBloqueado: boolean;
    diasHastaVencimiento: number;
    requiereAtencion: boolean;
    vehiculoTipo?: string;
    vehiculoPlaca?: string;
    createdAt: string;
    updatedAt: string;
}

export interface CreateContratistaInput {
    cedula: string;
    nombre: string;
    apellido: string;
    empresaId: string;
    fechaVencimientoPraind: string;
    // Campos de vehículo (opcionales, dependen del toggle)
    tieneVehiculo: boolean;
    tipoVehiculo?: TipoVehiculo;
    placa?: string;
    marca?: string;
    modelo?: string;
    color?: string;
}

export interface UpdateContratistaInput {
    id: string;
    cedula?: string;
    nombre?: string;
    apellido?: string;
    empresaId?: string;
    fechaVencimientoPraind?: string;
    isActive?: boolean;
    // Campos para actualizar vehículo
    tieneVehiculo?: boolean;
    tipoVehiculo?: string;
    placa?: string;
    marca?: string;
    modelo?: string;
    color?: string;
}

export interface ContratistaListResponse {
    contratistas: ContratistaResponse[];
    total: number;
    activos: number;
    conPraindVencido: number;
    requierenAtencion: number;
}