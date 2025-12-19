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
    /**
     * Validar si un contratista puede ingresar (Sincronizado con backend unificado)
     */
    validarIngresoContratista: async (contratistaId: string): Promise<ValidacionIngresoResponse> => {
        return await invoke('validate_ingreso_contratista', { contratistaId });
    },

    /**
     * Crear ingreso de contratista
     */
    crearIngresoContratista: async (input: CreateIngresoContratistaInput): Promise<IngresoResponse> => {
        const validated = CreateIngresoContratistaSchema.parse(input);
        return await invoke('create_ingreso_contratista', {
            input: validated,
            usuarioId: validated.usuarioIngresoId
        });
    },

    /**
     * Crear ingreso de visita
     */
    crearIngresoVisita: async (input: CreateIngresoVisitaInput): Promise<IngresoResponse> => {
        const validated = CreateIngresoVisitaSchema.parse(input);
        return await invoke('crear_ingreso_visita_v2', { input: validated });
    },

    /**
     * Crear ingreso de proveedor
     */
    crearIngresoProveedor: async (input: CreateIngresoProveedorInput): Promise<IngresoResponse> => {
        const validated = CreateIngresoProveedorSchema.parse(input);
        return await invoke('crear_ingreso_proveedor_v2', { input: validated });
    },

    // ==========================================
    // SALIDA
    // ==========================================

    /**
     * Validar si una persona puede salir (Específico Contratistas por ahora)
     */
    validarPuedeSalir: async (id: string): Promise<{ puedeSalir: boolean, mensaje?: string }> => {
        // TODO: Manejar tipos si es necesario, por ahora apunta a Contratista
        return await invoke('validate_exit_contratista', { ingresoId: id, gafeteDevuelto: null });
    },

    /**
     * Registrar salida (Contratistas)
     */
    registrarSalida: async (input: RegistrarSalidaInput): Promise<IngresoResponse> => {
        const validated = RegistrarSalidaSchema.parse(input);
        return await invoke('register_exit_contratista', { input: validated, usuarioId: input.usuarioSalidaId || "unknown" });
    },

    /**
     * Registrar salida verificando gafete (físico)
     */
    registrarSalidaConGafete: async (ingresoId: string, gafeteNumero: string, usuarioSalidaId: string): Promise<IngresoResponse> => {
        // Reutilizamos el comando de registro normal con validación extra si el backend lo soporta, 
        // o llamamos a register_exit_contratista asumiendo validaciones internas
        throw new Error("Este endpoint requiere migración a register_exit_contratista con parámetros adecuados");
    },

    // ... (Getters de salidas del dia se mantienen si existen en backend o se comentan)

    /**
     * Obtener salidas del día actual o de una fecha específica
     */
    getSalidasDelDia: async (fecha?: string): Promise<IngresoResponse[]> => {
        const targetDate = fecha || new Date().toISOString().split('T')[0];
        return await invoke('get_salidas_del_dia', { fecha: targetDate });
    },

    /**
     * Obtener salidas en un rango de fechas
     */
    getSalidasEnRango: async (fechaInicio: string, fechaFin: string): Promise<IngresoResponse[]> => {
        return await invoke('get_salidas_en_rango', {
            fechaInicio,
            fechaFin
        });
    },

    // ...

    // ==========================================
    // PERMANENCIA & ALERTAS
    // ==========================================

    /**
     * Obtener ingreso por ID con estado de permanencia calculado
     */
    getIngresoConEstado: async (ingresoId: string): Promise<IngresoConEstadoResponse> => {
        // Fallback: Obtener todos y filtrar (ineficiente pero funcional sin endpoint específico)
        const all = await invoke<IngresoConEstadoResponse[]>('get_permanencia_status');
        const found = all.find(i => i.id === ingresoId);
        if (!found) throw new Error("Ingreso no encontrado en activos");
        return found;
    },

    /**
     * Obtener todos los ingresos abiertos con alertas de tiempo
     */
    getIngresosAbiertosConAlertas: async (): Promise<IngresoConEstadoResponse[]> => {
        return await invoke('get_permanencia_status');
    },

    /**
     * Verificar contratistas que excedieron el tiempo límite (>= 14h)
     */
    verificarTiemposExcedidos: async (): Promise<AlertaTiempoExcedido[]> => {
        return await invoke('check_time_alerts');
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