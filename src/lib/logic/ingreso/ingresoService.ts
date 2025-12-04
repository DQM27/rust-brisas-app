// ============================================
// src/lib/logic/ingreso/ingresoService.ts
// ============================================

import { ingreso } from '$lib/api/ingreso';
import type {
    IngresoResponse,
    IngresoConEstadoResponse,
    IngresoListResponse,
    ValidacionIngresoResponse,
    CreateIngresoContratistaInput,
    RegistrarSalidaInput,
    ResolverAlertaInput,
    AlertaGafeteResponse
} from '$lib/types/ingreso';
import type {
    AutoSelectionResult,
    GafeteValidationResult,
    PrepararFormularioOutput,
    ValidarGafeteInput,
    ValidarModoVehiculoInput,
    ValidarFormularioCompletoInput
} from '$lib/types/ingreso-form.types';
import {
    GafeteValidationSchema,
    ModoVehiculoSchema,
    IngresoFormValidationSchema
} from '$lib/types/ingreso-form.types';
import { ZodError } from 'zod';

// ============================================
// TYPES FOR RESULTS
// ============================================

export type ServiceResult<T> =
    | { ok: true; data: T }
    | { ok: false; error: string };

// ============================================
// ERROR PARSING
// ============================================

function parseError(err: any): string {
    if (!err) return 'Ocurrió un error desconocido.';

    // Manejo de errores de Zod
    if (err instanceof ZodError) {
        return err.issues.map((e: any) => e.message).join(', ');
    }

    if (typeof err === 'string') {
        if (/bloqueado/i.test(err)) {
            return 'La persona se encuentra bloqueada y no puede ingresar.';
        }
        if (/no encontrado/i.test(err)) {
            return 'Registro no encontrado.';
        }
        if (/gafete/i.test(err) && /uso/i.test(err)) {
            return 'El gafete seleccionado ya está en uso.';
        }
        return err;
    }

    if (typeof err === 'object' && err.message) {
        return parseError(err.message);
    }

    return 'Ocurrió un error inesperado al procesar la solicitud.';
}

// ============================================
// VALIDACIONES DE NEGOCIO - LÓGICA PURA
// ============================================

/**
 * Calcular auto-selección inteligente basada en vehículos del contratista
 * 
 * Reglas de negocio:
 * - Sin vehículos → modo caminando
 * - 1 vehículo → modo vehículo con el único vehículo pre-seleccionado
 * - Múltiples vehículos → modo caminando, usuario debe elegir
 * 
 * @param contratistaData - Datos del contratista con vehículos
 * @returns Resultado de auto-selección con modo y vehículo sugeridos
 */
export function calcularAutoSeleccion(contratistaData: any): AutoSelectionResult {
    const vehiculos = contratistaData?.vehiculos || [];
    
    if (vehiculos.length === 0) {
        return {
            suggestedMode: 'caminando',
            suggestedVehicleId: null,
            reason: 'no_vehicles'
        };
    }
    
    if (vehiculos.length === 1) {
        return {
            suggestedMode: 'vehiculo',
            suggestedVehicleId: vehiculos[0].id,
            reason: 'single_vehicle'
        };
    }
    
    // Múltiples vehículos: dejar que el usuario elija
    return {
        suggestedMode: 'caminando',
        suggestedVehicleId: null,
        reason: 'multiple_vehicles'
    };
}

/**
 * Validar gafete contra lista de disponibles
 * 
 * Validaciones:
 * - Si está vacío, es válido (opcional)
 * - Si tiene valor, debe existir en la lista y estar disponible
 * - Genera sugerencias si no es válido
 * 
 * @param input - Número de gafete y lista de disponibles
 * @returns Resultado con validez y sugerencias
 */
export function validarGafete(input: ValidarGafeteInput): ServiceResult<GafeteValidationResult> {
    try {
        const normalizado = input.gafeteNumero.trim().toUpperCase();
        
        // Si está vacío, es válido (gafete es opcional)
        if (!normalizado) {
            return {
                ok: true,
                data: { isValid: true, suggestions: [] }
            };
        }
        
        // Validación con Zod
        const validation = GafeteValidationSchema.safeParse({
            numero: normalizado,
            gafetesDisponibles: input.gafetesDisponibles
        });
        
        if (validation.success) {
            return {
                ok: true,
                data: { isValid: true, suggestions: [] }
            };
        }
        
        // Generar sugerencias si no es válido
        const suggestions = input.gafetesDisponibles
            .filter(g => g.numero.includes(normalizado) && g.estaDisponible)
            .map(g => g.numero)
            .slice(0, 5);
        
        return {
            ok: true,
            data: { 
                isValid: false, 
                suggestions 
            }
        };
        
    } catch (err) {
        return {
            ok: false,
            error: parseError(err)
        };
    }
}

