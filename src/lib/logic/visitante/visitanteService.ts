
import { visitante } from '$lib/api/visitante';
import type { VisitanteResponse, CreateVisitanteInput } from '$lib/types/visitante';

export type ServiceResult<T> =
    | { ok: true; data: T }
    | { ok: false; error: string };

export async function createVisitante(input: CreateVisitanteInput): Promise<ServiceResult<VisitanteResponse>> {
    try {
        const data = await visitante.create(input);
        return { ok: true, data };
    } catch (err: any) {
        return { ok: false, error: parseError(err) };
    }
}

export async function searchVisitantes(query: string): Promise<ServiceResult<VisitanteResponse[]>> {
    try {
        const data = await visitante.search(query);
        return { ok: true, data };
    } catch (err: any) {
        return { ok: false, error: parseError(err) };
    }
}

export async function getVisitanteByCedula(cedula: string): Promise<ServiceResult<VisitanteResponse | null>> {
    try {
        const data = await visitante.getByCedula(cedula);
        return { ok: true, data };
    } catch (err: any) {
        return { ok: false, error: parseError(err) };
    }
}

export async function updateVisitante(id: string, input: CreateVisitanteInput): Promise<ServiceResult<VisitanteResponse>> {
    try {
        const data = await visitante.update(id, input);
        return { ok: true, data };
    } catch (err: any) {
        return { ok: false, error: parseError(err) };
    }
}

export async function deleteVisitante(id: string): Promise<ServiceResult<void>> {
    try {
        await visitante.delete(id);
        return { ok: true, data: undefined };
    } catch (err: any) {
        return { ok: false, error: parseError(err) };
    }
}

export async function restoreVisitante(id: string): Promise<ServiceResult<VisitanteResponse>> {
    try {
        const data = await visitante.restore(id);
        return { ok: true, data };
    } catch (err: any) {
        return { ok: false, error: parseError(err) };
    }
}

export async function getArchivedVisitantes(): Promise<ServiceResult<VisitanteResponse[]>> {
    try {
        const data = await visitante.listArchived();
        return { ok: true, data };
    } catch (err: any) {
        return { ok: false, error: parseError(err) };
    }
}

function parseError(err: any): string {
    if (typeof err === 'string') {
        if (/unique|cedula/i.test(err)) return 'La cédula ya existe.';
        return err;
    }
    return 'Error desconocido';
}
