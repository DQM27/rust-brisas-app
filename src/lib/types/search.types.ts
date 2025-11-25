// ==========================================
// src/lib/types/search.types.ts
// ==========================================

export interface SearchResult {
  id: string;
  tipo: string;
  score: number;
   cedula?: string;              
  nombreCompleto?: string;      
  empresaNombre?: string; 
}

export interface SearchState {
  query: string;
  results: SearchResult[];
  isLoading: boolean;
  error: string | null;
}

export type SearchFilter = 'all' | 'contratista' | 'blacklist' | 'vehicle';