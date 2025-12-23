// src/lib/types/listaNegra.ts

// ==========================================
// ENUMS
// ==========================================

export type NivelSeveridad = 'ALTO' | 'MEDIO' | 'BAJO';

// ==========================================
// MODELO DE DOMINIO
// ==========================================

export interface ListaNegra {
  id: string;
  cedula: string;
  nombre: string;
  segundoNombre?: string;
  apellido: string;
  segundoApellido?: string;
  empresaId?: string;
  empresaNombre?: string;
  nivelSeveridad: NivelSeveridad;
  motivoBloqueo: string;
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
  cedula: string;
  nombre: string;
  segundoNombre?: string;
  apellido: string;
  segundoApellido?: string;
  empresaId?: string;
  empresaNombre?: string;
  nivelSeveridad: NivelSeveridad;
  motivoBloqueo: string;
  bloqueadoPor: string;
  observaciones?: string;
}

export interface UpdateListaNegraInput {
  nivelSeveridad?: NivelSeveridad;
  motivoBloqueo?: string;
  observaciones?: string;
}

// ==========================================
// DTOs DE SALIDA
// ==========================================

export interface ListaNegraResponse {
  id: string;
  cedula: string;
  nombre: string;
  segundoNombre?: string;
  apellido: string;
  segundoApellido?: string;
  nombreCompleto: string;
  empresaId?: string;
  empresaNombre?: string;
  nivelSeveridad: NivelSeveridad;
  motivoBloqueo: string;
  bloqueadoPor: string;
  observaciones?: string;
  isActive: boolean;
  bloqueadoDesde: string;
  createdAt: string;
  updatedAt: string;
}

export interface ListaNegraListResponse {
  bloqueados: ListaNegraResponse[];
  total: number;
  activos: number;
  porNivel: {
    alto: number;
    medio: number;
    bajo: number;
  };
}

export interface BlockCheckResponse {
  isBlocked: boolean;
  nivelSeveridad?: NivelSeveridad;
  bloqueadoDesde?: string;
}

// ==========================================
// BÚSQUEDA DE PERSONAS PARA BLOQUEAR
// ==========================================

export interface PersonaSearchResult {
  tipoPersona: 'contratista' | 'proveedor' | 'visita';
  entityId: string;
  cedula: string;
  nombre: string;
  segundoNombre?: string;
  apellido: string;
  segundoApellido?: string;
  nombreCompleto: string;
  empresaId?: string;
  empresaNombre?: string;
  yaBloqueado: boolean;
}