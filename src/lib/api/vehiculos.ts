// src/lib/api/vehiculos.ts

import { invoke } from "@tauri-apps/api/core";
import type {
  VehiculoResponse,
  VehiculoListResponse,
  CreateVehiculoInput,
  UpdateVehiculoInput
} from '$lib/types/vehiculo';

export const vehiculos = {
  create: async (input: CreateVehiculoInput): Promise<VehiculoResponse> => {
    return await invoke('create_vehiculo', { input });
  },

  update: async (id: string, input: UpdateVehiculoInput): Promise<VehiculoResponse> => {
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

  getAll: async (): Promise<VehiculoListResponse> => {
    return await invoke('get_all_vehiculos');
  },

  getActivos: async (): Promise<VehiculoResponse[]> => {
    return await invoke('get_vehiculos_activos');
  },

  getByPropietario: async (propietarioId: string): Promise<VehiculoResponse[]> => {
    return await invoke('get_vehiculos_by_propietario', { propietarioId });
  },
};