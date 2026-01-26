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
	import QuickEmpresaCreateModal from './QuickEmpresaCreateModal.svelte';
	import {
		checkCedulaUnique,
		formatDateForDisplay,
		prepareCreatePayload,
		prepareUpdatePayload
	} from '$lib/logic/contratista/contratistaService';
	import { empresaStore } from '$lib/stores/empresaStore.svelte';
	import VehiculoManagerModal from '$lib/components/vehiculo/VehiculoManagerModal.svelte';
	import { shortcutRegistry, shortcutCommand, clearCommand } from '$lib/shortcuts';

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

					let payload;
					if (isEditMode && contratista?.id) {
						payload = prepareUpdatePayload(contratista.id, f.data);
					} else {
						payload = prepareCreatePayload(f.data);
					}

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
			shortcutRegistry.pushScope('modal');
			if (contratista) {
				reset({ data: initialData });
			} else {
				reset();
				cedulaDuplicateError = null;
			}
		} else {
			shortcutRegistry.popScope();
		}
	});

	// Handle global shortcuts
	$effect(() => {
		const cmd = $shortcutCommand;
		if (show && !loading && !readonly) {
			if (cmd?.command === 'cancel') {
				handleClose();
				clearCommand();
			} else if (cmd?.command === 'save') {
				const f = document.querySelector('form[method="POST"]') as HTMLFormElement;
				if (f) f.requestSubmit();
				clearCommand();
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
				const isUnique = await checkCedulaUnique(value, contratista?.id);

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

	function handleEmpresaCreated(id: string) {
		$form.empresaId = id;
		showEmpresaModal = false;
	}

	// --- UI PATTERNS (STANDARD CRUD) ---
	// --- UI PATTERNS (STANDARD CRUD) ---
	// Usando clases centralizadas de theme.css

	// Helper to determine field border color based on state
	function getFieldStateClass(field: keyof ContratistaFormData, value: any) {
		if ($errors[field]) return 'is-error';
		if (field === 'cedula' && cedulaDuplicateError) return 'is-error';

		// Success state: Solo si hay valor Y NO HAY errores
		if (value && String(value).trim() !== '') {
			return 'is-valid';
		}
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
									disabled={loading || isEditMode || readonly}
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
									disabled={loading || readonly}
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
									disabled={loading || readonly}
									class="form-input"
									{...$constraints.segundoNombre}
								/>
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
									disabled={loading || readonly}
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
									disabled={loading || readonly}
									class="form-input"
									{...$constraints.segundoApellido}
								/>
							</div>

							<!-- Empresa (Uses flex to accommodate + button) -->
							<div class="col-span-1 lg:col-span-2">
								<label for="empresaId" class="form-label"
									>Empresa <span class="text-error ml-0.5">*</span></label
								>
								<div class="flex gap-2 relative">
									<!-- Custom Dropdown Trigger -->
									<div class="relative flex-1">
										<button
											type="button"
											disabled={loading || empresaStore.loading || readonly}
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
										{#if showEmpresaDropdown && !readonly}
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

									{#if !readonly}
										<button
											type="button"
											onclick={() => (showEmpresaModal = true)}
											disabled={loading}
											class="px-3 py-1.5 rounded-lg border border-surface bg-surface-2 text-secondary hover:text-primary hover:border-border-emphasis transition-colors"
											title="Añadir nueva empresa"
										>
											<Plus size={16} />
										</button>
									{/if}
								</div>
								{#if $errors.empresaId}<p class="form-error">
										{$errors.empresaId}
									</p>{/if}
							</div>

							<!-- Fecha PRAIND (Full Width) -->
							<div class="col-span-1 lg:col-span-2">
								<label for="fechaVencimientoPraind" class="form-label"
									>Vencimiento PRAIND <span class="text-error ml-0.5">*</span></label
								>
								<input
									id="fechaVencimientoPraind"
									name="fechaVencimientoPraind"
									type="text"
									bind:value={$form.fechaVencimientoPraind}
									placeholder="DD/MM/YYYY"
									maxlength="10"
									disabled={loading || readonly}
									class="form-input {getFieldStateClass(
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
								{#if $errors.fechaVencimientoPraind}<p class="form-error">
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
									class="flex items-center gap-2 px-3 py-1.5 text-xs font-medium rounded-lg border border-surface bg-surface-2 text-secondary hover:text-primary hover:border-border-emphasis transition-colors"
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
							class="form-btn-outline-secondary"
						>
							Cancelar
						</button>

						{#if !readonly}
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
<!-- Modal para crear nueva empresa (Componente extraído) -->
<QuickEmpresaCreateModal
	show={showEmpresaModal}
	onClose={() => (showEmpresaModal = false)}
	onEmpresaCreated={handleEmpresaCreated}
/>

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
