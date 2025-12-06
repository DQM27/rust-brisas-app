// ============================================
// src/lib/logic/contratista/contratistaService.ts
// ============================================
// Servicio consolidado para contratista - reemplaza 6 archivos fragmentados

import { listContratistas, createContratista, updateContratista, deleteContratista, getContratista } from '$lib/api/contratista';
import { submitRegisterVehiculo } from '$lib/logic/vehiculo/submitRegisterVehiculo';
import { validateVehiculoInput } from '$lib/logic/vehiculo/validateVehiculoInput';
import { reindexAllContratistas } from '$lib/api/searchService';
import type { ContratistaResponse, CreateContratistaInput, UpdateContratistaInput } from '$lib/types/contratista';

// ============================================
// TYPES FOR RESULTS
// ============================================

export type ServiceResult<T> =
    | { ok: true; data: T }
    | { ok: false; error: string };

// ============================================
// VALIDATION
// ============================================

type ValidationResult =
    | { ok: true }
    | { ok: false; message: string };

function validateContratistaInput(
    nombre: string,
    apellido: string,
    cedula: string,
    empresaId: string,
    fechaVencimientoPraind: string
): ValidationResult {
    const n = (nombre || '').trim();
    const a = (apellido || '').trim();
    const c = (cedula || '').trim();
    const e = (empresaId || '').trim();
    const f = (fechaVencimientoPraind || '').trim();

    if (!n) return { ok: false, message: 'El nombre no puede estar vacío.' };
    if (n.length > 60) return { ok: false, message: 'Nombre demasiado largo.' };

    if (!a) return { ok: false, message: 'El apellido no puede estar vacío.' };
    if (a.length > 60) return { ok: false, message: 'Apellido demasiado largo.' };

    if (!c) return { ok: false, message: 'La cédula no puede estar vacía.' };
    if (c.length > 20) return { ok: false, message: 'La cédula es demasiado larga.' };

    if (!e) return { ok: false, message: 'Debe seleccionar una empresa.' };

    if (!f) return { ok: false, message: 'Debe ingresar la fecha de vencimiento.' };

    return { ok: true };
}

// ============================================
// ERROR PARSING
// ============================================

function parseError(err: any): string {
    if (!err) return 'Ocurrió un error desconocido.';

    if (typeof err === 'string') {
        if (/unique|cedula/i.test(err)) return 'Ya existe un contratista con esa cédula.';
        return err;
    }

    if (typeof err === 'object') {
        const msg = err.message ?? err.toString();
        if (/unique|cedula/i.test(msg)) return 'Ya existe un contratista con esa cédula.';
        return msg;
    }

    return 'Ocurrió un error inesperado.';
}

// ============================================
// PUBLIC API - FETCH OPERATIONS
// ============================================

/**
 * Obtener todos los contratistas
 */
export async function fetchAll(): Promise<ServiceResult<ContratistaResponse[]>> {
    try {
        const result = await listContratistas();
        return { ok: true, data: result.contratistas };
    } catch (err: any) {
        console.error('Error al cargar contratistas:', err);
        return { ok: false, error: parseError(err) };
    }
}

/**
 * Obtener solo contratistas activos
 */
export async function fetchActivos(): Promise<ServiceResult<ContratistaResponse[]>> {
    try {
        const result = await listContratistas();
        const activos = result.contratistas.filter(c => c.estado === "activo");
        return { ok: true, data: activos };
    } catch (err: any) {
        console.error('Error al cargar contratistas activos:', err);
        return { ok: false, error: parseError(err) };
    }
}

/**
 * Obtener un contratista por ID
 */
export async function fetchContratistaById(id: string): Promise<ServiceResult<ContratistaResponse>> {
    try {
        const contratista = await getContratista(id);
        return { ok: true, data: contratista };
    } catch (err: any) {
        console.error('Error al cargar contratista:', err);
        return { ok: false, error: parseError(err) };
    }
}

// ============================================
// PUBLIC API - CRUD OPERATIONS
// ============================================

