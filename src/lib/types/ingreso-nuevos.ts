export interface IngresoVisita {
	id: string;
	preRegistroId?: string;
	cedula: string;
	nombre: string;
	segundoNombre?: string;
	apellido: string;
	segundoApellido?: string;
	empresaNombre?: string;
	anfitrion: string;
	areaVisitada: string;
	motivo: string;
	gafete?: string;
	fechaIngreso: string;
	fechaSalida?: string;
	usuarioIngresoId: string;
	usuarioIngresoNombre?: string;
	usuarioSalidaId?: string;
	usuarioSalidaNombre?: string;
	observaciones?: string;
}

export interface IngresoProveedor {
	id: string;
	cedula: string;
	nombre: string;
	apellido: string;
	empresaId: string;
	areaVisitada: string;
	motivo: string;
	gafete?: string;
	modoIngreso?: string;
	placaVehiculo?: string;
	fechaIngreso: string;
	fechaSalida?: string;
	estado: 'ADENTRO' | 'SALIO';
	usuarioIngresoId: string;
	usuarioSalidaId?: string;
	observaciones?: string;
	// Populated fields
	usuarioIngresoNombre?: string;
	usuarioSalidaNombre?: string;
	empresaNombre?: string;
}

export interface CreateIngresoProveedorInput {
	cedula: string;
	nombre: string;
	apellido: string;
	segundoNombre?: string;
	segundoApellido?: string;
	proveedorId: string;
	areaVisitada: string;
	motivo: string;
	modoIngreso: string;
	// Vehicle fields
	placaVehiculo?: string;
	// Gafete
	gafeteNumero?: number;
	// Audit
	observaciones?: string;
}

export interface ProveedorCatalogItem {
	id: string;
	cedula: string;
	nombre: string;
	apellido: string;
	empresaId: string;
	empresaNombre: string;
	// Vehicle info
	vehiculoTipo?: string;
	vehiculoPlaca?: string;
	vehiculoMarca?: string;
	vehiculoModelo?: string;
	vehiculoColor?: string;
}

export interface ValidacionIngresoProveedorResponse {
	puedeIngresar: boolean;
	cedula: string;
	nombre: string;
	apellido: string;
	segundoNombre?: string;
	segundoApellido?: string;
	empresaNombre: string;
	motivoRechazo?: string;
	alertasGafete: string[];
	tieneGafetesPendientes: boolean;
	tieneIngresoAbierto: boolean;
}

export type PreRegistroEstado = 'PENDIENTE' | 'COMPLETADO' | 'CANCELADO' | 'NO_SHOW';

export interface PreRegistroVisita {
	id: string;
	cedula: string;
	nombre: string;
	apellido: string;
	segundoNombre?: string;
	segundoApellido?: string;
	empresaNombre?: string;
	fechaEsperada: string;
	horaEsperada: string;
	anfitrion: string;
	areaVisitada: string;
	motivo: string;
	modoIngreso: string;
	modo_ingreso?: string;
	placa?: string;
	estado: PreRegistroEstado;
	observaciones?: string;
	registradoPor?: any; // User object
	createdAt: string;
	// Backend compatibility
	empresa_id?: any;
	empresaId?: string;
}

export interface CreatePreRegistroInput {
	cedula: string;
	nombre: string;
	segundoNombre?: string;
	apellido: string;
	segundoApellido?: string;
	empresaNombre?: string;
	empresaId?: string;
	placa?: string;
	fechaEsperada: string;
	horaEsperada: string;
	anfitrion: string;
	areaVisitada: string;
	motivo: string;
	modoIngreso: string;
}
