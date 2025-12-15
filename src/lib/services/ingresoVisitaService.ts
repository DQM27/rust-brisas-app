import { invoke } from "@tauri-apps/api/core";
import type { IngresoVisita } from "$lib/types/ingreso-nuevos";

export interface CreateIngresoVisitaFullInput {
    // Datos Visitante
    cedula: string;
    nombre: string;
    apellido: string;
    empresa?: string;

    // Datos Ingreso
    anfitrion: string;
    area_visitada: string;
    motivo: string;
    gafete?: string;
    observaciones?: string;
    usuario_ingreso_id: string;

    // Opcional
    cita_id?: string;
}

export const ingresoVisitaService = {
    async createIngreso(input: CreateIngresoVisitaFullInput): Promise<IngresoVisita> {
        return await invoke<IngresoVisita>("crear_ingreso_visita_v2", { input });
    },

    async getActivos(): Promise<IngresoVisita[]> {
        return await invoke<IngresoVisita[]>("get_ingresos_visitas_activos");
    },

    /** Obtiene historial de visitas completadas */
    async getHistorial(): Promise<IngresoVisita[]> {
        // TODO: Backend debe implementar get_ingresos_visitas_historial
        // Por ahora retorna array vacío
        try {
            return await invoke<IngresoVisita[]>("get_ingresos_visitas_historial");
        } catch {
            console.warn("get_ingresos_visitas_historial no implementado en backend");
            return [];
        }
    },

    async registrarSalida(id: string, usuarioId: string, observaciones?: string): Promise<void> {
        return await invoke("registrar_salida_visita", { id, usuarioId, observaciones });
    }
};
