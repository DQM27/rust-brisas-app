
export interface VisitanteResponse {
    id: string;
    cedula: string;
    nombre: string;
    apellido: string;
    segundo_nombre?: string;
    segundo_apellido?: string;
    empresa?: string;
    empresa_id?: string;
    has_vehicle: boolean;
    created_at: string;
    updated_at: string;
    deleted_at?: string;
}

export interface CreateVisitanteInput {
    cedula: string;
    nombre: string;
    apellido: string;
    segundo_nombre?: string;
    segundo_apellido?: string;
    empresa?: string;
    empresa_id?: string;
    has_vehicle: boolean;
}

export interface UpdateVisitanteInput {
    nombre?: string;
    apellido?: string;
    segundo_nombre?: string;
    segundo_apellido?: string;
    empresa?: string;
    empresa_id?: string;
    has_vehicle?: boolean;
}
