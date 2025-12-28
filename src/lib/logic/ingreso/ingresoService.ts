import { invoke } from '@tauri-apps/api/core';
import type {
    TipoIngreso,
    ValidacionIngresoResult,
    FinalizarIngresoForm
} from './types';

// ==========================================
// TYPES FOR RESULTS
// ==========================================

export type ServiceResult<T> =
    | { ok: true; data: T }
    | { ok: false; error: string };

// ==========================================
// VALIDACIÓN
// ==========================================

export async function validarIngreso(tipo: TipoIngreso, id: string): Promise<ValidacionIngresoResult> {
    console.log(`[IngresoService] Validando ingreso ${tipo} para ID: ${id}`);

    try {
        let response: any;

        if (tipo === 'contratista') {
            response = await invoke('validate_ingreso_contratista', { contratistaId: id });
            return mapContratistaResponse(response);
        } else if (tipo === 'proveedor') {
            response = await invoke('validar_ingreso_proveedor', { proveedorId: id });
            return mapProveedorResponse(response);
        } else if (tipo === 'visita') {
            response = await invoke('validar_ingreso_visita', { visitanteId: id });
            return mapVisitaResponse(response);
        } else {
            throw new Error(`Tipo de ingreso no soportado: ${tipo}`);
        }
    } catch (error) {
        console.error(`[IngresoService] Error validando ingreso:`, error);
        throw error;
    }
}

// ==========================================
// VALIDACIÓN PREVIA (FORMULARIO)
// ==========================================

/**
 * Valida y prepara los datos iniciales para el formulario de ingreso
 */
export async function prepararFormularioIngreso(contratistaId: string): Promise<ServiceResult<{
    validacion: ValidacionIngresoResult;
    autoSeleccion: {
        suggestedMode: 'caminando' | 'vehiculo';
        suggestedVehicleId: string | null;
    }
}>> {
    try {
        const validacion = await validarIngreso('contratista', contratistaId);

        // Lógica de auto-selección simple
        let suggestedMode: 'caminando' | 'vehiculo' = 'caminando';
        let suggestedVehicleId: string | null = null;

        if (validacion.persona?.vehiculos && validacion.persona.vehiculos.length > 0) {
            suggestedMode = 'vehiculo';
            suggestedVehicleId = validacion.persona.vehiculos[0].id;
        }

        return {
            ok: true,
            data: {
                validacion,
                autoSeleccion: {
                    suggestedMode,
                    suggestedVehicleId
                }
            }
        };
    } catch (e: any) {
        return { ok: false, error: e.message || 'Error al validar contratista' };
    }
}

/**
 * Valida todo el estado del formulario antes de enviar
 */
export function validarFormularioCompleto(params: {
    contratistaValidated: boolean;
    canEnter: boolean;
    contratistaId: string | null;
    modoIngreso: string;
    vehiculoId?: string | null;
    gafeteNumero: string;
    tipoAutorizacion: 'praind' | 'correo';
}): ServiceResult<null> {
    if (!params.contratistaId) {
        return { ok: false, error: 'No se ha seleccionado un contratista.' };
    }
    if (!params.contratistaValidated) {
        return { ok: false, error: 'El contratista no ha sido validado correctamente.' };
    }
    if (!params.canEnter) {
        return { ok: false, error: 'El contratista no está autorizado para ingresar.' };
    }
    if (params.modoIngreso === 'vehiculo' && !params.vehiculoId) {
        return { ok: false, error: 'Debe seleccionar un vehículo para el ingreso en este modo.' };
    }
    return { ok: true, data: null };
}

/**
 * Valida si el gafete es válido y está disponible
 */
export function validarGafete(params: {
    gafeteNumero: string;
    gafetesDisponibles: any[];
}): ServiceResult<{ isValid: boolean; suggestions: string[] }> {
    const numero = params.gafeteNumero.trim().toUpperCase();
    if (!numero) {
        return { ok: true, data: { isValid: false, suggestions: [] } };
    }

    const gafete = params.gafetesDisponibles.find(g => g.numero === numero);
    if (!gafete) {
        return { ok: true, data: { isValid: false, suggestions: [] } };
    }

    return { ok: true, data: { isValid: true, suggestions: [] } };
}

/**
 * Valida coherencia del modo vehículo
 */
