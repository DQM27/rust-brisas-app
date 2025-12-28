export type TipoIngreso = 'contratista' | 'proveedor' | 'visita';

export type TipoAutorizacion =
    | { tipo: 'praind'; fecha_vencimiento: string }
    | { tipo: 'correo'; referencia?: string }
    | { tipo: 'excepcional'; autorizado_por: string; motivo: string };

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
    reportesPendientes: any[]; // Definir si es necesario
}

// Interfaz espejo de lo que retorna la validación del backend
// (ValidacionIngresoResponse, ValidacionIngresoProveedorResponse, etc.)
// Tratamos de unificar para el frontend
export interface ValidacionIngresoResult {
    puedeIngresar: boolean;
    motivoRechazo?: string;
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
        vehiculos?: any[];
    };

    // Atajos específicos para evitar errores de tipo en controladores
    contratista?: any;
    proveedor?: any;
    visitante?: any;

    tieneIngresoAbierto: boolean;
    ingresoAbierto?: any;
}

export type IngresoStep = 'SEARCH' | 'VALIDATION' | 'DETAILS' | 'CONFIRM';

export interface IngresoState {
    step: IngresoStep;
    tipoIngreso: TipoIngreso | null;
    candidateId: string | null;
    candidateData: any | null; // Datos crudos de la persona (Contratista | Proveedor | Visitante)
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
