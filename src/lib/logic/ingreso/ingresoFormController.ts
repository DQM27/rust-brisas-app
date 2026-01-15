// ==========================================
// src/lib/logic/ingreso/ingresoFormController.ts
// ==========================================

import { get } from 'svelte/store';
import { toast } from 'svelte-5-french-toast';
import { ingresoFormStore } from '$lib/stores/ingresoFormStore';
import { ingresoStore } from '$lib/stores/ingresoStore';
import * as ingresoService from './ingresoService';
import type { CreateIngresoContratistaInput } from '$lib/types/ingreso';
import type { GafeteResponse } from '$lib/types/gafete';
import type { ServiceResult } from './ingresoService';

/**
 * Controlador del formulario de ingreso
 *
 * Responsabilidades:
 * - Orquestar llamadas al service
 * - Actualizar el store según resultados
 * - Manejar feedback al usuario (toasts)
 * - Coordinar validaciones
 *
 * Este archivo contiene la "presentation logic" o "view model"
 * separada de los componentes UI
 */

// ==========================================
// CONTRATISTA - BÚSQUEDA Y VALIDACIÓN
// ==========================================

/**
 * Buscar y validar contratista para ingreso
 *
 * Orquesta:
 * 1. Llamada al service para validar
 * 2. Mostrar toasts según resultado
 * 3. Actualizar store con datos validados
 * 4. Aplicar auto-selección de vehículo
 *
 * @param contratistaId - ID del contratista a validar
 * @returns true si puede ingresar, false si no
 */
export async function buscarYValidarContratista(contratistaId: string): Promise<boolean> {
	// 1. Llamar al service para preparar todo
	const result = await ingresoService.prepararFormularioIngreso(contratistaId);

	if (!result.ok) {
		// Error en la validación
		toast.error(result.error);
		limpiarContratista();
		return false;
	}

	const { validacion, autoSeleccion } = result.data;

	// 2. Actualizar store con datos del contratista
	ingresoFormStore.setContratistaValidado({
		contratistaId: validacion.contratista?.id || contratistaId,
		contratistaNombre: validacion.contratista?.nombreCompleto || '',
		contratistaData: validacion.contratista,
		puedeIngresar: validacion.puedeIngresar,
		mensajeValidacion: validacion.puedeIngresar ? '' : 'Contratista no autorizado para ingresar.'
	});

	// 3. Aplicar auto-selección de vehículo
	ingresoFormStore.aplicarAutoSeleccion({
		modoSugerido: autoSeleccion.suggestedMode,
		vehiculoSugerido: autoSeleccion.suggestedVehicleId
	});

	// 4. Mostrar alertas si existen
	if (validacion.alertas && validacion.alertas.length > 0) {
		toast('Tiene alertas pendientes de gafetes', { icon: '⚠️' });
	}

	return true;
}

/**
 * Limpiar datos del contratista del formulario
 */
export function limpiarContratista(): void {
	ingresoFormStore.clearContratista();
}

// ==========================================
// MODO DE INGRESO Y VEHÍCULO
// ==========================================

/**
 * Cambiar modo de ingreso (caminando/vehículo)
 *
 * @param modo - Nuevo modo de ingreso
 * @param contratistaData - Datos del contratista (para validar vehículos)
 */
export function cambiarModoIngreso(modo: 'caminando' | 'vehiculo', contratistaData: any): void {
	const tieneVehiculos = contratistaData?.vehiculos?.length > 0;

	// Si cambia a vehículo pero no tiene vehículos, mostrar error
	if (modo === 'vehiculo' && !tieneVehiculos) {
		toast.error('El contratista no tiene vehículos registrados');
		return;
	}

	// Si cambia a caminando, limpiar vehículo
	if (modo === 'caminando') {
		ingresoFormStore.setModoIngreso('caminando', null);
		return;
	}

	// Si cambia a vehículo, mantener el vehículo actual si existe
	const estadoActual = get(ingresoFormStore);
	ingresoFormStore.setModoIngreso('vehiculo', estadoActual.vehiculoId);
}

/**
 * Seleccionar vehículo específico
 *
 * @param vehiculoId - ID del vehículo seleccionado
 */
export function seleccionarVehiculo(vehiculoId: string | null): void {
	ingresoFormStore.setVehiculo(vehiculoId);
}

// ==========================================
// VALIDACIÓN DE GAFETE
// ==========================================

/**
 * Validar número de gafete contra lista de disponibles
 *
 * @param gafeteNumero - Número de gafete a validar
 * @param gafetesDisponibles - Lista de gafetes disponibles
 * @returns Resultado de validación con sugerencias
 */
