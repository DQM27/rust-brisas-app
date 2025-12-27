
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
}

export interface CreateVisitanteInput {
    cedula: string;
    nombre: string;
    apellido: string;
    segundoNombre?: string;
    segundoApellido?: string;
    empresaId?: string;
    hasVehicle: boolean;
}

export interface UpdateVisitanteInput {
    nombre?: string;
    apellido?: string;
    segundoNombre?: string;
    segundoApellido?: string;
    empresaId?: string;
    hasVehicle?: boolean;
}
