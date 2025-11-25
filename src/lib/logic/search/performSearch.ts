// src/lib/logic/search/performSearch.ts

import { searchStore } from '$lib/stores/searchStore';
import { searchContratistas } from '$lib/api/searchService';

export async function performSearch(query: string, limit: number = 10) {
  // Validación mínima
  if (!query || query.trim().length < 2) {
    searchStore.clearResults();
    return;
  }

  searchStore.setLoading(true);

  try {
    const results = await searchContratistas(query.trim(), limit);
    searchStore.setResults(results);
  } catch (err: any) {
    searchStore.setError(err?.message || 'Error al buscar');
  }
}