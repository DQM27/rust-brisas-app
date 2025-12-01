import { writable } from 'svelte/store';
import type { GafeteResponse } from '$lib/types/gafete';

export interface IngresoFormState {
    contratistaId: string;
    contratistaNombre: string;
    contratistaData: any | null;
    puedeIngresar: boolean;
    mensajeValidacion: string;

    modoIngreso: "caminando" | "vehiculo";
    vehiculoId: string | null;
    gafeteNumero: string;
    tipoAutorizacion: string;
    observaciones: string;
}

const initialState: IngresoFormState = {
    contratistaId: "",
    contratistaNombre: "",
    contratistaData: null,
    puedeIngresar: false,
    mensajeValidacion: "",

    modoIngreso: "caminando",
    vehiculoId: null,
    gafeteNumero: "",
    tipoAutorizacion: "praind",
    observaciones: ""
};

function createIngresoFormStore() {
    const { subscribe, set, update } = writable<IngresoFormState>(initialState);

    return {
        subscribe,

        // Actualizar un campo específico
        setField: (field: keyof IngresoFormState, value: any) => {
            update(state => ({ ...state, [field]: value }));
        },

        // Establecer datos del contratista validado
        setContratista: (data: any, puedeIngresar: boolean, mensaje: string = "") => {
            update(state => {
                // Lógica de auto-selección de vehículo
                let modo: "caminando" | "vehiculo" = "caminando";
                let vehiculoId: string | null = null;

                if (puedeIngresar && data?.vehiculos?.length === 1) {
                    modo = "vehiculo";
                    vehiculoId = data.vehiculos[0].id;
                }

                return {
                    ...state,
                    contratistaId: data?.id || "",
                    contratistaNombre: data?.nombreCompleto || "",
                    contratistaData: data,
                    puedeIngresar,
                    mensajeValidacion: mensaje,
                    modoIngreso: modo,
                    vehiculoId: vehiculoId
                };
            });
        },

        // Limpiar todo el formulario
        reset: () => {
            set(initialState);
        },

        // Limpiar solo campos de ingreso (mantener contratista)
        resetIngresoFields: () => {
            update(state => ({
                ...state,
                gafeteNumero: "",
                observaciones: "",
                // Mantener modo y vehículo si ya estaban seleccionados
            }));
        }
    };
}

export const ingresoFormStore = createIngresoFormStore();
