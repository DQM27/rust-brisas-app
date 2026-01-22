<!-- src/lib/components/visitante/VisitanteFormModal.svelte -->
<script lang="ts">
	import { fade, fly, scale } from 'svelte/transition';
	import { X, User, Bike, Car, Plus, ChevronDown } from 'lucide-svelte';
	import { onMount, onDestroy } from 'svelte';
	import type {
		VisitanteResponse,
		CreateVisitanteInput,
		UpdateVisitanteInput
	} from '$lib/types/visitante';
	import { submitFetchActiveEmpresas } from '$lib/logic/empresa/empresaService';
	import { invoke } from '@tauri-apps/api/core';
	import { empresaStore } from '$lib/stores/empresaStore.svelte';
	import { submitCreateEmpresa } from '$lib/logic/empresa/empresaService';

	// Superforms & Zod v4
	import { superForm } from 'sveltekit-superforms';
	import { zod4 } from 'sveltekit-superforms/adapters';
	import { visitanteSchema, type VisitanteFormData } from '$lib/schemas/visitanteSchema';

	const defaultValues: VisitanteFormData = {
		cedula: '',
		nombre: '',
		segundoNombre: '',
		apellido: '',
		segundoApellido: '',
		empresaId: '',
		hasVehicle: false,
		tipoVehiculo: '',
		placa: '',
		marca: '',
		modelo: '',
		color: ''
	};

	interface Props {
		show: boolean;
		visitante?: VisitanteResponse | null;
		loading?: boolean;
		onSave: (data: CreateVisitanteInput | UpdateVisitanteInput) => Promise<boolean | void>;
		onClose: () => void;
	}

	let { show, visitante = null, loading = false, onSave, onClose }: Props = $props();

	// Derived Mode
	const isEditMode = $derived(!!visitante);
	const modalTitle = $derived(
		isEditMode ? `Editar: ${visitante?.nombre} ${visitante?.apellido}` : 'Nuevo Visitante'
	);

	// Empresas State
	let showEmpresaDropdown = $state(false);
	let showEmpresaModal = $state(false);
	let nuevaEmpresaNombre = $state('');
	let creatingEmpresa = $state(false);
	let empresaError = $state('');

	// Validation State for Real-time checks
	let checkTimeout: ReturnType<typeof setTimeout>;
	let cedulaDuplicateError = $state<string | null>(null);

	// Initial Data Construction
	const initialData = $derived.by(() => {
		return {
			cedula: visitante?.cedula || '',
			nombre: visitante?.nombre || '',
			segundoNombre: visitante?.segundoNombre || '',
			apellido: visitante?.apellido || '',
			segundoApellido: visitante?.segundoApellido || '',
			empresaId: visitante?.empresaId || '',
			hasVehicle: visitante?.hasVehicle || false,
			tipoVehiculo: '',
			placa: '',
			marca: '',
			modelo: '',
			color: ''
		};
	});

	// Superform Initialization with Zod v4 adapter
	const { form, errors, constraints, enhance, reset, validate } = superForm<VisitanteFormData>(
		defaultValues,
		{
			SPA: true,
			validators: zod4(visitanteSchema),
			resetForm: false,
			validationMethod: 'oninput',
			onUpdate: async ({ form: f }) => {
				if (f.valid) {
					if (cedulaDuplicateError) return;

					const payload: CreateVisitanteInput = {
						cedula: f.data.cedula,
						nombre: f.data.nombre,
						apellido: f.data.apellido,
						segundoNombre: f.data.segundoNombre || undefined,
						segundoApellido: f.data.segundoApellido || undefined,
						empresaId: f.data.empresaId || undefined,
						hasVehicle: f.data.hasVehicle,
						tipoVehiculo: f.data.hasVehicle ? f.data.tipoVehiculo : undefined,
						placa: f.data.hasVehicle ? f.data.placa?.toUpperCase() : undefined,
						marca: f.data.hasVehicle ? f.data.marca : undefined,
						modelo: f.data.hasVehicle ? f.data.modelo : undefined,
						color: f.data.hasVehicle ? f.data.color : undefined
					};

					const success = await onSave(payload);
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
			if (visitante) {
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

	function handleClose() {
		if (!loading) {
			onClose();
		}
	}

	// Real-time Validation for Cedula
	function handleCedulaInput(event: Event) {
		const input = event.target as HTMLInputElement;
		const value = input.value;

		$form.cedula = value;

		if (checkTimeout) clearTimeout(checkTimeout);

		validate('cedula');

		if (value.length < 4) {
			cedulaDuplicateError = null;
			return;
		}

		checkTimeout = setTimeout(async () => {
			try {
				const isUnique = await invoke<boolean>('check_unique', {
					table: 'visitante',
					field: 'cedula',
					value,
					excludeId: visitante?.id
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

	// Helper to determine field border color based on state
	function getFieldStateClass(field: keyof VisitanteFormData, value: any) {
		if ($errors[field]) return 'is-error';
		if (field === 'cedula' && cedulaDuplicateError) return 'is-error';

		if (value && String(value).trim() !== '') {
			return 'is-valid';
		}
		return '';
	}
</script>

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
			class="relative z-10 w-full max-w-[500px] max-h-[95vh] overflow-hidden rounded-xl bg-surface-2 shadow-2xl border border-surface flex flex-col"
			transition:fly={{ y: 20, duration: 200 }}
		>
			<!-- Header -->
			<div
				class="flex-none flex items-center justify-between px-5 py-4 bg-surface-2 border-b border-surface"
			>
				<h2 class="text-xl font-semibold text-primary flex items-center gap-2">
					<User size={20} class="text-accent" />
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
								<label for="cedula" class="form-label"
									>Cédula <span class="text-error ml-0.5">*</span></label
								>
								<input
									id="cedula"
									name="cedula"
									type="text"
									bind:value={$form.cedula}
									oninput={handleCedulaInput}
									placeholder="1-2345-6789"
									disabled={loading || isEditMode}
									class="form-input {getFieldStateClass('cedula', $form.cedula)}"
									{...$constraints.cedula}
								/>
								{#if $errors.cedula || cedulaDuplicateError}
									<p class="form-error">
										{$errors.cedula || cedulaDuplicateError}
									</p>
								{/if}
							</div>

							<!-- Nombre -->
							<div>
								<label for="nombre" class="form-label"
									>Nombre <span class="text-error ml-0.5">*</span></label
								>
								<input
									id="nombre"
									name="nombre"
									type="text"
									bind:value={$form.nombre}
									placeholder="Juan"
									disabled={loading}
									oninput={() => validate('nombre')}
									class="form-input {getFieldStateClass('nombre', $form.nombre)}"
									{...$constraints.nombre}
								/>
								{#if $errors.nombre}<p class="form-error">
										{$errors.nombre}
									</p>{/if}
							</div>

							<!-- Segundo Nombre -->
							<div>
								<label for="segundoNombre" class="form-label">Segundo Nombre</label>
								<input
									id="segundoNombre"
									name="segundoNombre"
									type="text"
									bind:value={$form.segundoNombre}
									oninput={() => validate('segundoNombre')}
									disabled={loading}
									class="form-input"
									{...$constraints.segundoNombre}
								/>
								{#if $errors.segundoNombre}<p class="form-error">{$errors.segundoNombre}</p>{/if}
							</div>

							<!-- Apellido -->
							<div>
								<label for="apellido" class="form-label"
									>Apellido <span class="text-error ml-0.5">*</span></label
								>
								<input
									id="apellido"
									name="apellido"
									type="text"
									bind:value={$form.apellido}
									placeholder="Pérez"
									disabled={loading}
									oninput={() => validate('apellido')}
									class="form-input {getFieldStateClass('apellido', $form.apellido)}"
									{...$constraints.apellido}
								/>
								{#if $errors.apellido}<p class="form-error">
										{$errors.apellido}
									</p>{/if}
							</div>

							<!-- Segundo Apellido -->
							<div>
								<label for="segundoApellido" class="form-label">Segundo Apellido</label>
								<input
									id="segundoApellido"
									name="segundoApellido"
									type="text"
									bind:value={$form.segundoApellido}
									oninput={() => validate('segundoApellido')}
									disabled={loading}
									class="form-input"
									{...$constraints.segundoApellido}
								/>
								{#if $errors.segundoApellido}<p class="form-error">
										{$errors.segundoApellido}
									</p>{/if}
							</div>

							<!-- Empresa (Full Width with Dropdown) -->
							<div class="col-span-1 lg:col-span-2">
								<label for="empresaId" class="form-label"
									>Empresa <span class="text-error ml-0.5">*</span></label
								>
								<div class="flex gap-2 relative">
									<!-- Custom Dropdown Trigger -->
									<div class="relative flex-1">
										<button
											type="button"
											disabled={loading || empresaStore.loading}
											onclick={() => (showEmpresaDropdown = !showEmpresaDropdown)}
											class="form-select w-full text-left flex items-center justify-between transition-colors {showEmpresaDropdown
												? 'border-accent ring-1 ring-accent/20'
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
											<div
												class="fixed inset-0 z-40"
												onclick={() => (showEmpresaDropdown = false)}
												role="presentation"
												aria-hidden="true"
											></div>

											<div
												class="form-dropdown absolute top-full left-0 right-0 mt-1 z-50 max-h-60 overflow-y-auto"
												transition:fly={{ y: -10, duration: 200 }}
											>
												{#if empresaStore.empresas.length === 0}
													<div class="px-3 py-2 text-sm text-secondary">No hay empresas</div>
												{:else}
													{#each empresaStore.empresas as empresa}
														<button
															type="button"
															onclick={() => {
																$form.empresaId = empresa.id;
																showEmpresaDropdown = false;
															}}
															class="form-dropdown-item justify-between group"
														>
															<span>{empresa.nombre}</span>
															{#if $form.empresaId === empresa.id}
																<svg
																	xmlns="http://www.w3.org/2000/svg"
																	class="h-4 w-4 text-primary"
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

									<button
										type="button"
										onclick={() => (showEmpresaModal = true)}
										disabled={loading}
										class="px-3 py-1.5 rounded-lg border border-surface bg-surface-2 text-secondary hover:text-primary hover:border-border-emphasis transition-colors"
										title="Añadir nueva empresa"
									>
										<Plus size={16} />
									</button>
								</div>
								{#if $errors.empresaId}<p class="form-error">
										{$errors.empresaId}
									</p>{/if}
							</div>
						</div>

						<!-- Vehículo Section -->
						<div class="bg-surface-1 rounded-lg border border-surface p-5 space-y-4">
							<!-- Toggle Vehículo -->
							<div class="flex items-center justify-between">
								<span class="text-sm font-medium text-secondary">¿Tiene vehículo?</span>
								<label class="relative inline-flex items-center cursor-pointer">
									<input type="checkbox" bind:checked={$form.hasVehicle} class="sr-only peer" />
									<div
										class="w-9 h-5 bg-surface-3 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:bg-blue-600 after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:rounded-full after:h-4 after:w-4 after:transition-all"
									></div>
								</label>
							</div>

							<!-- Campos de Vehículo (condicional) -->
							{#if $form.hasVehicle}
								<div class="space-y-4 pt-2" transition:fly={{ y: -10, duration: 200 }}>
									<!-- Tipo de Vehículo - Botones de Selección -->
									<div>
										<span class="form-label"
											>Tipo de Vehículo <span class="text-error ml-0.5">*</span></span
										>
										<div class="grid grid-cols-2 gap-3">
											<button
												type="button"
												onclick={() => {
													$form.tipoVehiculo = 'motocicleta';
													validate('tipoVehiculo');
												}}
												class="flex items-center justify-center gap-2 p-3 rounded-lg border-2 transition-all
													{$form.tipoVehiculo === 'motocicleta'
													? 'border-success bg-success/10 text-success'
													: 'border-surface hover:border-success/50 text-secondary hover:text-success'}"
											>
												<Bike size={20} />
												<span class="font-medium">Moto</span>
											</button>
											<button
												type="button"
												onclick={() => {
													$form.tipoVehiculo = 'automovil';
													validate('tipoVehiculo');
												}}
												class="flex items-center justify-center gap-2 p-3 rounded-lg border-2 transition-all
													{$form.tipoVehiculo === 'automovil'
													? 'border-success bg-success/10 text-success'
													: 'border-surface hover:border-success/50 text-secondary hover:text-success'}"
											>
												<Car size={20} />
												<span class="font-medium">Auto</span>
											</button>
										</div>
										{#if $errors.tipoVehiculo}<p class="form-error">{$errors.tipoVehiculo}</p>{/if}
									</div>

									<!-- Placa -->
									<div>
										<label for="placa" class="form-label"
											>Placa <span class="text-error ml-0.5">*</span></label
										>
										<input
											id="placa"
											name="placa"
											type="text"
											bind:value={$form.placa}
											oninput={() => validate('placa')}
											placeholder="ABC-123"
											disabled={loading}
											class="form-input uppercase {getFieldStateClass('placa', $form.placa)}"
											{...$constraints.placa}
										/>
										{#if $errors.placa}<p class="form-error">{$errors.placa}</p>{/if}
									</div>

									<!-- Marca y Modelo -->
									<div class="grid grid-cols-2 gap-4">
										<div>
											<label for="marca" class="form-label">Marca</label>
											<input
												id="marca"
												name="marca"
												type="text"
												bind:value={$form.marca}
												placeholder="Toyota, Honda..."
												disabled={loading}
												class="form-input"
												{...$constraints.marca}
											/>
										</div>
										<div>
											<label for="modelo" class="form-label">Modelo</label>
											<input
												id="modelo"
												name="modelo"
												type="text"
												bind:value={$form.modelo}
												placeholder="Corolla, Civic..."
												disabled={loading}
												class="form-input"
												{...$constraints.modelo}
											/>
										</div>
									</div>

									<!-- Color -->
									<div>
										<label for="color" class="form-label">Color</label>
										<input
											id="color"
											name="color"
											type="text"
											bind:value={$form.color}
											placeholder="Blanco, Negro..."
											disabled={loading}
											class="form-input"
											{...$constraints.color}
										/>
									</div>
								</div>
							{/if}
						</div>
					</div>

					<!-- Footer Actions -->
					<div
						class="flex-none flex items-center justify-end gap-3 px-6 py-4 border-t border-surface bg-surface-1 sticky bottom-0 z-20"
					>
						<button
							type="button"
							onclick={handleClose}
							disabled={loading}
							class="form-btn-outline-secondary"
						>
							Cancelar
						</button>

						<button
							type="submit"
							disabled={loading || !!cedulaDuplicateError}
							class="form-btn-outline-success"
						>
							{#if loading}
								<span
									class="w-4 h-4 rounded-full border-2 border-current border-t-transparent animate-spin"
								></span>
							{/if}
							{isEditMode ? 'Guardar Cambios' : 'Crear Visitante'}
						</button>
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
					<label for="newEmpresa" class="form-label">Nombre Comercial</label>
					<input
						id="newEmpresa"
						type="text"
						bind:value={nuevaEmpresaNombre}
						placeholder="Ej: Servicios Generales S.A."
						disabled={creatingEmpresa}
						class="form-input"
						onkeydown={(e) => e.key === 'Enter' && handleCrearEmpresa()}
					/>
				</div>
			</div>

			<div class="flex justify-end gap-2 px-5 py-3 border-t border-surface bg-surface-1">
				<button
					type="button"
					disabled={creatingEmpresa}
					onclick={() => (showEmpresaModal = false)}
					class="form-btn-outline-secondary py-1.5 px-3 text-xs"
				>
					Cancelar
				</button>
				<button
					type="button"
					disabled={creatingEmpresa || !nuevaEmpresaNombre.trim()}
					onclick={handleCrearEmpresa}
					class="form-btn-outline-success py-1.5 px-3 text-xs"
				>
					{creatingEmpresa ? 'Guardando...' : 'Guardar'}
				</button>
			</div>
		</div>
	</div>
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
