<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { fade, scale, fly } from 'svelte/transition';
	import { toast } from 'svelte-5-french-toast';
	import {
		X,
		ChevronDown,
		Plus,
		ShieldCheck,
		SearchX,
		Building2,
		UserCircle,
		MapPin,
		MessageSquare,
		IdCard
	} from 'lucide-svelte';

	// Superforms & Zod v4
	import { superForm } from 'sveltekit-superforms';
	import { zod4 } from 'sveltekit-superforms/adapters';
	import {
		ingresoVisitaSchema,
		ingresoVisitaSchemaBase,
		type IngresoVisitaFormData
	} from '$lib/schemas/visitaSchema';

	// Logic
	import { ingresoVisitaService } from '$lib/services/ingresoVisitaService';
	import { getVisitanteByCedula } from '$lib/logic/visitante/visitanteService';
	import { empresaStore } from '$lib/stores/empresaStore.svelte';
	import { submitCreateEmpresa } from '$lib/logic/empresa/empresaService';
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

	// UI State
	let showEmpresaDropdown = $state(false);
	let showEmpresaModal = $state(false);
	let nuevaEmpresaNombre = $state('');
	let creatingEmpresa = $state(false);
	let empresaError = $state('');
	let checkTimeout: ReturnType<typeof setTimeout>;
	let cedulaDuplicateError = $state<string | null>(null);

	const defaultValues: IngresoVisitaFormData = {
		cedula: '',
		nombre: '',
		segundoNombre: '',
		apellido: '',
		segundoApellido: '',
		empresaId: '',
		anfitrion: '',
		areaVisitada: '',
		motivo: '',
		gafete: '',
		observaciones: ''
	};

	// Superform Initialization
	const { form, errors, constraints, enhance, reset, validate } = superForm<IngresoVisitaFormData>(
		defaultValues,
		{
			SPA: true,
			validators: zod4(ingresoVisitaSchema),
			resetForm: false,
			validationMethod: 'oninput',
			onUpdate: async ({ form: f }) => {
				if (f.valid) {
					if (validationResult && !validationResult.puedeIngresar) {
						toast.error('Acceso restringido para esta persona');
						return;
					}

					loading = true;
					try {
						// Map empresaId to name for the backend
						const selectedEmpresa = empresaStore.empresas.find((e) => e.id === f.data.empresaId);

						await ingresoVisitaService.createIngreso({
							cedula: f.data.cedula.trim(),
							nombre: f.data.nombre.trim(),
							segundo_nombre: f.data.segundoNombre.trim() || undefined,
							apellido: f.data.apellido.trim(),
							segundo_apellido: f.data.segundoApellido.trim() || undefined,
							empresa_nombre: selectedEmpresa?.nombre || undefined,
							anfitrion: f.data.anfitrion.trim(),
							area_visitada: f.data.areaVisitada.trim(),
							motivo: f.data.motivo.trim(),
							gafete: f.data.gafete.trim() || undefined,
							observaciones: f.data.observaciones.trim() || undefined,
							usuario_ingreso_id: $currentUser?.id || ''
						});

						toast.success('Ingreso de visita registrado');
						handleClose();
						if (onComplete) onComplete();
					} catch (e: any) {
						console.error(e);
						toast.error('Error al registrar: ' + (e.message || String(e)));
					} finally {
						loading = false;
					}
				}
			}
		}
	);

	// Sync y Reset al abrir
	$effect(() => {
		if (show) {
			empresaStore.init();
			if (initialPerson) {
				fillPersonData(initialPerson);
			} else {
				reset();
				validationResult = null;
				cedulaDuplicateError = null;
			}
		}
	});

	onDestroy(() => {
		if (checkTimeout) clearTimeout(checkTimeout);
	});

	async function fillPersonData(person: any) {
		const data = {
			cedula: person.cedula || '',
			nombre: person.nombre || '',
			segundoNombre: person.segundoNombre || '',
			apellido: person.apellido || '',
			segundoApellido: person.segundoApellido || '',
			empresaId: '', // We'll try to find it by name if possible
			anfitrion: '',
			areaVisitada: '',
			motivo: '',
			gafete: '',
			observaciones: ''
		};

		// Try to match enterprise name to ID for the dropdown
		if (person.empresa_nombre) {
			const matched = empresaStore.empresas.find(
				(e) => e.nombre.toLowerCase() === person.empresa_nombre.toLowerCase()
			);
			if (matched) data.empresaId = matched.id;
		}

		reset({ data });

		if (person.cedula) {
			validarAcceso(person.cedula);
		}
	}

	async function handleCedulaInput(event: Event) {
		const input = event.target as HTMLInputElement;
		const val = input.value;
		$form.cedula = val;

		if (checkTimeout) clearTimeout(checkTimeout);
		validate('cedula');

		if (val.length < 5) {
			validationResult = null;
			return;
		}

		checkTimeout = setTimeout(async () => {
			searchingPerson = true;
			try {
				const res = await getVisitanteByCedula(val);
				if (res.ok && res.data) {
					const p = res.data;
					$form.nombre = p.nombre;
					$form.segundoNombre = p.segundoNombre || '';
					$form.apellido = p.apellido;
					$form.segundoApellido = p.segundoApellido || '';

					// Sync company if found
					if (p.empresaNombre) {
						const matched = empresaStore.empresas.find(
							(e) => e.nombre.toLowerCase() === p.empresaNombre!.toLowerCase()
						);
						if (matched) $form.empresaId = matched.id;
					}
					toast.success('Visitante encontrado');
				}
				await validarAcceso(val);
			} catch (e) {
				console.error(e);
			} finally {
				searchingPerson = false;
			}
		}, 500);
	}

	async function validarAcceso(ced: string) {
		try {
			validationResult = await ingresoVisitaService.validarIngreso(ced);
			if (validationResult && !validationResult.puedeIngresar) {
				invoke('play_alert_sound');
				toast.error(validationResult.motivoRechazo || 'Persona no autorizada');
			}
		} catch (e) {
			validationResult = { puedeIngresar: true };
		}
	}

	async function handleCrearEmpresa() {
		if (!nuevaEmpresaNombre.trim()) return;
		creatingEmpresa = true;
		empresaError = '';
		const result = await submitCreateEmpresa(nuevaEmpresaNombre);
		if (result.ok) {
			empresaStore.add(result.empresa);
			$form.empresaId = result.empresa.id;
			nuevaEmpresaNombre = '';
			showEmpresaModal = false;
		} else {
			empresaError = result.error;
		}
		creatingEmpresa = false;
	}

	function handleClose() {
		if (!loading) {
			show = false;
			reset();
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (!show) return;
		if (e.key === 'Escape') handleClose();
		if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 's') {
			e.preventDefault();
			const f = document.querySelector('form[method="POST"]') as HTMLFormElement;
			if (f) f.requestSubmit();
		}
	}

	// UI Helpers
	const inputClass =
		'w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[38px] text-sm text-white placeholder:text-gray-600 transition-all outline-none focus:border-blue-500/50 focus:ring-1 focus:ring-blue-500/20 disabled:opacity-50';
	const labelClass = 'block text-[11px] font-medium text-secondary mb-1 uppercase tracking-wider';
	const errorClass = 'text-[11px] text-red-500 mt-0.5';

	function getFieldStateClass(field: keyof IngresoVisitaFormData, value: any) {
		if ($errors[field]) return '!border-red-500/50 !ring-1 !ring-red-500/20';
		if (value && String(value).trim() !== '')
			return '!border-green-500/50 !ring-1 !ring-green-500/20';
		return '';
	}
