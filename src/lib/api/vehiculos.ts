// src/lib/api/vehiculos.ts

import { invoke } from "@tauri-apps/api/core";
import type { VehiculoResponse, VehiculoListResponse } from '$lib/types/vehiculo';

export const vehiculos = {
  create: async (input: {
    contratista_id: string;
    placa: string;
    marca?: string;
    modelo?: string;
    color?: string;
  }): Promise<VehiculoResponse> => {
    return await invoke('create_vehiculo', { input });
  },

  update: async (
    id: string,
    input: { marca?: string; modelo?: string; color?: string; is_active?: number }
  ): Promise<VehiculoResponse> => {
    return await invoke('update_vehiculo', { id, input });
  },

  delete: async (id: string): Promise<void> => {
    return await invoke('delete_vehiculo', { id });
  },

  getById: async (id: string): Promise<VehiculoResponse> => {
    return await invoke('get_vehiculo_by_id', { id });
  },

  getByPlaca: async (placa: string): Promise<VehiculoResponse> => {
    return await invoke('get_vehiculo_by_placa', { placa });
  },

  getAll: async (): Promise<VehiculoResponse[]> => {
    const result: VehiculoListResponse = await invoke('get_all_vehiculos');
    return result.vehiculos;
  },

  getActivos: async (): Promise<VehiculoResponse[]> => {
    return await invoke('get_vehiculos_activos');
  },

  getByContratista: async (contratista_id: string): Promise<VehiculoResponse[]> => {
    return await invoke('get_vehiculos_by_contratista', { contratista_id });
  },
};