export function validarModoVehiculo(params: {
    modoIngreso: string;
    vehiculoId?: string | null;
    tieneVehiculos: boolean;
}): ServiceResult<null> {
    if (params.modoIngreso === 'vehiculo') {
        if (!params.tieneVehiculos) {
            return { ok: false, error: 'El contratista no tiene vehículos registrados.' };
        }
        if (!params.vehiculoId) {
            return { ok: false, error: 'Debe seleccionar un vehículo.' };
        }
    }
    return { ok: true, data: null };
}

// ==========================================
// REGISTRO Y OPERACIONES
// ==========================================

export async function registrarEntrada(input: any): Promise<ServiceResult<any>> {
    try {
        const formData: FinalizarIngresoForm = {
            gafete: input.gafeteNumero || '',
            vehiculoId: input.vehiculoId,
            observaciones: input.observaciones,
            esExcepcional: input.tipoAutorizacion === 'excepcional',
            tipoAutorizacion: input.tipoAutorizacion || 'praind',
            modoIngreso: input.modoIngreso || 'caminando',
        };

        const res = await crearIngreso(
            'contratista',
            input.contratistaId,
            formData
        );

        return { ok: true, data: res };
    } catch (e: any) {
        return { ok: false, error: e.message || 'Error al registrar entrada' };
    }
}

export async function fetchAbiertos(): Promise<ServiceResult<any[]>> {
    try {
        const data = await invoke('get_ingresos_abiertos');
        return { ok: true, data: data as any[] };
    } catch (e: any) {
        return { ok: false, error: e.message || 'Error cargando ingresos activos' };
    }
}

// ==========================================
// CREACIÓN (FINALIZAR)
// ==========================================

export async function crearIngreso(
    tipo: TipoIngreso,
    candidateId: string,
    formData: FinalizarIngresoForm,
    extraData?: any,
    usuarioId?: string
): Promise<any> {
    // Validar que se proporcione un usuario válido
    if (!usuarioId) {
        throw new Error('Se requiere un usuario autenticado para registrar el ingreso');
    }
    console.log(`[IngresoService] Creando ingreso ${tipo}`, { candidateId, formData, extraData, usuarioId });

    try {
        if (tipo === 'contratista') {
            return await invoke('create_ingreso_contratista', {
                input: {
                    contratistaId: candidateId,
                    gafeteNumero: formData.gafete,
                    gafeteTipo: 'contratista',
                    vehiculoId: formData.vehiculoId || null,
                    tipoAutorizacion: formData.tipoAutorizacion || 'praind',
                    modoIngreso: formData.modoIngreso || 'caminando',
                    observaciones: formData.observaciones || null,
                    usuarioIngresoId: usuarioId
                },
                usuario_id: usuarioId
            });
        } else if (tipo === 'proveedor') {
            return await invoke('crear_ingreso_proveedor_v2', {
                input: {
                    proveedor_id: candidateId,
                    gafete: formData.gafete,
                    vehiculo_id: formData.vehiculoId || null,
                    observaciones: formData.observaciones || null,
                    autorizado_por: formData.esExcepcional ? formData.autorizadoPor : null,
                    motivo: formData.esExcepcional ? formData.motivoExcepcional : null,
                    guia_remision: extraData?.guiaRemision || null
                }
            });
        } else if (tipo === 'visita') {
            if (!extraData || !extraData.cedula) {
                throw new Error("Faltan datos requeridos para ingreso de visita (cedula, nombre, etc.)");
            }

            return await invoke('crear_ingreso_visita_v2', {
                input: {
                    cedula: extraData.cedula,
                    nombre: extraData.nombre,
                    apellido: extraData.apellido,
                    anfitrion: extraData.anfitrion,
                    areaVisitada: extraData.areaVisitada,
                    motivoVisita: extraData.motivo || 'Visita',
                    tipoAutorizacion: formData.tipoAutorizacion || 'correo',
                    modoIngreso: formData.modoIngreso || 'caminando',
                    gafeteNumero: formData.gafete,
                    observaciones: formData.observaciones || null,
                    usuarioIngresoId: usuarioId,
                }
            });
        }
    } catch (error) {
        console.error(`[IngresoService] Error creando ingreso:`, error);
        throw error;
    }
}

// ==========================================
// SALIDA Y TIEMPO
// ==========================================

/**
 * Registra la salida de una persona
 */
export async function registrarSalida(params: {
    ingresoId: string;
    devolvioGafete: boolean;
    observacionesSalida: string;
    usuarioSalidaId: string;
}): Promise<ServiceResult<any>> {
    try {
        const res = await invoke('registrar_salida', {
            input: {
                ingreso_id: params.ingresoId,
                devolvio_gafete: params.devolvioGafete,
                observaciones: params.observacionesSalida,
                usuario_salida_id: params.usuarioSalidaId
            }
        });
        return { ok: true, data: res };
    } catch (e: any) {
        return { ok: false, error: e.message || 'Error al registrar salida' };
    }
}

