<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { fade, scale, slide } from 'svelte/transition';
	import { toast } from 'svelte-5-french-toast';
	import { X, ChevronDown, ChevronRight, Car, FileText, AlertTriangle } from 'lucide-svelte';

	// Components
	import PersonaFinder from './shared/persona/PersonaFinder.svelte';
	import GafeteInput from './shared/gafete/GafeteInput.svelte';

	// Logic
	import { ingresoService } from '$lib/logic/ingreso/ingresoService';
	import type { ValidacionIngresoResult } from '$lib/logic/ingreso/types';
	import type { ContratistaResponse, EstadoContratista } from '$lib/types/contratista';
	import type { SearchResult } from '$lib/types/search.types';

	import { currentUser } from '$lib/stores/auth';
	import { invoke } from '@tauri-apps/api/core';

	import type { VehiculoResponse } from '$lib/types/vehiculo';

	interface FormPerson extends Partial<ContratistaResponse> {
		vehiculos?: VehiculoResponse[];
		empresa?: string; // Legacy name often returned by search/validation
		tipo?: string;
	}

	// Helper para normalizar IDs de SurrealDB (maneja strings, objetos y notación ⟨...⟩)
	function stringifyRecordId(id: unknown): string {
		if (!id) return '';
		if (typeof id === 'string') return id.replace(/[⟨⟩]/g, '');

		if (typeof id === 'object') {
			// Caso 1: Estructura { tb: 'user', id: 'abc' } o { tb: 'user', id: { String: 'abc' } }
			const tb = (id as any).tb;
			const innerId = (id as any).id?.String || (id as any).id;
			if (tb && innerId) {
				return `${tb}:${innerId}`.replace(/[⟨⟩]/g, '');
			}
		}

		return String(id).replace(/[⟨⟩]/g, '');
	}

	// Props
	interface Props {
		show: boolean;
		initialPerson?: FormPerson | null;
	}

	let { show = $bindable(false), initialPerson = null }: Props = $props();

	// State
	let loading = $state(false);
	let selectedPerson = $state<FormPerson | null>(null);
	let validationResult = $state<ValidacionIngresoResult | null>(null);
	let gafete = $state('');
	let vehiculoId = $state<string | null>(null);
	let tipoAutorizacion = $state<'praind' | 'correo'>('praind');
	let observaciones = $state('');
	let showObservaciones = $state(false);
	let personaFinderRef = $state<PersonaFinder>();

	const dispatch = createEventDispatcher();

	// Computed: tiene PRAIND vigente?
	let tienePraind = $derived(
		validationResult?.persona?.praindVigente === true ||
			validationResult?.contratista?.praindVencido === false
	);

	// Computed: vehículos disponibles
	let vehiculosDisponibles = $derived(selectedPerson?.vehiculos || []);
	let tieneVehiculos = $derived(vehiculosDisponibles.length > 0);

	// Auto-seleccionar vehículo único
	$effect(() => {
		if (vehiculosDisponibles.length === 1 && !vehiculoId) {
			vehiculoId = vehiculosDisponibles[0].id;
		}
	});

	// Modo de ingreso derivado
	let modoIngreso = $derived(vehiculoId ? 'vehiculo' : 'caminando');

	// Resetear al abrir para asegurar estado limpio
	$effect(() => {
		if (show) {
			if (initialPerson) {
				// Ejecutar validación para la persona pre-seleccionada
				handlePersonSelect({
					detail: {
						id: initialPerson.id,
						type: initialPerson.tipo?.toLowerCase() || 'contratista',
						data: initialPerson
					}
				} as CustomEvent);
			} else {
				setTimeout(() => {
					if (personaFinderRef) personaFinderRef.focus();
				}, 100);
			}
		}
	});

	// ==========================================
	// HANDLERS
	// ==========================================
	async function handlePersonSelect(
		event: CustomEvent<{ id: string; type: string; data: SearchResult | FormPerson }>
	) {
		const { id, type, data } = event.detail;

		// Solo contratistas por ahora
		if (type !== 'contratista') {
			toast.error('Por ahora solo se permiten contratistas');
			return;
		}

		// Sanitize SearchResult to match FormPerson (handle nulls)
		selectedPerson = {
			...data,
			cedula: data.cedula || undefined,
			nombreCompleto: data.nombreCompleto || undefined,
			empresaNombre: (data as any).empresaNombre || undefined
		} as FormPerson;

		// Validar automáticamente
		try {
			loading = true;
			validationResult = await ingresoService.validarIngreso('contratista', id);

			// DEBUG: Ver estructura de datos para PRAIND

			if (validationResult.persona) {
				// Merge validation data. Note: validation returns string state, we need to handle that.
				const validatedPerson: Partial<FormPerson> = { ...(validationResult.persona as any) };
				// Safe assignment avoiding type mismatch with Enum
				selectedPerson = { ...selectedPerson, ...validatedPerson } as FormPerson;
			}

			if (!validationResult.puedeIngresar) {
				// Trigger native alert sound
				invoke('play_alert_sound');
				toast.error(validationResult.motivoRechazo || 'Contratista no autorizado');
			}
		} catch (e: unknown) {
			const msg = e instanceof Error ? e.message : String(e);
			toast.error('Error al validar: ' + msg);
			validationResult = null;
		} finally {
			loading = false;
		}
	}

	async function handleSubmit() {
		if (!selectedPerson || !validationResult) {
			toast.error('Complete todos los campos requeridos');
			return;
		}

		if (!validationResult.puedeIngresar) {
			toast.error('Esta persona no está autorizada para ingresar');
			return;
		}

		try {
			loading = true;

			const finalGafete = gafete.trim() || 'S/G';

			const contratistaIdStr = stringifyRecordId(selectedPerson.id);
			const usuarioIdStr = stringifyRecordId($currentUser?.id);

			if (!usuarioIdStr) {
				toast.error(
					'No se pudo identificar su sesión de usuario. Por favor reintente o refresque la página.'
				);
				loading = false;
				return;
			}

			await ingresoService.crearIngreso(
				'contratista',
				contratistaIdStr,
				{
					gafete: finalGafete,
					vehiculoId: vehiculoId,
					observaciones: observaciones.trim() || '',
					esExcepcional: !tienePraind,
					tipoAutorizacion: tienePraind ? 'praind' : tipoAutorizacion,
					modoIngreso: modoIngreso
				},
				selectedPerson,
				usuarioIdStr
			);

			toast.success('¡Ingreso registrado exitosamente!');
			dispatch('complete');
			reset(); // Limpiar inmediatamente tras éxito
			handleClose();
		} catch (e: any) {
			console.error('[IngresoFormModal] Error completo:', e);
			// Parsear mensaje de error del backend (formato thiserror)
			let errorMsg = 'Error al registrar ingreso';

			if (typeof e === 'string') {
				errorMsg = e;
			} else if (e?.type) {
				// Mapear tipos de error a mensajes amigables
				const errorMessages: Record<string, string> = {
					GafeteNotAvailable: 'El gafete especificado no está disponible',
					AlreadyInside: 'El contratista ya tiene un ingreso activo',
					ContratistaNotFound: 'Contratista no encontrado',
					Blacklisted: e.message ? `En lista negra: ${e.message}` : 'Persona en lista negra',
					PraindExpired: e.message ? `PRAIND vencido: ${e.message}` : 'PRAIND vencido',
					ContratistaInactive: 'El contratista no está activo',
					Validation: e.message || 'Error de validación',
					Database: 'Error de base de datos',
					Gafete: e.message || 'Error con el gafete'
				};
				errorMsg = errorMessages[e.type] || e.message || e.type;
			} else if (e?.message) {
				errorMsg = e.message;
			}

			toast.error(errorMsg);
		} finally {
			loading = false;
		}
	}

	function handleClose() {
		if (loading) return;
		show = false;
		reset();
	}

	function reset() {
		selectedPerson = null;
		validationResult = null;
		gafete = '';
		vehiculoId = null;
		tipoAutorizacion = 'praind';
		observaciones = '';
		showObservaciones = false;
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			handleClose();
		}
		// Ctrl+S para guardar
		if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 's') {
			e.preventDefault();
			if (show && !loading && selectedPerson && validationResult?.puedeIngresar) {
				handleSubmit();
			}
		}
	}

	function getSeverityClasses(severity?: string) {
		const base = 'flex items-center gap-2 text-sm px-3 py-2 rounded-md border transition-colors';
		const upperSeverity = severity?.toUpperCase();

		if (upperSeverity === 'ALTO') {
			return `${base} bg-red-50 text-red-700 border-red-200 dark:bg-red-900/30 dark:text-red-300 dark:border-red-800`;
		}
		if (upperSeverity === 'MEDIO') {
			return `${base} bg-yellow-50 text-yellow-700 border-yellow-200 dark:bg-yellow-900/30 dark:text-yellow-300 dark:border-yellow-800`;
		}
		if (upperSeverity === 'BAJO') {
			return `${base} bg-gray-50 text-gray-700 border-gray-200 dark:bg-gray-900/30 dark:text-gray-300 dark:border-gray-800`;
		}

		// Default error style
		return `${base} text-red-700 bg-red-50 border-red-200 dark:text-red-300 dark:bg-red-900/30 dark:border-red-800`;
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if show}
	<!-- Overlay -->
	<div
		class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4"
		transition:fade={{ duration: 150 }}
		role="dialog"
		aria-modal="true"
	>
		<!-- Modal -->
		<div
			class="bg-surface-2 rounded-lg shadow-surface-xl border border-surface max-w-md w-full max-h-[90vh] overflow-visible flex flex-col"
			transition:scale={{ duration: 200, start: 0.95 }}
		>
			<!-- Header -->
			<div class="flex items-center justify-between px-6 py-4 border-b border-surface">
				<div>
					<h2 class="text-xl font-semibold text-primary">Nuevo Ingreso</h2>
					<p class="text-sm text-secondary mt-1">Busca por cédula o nombre del contratista</p>
				</div>
				<button
					onclick={handleClose}
					class="p-2 hover:bg-surface-hover rounded-md transition-colors"
					disabled={loading}
				>
					<X size={20} class="text-secondary" />
				</button>
			</div>

			<!-- Content -->
			<div class="p-6 space-y-6">
				<!-- Buscador (Solo si NO hay persona inicial) -->
				{#if !initialPerson}
					<div>
						<PersonaFinder bind:this={personaFinderRef} on:select={handlePersonSelect} />
					</div>
				{/if}

				<!-- Persona seleccionada -->
				{#if selectedPerson && validationResult}
					<div class="p-4 bg-surface-1 rounded-lg border border-surface" transition:fade>
						<!-- Datos de la persona -->
						<div class="space-y-2 mb-4">
							<div class="flex items-center">
								<span
									class="text-[12px] font-bold uppercase tracking-wider text-gray-500 w-20 shrink-0"
									>Nombre</span
								>
								<span class="text-primary font-semibold text-sm">
									{selectedPerson.nombreCompleto ||
										`${selectedPerson.nombre} ${selectedPerson.apellido}`}
								</span>
							</div>
							<div class="flex items-center">
								<span
									class="text-[12px] font-bold uppercase tracking-wider text-gray-500 w-20 shrink-0"
									>Cédula</span
								>
								<span class="text-primary font-mono text-sm">{selectedPerson.cedula || 'N/A'}</span>
							</div>
							<div class="flex items-center">
								<span
									class="text-[12px] font-bold uppercase tracking-wider text-gray-500 w-20 shrink-0"
									>Empresa</span
								>
								<span class="text-primary text-sm">
									{selectedPerson.empresaNombre || selectedPerson.empresa || 'Sin empresa'}
								</span>
							</div>
						</div>

						<!-- Alertas de Gafete Pendientes (Amarillo) - Mostrar SIEMPRE si hay alertas -->
						{#if validationResult.alertas && validationResult.alertas.length > 0}
							<div
								class="mb-4 flex flex-col gap-1 text-yellow-500 bg-yellow-500/10 border border-yellow-500/20 p-2.5 rounded-lg text-xs"
								transition:slide
							>
								<div class="flex items-center gap-2">
									<AlertTriangle size={16} />
									<span class="font-medium">
										⚠️ Debe {validationResult.alertas.length} gafete(s)
									</span>
								</div>
								{#each validationResult.alertas as alerta}
									<span class="ml-6 text-yellow-400/80">{alerta}</span>
								{/each}
							</div>
						{/if}

						<!-- Recordatorio PRAIND (Amarillo) - Solo si requiere atención o está vencido -->
						{#if validationResult.contratista?.requiereAtencion || (validationResult.contratista?.diasHastaVencimiento !== undefined && validationResult.contratista.diasHastaVencimiento < 0)}
							{@const days = validationResult.contratista.diasHastaVencimiento}
							<div
								class="mb-4 flex items-center gap-2 text-yellow-500 bg-yellow-500/10 border border-yellow-500/20 p-2.5 rounded-lg text-xs"
								transition:slide
							>
								<AlertTriangle size={16} />
								<span class="font-medium">
									{#if days < 0}
										El PRAIND venció hace {Math.abs(days)} días
									{:else if days === 0}
										El PRAIND vence hoy
									{:else}
										El PRAIND vence en <strong class="font-bold">{days}</strong>
										días
									{/if}
								</span>
							</div>
						{:else if validationResult.puedeIngresar}
							<!-- Badge Verde solo si puede ingresar -->
							<div
								class="flex items-center gap-2 text-sm text-success bg-success bg-opacity-10 px-3 py-2 rounded-md mb-4"
								transition:fade
							>
								<span class="font-medium">✓ Acceso Autorizado</span>
							</div>
						{/if}

						<!-- Motivo de Rechazo (Rojo) - Solo si NO puede ingresar -->
						{#if !validationResult.puedeIngresar}
							<div class={getSeverityClasses(validationResult.severidadListaNegra)}>
								<span class="font-medium">
									✗ {validationResult.motivoRechazo || 'No autorizado'}
								</span>
							</div>
						{/if}

						<!-- Form (solo si está autorizado) -->
						{#if validationResult.puedeIngresar}
							<div class="mt-4 space-y-3">
								<!-- Gafete -->
								<GafeteInput bind:value={gafete} autofocus disabled={loading} />

								<!-- Vehículo - Solo si tiene vehículos registrados -->
								{#if tieneVehiculos}
									<div class="space-y-1.5" transition:slide>
										<label class="flex items-center gap-2 text-sm font-medium text-secondary">
											<Car size={16} />
											Vehículo
											<span class="text-xs text-tertiary">(opcional)</span>
										</label>
										<select
											class="w-full bg-surface-1 border border-surface rounded-md px-3 py-2 text-sm text-primary focus:outline-none focus:ring-2 focus:ring-accent"
											bind:value={vehiculoId}
											disabled={loading}
										>
											<option value={null}>Sin vehículo (caminando)</option>
											{#each vehiculosDisponibles as v}
												<option value={v.id}>
													{v.placa} - {v.marca || ''}
													{v.modelo || ''}
												</option>
											{/each}
										</select>
									</div>
								{/if}

								<!-- Tipo de Autorización - Solo si NO tiene PRAIND -->
								{#if !tienePraind}
									<div
										class="space-y-1.5 p-3 bg-yellow-500/10 border border-yellow-500/30 rounded-md"
										transition:slide
									>
										<label class="flex items-center gap-2 text-sm font-medium text-yellow-300">
											<FileText size={16} />
											Tipo de Autorización
										</label>
										<p class="text-xs text-yellow-400/80 mb-2">
											El contratista no tiene PRAIND vigente. Seleccione el tipo de autorización.
										</p>
										<div class="flex gap-3">
											<label class="flex items-center gap-2 cursor-pointer">
												<input
													type="radio"
													name="tipoAutorizacion"
													value="praind"
													bind:group={tipoAutorizacion}
													class="radio radio-sm radio-warning"
													disabled={loading}
												/>
												<span class="text-sm text-primary">PRAIND</span>
											</label>
											<label class="flex items-center gap-2 cursor-pointer">
												<input
													type="radio"
													name="tipoAutorizacion"
													value="correo"
													bind:group={tipoAutorizacion}
													class="radio radio-sm radio-warning"
													disabled={loading}
												/>
												<span class="text-sm text-primary">Correo</span>
											</label>
										</div>
									</div>
								{/if}

								<!-- Observaciones - Toggle colapsable -->
								<div class="border-t border-surface pt-2">
									<button
										type="button"
										onclick={() => (showObservaciones = !showObservaciones)}
										class="flex items-center gap-1.5 text-secondary hover:text-primary transition-colors text-sm"
									>
										{#if showObservaciones}
											<ChevronDown size={14} />
										{:else}
											<ChevronRight size={14} />
										{/if}
										<span>Observaciones</span>
										{#if !showObservaciones && observaciones.trim()}
											<span class="w-1.5 h-1.5 rounded-full bg-blue-500"></span>
										{/if}
									</button>

									{#if showObservaciones}
										<div class="mt-2" transition:slide>
											<div
												class="obs-container w-full bg-black/20 border border-white/10 rounded-lg focus-within:border-blue-500/50 focus-within:ring-1 focus-within:ring-blue-500/20 transition-all outline-none"
											>
												<textarea
													class="w-full bg-transparent px-3 py-2 text-sm text-white placeholder:text-gray-500 resize-none focus:outline-none outline-none border-none appearance-none ring-0"
													rows="2"
													placeholder="Notas adicionales..."
													bind:value={observaciones}
													disabled={loading}
												></textarea>
											</div>
										</div>
									{/if}
								</div>
							</div>
						{/if}
					</div>
				{/if}
			</div>

			<!-- Footer con Botones Reactivos (Alta Contraste) -->
			<div
				class="flex items-center justify-end gap-3 px-6 py-4 border-t border-surface bg-surface-1"
			>
				<button
					type="button"
					onclick={handleClose}
					disabled={loading}
					class="flex items-center justify-center gap-2 px-4 py-2.5 rounded-lg border-2 transition-all duration-200 border-surface text-secondary hover:border-white/60 hover:text-white/80 focus:outline-none disabled:opacity-50"
				>
					Cancelar
				</button>
				{#if validationResult?.puedeIngresar}
					<button
						type="button"
						onclick={handleSubmit}
						disabled={loading}
						class="flex items-center justify-center gap-2 px-6 py-2.5 rounded-lg border-2 transition-all duration-200 border-surface text-secondary hover:border-success hover:text-success focus:outline-none disabled:opacity-50 font-semibold"
					>
						{#if loading}
							<span class="inline-block animate-spin mr-1">⏳</span>
						{/if}
						Registrar Ingreso
					</button>
				{/if}
			</div>
		</div>
	</div>
{/if}

<style>
	/* Asegurar que el modal esté por encima de todo */
	:global(body:has(.fixed.z-50)) {
		overflow: hidden;
	}

	/* Observaciones container - mismo estilo que GafeteInput */
	.obs-container,
	.obs-container *:focus {
		outline: none !important;
		box-shadow: none !important;
	}

	.obs-container:focus-within {
		border-color: rgba(59, 130, 246, 0.5) !important;
		box-shadow: 0 0 0 1px rgba(59, 130, 246, 0.2) !important;
	}
</style>
