// ==========================================
// src/lib/stores/proveedorFormStore.ts
// ==========================================
// Store para el formulario de ingreso de PROVEEDORES

import { writable, derived } from 'svelte/store';
import type {
	ProveedorFormData,
	ValidacionProveedorResult
} from '$lib/logic/ingreso/proveedorService';
import { validarDatosProveedor } from '$lib/logic/ingreso/proveedorService';
import type { ProveedorResponse } from '$lib/types/proveedor';
import type { AlertaGafeteResponse } from '$lib/types/ingreso';

interface ProveedorWithVehicles extends ProveedorResponse {
	vehiculos?: {
		id: string;
		placa: string;
		marca: string;
		modelo: string;
		color: string;
		tipo_vehiculo: string;
	}[];
}

// ==========================================
// ESTADO INICIAL
// ==========================================

// Extended state including validation info
export interface ProveedorFormState extends ProveedorFormData {
	// Estado de validación
	proveedorId: string;
	proveedorNombre: string;
	proveedorData: ProveedorWithVehicles | null;
	puedeIngresar: boolean;
	mensajeValidacion: string;
	alertas: AlertaGafeteResponse[];
	vehiculoId?: string;
}

const initialFormData: ProveedorFormState = {
	cedula: '',
	nombre: '',
	apellido: '',
	empresaId: '',
	areaVisitada: '',
	motivo: '',
	modoIngreso: 'caminando',
	vehiculoPlaca: undefined,
	vehiculoMarca: undefined,
	vehiculoModelo: undefined,
	vehiculoColor: undefined,
	vehiculoTipo: 'automovil',
	gafeteNumero: undefined,
	observaciones: undefined,

	// Validation state
	proveedorId: '',
	proveedorNombre: '',
	proveedorData: null,
	puedeIngresar: false,
	mensajeValidacion: '',
	alertas: [],
	vehiculoId: undefined
};

// ==========================================
// STORES BASE
// ==========================================

/** Datos del formulario */
export const proveedorFormData = writable<ProveedorFormState>(initialFormData);

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
export const validacion = derived(proveedorFormData, ($formData): ValidacionProveedorResult => {
	return validarDatosProveedor($formData);
});

/** Indica si el formulario es válido */
export const isFormValid = derived(validacion, ($validacion) => $validacion.isValid);

/** Indica si se debe mostrar el campo de placa */
export const shouldShowPlaca = derived(
	proveedorFormData,
	($formData) => $formData.modoIngreso === 'vehiculo'
);

// ==========================================
// ACCIONES
// ==========================================

/** Actualiza un campo del formulario */
export function updateField<K extends keyof ProveedorFormState>(
	field: K,
	value: ProveedorFormState[K]
) {
	proveedorFormData.update((data) => ({
		...data,
		[field]: value
	}));

	// Limpiar error del campo si existe
	formErrors.update((errors) => {
		const { [field]: _, ...rest } = errors;
		return rest;
	});
}

/** Reinicia el formulario al estado inicial */
export function resetForm() {
	proveedorFormData.set(initialFormData);
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
	proveedorFormData.update((data) => ({
		...data,
		modoIngreso: modo,
		// Si cambiamos a vehículo, tratar de preservar datos si ya existían o usar seleccionado
		vehiculoPlaca: modo === 'caminando' ? undefined : data.vehiculoPlaca
	}));
}

/** Establece el proveedor validado */
export function setProveedorValidado(data: {
	proveedorId: string;
	proveedorNombre: string;
	proveedorData: ProveedorWithVehicles;
	puedeIngresar: boolean;
	mensajeValidacion: string;
	alertas: AlertaGafeteResponse[];
}) {
	proveedorFormData.update((state) => ({
		...state,
		...data,
		// Pre-fill fields from provider data
		cedula: data.proveedorData?.cedula || state.cedula,
		nombre: data.proveedorData?.nombre || state.nombre,
		apellido: data.proveedorData?.apellido || state.apellido,
		empresaId: data.proveedorData?.empresaId || state.empresaId
	}));
}

/** Selecciona un vehículo del catálogo */
export function setVehiculoCatalogo(vehiculoId: string | null) {
	proveedorFormData.update((state) => {
		if (!vehiculoId) {
			return {
				...state,
				vehiculoId: undefined,
				vehiculoPlaca: undefined,
				vehiculoMarca: undefined,
				vehiculoModelo: undefined,
				vehiculoColor: undefined
			};
		}

		const vehiculo = state.proveedorData?.vehiculos?.find((v) => v.id === vehiculoId);
		if (vehiculo) {
			return {
				...state,
				vehiculoId,
				vehiculoPlaca: vehiculo.placa,
				vehiculoMarca: vehiculo.marca,
				vehiculoModelo: vehiculo.modelo,
				vehiculoColor: vehiculo.color,
				vehiculoTipo: vehiculo.tipo_vehiculo
			};
		}
		return state;
	});
}