/**
 * Registrar nuevo contratista (con validación y vehículo opcional)
 */
export async function register(input: CreateContratistaInput): Promise<ServiceResult<ContratistaResponse>> {
    const { nombre, apellido, cedula, empresaId, fechaVencimientoPraind, tieneVehiculo, tipoVehiculo, placa, marca, modelo, color } = input;

    // 1. Validar contratista
    const validation = validateContratistaInput(
        nombre,
        apellido,
        cedula,
        empresaId,
        fechaVencimientoPraind
    );

    if (!validation.ok) {
        return { ok: false, error: validation.message };
    }

    // 2. Si tiene vehículo, validar vehículo
    if (tieneVehiculo) {
        if (!tipoVehiculo) {
            return { ok: false, error: 'Debe seleccionar un tipo de vehículo.' };
        }

        const vehiculoValidation = validateVehiculoInput(
            tipoVehiculo,
            placa || '',
            marca
        );

        if (!vehiculoValidation.ok) {
            return { ok: false, error: vehiculoValidation.message };
        }
    }

    // 3. Crear contratista
    try {
        const contratista = await createContratista(input);

        // 4. Si tiene vehículo, registrarlo
        if (tieneVehiculo && tipoVehiculo && placa) {
            const vehiculoResult = await submitRegisterVehiculo({
                contratistaId: contratista.id,
                tipoVehiculo,
                placa,
                marca,
                modelo,
                color
            });

            if (!vehiculoResult.ok) {
                return { ok: false, error: `Error al registrar vehículo: ${vehiculoResult.error}` };
            }
        }

        // 5. Reindexar búsqueda
        await reindexAllContratistas();

        return { ok: true, data: contratista };
    } catch (err: any) {
        console.error('Error al registrar contratista:', err);
        return { ok: false, error: parseError(err) };
    }
}

/**
 * Actualizar contratista existente
 */
export async function update(id: string, input: UpdateContratistaInput): Promise<ServiceResult<ContratistaResponse>> {
    try {
        const contratista = await updateContratista(input);

        // Reindexar búsqueda
        await reindexAllContratistas();

        return { ok: true, data: contratista };
    } catch (err: any) {
        console.error('Error al actualizar contratista:', err);
        return { ok: false, error: parseError(err) };
    }
}

/**
 * Eliminar contratista
 */
export async function remove(id: string): Promise<ServiceResult<void>> {
    try {
        await deleteContratista(id);

        // Reindexar búsqueda
        await reindexAllContratistas();

        return { ok: true, data: undefined };
    } catch (err: any) {
        console.error('Error al eliminar contratista:', err);
        return { ok: false, error: parseError(err) };
    }
}

// ============================================
// COMPATIBILITY LAYER (para transición suave)
// ============================================

// Aliases para compatibilidad con código existente
export const fetchAllContratistas = fetchAll;
export const fetchActivosContratistas = fetchActivos;
export const registerContratista = register;

// Alias para submitRegisterContratista (mantiene el mismo tipo de retorno)
export async function submitRegisterContratista(
    input: CreateContratistaInput
): Promise<{ ok: true; contratista: ContratistaResponse } | { ok: false; error: string }> {
    const result = await register(input);
    if (result.ok) {
        return { ok: true, contratista: result.data };
    } else {
        return { ok: false, error: result.error };
    }
}

// Alias para submitFetchAllContratistas
export async function submitFetchAllContratistas(): Promise<{ ok: true; contratistas: ContratistaResponse[] } | { ok: false; error: string }> {
    const result = await fetchAll();
    if (result.ok) {
        return { ok: true, contratistas: result.data };
    } else {
        return { ok: false, error: result.error };
    }
}

// Alias para submitFetchActiveContratistas
export async function submitFetchActiveContratistas(): Promise<{ ok: true; contratistas: ContratistaResponse[] } | { ok: false; error: string }> {
    const result = await fetchActivos();
    if (result.ok) {
        return { ok: true, contratistas: result.data };
    } else {
        return { ok: false, error: result.error };
    }
}

