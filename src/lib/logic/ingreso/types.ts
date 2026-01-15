export type TipoIngreso = 'contratista' | 'proveedor' | 'visita';

export type TipoAutorizacion =
	| { tipo: 'praind'; fecha_vencimiento: string }
	| { tipo: 'correo'; referencia?: string }
	| { tipo: 'excepcional'; autorizado_por: string; motivo: string };

import type { VehiculoResponse } from '$lib/types/vehiculo';
import type { ContratistaResponse } from '$lib/types/contratista';
import type { ProveedorResponse } from '$lib/types/proveedor';
import type { VisitanteResponse } from '$lib/types/visitante';
import type { IngresoResponse, ValidacionIngresoResponse } from '$lib/types/ingreso';

export type MotivoBloqueo =
	| { tipo: 'lista_negra'; motivo: string }
	| { tipo: 'ingreso_activo' }
	| { tipo: 'estado_invalido'; estado: string }
	| { tipo: 'autorizacion_invalida'; motivo: string }
	| { tipo: 'gafetes_pendientes'; cantidad: number };

export interface ResultadoValidacion {
	puedeIngresar: boolean;
	bloqueos: MotivoBloqueo[];
	alertas: string[];
	// reportesPendientes: any[]; // Definir si es necesario - Commented out as unused or unknown
}

// Interfaz espejo de lo que retorna la validación del backend
// (ValidacionIngresoResponse, ValidacionIngresoProveedorResponse, etc.)
// Tratamos de unificar para el frontend
export interface ValidacionIngresoResult {
	puedeIngresar: boolean;
	motivoRechazo?: string;
	severidadListaNegra?: string;
	alertas: string[];

	// Datos de la persona encontrada (unificado)
	persona?: {
		id: string;
		cedula: string;
		nombre: string;
		apellido: string;
		nombreCompleto?: string;
		empresa?: string;
		empresaId?: string;
		estado?: string;
		vehiculos?: VehiculoResponse[];
		praindVigente?: boolean;
	};

	// Atajos específicos para evitar errores de tipo en controladores
	contratista?: ContratistaResponse;
	proveedor?: ProveedorResponse;
	visitante?: VisitanteResponse;

	tieneIngresoAbierto: boolean;
	ingresoAbierto?: IngresoResponse;
}

export type IngresoStep = 'SEARCH' | 'VALIDATION' | 'DETAILS' | 'CONFIRM';

export interface IngresoState {
	step: IngresoStep;
	tipoIngreso: TipoIngreso | null;
	candidateId: string | null;
	candidateData: ContratistaResponse | ProveedorResponse | VisitanteResponse | null; // Datos crudos de la persona
	validationResult: ValidacionIngresoResult | null;

	// Input final
	form: FinalizarIngresoForm;
}

export interface FinalizarIngresoForm {
	gafete: string;
	vehiculoId?: string | null;
	observaciones?: string;
	esExcepcional?: boolean;
	tipoAutorizacion: string;
	modoIngreso: string;
	// Campos específicos
	autorizadoPor?: string; // Para excepcional
	motivoExcepcional?: string; // Para excepcional
	areaVisitada?: string;
	motivo?: string;
}
