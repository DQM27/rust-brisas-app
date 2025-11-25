// ==========================================
// src/lib/services/searchService.ts
// ==========================================

import { invoke } from "@tauri-apps/api/core";
import type { SearchResult } from "$lib/types/search.types";

export async function searchContratistas(
  query: string,
  limit?: number
): Promise<SearchResult[]> {
  return await invoke("search_contratistas", { query, limit });
}

export async function reindexAllContratistas(): Promise<void> {
  return await invoke("reindex_all_contratistas");
}