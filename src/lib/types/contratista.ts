// src/lib/types/contratista.ts

// ======================================================
// TIPOS CENTRALES DEL DOMINIO
// (Alineados con cómo Rust devuelve datos - camelCase)
// ======================================================

// ------------- Empresa ----------------
export interface Empresa {
  id: string;
  nombre: string;
  // Nota: en Rust es `is_active`, en TS usamos camelCase para conveniencia
  isActive: boolean;
  fechaRegistro?: string; // opcional, normalmente TEXT en SQLite
}

// ------------- Contratista (modelo base, puede ser usado en formularios) ------------
export interface Contratista {
  id: string;
  cedula: string;
  nombre: string;
  apellido: string;

  empresaId: string;
  empresaNombre?: string; // opcional si el backend lo añade

  fechaRegistro?: string;
  fechaVencimientoPraind: string; // string (YYYY-MM-DD)
  isActive: boolean;
}

// ======================================================
// RESPONSES (exactos al DTO de salida de Rust)
// ======================================================

export type EstadoContratista = "activo" | "inactivo" | "suspendido";

/**
 * ContratistaResponse: refleja exactamente ContratistaResponse del backend (serde rename camelCase)
 */
export interface ContratistaResponse {
  id: string;
  cedula: string;
  nombre: string;
  apellido: string;
  nombreCompleto: string;

  empresaId: string;
  empresaNombre: string;

  fechaVencimientoPraind: string; // YYYY-MM-DD

  estado: EstadoContratista;

  puedeIngresar: boolean;
  praindVencido: boolean;
  diasHastaVencimiento: number;
  requiereAtencion: boolean;

  createdAt: string;
  updatedAt: string;
}

/**
 * Respuesta cuando el backend devuelve el listado (ContratistaListResponse en Rust)
 */
export interface ContratistaListResponse {
  contratistas: ContratistaResponse[];
  total: number;
  activos: number;
  conPraindVencido: number;
  requierenAtencion: number;
}

// ======================================================
// TIPOS PARA CREAR / ACTUALIZAR CONTRATISTAS
// ======================================================

export interface CreateContratistaInput {
  cedula: string;
  nombre: string;
  apellido: string;
  empresaId: string;
  fechaVencimientoPraind: string; // YYYY-MM-DD desde <input type="date" />
}

export interface UpdateContratistaInput {
  id: string;

  cedula?: string;
  nombre?: string;
  apellido?: string;
  empresaId?: string;
  fechaVencimientoPraind?: string;

  // Si tu backend soporta cambiar estado/activo, mantén esto opcional
  isActive?: boolean;
}

// ======================================================
// TIPOS EXTRAS PARA EMPRESAS (Opcionales)
// ======================================================

// Crear empresa
export interface CreateEmpresaInput {
  nombre: string;
}

// Respuesta de listar empresas (coincide con EmpresaListResponse del backend)
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
