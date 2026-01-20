// ==========================================
// src/lib/services/searchService.ts
// ==========================================

import { invoke } from '@tauri-apps/api/core';
import type { SearchResult } from '$lib/types/search.types';

export async function searchGlobal(query: string, limit?: number): Promise<SearchResult[]> {
	return await invoke('search_omnibox', { query, limit });
}

export async function searchByType(
	query: string,
	tipo: string,
	limit?: number
): Promise<SearchResult[]> {
	return await invoke('search_by_type', { query, tipo, limit });
}

export async function reindexGlobalSearch(): Promise<void> {
	return await invoke('reindex_global_search');
}
