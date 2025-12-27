// ============================================
// src/lib/api/contratista.ts
// ============================================
// API Layer para contratistas - Tauri invoke calls

import { invoke } from "@tauri-apps/api/core";
import type {
  ContratistaResponse,
  ContratistaListResponse,
  CreateContratistaInput,
  UpdateContratistaInput
} from "$lib/types/contratista";

/**
 * API object for contratista operations
 * Follows the same pattern as users API
 */
export const contratistas = {
  create: async (input: CreateContratistaInput): Promise<ContratistaResponse> => {
    return await invoke<ContratistaResponse>("create_contratista", { input });
  },

  list: async (): Promise<ContratistaListResponse> => {
    return await invoke<ContratistaListResponse>("get_all_contratistas");
  },

  getById: async (id: string): Promise<ContratistaResponse> => {
    return await invoke<ContratistaResponse>("get_contratista_by_id", { id });
  },

  update: async (id: string, input: UpdateContratistaInput): Promise<ContratistaResponse> => {
    return await invoke<ContratistaResponse>("update_contratista", { id, input });
  },

  delete: async (id: string): Promise<boolean> => {
    return await invoke<boolean>("delete_contratista", { id });
  },

  changeEstado: async (id: string, estado: string): Promise<ContratistaResponse> => {
    return await invoke<ContratistaResponse>("cambiar_estado_contratista", { id, input: { id, estado } });
  },

  restore: async (id: string): Promise<void> => {
    return await invoke<void>("restore_contratista", { id });
  },

  listArchived: async (): Promise<ContratistaResponse[]> => {
    return await invoke<ContratistaResponse[]>("get_archived_contratistas");
  },
};