/**
 * Validar modo de ingreso con vehículo
 * 
 * Validaciones:
 * - Si modo es "vehículo", debe tener vehiculoId
 * - Si modo es "vehículo", el contratista debe tener vehículos
 * 
 * @param input - Modo, vehículo seleccionado y disponibilidad
 * @returns Resultado de validación
 */
export function validarModoVehiculo(input: ValidarModoVehiculoInput): ServiceResult<boolean> {
    try {
        ModoVehiculoSchema.parse({
            modoIngreso: input.modoIngreso,
            vehiculoId: input.vehiculoId,
            tieneVehiculos: input.tieneVehiculos
        });
        
        return { ok: true, data: true };
        
    } catch (err) {
        if (err instanceof ZodError) {
            return {
                ok: false,
                error: err.issues[0].message
            };
        }
        return {
            ok: false,
            error: 'Error al validar modo de ingreso'
        };
    }
}

/**
 * Validar formulario completo antes de submit
 * 
 * Última validación antes de enviar al backend
 * Verifica que todos los campos requeridos estén correctos
 * 
 * @param input - Datos completos del formulario
 * @returns Resultado de validación
 */
export function validarFormularioCompleto(input: ValidarFormularioCompletoInput): ServiceResult<boolean> {
    try {
        IngresoFormValidationSchema.parse({
            contratistaValidated: input.contratistaValidated,
            canEnter: input.canEnter,
            contratistaId: input.contratistaId,
            modoIngreso: input.modoIngreso,
            vehiculoId: input.vehiculoId,
            gafeteNumero: input.gafeteNumero,
            tipoAutorizacion: input.tipoAutorizacion
        });
        
        return { ok: true, data: true };
        
    } catch (err) {
        if (err instanceof ZodError) {
            return {
                ok: false,
                error: err.issues.map(i => i.message).join(', ')
            };
        }
        return {
            ok: false,
            error: 'Error al validar formulario'
        };
    }
}

// ============================================
// ORQUESTADORES - FLUJOS COMPLETOS
// ============================================

/**
 * Preparar formulario de ingreso
 * 
 * Orquestador principal que:
 * 1. Valida si el contratista puede ingresar
 * 2. Calcula auto-selección de vehículo
 * 3. Retorna todo preparado para el formulario
 * 
 * @param contratistaId - ID del contratista a validar
 * @returns Validación completa y auto-selección
 */
export async function prepararFormularioIngreso(
    contratistaId: string
): Promise<ServiceResult<PrepararFormularioOutput>> {
    try {
        // 1. Validar si puede ingresar
        const validacionResult = await validarIngreso(contratistaId);
        
        if (!validacionResult.ok) {
            return validacionResult as ServiceResult<PrepararFormularioOutput>;
        }
        
        const validacion = validacionResult.data;
        
        // 2. Verificar que puede ingresar
        if (!validacion.puedeIngresar) {
            return {
                ok: false,
                error: validacion.motivoRechazo || 'No autorizado para ingresar'
            };
        }
        
        // 3. Calcular auto-selección basada en vehículos
        const autoSeleccion = calcularAutoSeleccion(validacion.contratista);
        
        // 4. Retornar todo preparado
        return {
            ok: true,
            data: {
                validacion,
                autoSeleccion
            }
        };
        
    } catch (err) {
        return {
            ok: false,
            error: parseError(err)
        };
    }
}

// ============================================
// PUBLIC API - INGRESO OPERATIONS
// ============================================

/**
 * Validar si un contratista puede ingresar
 */
export async function validarIngreso(contratistaId: string): Promise<ServiceResult<ValidacionIngresoResponse>> {
    try {
        const data = await ingreso.validarIngresoContratista(contratistaId);
        return { ok: true, data };
    } catch (err: any) {
        console.error('Error al validar ingreso:', err);
        return { ok: false, error: parseError(err) };
    }
}

/**
 * Registrar entrada de contratista
 */
export async function registrarEntrada(input: CreateIngresoContratistaInput): Promise<ServiceResult<IngresoResponse>> {
    try {
        const data = await ingreso.crearIngresoContratista(input);
        return { ok: true, data };
    } catch (err: any) {
        console.error('Error al registrar entrada:', err);
        return { ok: false, error: parseError(err) };
    }
}

