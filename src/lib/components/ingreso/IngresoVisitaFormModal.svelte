<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { fade, scale, slide } from 'svelte/transition';
	import { toast } from 'svelte-5-french-toast';
	import {
		X,
		ChevronDown,
		ChevronRight,
		AlertTriangle,
		User,
		MapPin,
		MessageSquare,
		Plus
	} from 'lucide-svelte';

	// Components
	import GafeteInput from './shared/gafete/GafeteInput.svelte';

	// Logic
	import { ingresoVisitaService } from '$lib/services/ingresoVisitaService';
	import { currentUser } from '$lib/stores/auth';
	import { invoke } from '@tauri-apps/api/core';

	// Props
	interface Props {
		show: boolean;
		initialPerson?: any | null;
	}

	let { show = $bindable(false), initialPerson = null }: Props = $props();

	// State
	let loading = $state(false);
	let selectedPerson = $state<any>(null);
	let validationResult = $state<any>(null);
	let gafete = $state('');

	let anfitrion = $state('');
	let areaVisitada = $state('');
	let motivo = $state('');
	let observaciones = $state('');

	let showObservaciones = $state(false);
	let submitted = $state(false);

	const dispatch = createEventDispatcher();

	// Reset y Validar al abrir
	$effect(() => {
		if (show && initialPerson) {
			handlePersonSelect(initialPerson);
		}
	});

	async function handlePersonSelect(person: any) {
		selectedPerson = person;
		loading = true;
		try {
			if (person.id) {
				validationResult = await ingresoVisitaService.validarIngreso(person.id);
			} else {
				// Es una persona nueva (no está en el sistema)
				validationResult = { puedeIngresar: true, alerts: [] };
			}

			if (!validationResult.puedeIngresar) {
				invoke('play_alert_sound');
				toast.error(validationResult.motivoRechazo || 'Persona no autorizada');
			}
		} catch (e: unknown) {
			console.error(e);
			toast.error('Error al validar visitante');
			validationResult = { puedeIngresar: true }; // Fallback
		} finally {
			loading = false;
		}
	}

	async function handleSubmit() {
		if (!selectedPerson || !validationResult?.puedeIngresar) return;

		if (!anfitrion.trim() || !areaVisitada.trim() || !motivo.trim()) {
			submitted = true;
			toast.error('Por favor complete los campos requeridos');
			return;
		}

		submitted = true;
		loading = true;
		try {
			await ingresoVisitaService.createIngreso({
				cedula: selectedPerson.cedula,
				nombre: selectedPerson.nombre,
				apellido: selectedPerson.apellido,
				empresa: selectedPerson.empresa || selectedPerson.procedencia,
				anfitrion,
				area_visitada: areaVisitada,
				motivo,
				gafete: gafete.trim() || undefined,
				observaciones: observaciones || undefined,
				usuario_ingreso_id: $currentUser?.id || ''
			});

			toast.success('Ingreso de visita registrado');
			dispatch('complete');
			handleClose();
		} catch (e: unknown) {
			console.error(e);
			const msg = e instanceof Error ? e.message : String(e);
			toast.error('Error al registrar: ' + msg);
		} finally {
			loading = false;
		}
	}

	function handleClose() {
		show = false;
		reset();
	}

	function reset() {
		selectedPerson = null;
		validationResult = null;
		gafete = '';
		anfitrion = '';
		areaVisitada = '';
		motivo = '';
		observaciones = '';
		showObservaciones = false;
		submitted = false;
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') handleClose();
		if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 's') {
			e.preventDefault();
			handleSubmit();
		}
	}

	// --- UI PATTERNS ---
	const inputClass =
		'w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[34px] text-sm text-white placeholder:text-gray-500 transition-all outline-none';
	const labelClass = 'block text-xs font-medium text-secondary mb-1';

	function getFieldStateClass(value: any, isRequired = false) {
		const hasValue = value && String(value).trim() !== '';
		if (isRequired && !hasValue && submitted) return '!border-red-500/50 !ring-1 !ring-red-500/20';
		if (isRequired && hasValue) return '!border-green-500/50 !ring-1 !ring-green-500/20';
		return 'border-white/10';
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if show}
	<div
		class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4"
		transition:fade
		onclick={(e) => e.target === e.currentTarget && handleClose()}
		onkeydown={(e) => e.key === 'Enter' && e.target === e.currentTarget && handleClose()}
		role="button"
		tabindex="0"
	>
		<div
			class="bg-surface-2 rounded-lg shadow-surface-xl border border-surface max-w-md w-full max-h-[95vh] flex flex-col overflow-hidden"
			transition:scale={{ start: 0.95 }}
		>
			<!-- Header -->
			<div class="px-6 py-4 border-b border-surface bg-surface-2 flex items-center justify-between">
				<div>
					<h2 class="text-xl font-semibold text-primary">Ingreso de Visita</h2>
					<p class="text-xs text-secondary mt-0.5">Registro directo de visitante</p>
				</div>
				<button
					onclick={handleClose}
					class="p-2 hover:bg-surface-hover rounded-md transition-colors"
				>
					<X size={20} class="text-secondary" />
				</button>
			</div>

			<!-- Body -->
			<div class="flex-1 overflow-y-auto">
				<div class="p-6 space-y-6">
					{#if selectedPerson}
						<div class="p-4 bg-surface-1 rounded-lg border border-surface space-y-3">
							<div class="flex justify-between items-start">
								<div>
									<p class="text-[10px] font-bold uppercase text-gray-500">Visitante</p>
									<p class="text-primary font-semibold">
										{selectedPerson.nombre}
										{selectedPerson.apellido}
									</p>
									<p class="text-xs text-secondary font-mono">{selectedPerson.cedula}</p>
								</div>
								{#if selectedPerson.empresa || selectedPerson.procedencia}
									<div class="text-right">
										<p class="text-[10px] font-bold uppercase text-gray-500">Empresa</p>
										<p class="text-xs text-primary">
											{selectedPerson.empresa || selectedPerson.procedencia}
										</p>
									</div>
								{/if}
							</div>

							<!-- Validaciones -->
							{#if validationResult}
								{#if !validationResult.puedeIngresar}
									<div
										class="flex items-center gap-2 p-2 bg-red-500/10 border border-red-500/20 rounded text-red-500 text-xs"
									>
										<AlertTriangle size={14} />
										<span>{validationResult.motivoRechazo || 'PROHIBIDO EL INGRESO'}</span>
									</div>
								{:else}
									<div
										class="flex items-center gap-2 p-2 bg-green-500/10 border border-green-500/20 rounded text-green-500 text-xs"
									>
										<Plus size={14} />
										<span>Autorizado para ingresar</span>
									</div>
								{/if}
							{/if}

							{#if validationResult?.puedeIngresar}
								<div class="space-y-4 pt-2">
									<GafeteInput bind:value={gafete} autofocus disabled={loading} />

									<div class="grid grid-cols-1 gap-3">
										<div>
											<label for="anfitrion" class={labelClass}
												>Anfitrión <span class="text-red-500">*</span></label
											>
											<div class="relative">
												<User size={14} class="absolute left-3 top-2.5 text-gray-500" />
												<input
													id="anfitrion"
													class="{inputClass} pl-9 {getFieldStateClass(anfitrion, true)}"
													bind:value={anfitrion}
													placeholder="¿A quién visita?"
												/>
											</div>
										</div>

										<div>
											<label for="area" class={labelClass}
												>Área <span class="text-red-500">*</span></label
											>
											<div class="relative">
												<MapPin size={14} class="absolute left-3 top-2.5 text-gray-500" />
												<input
													id="area"
													class="{inputClass} pl-9 {getFieldStateClass(areaVisitada, true)}"
													bind:value={areaVisitada}
													placeholder="Departamento o área"
												/>
											</div>
										</div>

										<div>
											<label for="motivo" class={labelClass}
												>Motivo <span class="text-red-500">*</span></label
											>
											<div class="relative">
												<MessageSquare size={14} class="absolute left-3 top-2.5 text-gray-500" />
												<input
													id="motivo"
													class="{inputClass} pl-9 {getFieldStateClass(motivo, true)}"
													bind:value={motivo}
													placeholder="Ej. Reunión de negocios"
												/>
											</div>
										</div>
									</div>

									<!-- Observaciones toggle -->
									<div class="pt-2">
										<button
											type="button"
											onclick={() => (showObservaciones = !showObservaciones)}
											class="text-xs text-secondary hover:text-primary flex items-center gap-1 transition-colors"
										>
											{#if showObservaciones}<ChevronDown size={12} />{:else}<ChevronRight
													size={12}
												/>{/if}
											Observaciones adicionales
										</button>
										{#if showObservaciones}
											<div transition:slide class="mt-2">
												<textarea
													bind:value={observaciones}
													class="w-full bg-black/20 border border-white/10 rounded-lg px-3 py-2 text-xs text-white resize-none outline-none focus:border-blue-500/50"
													rows="2"
												></textarea>
											</div>
										{/if}
									</div>
								</div>
							{/if}
						</div>
					{/if}
				</div>
			</div>

			<!-- Footer -->
			<div class="px-6 py-4 border-t border-surface bg-surface-1 flex justify-end gap-3">
				<button
					onclick={handleClose}
					disabled={loading}
					class="px-4 py-2.5 rounded-lg border-2 transition-all duration-200 border-surface text-secondary hover:border-white/60 hover:text-white/80 focus:outline-none disabled:opacity-50 text-sm"
				>
					Cancelar
				</button>
				{#if validationResult?.puedeIngresar}
					<button
						onclick={handleSubmit}
						disabled={loading}
						class="px-6 py-2.5 rounded-lg border-2 transition-all duration-200 border-surface text-secondary hover:border-blue-500 hover:text-blue-500 focus:outline-none disabled:opacity-50 text-sm font-semibold flex items-center gap-2"
					>
						{#if loading}
							<span
								class="w-4 h-4 rounded-full border-2 border-current border-t-transparent animate-spin"
							></span>
						{/if}
						Registrar Ingreso
					</button>
				{/if}
			</div>
		</div>
	</div>
{/if}

<style>
	input:focus,
	textarea:focus {
		border-color: rgba(59, 130, 246, 0.5) !important;
		box-shadow: 0 0 0 1px rgba(59, 130, 246, 0.2) !important;
	}
</style>
