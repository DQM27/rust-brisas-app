// src/lib/types/contratista.ts

// ======================================================
// TIPOS CENTRALES DEL DOMINIO
// ======================================================

export type EstadoContratista = "activo" | "inactivo" | "suspendido";

// Modelo base (útil para formularios)
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

// DTO que devuelve el backend (enriquecido)
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

// Create / Update inputs (lo que envía el frontend)
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

// List response
export interface ContratistaListResponse {
  contratistas: ContratistaResponse[];
  total: number;
  activos: number;
  conPraindVencido: number;
  requierenAtencion: number;
}

// Empresa helpers
export interface EmpresaResponse {
  id: string;
  nombre: string;
  isActive: boolean;
  totalContratistas: number;
  createdAt: string;
  updatedAt: string;
}

export interface EmpresaListResponse {
  empresas: EmpresaResponse[];
  total: number;
  activas: number;
}
