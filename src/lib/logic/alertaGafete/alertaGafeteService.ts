import { alertaGafete } from "$lib/api/alertaGafete";
import type { AlertaGafeteResponse } from "$lib/types/ingreso";

// Result type similar to other services
export type ServiceResult<T> = {
    ok: true;
    data: T;
} | {
    ok: false;
    error: string;
};

/**
 * Resolver una alerta de gafete
 */
export async function resolverAlerta(
    alertaId: string,
    notas?: string
): Promise<ServiceResult<AlertaGafeteResponse>> {
    try {
        const data = await alertaGafete.resolver(alertaId, notas);
        return { ok: true, data };
    } catch (error) {
        return {
            ok: false,
            error: error instanceof Error ? error.message : String(error),
        };
    }
}

/**
 * Obtener alertas pendientes de una persona
 */
export async function getAlertasPendientesByCedula(
    cedula: string
): Promise<ServiceResult<AlertaGafeteResponse[]>> {
    try {
        const data = await alertaGafete.getPendientesByCedula(cedula);
        return { ok: true, data };
    } catch (error) {
        return {
            ok: false,
            error: error instanceof Error ? error.message : String(error),
        };
    }
}

/**
 * Obtener todas las alertas
 */
export async function getAllAlertas(
    resuelto?: boolean
): Promise<ServiceResult<AlertaGafeteResponse[]>> {
    try {
        const data = await alertaGafete.getAll(resuelto);
        return { ok: true, data };
    } catch (error) {
        return {
            ok: false,
            error: error instanceof Error ? error.message : String(error),
        };
    }
}