</script>

```svelte
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
			class="relative z-10 w-full max-w-4xl max-h-[95vh] overflow-hidden rounded-xl bg-surface-2 shadow-2xl border border-surface flex flex-col"
			transition:fly={{ y: 20, duration: 200 }}
		>
			<!-- Header -->
			<div
				class="flex-none flex items-center justify-between px-5 py-4 bg-surface-2 border-b border-surface"
			>
				<h2 class="text-xl font-semibold text-primary">Ingreso de Visita</h2>
				<button
					onclick={handleClose}
					class="text-gray-400 hover:text-white transition-colors"
					aria-label="Cerrar"
				>
					<X size={20} />
				</button>
			</div>

			<!-- Body -->
			<div class="flex-1 overflow-y-auto custom-scrollbar">
				<form method="POST" use:enhance class="p-5 space-y-5">
					<div class="grid grid-cols-1 lg:grid-cols-2 gap-5 items-start">
						<!-- Columna 1: Datos Personales -->
						<div
							class="bg-surface-1 rounded-lg border border-surface p-5 grid grid-cols-1 md:grid-cols-2 gap-4"
						>
							<div class="md:col-span-2">
								<label for="cedula" class={labelClass}>
									Cédula <span class="text-red-500 ml-0.5">*</span>
								</label>
								<div class="relative">
									<input
										id="cedula"
										name="cedula"
										class="{inputClass} {getFieldStateClass('cedula', $form.cedula)}"
										bind:value={$form.cedula}
										oninput={handleCedulaInput}
										placeholder="1-2345-6789"
										disabled={loading}
										{...$constraints.cedula}
									/>
									{#if searchingPerson}
										<div class="absolute right-3 top-3">
											<div
												class="w-4 h-4 border-2 border-blue-500/30 border-t-blue-500 rounded-full animate-spin"
											></div>
										</div>
									{/if}
								</div>
								{#if $errors.cedula}<p class={errorClass}>{$errors.cedula}</p>{/if}
							</div>

							<div>
								<label for="nombre" class={labelClass}
									>Nombre <span class="text-red-500 ml-0.5">*</span></label
								>
								<input
									id="nombre"
									name="nombre"
									class="{inputClass} {getFieldStateClass('nombre', $form.nombre)}"
									bind:value={$form.nombre}
									oninput={() => validate('nombre')}
									placeholder="Juan"
									disabled={loading}
								/>
								{#if $errors.nombre}<p class={errorClass}>{$errors.nombre}</p>{/if}
							</div>

							<div>
								<label for="segundoNombre" class={labelClass}>Segundo Nombre</label>
								<input
									id="segundoNombre"
									name="segundoNombre"
									class={inputClass}
									bind:value={$form.segundoNombre}
									placeholder=""
									disabled={loading}
								/>
							</div>

							<div>
								<label for="apellido" class={labelClass}
									>Apellido <span class="text-red-500 ml-0.5">*</span></label
								>
								<input
									id="apellido"
									name="apellido"
									class="{inputClass} {getFieldStateClass('apellido', $form.apellido)}"
									bind:value={$form.apellido}
									oninput={() => validate('apellido')}
									placeholder="Pérez"
									disabled={loading}
								/>
								{#if $errors.apellido}<p class={errorClass}>{$errors.apellido}</p>{/if}
							</div>

							<div>
								<label for="segundoApellido" class={labelClass}>Segundo Apellido</label>
								<input
									id="segundoApellido"
									name="segundoApellido"
									class={inputClass}
									bind:value={$form.segundoApellido}
									placeholder=""
									disabled={loading}
								/>
							</div>

							<div class="md:col-span-2 relative">
								<label for="empresaId" class={labelClass}
									>Empresa <span class="text-red-500 ml-0.5">*</span></label
								>
								<div class="flex gap-2 relative">
									<div class="relative flex-1">
										<button
											type="button"
											disabled={loading || empresaStore.loading}
											onclick={() => (showEmpresaDropdown = !showEmpresaDropdown)}
											class="{inputClass} flex items-center justify-between cursor-pointer w-full text-left {showEmpresaDropdown
												? '!border-blue-500/50 !ring-1 !ring-blue-500/20'
												: getFieldStateClass('empresaId', $form.empresaId)}"
										>
											<span class="truncate">
												{#if empresaStore.loading}
													Cargando...
												{:else}
													{empresaStore.empresas.find((e) => e.id === $form.empresaId)?.nombre ||
														'Seleccione empresa'}
												{/if}
											</span>
											<ChevronDown size={16} class="text-secondary" />
										</button>

										{#if showEmpresaDropdown}
											<div
												class="fixed inset-0 z-40"
												onclick={() => (showEmpresaDropdown = false)}
												role="presentation"
											></div>
											<div
												class="absolute z-50 w-full mt-1 bg-[#1c2128] border border-white/10 rounded-lg shadow-xl overflow-hidden p-1 origin-top max-h-60 overflow-y-auto"
												transition:fly={{ y: -10, duration: 200 }}
											>
												{#if empresaStore.empresas.length === 0}
													<div class="px-3 py-2 text-sm text-gray-500">No hay empresas</div>
												{:else}
													{#each empresaStore.empresas as empresa}
														<button
															type="button"
															onclick={() => {
																$form.empresaId = empresa.id;
																showEmpresaDropdown = false;
																validate('empresaId');
															}}
															class="w-full text-left px-3 py-1.5 text-sm text-gray-300 hover:bg-white/10 rounded-md transition-colors flex items-center justify-between group"
														>
															<span>{empresa.nombre}</span>
															{#if $form.empresaId === empresa.id}
																<ShieldCheck size={14} class="text-blue-400" />
															{/if}
														</button>
													{/each}
												{/if}
											</div>
										{/if}
									</div>
									<button
										type="button"
										onclick={() => (showEmpresaModal = true)}
										disabled={loading}
										class="px-2.5 rounded-lg border border-white/10 bg-black/20 text-secondary hover:text-white hover:bg-white/5 transition-colors"
										title="Añadir nueva empresa"
									>
										<Plus size={18} />
									</button>
								</div>
								{#if $errors.empresaId}<p class={errorClass}>{$errors.empresaId}</p>{/if}
							</div>
						</div>

						<!-- Columna 2: Datos del Ingreso y Validación -->
						<div class="space-y-5">
							<!-- Validación Status -->
							{#if validationResult}
								<div
									class="p-4 rounded-xl border flex items-start gap-4 transition-all {!validationResult.puedeIngresar
										? 'bg-red-500/10 border-red-500/20'
										: 'bg-green-500/10 border-green-500/20'}"
									transition:fade
								>
									{#if validationResult.puedeIngresar}
										<div class="p-2 bg-green-500/20 rounded-lg text-green-400">
											<ShieldCheck size={20} />
										</div>
										<div>
											<h4 class="text-sm font-bold text-green-400 italic">ACREDITACIÓN VÁLIDA</h4>
											<p class="text-xs text-green-500/80 mt-1">
												El visitante no presenta restricciones.
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
													'Existen registros de seguridad que impiden el ingreso.'}
											</p>
										</div>
									{/if}
								</div>
							{/if}

							<!-- Seccion 2: Datos del Ingreso -->
							<div class="bg-surface-1 rounded-lg border border-surface p-5 space-y-4">
								<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
									<div class="md:col-span-2">
										<label for="motivo" class={labelClass}
											>Motivo <span class="text-red-500 ml-0.5">*</span></label
										>
										<div class="relative">
											<MessageSquare size={14} class="absolute left-3 top-3 text-gray-500" />
											<input
												id="motivo"
												name="motivo"
												class="{inputClass} pl-9 {getFieldStateClass('motivo', $form.motivo)}"
												bind:value={$form.motivo}
												oninput={() => validate('motivo')}
												placeholder="Ej. Entrevista, mantenimiento, reunión..."
											/>
										</div>
										{#if $errors.motivo}<p class={errorClass}>{$errors.motivo}</p>{/if}
									</div>

									<div>
										<label for="anfitrion" class={labelClass}
											>Anfitrión <span class="text-red-500 ml-0.5">*</span></label
										>
										<div class="relative">
											<UserCircle size={14} class="absolute left-3 top-3 text-gray-500" />
											<input
												id="anfitrion"
												name="anfitrion"
												class="{inputClass} pl-9 {getFieldStateClass('anfitrion', $form.anfitrion)}"
												bind:value={$form.anfitrion}
												oninput={() => validate('anfitrion')}
												placeholder="¿A quién visita?"
											/>
										</div>
										{#if $errors.anfitrion}<p class={errorClass}>{$errors.anfitrion}</p>{/if}
									</div>

									<div>
										<label for="area" class={labelClass}
											>Área <span class="text-red-500 ml-0.5">*</span></label
										>
										<div class="relative">
											<MapPin size={14} class="absolute left-3 top-3 text-gray-500" />
											<input
												id="area"
												name="areaVisitada"
												class="{inputClass} pl-9 {getFieldStateClass(
													'areaVisitada',
													$form.areaVisitada
												)}"
												bind:value={$form.areaVisitada}
												oninput={() => validate('areaVisitada')}
												placeholder="Piso, etc."
											/>
										</div>
										{#if $errors.areaVisitada}<p class={errorClass}>{$errors.areaVisitada}</p>{/if}
									</div>

									<div class="md:col-span-2">
										<label
											class="block text-[11px] font-bold uppercase tracking-wider text-secondary mb-2"
											for="gafete"
										>
											Gafete
										</label>
										<div class="flex items-center">
											<div class="relative w-[80px] group">
												<div
													class="absolute inset-0 bg-blue-500/5 rounded-lg opacity-0 group-focus-within:opacity-100 transition-opacity"
												></div>
												<input
													id="gafete"
													name="gafete"
													type="text"
													bind:value={$form.gafete}
													placeholder="00"
													class="w-full h-12 bg-black/20 border border-white/10 rounded-lg text-center font-mono text-2xl tracking-widest text-white focus:outline-none focus:border-blue-500/50 focus:ring-1 focus:ring-blue-500/20 transition-all placeholder:text-gray-700"
													autocomplete="off"
												/>
											</div>
										</div>
									</div>
								</div>

								<textarea
									name="observaciones"
									bind:value={$form.observaciones}
									class="w-full bg-black/30 border border-white/10 rounded-xl px-4 py-3 text-sm text-white resize-none outline-none focus:border-blue-500/50"
									rows="2"
									placeholder="Observaciones..."
								></textarea>
							</div>
						</div>
					</div>

					<!-- Footer Actions (Inside Form) -->
					<div
						class="flex-none flex items-center justify-center gap-4 px-6 py-4 border-t border-surface bg-surface-1 sticky bottom-0 z-20"
					>
						<button
							type="button"
							onclick={handleClose}
							disabled={loading}
							class="px-4 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-white/60 hover:text-white text-sm"
						>
							Cancelar
						</button>
						<button
							type="submit"
							disabled={loading}
							class="px-6 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-success hover:text-success text-sm flex items-center justify-center gap-2"
						>
							{#if loading}
								<div
									class="w-4 h-4 border-2 border-current border-t-transparent rounded-full animate-spin"
								></div>
							{/if}
							Crear Ingreso
						</button>
					</div>
				</form>
			</div>
		</div>
	</div>
{/if}

<!-- Modal para nueva empresa (Anidado) -->
{#if showEmpresaModal}
	<div
		class="fixed inset-0 z-[110] flex items-center justify-center p-4 bg-black/70 backdrop-blur-sm"
		transition:fade={{ duration: 150 }}
	>
		<div
			class="absolute inset-0"
			onclick={() => !creatingEmpresa && (showEmpresaModal = false)}
			role="presentation"
		></div>

		<div
			class="relative w-full max-w-sm rounded-xl bg-surface-2 shadow-2xl border border-surface overflow-hidden"
			transition:scale={{ start: 0.95 }}
		>
			<div class="px-5 py-4 border-b border-surface bg-surface-1">
				<h3 class="text-base font-semibold text-primary">Nueva Empresa</h3>
			</div>

			<div class="p-5 space-y-4">
				{#if empresaError}
					<div class="rounded-lg bg-red-500/10 border border-red-500/20 p-3 text-xs text-red-300">
						{empresaError}
					</div>
				{/if}
				<div class="space-y-1">
					<label for="newEmpresa" class={labelClass}>Nombre Comercial</label>
					<input
						id="newEmpresa"
						type="text"
						bind:value={nuevaEmpresaNombre}
						placeholder="Ej: Brisas S.A."
						disabled={creatingEmpresa}
						class={inputClass}
						onkeydown={(e) => e.key === 'Enter' && handleCrearEmpresa()}
					/>
				</div>
			</div>

			<div class="flex justify-end gap-2 px-5 py-4 border-t border-surface bg-surface-1">
				<button
					type="button"
					onclick={() => (showEmpresaModal = false)}
					class="px-4 py-1.5 text-xs font-medium rounded-lg border-2 border-surface text-secondary hover:border-white/60 hover:text-white"
				>
					Cancelar
				</button>
				<button
					type="button"
					disabled={creatingEmpresa || !nuevaEmpresaNombre.trim()}
					onclick={handleCrearEmpresa}
					class="px-5 py-1.5 text-xs font-medium rounded-lg border-2 border-surface text-secondary hover:border-success hover:text-success"
				>
					{creatingEmpresa ? 'Guardando...' : 'Guardar'}
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
