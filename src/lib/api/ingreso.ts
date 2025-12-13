import { invoke } from "@tauri-apps/api/core";
import {
    CreateIngresoContratistaSchema,
    CreateIngresoVisitaSchema,
    CreateIngresoProveedorSchema,
    RegistrarSalidaSchema,
    ResolverAlertaSchema,
    type CreateIngresoContratistaInput,
    type CreateIngresoVisitaInput,
    type CreateIngresoProveedorInput,
    type RegistrarSalidaInput,
    type ResolverAlertaInput,
    type IngresoResponse,
    type IngresoConEstadoResponse,
    type IngresoListResponse,
    type ValidacionIngresoResponse,
    type AlertaGafeteResponse,
    type ResumenPermanencias,
    type AlertaTiempoExcedido,
    type AlertaListaNegra,
} from '$lib/types/ingreso';

export const ingreso = {
    // ==========================================
    // ENTRADA
    // ==========================================

    /**
     * Validar si un contratista puede ingresar
     */
    validarIngresoContratista: async (contratistaId: string): Promise<ValidacionIngresoResponse> => {
        return await invoke('validar_ingreso_contratista', { contratistaId });
    },

    /**
     * Crear ingreso de contratista
     */
    crearIngresoContratista: async (input: CreateIngresoContratistaInput): Promise<IngresoResponse> => {
        const validated = CreateIngresoContratistaSchema.parse(input);
        return await invoke('crear_ingreso_contratista', { input: validated });
    },

    /**
     * Crear ingreso de visita
     */
    crearIngresoVisita: async (input: CreateIngresoVisitaInput): Promise<IngresoResponse> => {
        const validated = CreateIngresoVisitaSchema.parse(input);
        return await invoke('crear_ingreso_visita', { input: validated });
    },

    /**
     * Crear ingreso de proveedor
     */
    crearIngresoProveedor: async (input: CreateIngresoProveedorInput): Promise<IngresoResponse> => {
        const validated = CreateIngresoProveedorSchema.parse(input);
        return await invoke('crear_ingreso_proveedor', { input: validated });
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
     * Obtener salidas del día actual o de una fecha específica
     */
    getSalidasDelDia: async (fecha?: string): Promise<IngresoResponse[]> => {
        const targetDate = fecha || new Date().toISOString().split('T')[0]; // YYYY-MM-DD
        return await invoke('get_salidas_del_dia', { fecha: targetDate });
    },

    /**
     * Obtener salidas en un rango de fechas (query optimizada en backend)
     */
    getSalidasEnRango: async (fechaInicio: string, fechaFin: string): Promise<IngresoResponse[]> => {
        return await invoke('get_salidas_en_rango', { 
            fechaInicio, 
            fechaFin 
        });
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

    getAbiertos: async (): Promise<IngresoResponse[]> => {
        return await invoke('get_ingresos_abiertos');
    },

    getByGafete: async (gafeteNumero: string): Promise<IngresoResponse | null> => {
        return await invoke('get_ingreso_by_gafete', { gafeteNumero });
    },

    // ==========================================
    // PERMANENCIA & ALERTAS
    // ==========================================

    /**
     * Obtener ingreso por ID con estado de permanencia calculado
     */
    getIngresoConEstado: async (ingresoId: string): Promise<IngresoConEstadoResponse> => {
        return await invoke('get_ingreso_con_estado', { ingresoId });
    },

    /**
     * Obtener todos los ingresos abiertos con alertas de tiempo
     */
    getIngresosAbiertosConAlertas: async (): Promise<IngresoConEstadoResponse[]> => {
        return await invoke('get_ingresos_abiertos_con_alertas');
    },

    /**
     * Verificar contratistas que excedieron el tiempo límite (>= 14h)
     */
    verificarTiemposExcedidos: async (): Promise<AlertaTiempoExcedido[]> => {
        return await invoke('verificar_tiempos_excedidos');
    },

    /**
     * Verificar contratistas próximos al límite (>= 13h 30min)
     */
    verificarAlertasTempranas: async (): Promise<AlertaTiempoExcedido[]> => {
        return await invoke('verificar_alertas_tempranas');
    },

    /**
     * Verificar si un contratista fue bloqueado mientras estaba dentro
     */
    verificarCambioListaNegra: async (ingresoId: string): Promise<AlertaListaNegra | null> => {
        return await invoke('verificar_cambio_lista_negra', { ingresoId });
    },

    /**
     * Verificar cambios en lista negra para todos los ingresos abiertos
     */
    verificarCambiosListaNegraMasivo: async (): Promise<AlertaListaNegra[]> => {
        return await invoke('verificar_cambios_lista_negra_masivo');
    },

    /**
     * Obtener resumen de estado de todos los ingresos abiertos
     */
    getResumenPermanencias: async (): Promise<ResumenPermanencias> => {
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