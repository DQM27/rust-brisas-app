<script lang="ts">
	import { onMount } from 'svelte';
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
		Plus,
		Search,
		Building2,
		ShieldCheck,
		SearchX,
		IdCard,
		LogOut
	} from 'lucide-svelte';

	// Components
	import GafeteInput from './shared/gafete/GafeteInput.svelte';

	// Logic
	import { ingresoVisitaService } from '$lib/services/ingresoVisitaService';
	import { getVisitanteByCedula } from '$lib/logic/visitante/visitanteService';
	import { submitFetchActiveEmpresas } from '$lib/logic/empresa/empresaService';
	import { currentUser } from '$lib/stores/auth';
	import { invoke } from '@tauri-apps/api/core';

	// Props
	interface Props {
		show: boolean;
		initialPerson?: any | null;
		onComplete?: () => void;
	}

	let { show = $bindable(false), initialPerson = null, onComplete }: Props = $props();

	// State
	let loading = $state(false);
	let searchingPerson = $state(false);
	let validationResult = $state<any>(null);

	// Visitor Data
	let cedula = $state('');
	let nombre = $state('');
	let apellido = $state('');
	let empresa_nombre = $state('');

	// Ingress Data
	let anfitrion = $state('');
	let areaVisitada = $state('');
	let motivo = $state('');
	let gafete = $state('');
	let observaciones = $state('');

	// UI State
	let showObservaciones = $state(false);
	let submitted = $state(false);
	let empresasSugeridas = $state<string[]>([]);
	let showEmpresaSuggestions = $state(false);

	// Reset y Validar al abrir
	$effect(() => {
		if (show) {
			if (initialPerson) {
				fillPersonData(initialPerson);
			} else {
				reset();
			}
		}
	});

	onMount(async () => {
		const res = await submitFetchActiveEmpresas();
		if (res.ok) {
			empresasSugeridas = res.empresas.map((e) => e.nombre);
		}
	});

	function fillPersonData(person: any) {
		cedula = person.cedula || '';
		nombre = person.nombre || '';
		apellido = person.apellido || '';
		empresa_nombre = person.empresa || person.empresa_nombre || person.procedencia || '';

		if (person.id || person.cedula) {
			validarAcceso(person.cedula);
		}
	}

	async function handleCedulaBlur() {
		if (!cedula || cedula.length < 5) return;

		searchingPerson = true;
		try {
			const res = await getVisitanteByCedula(cedula);
			if (res.ok && res.data) {
				const p = res.data;
				nombre = p.nombre;
				apellido = p.apellido;
				empresa_nombre = p.empresaNombre || '';
				toast.success('Visitante encontrado');
			}
			await validarAcceso(cedula);
		} catch (e) {
			console.error(e);
		} finally {
			searchingPerson = false;
		}
	}

	async function validarAcceso(ced: string) {
		try {
			// Usamos la cédula para validar
			validationResult = await ingresoVisitaService.validarIngreso(ced);

			if (validationResult && !validationResult.puedeIngresar) {
				invoke('play_alert_sound');
				toast.error(validationResult.motivoRechazo || 'Persona no autorizada');
			}
		} catch (e: unknown) {
			console.error(e);
			validationResult = { puedeIngresar: true }; // Fallback
		}
	}

	async function handleSubmit() {
		if (validationResult && !validationResult.puedeIngresar) {
			toast.error(
				'No se puede registrar el ingreso: ' + (validationResult.motivoRechazo || 'Bloqueado')
			);
			return;
		}

		if (
			!cedula.trim() ||
			!nombre.trim() ||
			!apellido.trim() ||
			!anfitrion.trim() ||
			!areaVisitada.trim() ||
			!motivo.trim()
		) {
			submitted = true;
			toast.error('Por favor complete los campos requeridos');
			return;
		}

		loading = true;
		try {
			await ingresoVisitaService.createIngreso({
				cedula: cedula.trim(),
				nombre: nombre.trim(),
				apellido: apellido.trim(),
				empresa_nombre: empresa_nombre.trim() || undefined,
				anfitrion: anfitrion.trim(),
				area_visitada: areaVisitada.trim(),
				motivo: motivo.trim(),
				gafete: gafete.trim() || undefined,
				observaciones: observaciones.trim() || undefined,
				usuario_ingreso_id: $currentUser?.id || ''
			});

			toast.success('Ingreso de visita registrado');
			show = false;
			reset();
			if (onComplete) onComplete();
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
		validationResult = null;
		cedula = '';
		nombre = '';
		apellido = '';
		empresa_nombre = '';
		gafete = '';
		anfitrion = '';
		areaVisitada = '';
		motivo = '';
		observaciones = '';
		showObservaciones = false;
		submitted = false;
		searchingPerson = false;
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
		'w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[34px] text-sm text-white placeholder:text-gray-500 transition-all outline-none focus:border-blue-500/50 focus:ring-1 focus:ring-blue-500/20';
	const labelClass = 'block text-[11px] font-medium text-secondary mb-1 uppercase tracking-wider';

	function getFieldStateClass(value: any, isRequired = false) {
		const hasValue = value && String(value).trim() !== '';
		if (isRequired && !hasValue && submitted) return '!border-red-500/50 !ring-1 !ring-red-500/20';
		if (isRequired && hasValue) return '!border-green-500/50 !ring-1 !ring-green-500/20';
		return 'border-white/10';
	}

	let filteredCompanies = $derived(
		empresa_nombre.length > 1
			? empresasSugeridas.filter(
					(c) =>
						c.toLowerCase().includes(empresa_nombre.toLowerCase()) &&
						c.toLowerCase() !== empresa_nombre.toLowerCase()
				)
			: []
	);

	function selectCompany(name: string) {
		empresa_nombre = name;
		showEmpresaSuggestions = false;
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if show}
	<div
		class="fixed inset-0 bg-black/60 backdrop-blur-md z-[100] flex items-center justify-center p-4"
		transition:fade
		onclick={(e) => e.target === e.currentTarget && handleClose()}
		onkeydown={(e) =>
			(e.key === 'Escape' || e.key === 'Enter') && e.target === e.currentTarget && handleClose()}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<div
			class="bg-surface-2 rounded-xl shadow-2xl border border-white/10 max-w-2xl w-full max-h-[90vh] flex flex-col overflow-hidden"
			transition:scale={{ start: 0.95 }}
			onclick={(e) => e.stopPropagation()}
		>
			<!-- Header -->
			<div
				class="px-6 py-5 border-b border-white/5 bg-gradient-to-r from-blue-600/10 to-transparent flex items-center justify-between"
			>
				<div class="flex items-center gap-3">
					<div class="p-2 bg-blue-500/20 rounded-lg text-blue-400">
						<Plus size={24} />
					</div>
					<div>
						<h2 class="text-xl font-bold text-white tracking-tight">Registro de Ingreso</h2>
						<p class="text-xs text-secondary mt-0.5">Módulo de Visitantes Ocasionales</p>
					</div>
				</div>
				<button
					onclick={handleClose}
					class="p-2 hover:bg-white/10 rounded-full transition-all text-secondary hover:text-white"
				>
					<X size={20} />
				</button>
			</div>

			<!-- Body -->
			<div class="flex-1 overflow-y-auto custom-scrollbar">
				<div class="p-6 space-y-8">
					<!-- Seccion 1: Datos Personales -->
					<div class="space-y-4">
						<div class="flex items-center gap-2 pb-1 border-b border-white/5">
							<User size={14} class="text-blue-400" />
							<h3 class="text-sm font-semibold text-white/90 uppercase tracking-widest">
								Datos del Visitante
							</h3>
						</div>

						<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
							<div class="md:col-span-2">
								<label for="cedula" class={labelClass}
									>Cédula / Identificación <span class="text-xs text-secondary font-normal ml-2"
										>(Presione Enter para buscar)</span
									> <span class="text-red-500">*</span></label
								>
								<div class="relative">
									<Search size={14} class="absolute left-3 top-2.5 text-gray-500" />
									<input
										id="cedula"
										class="{inputClass} pl-9 {getFieldStateClass(cedula, true)}"
										bind:value={cedula}
										onblur={handleCedulaBlur}
										onkeydown={(e) => e.key === 'Enter' && handleCedulaBlur()}
										placeholder="Ingrese identificación..."
									/>
									{#if searchingPerson}
										<div class="absolute right-3 top-2.5">
											<div
												class="w-3.5 h-3.5 border-2 border-blue-500/30 border-t-blue-500 rounded-full animate-spin"
											></div>
										</div>
									{/if}
								</div>
							</div>

							<div>
								<label for="nombre" class={labelClass}
									>Nombres <span class="text-red-500">*</span></label
								>
								<input
									id="nombre"
									class="{inputClass} {getFieldStateClass(nombre, true)}"
									bind:value={nombre}
									placeholder="Ej. Jenna"
								/>
							</div>

							<div>
								<label for="apellido" class={labelClass}
									>Apellidos <span class="text-red-500">*</span></label
								>
								<input
									id="apellido"
									class="{inputClass} {getFieldStateClass(apellido, true)}"
									bind:value={apellido}
									placeholder="Ej. Ortega"
								/>
							</div>

							<div class="md:col-span-2 relative">
								<label for="empresa" class={labelClass}>Empresa / Procedencia</label>
								<div class="relative">
									<Building2 size={14} class="absolute left-3 top-2.5 text-gray-500" />
									<input
										id="empresa"
										class="{inputClass} pl-9"
										bind:value={empresa_nombre}
										onfocus={() => (showEmpresaSuggestions = true)}
										onblur={() => setTimeout(() => (showEmpresaSuggestions = false), 200)}
										placeholder="Ej. Gomita S.A."
										autocomplete="off"
									/>
								</div>

								{#if showEmpresaSuggestions && filteredCompanies.length > 0}
									<div
										class="absolute z-50 left-0 right-0 mt-1 bg-surface-3 border border-white/10 rounded-lg shadow-xl max-h-40 overflow-y-auto py-1"
										transition:slide={{ duration: 150 }}
									>
										{#each filteredCompanies as comp}
											<button
												class="w-full text-left px-4 py-2 text-sm text-secondary hover:bg-white/5 hover:text-white transition-colors flex items-center gap-2"
												onclick={() => selectCompany(comp)}
											>
												<Building2 size={12} class="opacity-50" />
												{comp}
											</button>
										{/each}
									</div>
								{/if}
							</div>
						</div>
					</div>

					<!-- Validación Status -->
					{#if validationResult}
						<div
							class="p-4 rounded-xl border flex items-start gap-4 transition-all {!validationResult.puedeIngresar
								? 'bg-red-500/10 border-red-500/20'
								: 'bg-green-500/10 border-green-500/20'}"
							transition:slide
						>
							{#if validationResult.puedeIngresar}
								<div class="p-2 bg-green-500/20 rounded-lg text-green-400">
									<ShieldCheck size={20} />
								</div>
								<div>
									<h4 class="text-sm font-bold text-green-400 italic">ACREDITACIÓN VÁLIDA</h4>
									<p class="text-xs text-green-500/80 mt-1">
										El visitante no presenta restricciones para el ingreso.
									</p>
								</div>
							{:else}
								<div class="p-2 bg-red-500/20 rounded-lg text-red-400">
									<SearchX size={20} />
								</div>
								<div class="flex-1">
									<h4 class="text-sm font-bold text-red-400 italic">ACCESO RESTRINGIDO</h4>
									<p class="text-xs text-red-500/80 mt-1 leading-relaxed">
										{validationResult.motivoRechazo ||
											'Existen registros de seguridad que impiden el ingreso de esta persona.'}
									</p>
								</div>
							{/if}
						</div>
					{/if}

					<!-- Seccion 2: Datos del Ingreso -->
					<div class="space-y-4">
						<div class="flex items-center gap-2 pb-1 border-b border-white/5">
							<LogOut size={14} class="text-blue-400 rotate-180" />
							<h3 class="text-sm font-semibold text-white/90 uppercase tracking-widest">
								Detalles de la Visita
							</h3>
						</div>

						<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
							<div class="md:col-span-2">
								<label for="gafete" class={labelClass}>Gafete Asignado</label>
								<div class="relative">
									<IdCard size={14} class="absolute left-3 top-2.5 text-gray-500" />
									<input
										id="gafete"
										class="{inputClass} pl-9"
										bind:value={gafete}
										placeholder="Número de gafete físico"
									/>
								</div>
							</div>

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
									>Área Destino <span class="text-red-500">*</span></label
								>
								<div class="relative">
									<MapPin size={14} class="absolute left-3 top-2.5 text-gray-500" />
									<input
										id="area"
										class="{inputClass} pl-9 {getFieldStateClass(areaVisitada, true)}"
										bind:value={areaVisitada}
										placeholder="Piso, departamento, etc."
									/>
								</div>
							</div>

							<div class="md:col-span-2">
								<label for="motivo" class={labelClass}
									>Motivo del Ingreso <span class="text-red-500">*</span></label
								>
								<div class="relative">
									<MessageSquare size={14} class="absolute left-3 top-2.5 text-gray-500" />
									<input
										id="motivo"
										class="{inputClass} pl-9 {getFieldStateClass(motivo, true)}"
										bind:value={motivo}
										placeholder="Ej. Entrevista, mantenimiento, reunión..."
									/>
								</div>
							</div>
						</div>

						<!-- Observaciones toggle -->
						<div class="pt-2">
							<button
								type="button"
								onclick={() => (showObservaciones = !showObservaciones)}
								class="text-xs text-secondary hover:text-white flex items-center gap-1.5 px-2 py-1 rounded hover:bg-white/5 transition-all"
							>
								{#if showObservaciones}<ChevronDown size={14} />{:else}<ChevronRight
										size={14}
									/>{/if}
								Información Adicional
							</button>
							{#if showObservaciones}
								<div transition:slide={{ duration: 200 }} class="mt-3">
									<textarea
										bind:value={observaciones}
										class="w-full bg-black/30 border border-white/10 rounded-xl px-4 py-3 text-sm text-white resize-none outline-none focus:border-blue-500/50 transition-all"
										rows="3"
										placeholder="Cualquier aclaración relevante para este ingreso..."
									></textarea>
								</div>
							{/if}
						</div>
					</div>
				</div>
			</div>

			<!-- Footer -->
			<div class="px-6 py-5 border-t border-white/5 bg-surface-1 flex justify-end gap-3">
				<button
					onclick={handleClose}
					disabled={loading}
					class="px-5 py-2.5 rounded-lg border border-white/10 text-secondary hover:bg-white/5 hover:text-white transition-all text-sm font-medium disabled:opacity-50"
				>
					Cancelar
				</button>
				<button
					onclick={handleSubmit}
					disabled={loading || (validationResult && !validationResult.puedeIngresar)}
					class="px-8 py-2.5 rounded-lg bg-blue-600 hover:bg-blue-500 text-white shadow-lg shadow-blue-500/20 transition-all text-sm font-bold flex items-center gap-2 disabled:opacity-40 disabled:grayscale disabled:cursor-not-allowed"
				>
					{#if loading}
						<div
							class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"
						></div>
					{/if}
					REGISTRAR INGRESO
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.custom-scrollbar::-webkit-scrollbar {
		width: 6px;
	}
	.custom-scrollbar::-webkit-scrollbar-track {
		background: transparent;
	}
	.custom-scrollbar::-webkit-scrollbar-thumb {
		background: rgba(255, 255, 255, 0.1);
		border-radius: 10px;
	}
	.custom-scrollbar::-webkit-scrollbar-thumb:hover {
		background: rgba(255, 255, 255, 0.2);
	}
</style>
