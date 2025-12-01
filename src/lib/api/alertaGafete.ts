import { invoke } from "@tauri-apps/api/core";
import type { AlertaGafeteResponse } from "$lib/types/ingreso";

export const alertaGafete = {
    /**
     * Resolver una alerta de gafete (marcar como devuelto/pagado)
     */
    resolver: async (
        alertaId: string,
        notas?: string
    ): Promise<AlertaGafeteResponse> => {
        return await invoke("resolver_alerta_gafete", {
            input: {
                alertaId,
                notas,
            },
        });
    },

    /**
     * Obtener alertas pendientes por cédula
     */
    getPendientesByCedula: async (cedula: string): Promise<AlertaGafeteResponse[]> => {
        return await invoke("get_alertas_pendientes_by_cedula", {
            cedula,
        });
    },

    /**
     * Obtener todas las alertas (opcionalmente filtradas por estado resuelto)
     */
    getAll: async (resuelto?: boolean): Promise<AlertaGafeteResponse[]> => {
        return await invoke("get_all_alertas_gafetes", {
            resuelto,
        });
    },
};
