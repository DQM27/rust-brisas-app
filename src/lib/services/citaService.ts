import { invoke } from "@tauri-apps/api/core";
import type { CitaPopulated, CreateCitaInput, CreateVisitanteInput, Cita } from "$lib/types/cita";

export const citaService = {
    async createCita(cita: CreateCitaInput, visitante: CreateVisitanteInput | null): Promise<Cita> {
        return await invoke<Cita>("create_cita", { cita, visitante });
    },

    async getCitasHoy(): Promise<CitaPopulated[]> {
        return await invoke<CitaPopulated[]>("get_citas_hoy");
    },

    /** Obtiene citas pendientes (por ahora usa get_citas_hoy) */
    async getCitasPendientes(): Promise<CitaPopulated[]> {
        // TODO: Backend debe implementar get_citas_pendientes para incluir fechas futuras
        return await invoke<CitaPopulated[]>("get_citas_hoy");
    },

    async procesarIngresoCita(citaId: string, gafete: string, usuarioId: string): Promise<string> {
        return await invoke<string>("procesar_ingreso_cita", { citaId, gafete, usuarioId });
    },

    /** Cancela una cita pendiente */
    async cancelarCita(citaId: string): Promise<void> {
        return await invoke<void>("cancelar_cita", { citaId });
    }
};
