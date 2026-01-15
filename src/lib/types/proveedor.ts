// ==========================================
// src/lib/types/proveedor.ts
// ==========================================
// TypeScript interfaces for Proveedor module

export type EstadoProveedor = 'ACTIVO' | 'INACTIVO' | 'SUSPENDIDO';

// Response from backend (camelCase due to serde rename_all)
export interface ProveedorResponse {
	id: string;
	cedula: string;
	nombre: string;
	segundoNombre?: string;
	apellido: string;
	segundoApellido?: string;
	nombreCompleto?: string;
	empresaId: string;
	empresaNombre: string;
	estado: EstadoProveedor | string;
	puedeIngresar: boolean;
	// Vehicle info
	vehiculoTipo?: string;
	vehiculoPlaca?: string;
	vehiculoMarca?: string;
	vehiculoModelo?: string;
	vehiculoColor?: string;
	// Timestamps
	createdAt: string;
	updatedAt: string;
	deletedAt?: string;
}

// Input for creating a new proveedor
export interface CreateProveedorInput {
	cedula: string;
	nombre: string;
	segundoNombre?: string;
	apellido: string;
	segundoApellido?: string;
	empresaId: string;
	// Vehicle (optional)
	tieneVehiculo?: boolean;
	tipoVehiculo?: string;
	placa?: string;
	marca?: string;
	modelo?: string;
	color?: string;
}

// Input for updating a proveedor
export interface UpdateProveedorInput {
	nombre?: string;
	segundoNombre?: string;
	apellido?: string;
	segundoApellido?: string;
	empresaId?: string;
	estado?: string;
	tieneVehiculo?: boolean;
	tipoVehiculo?: string;
	placa?: string;
	marca?: string;
	modelo?: string;
	color?: string;
}

// List response with statistics
export interface ProveedorListResponse {
	proveedores: ProveedorResponse[];
	total: number;
	activos: number;
	inactivos: number;
}
