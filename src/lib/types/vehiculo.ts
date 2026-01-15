// src/lib/types/vehiculo.ts

export type TipoVehiculo = 'motocicleta' | 'automovil' | 'camioneta' | 'camion' | 'otro';

export interface Vehiculo {
	id: string;
	propietario: string; // RecordId string
	tipoVehiculo: TipoVehiculo;
	placa: string;
	marca?: string;
	modelo?: string;
	color?: string;
	isActive: boolean;
	createdAt: string;
	updatedAt: string;
}

export type PropietarioTipo = 'contratista' | 'proveedor' | 'visitante';

export interface VehiculoResponse {
	id: string;
	propietarioId: string;
	propietarioNombre: string;
	propietarioCedula: string;
	propietarioTipo: PropietarioTipo;
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
	propietarioId: string;
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
