export interface Visitante {
	id: string;
	cedula: string;
	nombre: string;
	apellido: string;
	segundo_nombre?: string;
	segundo_apellido?: string;
	empresa?: string;
	has_vehicle: boolean;
	created_at: string;
	updated_at: string;
}

export interface CreateVisitanteInput {
	cedula: string;
	nombre: string;
	apellido: string;
	segundo_nombre?: string;
	segundo_apellido?: string;
	empresa?: string;
	has_vehicle: boolean;
}

export interface Cita {
	id: string;
	visitante_id: string;
	fecha_cita: string; // ISO string format
	anfitrion: string;
	area_visitada: string;
	motivo: string;
	estado: 'PENDIENTE' | 'COMPLETADA' | 'CANCELADA' | 'EXPIRADA';
	registrado_por: string;
	created_at: string;
	updated_at: string;
}

export interface CitaPopulated {
	id: string;
	fecha_cita: string;
	anfitrion: string;
	area_visitada: string;
	motivo: string;
	estado: string;
	visitante_id: string;
	visitante_cedula: string;
	visitante_nombre: string;
	visitante_apellido: string;
	visitante_nombre_completo: string;
	visitante_empresa?: string;
}

export interface CreateCitaInput {
	visitante_id: string;
	fecha_cita: string;
	anfitrion: string;
	area_visitada: string;
	motivo: string;
	registrado_por: string;
}
