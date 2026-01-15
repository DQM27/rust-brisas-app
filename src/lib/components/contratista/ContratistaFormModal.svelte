<script lang="ts">
	import { fade, fly, scale } from 'svelte/transition';
	import { X, Car, Plus, ChevronDown } from 'lucide-svelte';
	import { onMount, onDestroy } from 'svelte';
	import type {
		ContratistaResponse,
		CreateContratistaInput,
		UpdateContratistaInput
	} from '$lib/types/contratista';
	import { submitCreateEmpresa } from '$lib/logic/empresa/empresaService';
	import { invoke } from '@tauri-apps/api/core';
	import { empresaStore } from '$lib/stores/empresaStore.svelte';
	import VehiculoManagerModal from '$lib/components/vehiculo/VehiculoManagerModal.svelte';

	// Superforms & Zod v4
	import { superForm } from 'sveltekit-superforms';
	import { zod4 } from 'sveltekit-superforms/adapters';
	import { contratistaSchema, type ContratistaFormData } from '$lib/schemas/contratistaSchema';

	const defaultValues: ContratistaFormData = {
		cedula: '',
		nombre: '',
		segundoNombre: '',
		apellido: '',
		segundoApellido: '',
		empresaId: '',
		fechaVencimientoPraind: ''
	};

	interface Props {
		show: boolean;
		contratista?: ContratistaResponse | null;
		loading?: boolean;
		onSave: (data: CreateContratistaInput | UpdateContratistaInput) => Promise<boolean | void>;
		onClose: () => void;
		readonly?: boolean;
	}

	let {
		show,
		contratista = null,
		loading = false,
		onSave,
		onClose,
		readonly = false
	}: Props = $props();

	// Derived Mode
	const isEditMode = $derived(!!contratista);
	const modalTitle = $derived(
		readonly
			? `Ver Detalle: ${contratista?.nombre} ${contratista?.apellido}`
			: isEditMode
				? `Editar: ${contratista?.nombre} ${contratista?.apellido}`
				: 'Nuevo Contratista'
	);

	// Empresas State
	let showEmpresaDropdown = $state(false);
	let showEmpresaModal = $state(false);
	let nuevaEmpresaNombre = $state('');
	let creatingEmpresa = $state(false);
	let empresaError = $state('');

	// Vehicle Modal State
	let showVehiculoModal = $state(false);

	// Validation State for Real-time checks
	let checkTimeout: ReturnType<typeof setTimeout>;
	let cedulaDuplicateError = $state<string | null>(null);

	// Initial Data Construction
	const initialData = $derived.by(() => {
		return {
			cedula: contratista?.cedula || '',
			nombre: contratista?.nombre || '',
			segundoNombre: contratista?.segundoNombre || '',
			apellido: contratista?.apellido || '',
			segundoApellido: contratista?.segundoApellido || '',
			empresaId: contratista?.empresaId || '',
			fechaVencimientoPraind: contratista?.fechaVencimientoPraind
				? formatDateForDisplay(contratista.fechaVencimientoPraind)
				: ''
		};
	});

	// Superform Initialization with Zod v4 adapter
	const { form, errors, constraints, enhance, reset, validate } = superForm<ContratistaFormData>(
		defaultValues,
		{
			SPA: true,
			validators: zod4(contratistaSchema),
			resetForm: false, // We control reset manually when modal opens/closes
			validationMethod: 'oninput',
			onUpdate: async ({ form: f }) => {
				if (f.valid) {
					if (cedulaDuplicateError) return; // Block submit if duplicate

					const data = {
						...f.data,
						fechaVencimientoPraind: formatDateForBackend(f.data.fechaVencimientoPraind)
					};

					const payload: CreateContratistaInput = {
						cedula: data.cedula,
						nombre: data.nombre,
						apellido: data.apellido,
						empresaId: data.empresaId,
						fechaVencimientoPraind: data.fechaVencimientoPraind,
						tieneVehiculo: false // Default to false for creation via this modal
					};

					if (data.segundoNombre) payload.segundoNombre = data.segundoNombre;
					if (data.segundoApellido) payload.segundoApellido = data.segundoApellido;

					const success = await onSave(payload as any); // Cast because onSave expects union but payload is partial logic here
					if (success) {
						handleClose();
					}
				}
			}
		}
	);

	// Sync form with props when modal opens/changes
	$effect(() => {
		if (show) {
			if (contratista) {
				reset({ data: initialData });
			} else {
				reset();
				cedulaDuplicateError = null;
			}
		}
	});

	// Load companies
	onMount(async () => {
		await empresaStore.init();
	});

	onDestroy(() => {
		if (checkTimeout) clearTimeout(checkTimeout);
	});

	// Helpers
	function formatDateForDisplay(isoDate: string): string {
		if (!isoDate) return '';
		const [year, month, day] = isoDate.split('T')[0].split('-');
		return `${day}/${month}/${year}`;
	}

	function formatDateForBackend(displayDate: string): string {
		if (!displayDate || displayDate.length !== 10) return '';
		const [day, month, year] = displayDate.split('/');
		return `${year}-${month}-${day}`;
	}

	function handleClose() {
		if (!loading) {
			onClose();
		}
	}

	// Real-time Validation for Cedula
	function handleCedulaInput(event: Event) {
		const input = event.target as HTMLInputElement;
		const value = input.value;

		// Update superform store
		$form.cedula = value;

		if (checkTimeout) clearTimeout(checkTimeout);

		// Forzar validación de Superforms para feedback instantáneo (regex, etc)
		validate('cedula');

		if (value.length < 4) {
			cedulaDuplicateError = null;
			return;
		}

		checkTimeout = setTimeout(async () => {
			try {
				const isUnique = await invoke<boolean>('check_unique', {
					table: 'contratista',
					field: 'cedula',
					value,
					excludeId: contratista?.id
				});

				if (!isUnique) {
					cedulaDuplicateError = 'Esta cédula ya está registrada.';
				} else {
					cedulaDuplicateError = null;
				}
			} catch (e) {
				console.error('Error checking uniqueness:', e);
			}
		}, 400);
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

	// --- UI PATTERNS (STANDARD CRUD) ---
	const inputClass =
		'w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[34px] text-sm text-white placeholder:text-gray-500 focus:outline-none focus:!border-blue-500/50 focus:!ring-1 focus:!ring-blue-500/20 disabled:opacity-50 transition-all';
	const labelClass = 'block text-xs font-medium text-secondary mb-1';
	const errorClass = 'text-xs text-red-500 mt-0.5';

	// Helper to determine field border color based on state
	function getFieldStateClass(field: keyof ContratistaFormData, value: any) {
		if ($errors[field]) return '!border-red-500/50 !ring-1 !ring-red-500/20';
		if (field === 'cedula' && cedulaDuplicateError)
			return '!border-red-500/50 !ring-1 !ring-red-500/20';

		// Success state: Solo si hay valor Y NO HAY errores
		if (value && String(value).trim() !== '') {
			return '!border-green-500/50 !ring-1 !ring-green-500/20';
		}

		return '';
	}

	// Handler para Ctrl+S
	function handleKeydown(e: KeyboardEvent) {
		if (!show || readonly || loading) return;
		if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 's') {
			e.preventDefault();
			const form = document.querySelector('form[method="POST"]') as HTMLFormElement;
			if (form) {
				form.requestSubmit();
			}
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if show}
	<!-- Backdrop -->
	<div
		class="fixed inset-0 z-50 flex items-center justify-center p-4"
		transition:fade={{ duration: 150 }}
	>
		<div
			class="absolute inset-0 bg-black/60 backdrop-blur-sm"
			role="button"
			tabindex="0"
			onclick={handleClose}
			onkeydown={(e) => e.key === 'Escape' && handleClose()}
		></div>

		<!-- Modal Container -->
		<div
			class="relative z-10 w-full max-w-[400px] max-h-[95vh] overflow-hidden rounded-xl bg-surface-2 shadow-2xl border border-surface flex flex-col"
			transition:fly={{ y: 20, duration: 200 }}
		>
			<!-- Header -->
			<div
				class="flex-none flex items-center justify-between px-3 py-3 bg-surface-2 border-b border-surface"
			>
				<h2 class="text-xl font-semibold text-primary">
					{modalTitle}
				</h2>
				<button
					onclick={handleClose}
					disabled={loading}
					class="p-1.5 rounded-lg text-secondary hover:text-primary hover:bg-surface-3 transition-colors"
				>
					<X size={20} />
				</button>
			</div>

			<!-- Form (Scrollable Content) -->
			<div class="flex-1 overflow-y-auto">
				<form method="POST" use:enhance class="contents">
					<div class="p-6 space-y-4">
						<!-- Main Card -->
						<div
							class="bg-surface-1 rounded-lg border border-surface p-5 grid grid-cols-1 lg:grid-cols-2 gap-4"
						>
							<!-- Cédula (Full Width) -->
							<div class="col-span-1 lg:col-span-2">
								<label for="cedula" class={labelClass}
									>Cédula <span class="text-red-500 ml-0.5">*</span></label
								>
								<input
									id="cedula"
									name="cedula"
									type="text"
									bind:value={$form.cedula}
									oninput={handleCedulaInput}
									placeholder="1-2345-6789"
									disabled={loading || isEditMode || readonly}
									class="{inputClass} {getFieldStateClass('cedula', $form.cedula)}"
									{...$constraints.cedula}
								/>
								{#if $errors.cedula || cedulaDuplicateError}
									<p class={errorClass}>
										{$errors.cedula || cedulaDuplicateError}
									</p>
								{/if}
							</div>

							<!-- Nombre -->
							<div>
								<label for="nombre" class={labelClass}
									>Nombre <span class="text-red-500 ml-0.5">*</span></label
								>
								<input
									id="nombre"
									name="nombre"
									type="text"
									bind:value={$form.nombre}
									placeholder="Juan"
									disabled={loading || readonly}
									oninput={() => validate('nombre')}
									class="{inputClass} {getFieldStateClass('nombre', $form.nombre)}"
									{...$constraints.nombre}
								/>
								{#if $errors.nombre}<p class={errorClass}>
										{$errors.nombre}
									</p>{/if}
							</div>

							<!-- Segundo Nombre -->
							<div>
								<label for="segundoNombre" class={labelClass}>Segundo Nombre</label>
								<input
									id="segundoNombre"
									name="segundoNombre"
									type="text"
									bind:value={$form.segundoNombre}
									oninput={() => validate('segundoNombre')}
									disabled={loading || readonly}
									class={inputClass}
									{...$constraints.segundoNombre}
								/>
							</div>

							<!-- Apellido -->
							<div>
								<label for="apellido" class={labelClass}
									>Apellido <span class="text-red-500 ml-0.5">*</span></label
								>
								<input
									id="apellido"
									name="apellido"
									type="text"
									bind:value={$form.apellido}
									placeholder="Pérez"
									disabled={loading || readonly}
									oninput={() => validate('apellido')}
									class="{inputClass} {getFieldStateClass('apellido', $form.apellido)}"
									{...$constraints.apellido}
								/>
								{#if $errors.apellido}<p class={errorClass}>
										{$errors.apellido}
									</p>{/if}
							</div>

							<!-- Segundo Apellido -->
							<div>
								<label for="segundoApellido" class={labelClass}>Segundo Apellido</label>
								<input
									id="segundoApellido"
									name="segundoApellido"
									type="text"
									bind:value={$form.segundoApellido}
									oninput={() => validate('segundoApellido')}
									disabled={loading || readonly}
									class={inputClass}
									{...$constraints.segundoApellido}
								/>
							</div>

							<!-- Empresa (Uses flex to accommodate + button) -->
							<div class="col-span-1 lg:col-span-2">
								<label for="empresaId" class={labelClass}
									>Empresa <span class="text-red-500 ml-0.5">*</span></label
								>
								<div class="flex gap-2 relative">
									<!-- Custom Dropdown Trigger -->
									<div class="relative flex-1">
										<button
											type="button"
											disabled={loading || empresaStore.loading || readonly}
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

										<!-- Dropdown Options -->
										{#if showEmpresaDropdown && !readonly}
											<!-- Backdrop -->
											<div
												class="fixed inset-0 z-40"
												onclick={() => (showEmpresaDropdown = false)}
												role="presentation"
												aria-hidden="true"
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
															}}
															class="w-full text-left px-3 py-1.5 text-sm text-gray-300 hover:bg-white/10 rounded-md transition-colors flex items-center justify-between group"
														>
															<span>{empresa.nombre}</span>
															{#if $form.empresaId === empresa.id}
																<svg
																	xmlns="http://www.w3.org/2000/svg"
																	class="h-4 w-4 text-white"
																	viewBox="0 0 20 20"
																	fill="currentColor"
																>
																	<path
																		fill-rule="evenodd"
																		d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
																		clip-rule="evenodd"
																	/>
																</svg>
															{/if}
														</button>
													{/each}
												{/if}
											</div>
										{/if}
									</div>

									{#if !readonly}
										<button
											type="button"
											onclick={() => (showEmpresaModal = true)}
											disabled={loading}
											class="px-3 py-1.5 rounded-lg border border-white/10 bg-black/20 text-secondary hover:text-white hover:bg-white/5 transition-colors"
											title="Añadir nueva empresa"
										>
											<Plus size={16} />
										</button>
									{/if}
								</div>
								{#if $errors.empresaId}<p class={errorClass}>
										{$errors.empresaId}
									</p>{/if}
							</div>

							<!-- Fecha PRAIND (Full Width) -->
							<div class="col-span-1 lg:col-span-2">
								<label for="fechaVencimientoPraind" class={labelClass}
									>Vencimiento PRAIND <span class="text-red-500 ml-0.5">*</span></label
								>
								<input
									id="fechaVencimientoPraind"
									name="fechaVencimientoPraind"
									type="text"
									bind:value={$form.fechaVencimientoPraind}
									placeholder="DD/MM/YYYY"
									maxlength="10"
									disabled={loading || readonly}
									class="{inputClass} {getFieldStateClass(
										'fechaVencimientoPraind',
										$form.fechaVencimientoPraind
									)}"
									{...$constraints.fechaVencimientoPraind}
									oninput={(e) => {
										const input = e.target as HTMLInputElement;
										let value = input.value.replace(/[^\d/]/g, '');
										if (value.length >= 3 && value[2] !== '/') {
											value = value.slice(0, 2) + '/' + value.slice(2);
										}
										if (value.length >= 6 && value[5] !== '/') {
											value = value.slice(0, 5) + '/' + value.slice(5);
										}
										value = value.slice(0, 10);
										$form.fechaVencimientoPraind = value;
										input.value = value;
									}}
								/>
								{#if $errors.fechaVencimientoPraind}<p class={errorClass}>
										{$errors.fechaVencimientoPraind}
									</p>{/if}
							</div>
						</div>

						<!-- Sección Vehículos (solo en modo edición) -->
						{#if isEditMode && contratista?.id}
							<div class="flex items-center justify-between px-2 pt-2">
								<span class="text-xs font-semibold text-secondary uppercase tracking-wider"
									>Vehículos Registrados</span
								>
								<button
									type="button"
									onclick={() => (showVehiculoModal = true)}
									class="flex items-center gap-2 px-3 py-1.5 text-xs font-medium rounded-lg border border-white/10 bg-black/20 text-secondary hover:text-white hover:bg-white/5 transition-colors"
								>
									<Car size={14} />
									Gestionar Flotilla
								</button>
							</div>
						{/if}
					</div>

					<!-- Footer Actions -->
					<div
						class="flex-none flex items-center justify-end gap-3 px-6 py-4 border-t border-surface bg-surface-1 sticky bottom-0 z-20"
					>
						<button
							type="button"
							onclick={handleClose}
							disabled={loading}
							class="px-4 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-white/60 hover:text-white/80 text-sm"
						>
							Cancelar
						</button>

						{#if !readonly}
							<button
								type="submit"
								disabled={loading || !!cedulaDuplicateError}
								class="px-6 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-success hover:text-success text-sm disabled:opacity-50 flex items-center gap-2"
							>
								{#if loading}
									<span
										class="w-4 h-4 rounded-full border-2 border-current border-t-transparent animate-spin"
									></span>
								{/if}
								{isEditMode ? 'Guardar Cambios' : 'Crear Contratista'}
							</button>
						{/if}
					</div>
				</form>
			</div>
		</div>
	</div>
{/if}

<!-- Modal para crear nueva empresa (Mini version inline) -->
{#if showEmpresaModal}
	<div
		class="fixed inset-0 z-[60] flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm"
		transition:fade={{ duration: 200 }}
	>
		<div
			class="absolute inset-0"
			role="button"
			tabindex="0"
			onclick={() => !creatingEmpresa && (showEmpresaModal = false)}
			onkeydown={(e) => e.key === 'Escape' && !creatingEmpresa && (showEmpresaModal = false)}
		></div>

		<div
			class="relative w-full max-w-sm rounded-lg bg-surface-2 shadow-xl border border-surface overflow-hidden"
			transition:scale={{ start: 0.95, duration: 200 }}
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
					class="px-3 py-1.5 text-xs font-medium rounded-lg border-2 border-surface text-secondary transition-all duration-200 hover:border-success hover:text-success disabled:opacity-50"
				>
					{creatingEmpresa ? 'Guardando...' : 'Guardar'}
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Vehiculo Modal (Nested) -->
{#if showVehiculoModal && contratista}
	<VehiculoManagerModal
		show={showVehiculoModal}
		propietarioId={contratista.id}
		propietarioNombre={contratista?.nombre + ' ' + contratista?.apellido}
		onClose={() => (showVehiculoModal = false)}
	/>
{/if}

<style>
	/* Standardized input focus style */
	input:focus {
		border-color: rgba(59, 130, 246, 0.5) !important;
		box-shadow: 0 0 0 1px rgba(59, 130, 246, 0.2) !important;
		outline: none !important;
	}

	/* Autofill Fix for Dark Theme */
	input:-webkit-autofill {
		-webkit-text-fill-color: white !important;
		-webkit-box-shadow: 0 0 0px 1000px #1c2128 inset !important;
		transition: background-color 5000s ease-in-out 0s;
	}
</style>