export function validarGafete(
	gafeteNumero: string,
	gafetesDisponibles: GafeteResponse[]
): ServiceResult<{ isValid: boolean; suggestions: string[] }> {
	return ingresoService.validarGafete({
		gafeteNumero,
		gafetesDisponibles
	});
}

/**
 * Establecer número de gafete
 *
 * @param gafeteNumero - Número de gafete
 */
export function establecerGafete(gafeteNumero: string): void {
	ingresoFormStore.setGafete(gafeteNumero);
}

// ==========================================
// CAMPOS ADICIONALES
// ==========================================

/**
 * Establecer tipo de autorización
 */
export function establecerTipoAutorizacion(tipo: string): void {
	ingresoFormStore.setTipoAutorizacion(tipo);
}

/**
 * Establecer observaciones
 */
export function establecerObservaciones(observaciones: string): void {
	ingresoFormStore.setObservaciones(observaciones);
}

// ==========================================
// SUBMIT - REGISTRAR ENTRADA
// ==========================================

/**
 * Validar y registrar entrada de contratista
 *
 * Orquesta:
 * 1. Validación completa del formulario
 * 2. Validación específica de gafete
 * 3. Validación de modo vehículo
 * 4. Registro en el backend
 * 5. Actualización de stores
 * 6. Reset del formulario
 *
 * @param usuarioId - ID del usuario que registra la entrada
 * @param gafetesDisponibles - Lista de gafetes disponibles (para validación)
 * @returns true si se registró correctamente, false si no
 */
export async function registrarEntrada(
	usuarioId: string,
	gafetesDisponibles: GafeteResponse[]
): Promise<boolean> {
	const estado = get(ingresoFormStore);

	// 1. Validación completa del formulario
	const validacionFormulario = ingresoService.validarFormularioCompleto({
		contratistaValidated: estado.contratistaId !== '' && estado.puedeIngresar,
		canEnter: estado.puedeIngresar,
		contratistaId: estado.contratistaId,
		modoIngreso: estado.modoIngreso,
		vehiculoId: estado.vehiculoId,
		gafeteNumero: estado.gafeteNumero,
		tipoAutorizacion: estado.tipoAutorizacion as 'praind' | 'correo'
	});

	if (!validacionFormulario.ok) {
		toast.error(validacionFormulario.error);
		return false;
	}

	// 2. Validación de gafete si se proporcionó
	if (estado.gafeteNumero.trim()) {
		const validacionGafete = ingresoService.validarGafete({
			gafeteNumero: estado.gafeteNumero,
			gafetesDisponibles
		});

		if (!validacionGafete.ok || !validacionGafete.data.isValid) {
			toast.error('Número de gafete inválido o no disponible');
			return false;
		}
	}

	// 3. Validación de modo vehículo
	const tieneVehiculos = estado.contratistaData?.vehiculos?.length > 0;
	const validacionModo = ingresoService.validarModoVehiculo({
		modoIngreso: estado.modoIngreso,
		vehiculoId: estado.vehiculoId,
		tieneVehiculos
	});

	if (!validacionModo.ok) {
		toast.error(validacionModo.error);
		return false;
	}

	// 4. Construir payload
	const input: CreateIngresoContratistaInput = {
		contratistaId: estado.contratistaId,
		tipoAutorizacion: estado.tipoAutorizacion,
		modoIngreso: estado.modoIngreso,
		gafeteNumero: estado.gafeteNumero.trim().toUpperCase() || null,
		vehiculoId: estado.modoIngreso === 'vehiculo' ? estado.vehiculoId : null,
		observaciones: estado.observaciones.trim() || null,
		usuarioIngresoId: usuarioId
	};

	// 5. Registrar en el backend
	const result = await ingresoService.registrarEntrada(input);

	if (!result.ok) {
		toast.error(result.error);
		return false;
	}

	// 6. Actualizar store de ingresos activos
	ingresoStore.add(result.data);

	// 7. Mostrar éxito
	toast.success('Entrada registrada correctamente');

	// 8. Reset del formulario
	ingresoFormStore.reset();

	return true;
}

// ==========================================
// RESET
// ==========================================

/**
 * Resetear formulario completo
 */
export function resetearFormulario(): void {
	ingresoFormStore.reset();
}

/**
 * Resetear solo campos de ingreso (mantener contratista)
 */
export function resetearCamposIngreso(): void {
	ingresoFormStore.resetIngresoFields();
}
