// ============================================
// src/lib/logic/ingreso/ingresoService.ts
// ============================================

import { ingreso } from '$lib/api/ingreso';
import type {
    IngresoResponse,
    IngresoListResponse,
    ValidacionIngresoResponse,
    CreateIngresoContratistaInput,
    RegistrarSalidaInput,
    ResolverAlertaInput,
    AlertaGafeteResponse
} from '$lib/types/ingreso';
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
export async function fetchSalidasDelDia(): Promise<ServiceResult<IngresoListResponse>> {
    try {
        const data = await ingreso.getSalidasDelDia();
        return { ok: true, data };
    } catch (err: any) {
        console.error('Error al obtener salidas del día:', err);
        return { ok: false, error: parseError(err) };
    }
}

// ============================================
// PUBLIC API - CONSULTAS
// ============================================

export async function fetchAbiertos(): Promise<ServiceResult<IngresoListResponse>> {
    try {
        const data = await ingreso.getAbiertos();
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
