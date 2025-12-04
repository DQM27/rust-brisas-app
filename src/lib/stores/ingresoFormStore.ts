import { writable } from 'svelte/store';
import type { IngresoFormState } from '$lib/types/ingreso-form.types';

// ==========================================
// ESTADO INICIAL
// ==========================================

const initialState: IngresoFormState = {
    // Datos del contratista
    contratistaId: '',
    contratistaNombre: '',
    contratistaData: null,
    puedeIngresar: false,
    mensajeValidacion: '',

    // Datos del ingreso
    modoIngreso: 'caminando',
    vehiculoId: null,
    gafeteNumero: '',
    tipoAutorizacion: 'praind',
    observaciones: ''
};

// ==========================================
// STORE FACTORY
// ==========================================

/**
 * Store para el formulario de ingreso
 * 
 * Responsabilidades:
 * - Mantener el estado del formulario
 * - Proveer métodos simples de actualización
 * - NO contiene lógica de negocio
 * 
 * La lógica de validación y auto-selección está en ingresoService
 */
function createIngresoFormStore() {
    const { subscribe, set, update } = writable<IngresoFormState>(initialState);

    return {
        subscribe,

        // ==========================================
        // MÉTODOS DE ACTUALIZACIÓN INDIVIDUAL
        // ==========================================

        /**
         * Actualizar un campo específico del formulario
         */
        setField: <K extends keyof IngresoFormState>(
            field: K,
            value: IngresoFormState[K]
        ) => {
            update(state => ({ ...state, [field]: value }));
        },

        /**
         * Actualizar múltiples campos a la vez
         */
        updateFields: (fields: Partial<IngresoFormState>) => {
            update(state => ({ ...state, ...fields }));
        },

        // ==========================================
        // MÉTODOS ESPECÍFICOS DE CONTRATISTA
        // ==========================================

        /**
         * Establecer datos del contratista después de validación
         * 
         * NOTA: Este método solo actualiza el estado.
         * La lógica de validación debe ejecutarse en el service ANTES de llamar esto.
         */
        setContratistaValidado: (data: {
            contratistaId: string;
            contratistaNombre: string;
            contratistaData: any;
            puedeIngresar: boolean;
            mensajeValidacion: string;
        }) => {
            update(state => ({
                ...state,
                contratistaId: data.contratistaId,
                contratistaNombre: data.contratistaNombre,
                contratistaData: data.contratistaData,
                puedeIngresar: data.puedeIngresar,
                mensajeValidacion: data.mensajeValidacion
            }));
        },

        /**
         * Limpiar datos del contratista
         */
        clearContratista: () => {
            update(state => ({
                ...state,
                contratistaId: '',
                contratistaNombre: '',
                contratistaData: null,
                puedeIngresar: false,
                mensajeValidacion: ''
            }));
        },

        // ==========================================
        // MÉTODOS ESPECÍFICOS DE INGRESO
        // ==========================================

        /**
         * Establecer modo de ingreso y vehículo
         * 
         * NOTA: La validación debe hacerse en el service antes.
         */
        setModoIngreso: (modo: 'caminando' | 'vehiculo', vehiculoId: string | null = null) => {
            update(state => ({
                ...state,
                modoIngreso: modo,
                vehiculoId: modo === 'caminando' ? null : vehiculoId
            }));
        },

        /**
         * Establecer vehículo seleccionado
         */
        setVehiculo: (vehiculoId: string | null) => {
            update(state => ({ ...state, vehiculoId }));
        },

        /**
         * Establecer número de gafete
         */
        setGafete: (gafeteNumero: string) => {
            update(state => ({ ...state, gafeteNumero }));
        },

        /**
         * Establecer tipo de autorización
         */
        setTipoAutorizacion: (tipo: string) => {
            update(state => ({ ...state, tipoAutorizacion: tipo }));
        },

        /**
         * Establecer observaciones
         */
        setObservaciones: (observaciones: string) => {
            update(state => ({ ...state, observaciones }));
        },

        // ==========================================
        // MÉTODOS DE RESET
        // ==========================================

        /**
         * Resetear todo el formulario al estado inicial
         */
        reset: () => {
            set(initialState);
        },

        /**
         * Resetear solo los campos de ingreso, mantener contratista
         * Útil para registrar múltiples ingresos del mismo contratista
         */
        resetIngresoFields: () => {
            update(state => ({
                ...state,
                modoIngreso: 'caminando',
                vehiculoId: null,
                gafeteNumero: '',
                tipoAutorizacion: 'praind',
                observaciones: ''
            }));
        },

        /**
         * Aplicar auto-selección calculada por el service
         * 
         * Este método recibe el resultado del service y lo aplica al estado.
         * NO contiene lógica de negocio, solo actualiza el estado.
         */
        aplicarAutoSeleccion: (autoSeleccion: {
            modoSugerido: 'caminando' | 'vehiculo';
            vehiculoSugerido: string | null;
        }) => {
            update(state => ({
                ...state,
                modoIngreso: autoSeleccion.modoSugerido,
                vehiculoId: autoSeleccion.vehiculoSugerido
            }));
        }
    };
}

// ==========================================
// EXPORT
// ==========================================

export const ingresoFormStore = createIngresoFormStore();