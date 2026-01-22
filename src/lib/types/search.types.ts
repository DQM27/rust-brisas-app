// ==========================================
// src/lib/types/search.types.ts
// ==========================================

export interface SearchResult {
	id: string;
	tipo: 'contratista' | 'usuario' | 'proveedor' | 'lista_negra' | 'visitante';
	score: number;
	cedula: string | null;
	nombre: string | null;
	segundoNombre: string | null;
	apellido: string | null;
	segundoApellido: string | null;
	nombreCompleto: string | null;
	empresaNombre: string | null;
	email: string | null;
}

export interface SearchState {
	query: string;
	results: SearchResult[];
	isLoading: boolean;
	error: string | null;
}

export type SearchFilter = 'all' | 'contratista' | 'blacklist' | 'vehicle';
