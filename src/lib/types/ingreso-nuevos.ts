export interface IngresoVisita {
	id: string;
	visitanteId: string;
	citaId?: string;
	anfitrion: string;
	areaVisitada: string;
	motivo: string;
	gafete?: string;
	fechaIngreso: string;
	fechaSalida?: string;
	estado: 'ADENTRO' | 'SALIO';
	usuarioIngresoId: string;
	usuarioSalidaId?: string;
	observaciones?: string;

	// Populated fields
	visitanteNombre: string;
	visitanteApellido: string;
	visitanteCedula: string;
	visitanteEmpresa?: string;
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
