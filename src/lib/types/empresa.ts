// Representa una empresa individual
export interface EmpresaResponse {
	id: string;
	nombre: string;
	is_active: boolean;
	total_contratistas: number;
	created_at: string; // ISO date string
	updated_at: string; // ISO date string
}

// Representa la respuesta completa al pedir todas las empresas
export interface EmpresaListResponse {
	empresas: EmpresaResponse[];
	total: number;
	activas: number;
}

// Input para crear una nueva empresa
export interface CreateEmpresaInput {
	nombre: string;
}

// Input para actualizar una empresa existente
export interface UpdateEmpresaInput {
	nombre?: string;
	is_active?: boolean;
}
