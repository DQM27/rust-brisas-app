import { invoke } from "@tauri-apps/api/core";
import {
    CreateIngresoContratistaSchema,
    RegistrarSalidaSchema,
    ResolverAlertaSchema,
    type CreateIngresoContratistaInput,
    type RegistrarSalidaInput,
    type ResolverAlertaInput,
    type IngresoResponse,
    type IngresoListResponse,
    type ValidacionIngresoResponse,
    type AlertaGafeteResponse
} from '$lib/types/ingreso';

export const ingreso = {
    // ==========================================
    // ENTRADA
    // ==========================================

    /**
     * Validar si un contratista puede ingresar
     */
    validarIngresoContratista: async (cedula: string): Promise<ValidacionIngresoResponse> => {
        return await invoke('validar_ingreso_contratista', { cedula });
    },

    /**
     * Crear ingreso de contratista
     */
    crearIngresoContratista: async (input: CreateIngresoContratistaInput): Promise<IngresoResponse> => {
        const validated = CreateIngresoContratistaSchema.parse(input);
        return await invoke('crear_ingreso_contratista', { input: validated });
    },

    // ==========================================
    // SALIDA
    // ==========================================

    /**
     * Validar si una persona puede salir (reglas de negocio)
     */
    validarPuedeSalir: async (id: string): Promise<{ puedeSalir: boolean, mensaje?: string }> => {
        return await invoke('validar_puede_salir', { id });
    },

    /**
     * Registrar salida
     */
    registrarSalida: async (input: RegistrarSalidaInput): Promise<IngresoResponse> => {
        const validated = RegistrarSalidaSchema.parse(input);
        return await invoke('registrar_salida', { input: validated });
    },

    /**
     * Registrar salida verificando gafete (físico)
     */
    registrarSalidaConGafete: async (ingresoId: string, gafeteNumero: string, usuarioSalidaId: string): Promise<IngresoResponse> => {
        return await invoke('registrar_salida_con_verificacion_gafete', { ingresoId, gafeteNumero, usuarioSalidaId });
    },

    /**
     * Obtener salidas del día actual
     */
    getSalidasDelDia: async (): Promise<IngresoListResponse> => {
        return await invoke('get_salidas_del_dia');
    },

    // ==========================================
    // CONSULTAS GENERALES
    // ==========================================

    getById: async (id: string): Promise<IngresoResponse> => {
        return await invoke('get_ingreso_by_id', { id });
    },

    getAll: async (): Promise<IngresoListResponse> => {
        return await invoke('get_all_ingresos');
    },

    getAbiertos: async (): Promise<IngresoListResponse> => {
        return await invoke('get_ingresos_abiertos');
    },

    getByGafete: async (gafeteNumero: string): Promise<IngresoResponse | null> => {
        return await invoke('get_ingreso_by_gafete', { gafeteNumero });
    },

    // ==========================================
    // PERMANENCIA & ALERTAS
    // ==========================================

    getResumenPermanencias: async (): Promise<any> => {
        return await invoke('get_resumen_permanencias');
    },

    getAlertasGafetes: async (): Promise<AlertaGafeteResponse[]> => {
        return await invoke('get_all_alertas_gafetes');
    },

    resolverAlertaGafete: async (input: ResolverAlertaInput): Promise<AlertaGafeteResponse> => {
        const validated = ResolverAlertaSchema.parse(input);
        return await invoke('resolver_alerta_gafete', { input: validated });
    }
};
