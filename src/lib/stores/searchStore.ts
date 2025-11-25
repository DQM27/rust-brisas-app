// ==========================================
// src/lib/stores/searchStore.ts
// ==========================================

import { writable, derived } from 'svelte/store';
import type { SearchState, SearchResult } from '$lib/types/search.types';

// Estado inicial
const initialState: SearchState = {
  query: '',
  results: [],
  isLoading: false,
  error: null,
};

// Store principal
function createSearchStore() {
  const { subscribe, set, update } = writable<SearchState>(initialState);

  return {
    subscribe,
    
    // Actualizar query
    setQuery: (query: string) => update(state => ({ ...state, query })),
    
    // Actualizar resultados
    setResults: (results: SearchResult[]) => 
      update(state => ({ ...state, results, isLoading: false, error: null })),
    
    // Setear loading
    setLoading: (isLoading: boolean) => 
      update(state => ({ ...state, isLoading })),
    
    // Setear error
    setError: (error: string) => 
      update(state => ({ ...state, error, isLoading: false })),
    
    // Limpiar búsqueda
    clear: () => set(initialState),
    
    // Reset solo resultados
    clearResults: () => 
      update(state => ({ ...state, results: [], error: null })),
  };
}

export const searchStore = createSearchStore();

// Derived stores útiles
export const hasResults = derived(
  searchStore,
  $store => $store.results.length > 0
);

export const isSearching = derived(
  searchStore,
  $store => $store.query.length > 0
);