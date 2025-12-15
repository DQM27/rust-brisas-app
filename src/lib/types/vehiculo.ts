// src/lib/types/vehiculo.ts

export type TipoVehiculo = 'motocicleta' | 'automovil';

export interface Vehiculo {
  id: string;
  contrafistaId: string;
  tipoVehiculo: TipoVehiculo;
  placa: string;
  marca?: string;
  modelo?: string;
  color?: string;
  isActive: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface VehiculoResponse {
  id: string;
  contratistaId: string;
  contratistaNombre: string;
  contratistaCedula: string;
  empresaNombre: string;
  tipoVehiculo: TipoVehiculo;
  tipoVehiculoDisplay: string;
  placa: string;
  marca?: string;
  modelo?: string;
  color?: string;
  descripcionCompleta: string;
  isActive: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface VehiculoListResponse {
  vehiculos: VehiculoResponse[];
  total: number;
  activos: number;
  inactivos: number;
  porTipo: {
    motocicletas: number;
    automoviles: number;
  };
}

export interface CreateVehiculoInput {
  contratistaId: string;
  tipoVehiculo: TipoVehiculo;
  placa: string;
  marca?: string;
  modelo?: string;
  color?: string;
}

export interface UpdateVehiculoInput {
  tipoVehiculo?: TipoVehiculo;
  marca?: string;
  modelo?: string;
  color?: string;
  isActive?: boolean;
}