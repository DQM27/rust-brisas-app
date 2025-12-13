// ==========================================
// src/lib/stores/visitaFormStore.ts
// ==========================================
// Store para el formulario de ingreso de VISITAS

import { writable, derived, get } from 'svelte/store';
import type { VisitaFormData, ValidacionVisitaResult } from '$lib/logic/ingreso/visitaService';
import { validarDatosVisita } from '$lib/logic/ingreso/visitaService';

// ==========================================
// ESTADO INICIAL
// ==========================================

const initialFormData: VisitaFormData = {
    cedula: '',
    nombre: '',
    apellido: '',
    empresa: '',
    anfitrion: '',
    areaVisitada: '',
    motivoVisita: '',
    tipoAutorizacion: 'correo',
    modoIngreso: 'caminando',
    vehiculoPlaca: undefined,
    gafeteNumero: undefined,
    observaciones: undefined,
};

// ==========================================
// STORES BASE
// ==========================================

/** Datos del formulario */
export const visitaFormData = writable<VisitaFormData>(initialFormData);

/** Indica si el formulario está siendo enviado */
export const isSubmitting = writable<boolean>(false);

/** Errores de validación del formulario */
export const formErrors = writable<Record<string, string>>({});

/** Mensaje de éxito/error después del submit */
export const submitMessage = writable<{ type: 'success' | 'error'; text: string } | null>(null);

// ==========================================
// STORES DERIVADOS
// ==========================================

/** Validación en tiempo real del formulario */
export const validacion = derived(
    visitaFormData,
    ($formData): ValidacionVisitaResult => {
        return validarDatosVisita($formData);
    }
);

/** Indica si el formulario es válido */
export const isFormValid = derived(
    validacion,
    ($validacion) => $validacion.isValid
);

/** Indica si se debe mostrar el campo de placa */
export const shouldShowPlaca = derived(
    visitaFormData,
    ($formData) => $formData.modoIngreso === 'vehiculo'
);

// ==========================================
// ACCIONES
// ==========================================

/** Actualiza un campo del formulario */
export function updateField<K extends keyof VisitaFormData>(
    field: K,
    value: VisitaFormData[K]
) {
    visitaFormData.update(data => ({
        ...data,
        [field]: value
    }));

    // Limpiar error del campo si existe
    formErrors.update(errors => {
        const { [field]: _, ...rest } = errors;
        return rest;
    });
}

/** Reinicia el formulario al estado inicial */
export function resetForm() {
    visitaFormData.set(initialFormData);
    formErrors.set({});
    submitMessage.set(null);
    isSubmitting.set(false);
}

/** Establece errores de validación */
export function setErrors(errors: Record<string, string>) {
    formErrors.set(errors);
}

/** Establece el estado de submitting */
export function setSubmitting(value: boolean) {
    isSubmitting.set(value);
}

/** Muestra mensaje de éxito */
export function showSuccess(message: string) {
    submitMessage.set({ type: 'success', text: message });
    setTimeout(() => submitMessage.set(null), 5000);
}

/** Muestra mensaje de error */
export function showError(message: string) {
    submitMessage.set({ type: 'error', text: message });
}

/** Cambia el modo de ingreso y limpia placa si es necesario */
export function toggleModoIngreso(modo: 'caminando' | 'vehiculo') {
    visitaFormData.update(data => ({
        ...data,
        modoIngreso: modo,
        vehiculoPlaca: modo === 'caminando' ? undefined : data.vehiculoPlaca
    }));
}
