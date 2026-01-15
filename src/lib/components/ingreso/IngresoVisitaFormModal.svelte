<script lang="ts">
	import { onDestroy } from 'svelte';
	import { fade, fly, slide } from 'svelte/transition';
	import { toast } from 'svelte-5-french-toast';
	import {
		X,
		ChevronDown,
		Plus,
		CheckCircle,
		ShieldCheck,
		SearchX,
		ChevronRight
	} from 'lucide-svelte';

	// Superforms & Zod v4
	import { superForm } from 'sveltekit-superforms';
	import { zod4 } from 'sveltekit-superforms/adapters';
	import { ingresoVisitaSchema, type IngresoVisitaFormData } from '$lib/schemas/visitaSchema';

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
	let showObservaciones = $state(false);
	let checkTimeout: ReturnType<typeof setTimeout>;

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
				showObservaciones = false;
			}
		}
	});

	// Auto-show observaciones if value exists
	$effect(() => {
		if ($form.observaciones && $form.observaciones.trim().length > 0) {
			showObservaciones = true;
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
		const res = await submitCreateEmpresa(nuevaEmpresaNombre);

		if (res.ok && res.empresa) {
			await empresaStore.refresh(); // Recargar lista
			// Seleccionar la nueva empresa
			$form.empresaId = res.empresa.id;
			showEmpresaModal = false;
			nuevaEmpresaNombre = ''; // Reset
		} else if (!res.ok) {
			console.error('Error creando empresa:', res.error);
			toast.error('Error creando empresa');
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
		'w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 text-sm text-white placeholder:text-gray-500 transition-all outline-none focus:border-blue-500/50 focus:ring-1 focus:ring-blue-500/20 disabled:opacity-50 disabled:cursor-not-allowed';
	const labelClass = 'block text-[11px] font-bold uppercase tracking-wider text-secondary mb-1.5';
	const errorClass = 'text-[10px] text-red-400 mt-1 ml-0.5';

	function getFieldStateClass(field: keyof IngresoVisitaFormData, value?: any) {
		if ($errors[field]) return '!border-red-500/50 !ring-1 !ring-red-500/20';
		if (value && String(value).trim() !== '')
			return '!border-green-500/50 !ring-1 !ring-green-500/20';
		return '';
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
			class="relative z-10 w-full max-w-[480px] max-h-[95vh] overflow-hidden rounded-xl bg-surface-2 shadow-2xl border border-surface flex flex-col"
			transition:fly={{ y: 20, duration: 200 }}
		>
			<!-- Header -->
			<div
				class="flex-none flex items-center justify-between px-5 py-4 bg-surface-2 border-b border-surface"
			>
				<h2 class="text-lg font-semibold text-primary ml-1">Ingreso de Visita</h2>
				<button
					onclick={handleClose}
					class="p-1.5 rounded-lg text-secondary hover:text-primary hover:bg-surface-3 transition-colors"
					aria-label="Cerrar"
				>
					<X size={18} />
				</button>
			</div>

			<!-- Body -->
			<div class="flex-1 overflow-hidden">
				<form method="POST" use:enhance class="flex flex-col h-full">
					<!-- Scrollable Content -->
					<div class="flex-1 overflow-y-auto custom-scrollbar p-5 space-y-5">
						<!-- Validación Status -->
						{#if validationResult}
							<div
								class="p-4 rounded-xl border flex items-start gap-3 transition-all {!validationResult.puedeIngresar
									? 'bg-red-500/10 border-red-500/20'
									: 'bg-green-500/10 border-green-500/20'}"
								transition:fade
							>
								{#if validationResult.puedeIngresar}
									<div class="p-1.5 bg-green-500/20 rounded-lg text-green-400 mt-0.5">
										<ShieldCheck size={18} />
									</div>
									<div>
										<h4 class="text-sm font-bold text-green-400 italic">ACREDITACIÓN VÁLIDA</h4>
										<p class="text-xs text-green-500/80 mt-1">
											El visitante no presenta restricciones.
										</p>
									</div>
								{:else}
									<div class="p-1.5 bg-red-500/20 rounded-lg text-red-400 mt-0.5">
										<SearchX size={18} />
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

						<div class="bg-surface-1 rounded-lg border border-surface p-5 grid grid-cols-2 gap-4">
							<!-- Cédula (Full) -->
							<div class="col-span-2">
								<label for="cedula" class={labelClass}>
									Cédula <span class="text-red-500">*</span>
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
										<div class="absolute right-3 top-2.5">
											<div
												class="w-3.5 h-3.5 border-2 border-blue-500/30 border-t-blue-500 rounded-full animate-spin"
											></div>
										</div>
									{/if}
								</div>
								{#if $errors.cedula}<p class={errorClass}>{$errors.cedula}</p>{/if}
							</div>

							<!-- Nombre -->
							<div>
								<label for="nombre" class={labelClass}
									>Nombre <span class="text-red-500">*</span></label
								>
								<input
									id="nombre"
									name="nombre"
									class="{inputClass} {getFieldStateClass('nombre', $form.nombre)}"
									bind:value={$form.nombre}
									oninput={() => validate('nombre')}
									placeholder="Juan"
									disabled={loading || searchingPerson}
								/>
								{#if $errors.nombre}<p class={errorClass}>{$errors.nombre}</p>{/if}
							</div>

							<!-- Segundo Nombre -->
							<div>
								<label for="segundoNombre" class={labelClass}>Segundo Nombre</label>
								<input
									id="segundoNombre"
									name="segundoNombre"
									class={inputClass}
									bind:value={$form.segundoNombre}
									placeholder=""
									disabled={loading || searchingPerson}
								/>
							</div>

							<!-- Apellido -->
							<div>
								<label for="apellido" class={labelClass}
									>Apellido <span class="text-red-500">*</span></label
								>
								<input
									id="apellido"
									name="apellido"
									class="{inputClass} {getFieldStateClass('apellido', $form.apellido)}"
									bind:value={$form.apellido}
									oninput={() => validate('apellido')}
									placeholder="Pérez"
									disabled={loading || searchingPerson}
								/>
								{#if $errors.apellido}<p class={errorClass}>{$errors.apellido}</p>{/if}
							</div>

							<!-- Segundo Apellido -->
							<div>
								<label for="segundoApellido" class={labelClass}>Segundo Apellido</label>
								<input
									id="segundoApellido"
									name="segundoApellido"
									class={inputClass}
									bind:value={$form.segundoApellido}
									placeholder=""
									disabled={loading || searchingPerson}
								/>
							</div>

							<!-- Empresa (Full) -->
							<div class="col-span-2 relative">
								<label for="empresaId" class={labelClass}
									>Empresa <span class="text-red-500">*</span></label
								>
								<div class="flex gap-2 relative">
									<!-- Custom Dropdown Trigger -->
									<div class="relative flex-1">
										<button
											type="button"
											onclick={() => (showEmpresaDropdown = !showEmpresaDropdown)}
											disabled={loading || searchingPerson || empresaStore.loading}
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

										<!-- Dropdown Options -->
										{#if showEmpresaDropdown}
											<!-- Backdrop -->
											<!-- svelte-ignore a11y_click_events_have_key_events -->
											<!-- svelte-ignore a11y_no_static_element_interactions -->
											<div
												class="fixed inset-0 z-40"
												onclick={() => (showEmpresaDropdown = false)}
											></div>

											<div
												class="absolute z-50 w-full mt-1 bg-[#1c2128] border border-white/10 rounded-lg shadow-xl overflow-hidden p-1 origin-top max-h-60 overflow-y-auto"
												transition:fly={{ y: -5, duration: 200 }}
											>
												{#if !empresaStore.empresas || empresaStore.empresas.length === 0}
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
																<CheckCircle size={14} class="text-white" />
															{/if}
														</button>
													{/each}
												{/if}
											</div>
										{/if}
									</div>

									<!-- Add Button -->
									<button
										type="button"
										onclick={() => (showEmpresaModal = true)}
										disabled={loading || searchingPerson}
										class="px-3 py-1.5 rounded-lg border border-white/10 bg-black/20 text-secondary hover:text-white hover:bg-white/5 transition-colors"
										title="Añadir nueva empresa"
									>
										<Plus size={16} />
									</button>
								</div>
								{#if $errors.empresaId}<p class={errorClass}>{$errors.empresaId}</p>{/if}
							</div>

							<!-- Seccion de Ingreso Mixed In but visually grouped -->

							<!-- Anfitrión & Motivo -->
							<div>
								<label for="anfitrion" class={labelClass}
									>Anfitrión <span class="text-red-500">*</span></label
								>
								<input
									id="anfitrion"
									name="anfitrion"
									class="{inputClass} {getFieldStateClass('anfitrion', $form.anfitrion)}"
									bind:value={$form.anfitrion}
									oninput={() => validate('anfitrion')}
									placeholder="¿A quién visita?"
									disabled={loading}
								/>
								{#if $errors.anfitrion}<p class={errorClass}>{$errors.anfitrion}</p>{/if}
							</div>

							<div>
								<label for="motivo" class={labelClass}
									>Motivo <span class="text-red-500">*</span></label
								>
								<input
									id="motivo"
									name="motivo"
									class="{inputClass} {getFieldStateClass('motivo', $form.motivo)}"
									bind:value={$form.motivo}
									oninput={() => validate('motivo')}
									placeholder="Ej. Entrevista..."
									disabled={loading}
								/>
								{#if $errors.motivo}<p class={errorClass}>{$errors.motivo}</p>{/if}
							</div>

							<!-- Área & Gafete -->
							<div>
								<label for="area" class={labelClass}>Área <span class="text-red-500">*</span></label
								>
								<input
									id="area"
									name="areaVisitada"
									class="{inputClass} {getFieldStateClass('areaVisitada', $form.areaVisitada)}"
									bind:value={$form.areaVisitada}
									oninput={() => validate('areaVisitada')}
									placeholder="Piso, etc."
									disabled={loading}
								/>
								{#if $errors.areaVisitada}<p class={errorClass}>{$errors.areaVisitada}</p>{/if}
							</div>

							<div>
								<label
									class="block text-[11px] font-bold uppercase tracking-wider text-secondary mb-1.5"
									for="gafete"
								>
									Gafete
								</label>
								<div class="relative w-full group">
									<input
										id="gafete"
										name="gafete"
										type="text"
										bind:value={$form.gafete}
										placeholder="00"
										class="{inputClass} text-center font-mono tracking-widest"
										autocomplete="off"
										disabled={loading}
									/>
								</div>
							</div>

							<!-- Observaciones - Toggle colapsable -->
							<div class="col-span-2 border-t border-surface pt-2">
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
									{#if !showObservaciones && $form.observaciones?.trim()}
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
												bind:value={$form.observaciones}
												disabled={loading}
											></textarea>
										</div>
									</div>
								{/if}
							</div>
						</div>
					</div>

					<!-- Footer Actions -->
					<div
						class="flex-none flex items-center justify-end gap-3 px-6 py-4 border-t border-surface bg-surface-1"
					>
						<button
							type="button"
							onclick={handleClose}
							disabled={loading}
							class="flex items-center gap-2 px-3 py-2 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-white/60 hover:text-white/80 text-xs"
						>
							Cancelar
						</button>
						<button
							type="submit"
							disabled={loading}
							class="flex items-center gap-2 px-5 py-2 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-success hover:text-success text-xs disabled:opacity-50 disabled:cursor-not-allowed"
						>
							{loading ? 'Guardando...' : 'Crear Ingreso'}
						</button>
					</div>
				</form>
			</div>
		</div>
	</div>
{/if}

<!-- Modal Inline para Crear Empresa -->
{#if showEmpresaModal}
	<div
		class="fixed inset-0 z-[110] flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
		transition:fade={{ duration: 150 }}
	>
		<!-- Backdrop click -->
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div class="absolute inset-0" onclick={() => (showEmpresaModal = false)}></div>

		<div
			class="relative w-full max-w-[320px] bg-surface-2 rounded-xl shadow-2xl border border-surface overflow-hidden"
			transition:fly={{ y: 10, duration: 200 }}
		>
			<div class="px-5 py-4">
				<h3 class="text-sm font-semibold text-white mb-4">Nueva Empresa</h3>
				<div class="space-y-1">
					<label for="newEmpresa" class={labelClass}>Nombre Comercial</label>
					<input
						id="newEmpresa"
						type="text"
						bind:value={nuevaEmpresaNombre}
						placeholder="Ej: Servicios Generales S.A."
						disabled={creatingEmpresa}
						class={inputClass}
						onkeydown={(e) => e.key === 'Enter' && handleCrearEmpresa()}
					/>
				</div>
			</div>
			<div class="flex justify-end gap-2 px-5 py-3 border-t border-surface bg-surface-1">
				<button
					type="button"
					disabled={creatingEmpresa}
					onclick={() => (showEmpresaModal = false)}
					class="px-3 py-1.5 text-xs font-medium rounded-lg border-2 border-surface text-secondary transition-all duration-200 hover:border-white/60 hover:text-white/80"
				>
					Cancelar
				</button>
				<button
					type="button"
					disabled={creatingEmpresa || !nuevaEmpresaNombre.trim()}
					onclick={handleCrearEmpresa}
					class="px-3 py-1.5 text-xs font-medium rounded-lg border-2 border-surface text-secondary transition-all duration-200 hover:border-blue-500 hover:text-blue-500 disabled:opacity-50"
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

	/* Observaciones container - match IngresoFormModal */
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
