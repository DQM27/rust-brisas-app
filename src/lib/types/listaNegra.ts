// src/lib/types/listaNegra.ts

// ==========================================
// MODELO DE DOMINIO
// ==========================================

export interface ListaNegra {
  id: string;
  contratistaId?: string;
  cedula: string;
  nombre: string;
  apellido: string;
  motivoBloqueo: string;
  fechaInicioBloqueo: string;
  fechaFinBloqueo?: string;
  bloqueadoPor: string;
  observaciones?: string;
  isActive: boolean;
  createdAt: string;
  updatedAt: string;
}

// ==========================================
// DTOs DE ENTRADA
// ==========================================

export interface AddToListaNegraInput {
  contratistaId?: string;
  cedula?: string;
  nombre?: string;
  apellido?: string;
  motivoBloqueo: string;
  fechaFinBloqueo?: string;
  bloqueadoPor: string;
  observaciones?: string;
}

export interface UpdateListaNegraInput {
  motivoBloqueo?: string;
  fechaFinBloqueo?: string;
  observaciones?: string;
}

// ==========================================
// DTOs DE SALIDA
// ==========================================

export interface ListaNegraResponse {
  id: string;
  contratistaId?: string;
  cedula: string;
  nombre: string;
  apellido: string;
  nombreCompleto: string;
  motivoBloqueo: string;
  fechaInicioBloqueo: string;
  fechaFinBloqueo?: string;
  bloqueadoPor: string;
  observaciones?: string;
  isActive: boolean;
  esBloqueoPermanente: boolean;
  diasTranscurridos: number;
  empresaNombre?: string;
  createdAt: string;
  updatedAt: string;
}

export interface ListaNegraListResponse {
  bloqueados: ListaNegraResponse[];
  total: number;
  activos: number;
  permanentes: number;
  temporales: number;
}

export interface BlockCheckResponse {
  isBlocked: boolean;
  motivo?: string;
  bloqueadoDesde?: string;
  bloqueadoHasta?: string;
  bloqueadoPor?: string;
}