// ============================================
// PUBLIC API - SALIDA OPERATIONS
// ============================================

/**
 * Validar si puede salir (reglas de negocio)
 */
export async function validarSalida(id: string): Promise<ServiceResult<{ puedeSalir: boolean, mensaje?: string }>> {
    try {
        const data = await ingreso.validarPuedeSalir(id);
        return { ok: true, data };
    } catch (err: any) {
        console.error('Error al validar salida:', err);
        return { ok: false, error: parseError(err) };
    }
}

/**
 * Registrar salida
 */
export async function registrarSalida(input: RegistrarSalidaInput): Promise<ServiceResult<IngresoResponse>> {
    try {
        const data = await ingreso.registrarSalida(input);
        return { ok: true, data };
    } catch (err: any) {
        console.error('Error al registrar salida:', err);
        return { ok: false, error: parseError(err) };
    }
}

/**
 * Registrar salida con verificación de gafete físico
 */
export async function registrarSalidaConGafete(
    ingresoId: string,
    gafeteNumero: string,
    usuarioSalidaId: string
): Promise<ServiceResult<IngresoResponse>> {
    try {
        const data = await ingreso.registrarSalidaConGafete(ingresoId, gafeteNumero, usuarioSalidaId);
        return { ok: true, data };
    } catch (err: any) {
        console.error('Error al registrar salida con gafete:', err);
        return { ok: false, error: parseError(err) };
    }
}

/**
 * Obtener salidas del día
 */
export async function fetchSalidasDelDia(fecha?: string): Promise<ServiceResult<IngresoResponse[]>> {
    try {
        const data = await ingreso.getSalidasDelDia(fecha);
        return { ok: true, data };
    } catch (err: any) {
        console.error('Error al obtener salidas del día:', err);
        return { ok: false, error: parseError(err) };
    }
}

/**
 * Obtener salidas en un rango de fechas
 * 
 * Usa el comando Tauri optimizado del backend que filtra con SQL
 * en lugar de iterar día por día
 * 
 * @param startDate - Fecha de inicio (YYYY-MM-DD)
 * @param endDate - Fecha de fin (YYYY-MM-DD)
 * @returns Lista de salidas en el rango
 */
export async function fetchSalidasEnRango(
    startDate: string,
    endDate: string
): Promise<ServiceResult<IngresoResponse[]>> {
    try {
        // Validar fechas básicas
        if (startDate > endDate) {
            return {
                ok: false,
                error: 'La fecha de inicio no puede ser mayor que la fecha de fin'
            };
        }
        
        // Llamar al backend con query SQL optimizada
        const data = await ingreso.getSalidasEnRango(startDate, endDate);
        
        return { ok: true, data };
        
    } catch (err: any) {
        console.error('Error al obtener salidas en rango:', err);
        return { ok: false, error: parseError(err) };
    }
}

// ============================================
// PUBLIC API - CONSULTAS
// ============================================

/**
 * Obtener ingresos abiertos CON alertas de tiempo
 * Usa el endpoint optimizado que calcula el estado de permanencia
 */
export async function fetchAbiertos(): Promise<ServiceResult<IngresoConEstadoResponse[]>> {
    try {
        const data = await ingreso.getIngresosAbiertosConAlertas();
        return { ok: true, data };
    } catch (err: any) {
        console.error('Error al obtener ingresos abiertos:', err);
        return { ok: false, error: parseError(err) };
    }
}

export async function fetchByGafete(gafeteNumero: string): Promise<ServiceResult<IngresoResponse | null>> {
    try {
        const data = await ingreso.getByGafete(gafeteNumero);
        return { ok: true, data };
    } catch (err: any) {
        console.error('Error al buscar ingreso por gafete:', err);
        return { ok: false, error: parseError(err) };
    }
}

// ============================================
// PUBLIC API - ALERTAS
// ============================================

export async function fetchAlertasGafetes(): Promise<ServiceResult<AlertaGafeteResponse[]>> {
    try {
        const data = await ingreso.getAlertasGafetes();
        return { ok: true, data };
    } catch (err: any) {
        console.error('Error al obtener alertas de gafetes:', err);
        return { ok: false, error: parseError(err) };
    }
}

export async function resolverAlerta(input: ResolverAlertaInput): Promise<ServiceResult<AlertaGafeteResponse>> {
    try {
        const data = await ingreso.resolverAlertaGafete(input);
        return { ok: true, data };
    } catch (err: any) {
        console.error('Error al resolver alerta:', err);
        return { ok: false, error: parseError(err) };
    }
}