/**
 * Obtiene historial de salidas en un rango de fechas
 */
export async function fetchSalidasEnRango(startDate: string, endDate: string): Promise<ServiceResult<any[]>> {
    try {
        const data = await invoke('get_ingresos_salidas_rango', {
            start_date: startDate,
            end_date: endDate
        });
        return { ok: true, data: data as any[] };
    } catch (e: any) {
        return { ok: false, error: e.message || 'Error cargando historial de salidas' };
    }
}

/**
 * Evalúa el estado de tiempo de permanencia
 */
export function evaluateTimeStatus(entryDate: Date, current: Date) {
    const diffMs = current.getTime() - entryDate.getTime();
    const minutosTranscurridos = Math.floor(diffMs / 60000);

    // Umbrales sugeridos (pueden venir de config en el futuro)
    const LIMITE_NORMAL = 120; // 2 horas
    const LIMITE_ALERTA = 180; // 3 horas

    let estado: 'normal' | 'alerta_temprana' | 'tiempo_excedido' = 'normal';

    if (minutosTranscurridos > LIMITE_ALERTA) {
        estado = 'tiempo_excedido';
    } else if (minutosTranscurridos > LIMITE_NORMAL) {
        estado = 'alerta_temprana';
    }

    return {
        estado,
        minutosTranscurridos
    };
}


// ==========================================
// COMPATIBILITY EXPORT
// ==========================================

export const ingresoService = {
    validarIngreso,
    prepararFormularioIngreso,
    validarFormularioCompleto,
    validarGafete,
    validarModoVehiculo,
    registrarEntrada,
    fetchAbiertos,
    crearIngreso,
    registrarSalida,
    fetchSalidasEnRango,
    evaluateTimeStatus
};


// ==========================================
// MAPPERS
// ==========================================


function mapContratistaResponse(res: any): ValidacionIngresoResult {
    return {
        puedeIngresar: res.puedeIngresar,
        motivoRechazo: res.motivoRechazo,
        alertas: res.alertas || [],
        tieneIngresoAbierto: res.tieneIngresoAbierto,
        ingresoAbierto: res.ingresoAbierto,
        contratista: res.contratista,
        persona: res.contratista ? {
            id: res.contratista.id,
            cedula: res.contratista.cedula,
            nombre: res.contratista.nombre,
            apellido: res.contratista.apellido,
            nombreCompleto: `${res.contratista.nombre} ${res.contratista.apellido}`,
            empresa: res.contratista.empresa_nombre,
            empresaId: res.contratista.empresa_id || undefined,
            estado: res.contratista.estado,
            vehiculos: res.contratista.vehiculos || []
        } : undefined
    };
}

function mapProveedorResponse(res: any): ValidacionIngresoResult {
    return {
        puedeIngresar: res.puedeIngresar,
        motivoRechazo: res.motivoRechazo,
        alertas: res.alertas || [],
        tieneIngresoAbierto: res.tieneIngresoAbierto,
        ingresoAbierto: res.ingresoAbierto,
        proveedor: res.proveedor,
        persona: res.proveedor ? {
            id: res.proveedor.id,
            cedula: res.proveedor.cedula,
            nombre: res.proveedor.nombre,
            apellido: res.proveedor.apellido || '',
            nombreCompleto: `${res.proveedor.nombre} ${res.proveedor.apellido || ''}`,
            empresa: res.proveedor.empresa_nombre || res.proveedor.empresa,
            vehiculos: []
        } : undefined
    };
}

function mapVisitaResponse(res: any): ValidacionIngresoResult {
    return {
        puedeIngresar: res.puedeIngresar,
        motivoRechazo: res.motivoRechazo,
        alertas: res.alertas || [],
        tieneIngresoAbierto: res.tieneIngresoAbierto,
        ingresoAbierto: res.ingresoAbierto,
        visitante: res.visitante,
        persona: res.visitante ? {
            id: res.visitante.id,
            cedula: res.visitante.cedula,
            nombre: res.visitante.nombre,
            apellido: res.visitante.apellido,
            nombreCompleto: `${res.visitante.nombre} ${res.visitante.apellido}`,
            empresa: res.visitante.empresa_nombre || res.visitante.empresa,
            vehiculos: []
        } : undefined
    };
}