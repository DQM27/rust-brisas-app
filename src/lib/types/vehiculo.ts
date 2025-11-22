// src/lib/types/vehiculo.ts

export interface Vehiculo {
  id: string;
  contratista_id: string;
  placa: string;
  marca?: string;
  modelo?: string;
  color?: string;
  is_active: number;
  created_at: string;
  updated_at: string;
}

export interface VehiculoResponse extends Vehiculo {
  contratista_nombre?: string;
  contratista_cedula?: string;
  empresa_nombre?: string;
}

export interface VehiculoListResponse {
  vehiculos: VehiculoResponse[];
  total: number;
  activos: number;
  inactivos: number;
}
