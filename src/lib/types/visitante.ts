import type { VehiculoResponse } from './vehiculo';

export interface VisitanteResponse {
	id: string;
	cedula: string;
	nombre: string;
	apellido: string;
	segundoNombre?: string;
	segundoApellido?: string;
	empresaId?: string;
	empresaNombre?: string;
	hasVehicle: boolean;
	createdAt: string;
	updatedAt: string;
	deletedAt?: string;
	vehiculoTipo?: string;
	vehiculoPlaca?: string;
	vehiculos?: VehiculoResponse[];
}

export interface CreateVisitanteInput {
	cedula: string;
	nombre: string;
	apellido: string;
	segundoNombre?: string;
	segundoApellido?: string;
	empresaId?: string;
	hasVehicle: boolean;
	// Vehicle fields
	tipoVehiculo?: string;
	placa?: string;
	marca?: string;
	modelo?: string;
	color?: string;
}

export interface UpdateVisitanteInput {
	nombre?: string;
	apellido?: string;
	segundoNombre?: string;
	segundoApellido?: string;
	empresaId?: string;
	hasVehicle?: boolean;
}
