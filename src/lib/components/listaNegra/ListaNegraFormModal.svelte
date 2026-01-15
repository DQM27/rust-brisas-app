<!-- src/lib/components/listaNegra/ListaNegraFormModal.svelte -->
<!-- Modal para agregar/editar personas en lista negra (Validación Zod + Superforms) -->
<script lang="ts">
	import { fade, fly } from 'svelte/transition';
	import { X, User, CheckCircle, XCircle, ChevronDown, MonitorStop, Plus } from 'lucide-svelte';
	import { get } from 'svelte/store';
	import { currentUser } from '$lib/stores/auth';
	import { empresaStore } from '$lib/stores/empresaStore.svelte';
	import { submitCreateEmpresa } from '$lib/logic/empresa/empresaService';
	import PersonaFinder from '$lib/components/ingreso/shared/persona/PersonaFinder.svelte';
	import type { ListaNegraResponse, AddToListaNegraInput } from '$lib/types/listaNegra';
	import { AddToListaNegraSchema, type AddToListaNegraForm } from '$lib/schemas/listaNegraSchema';

	// Superforms & ZodAdapter
	import { superForm } from 'sveltekit-superforms';
	import { zod4 } from 'sveltekit-superforms/adapters';

	interface Props {
		show: boolean;
		bloqueado?: ListaNegraResponse | null;
		loading?: boolean;
		onSave: (data: AddToListaNegraInput) => Promise<boolean | void>;
		onClose: () => void;
	}

	let { show, bloqueado = null, loading = false, onSave, onClose }: Props = $props();

	// Modo derivado
	const isEditMode = $derived(!!bloqueado);
	const modalTitle = $derived(
		isEditMode ? `Editar: ${bloqueado?.nombreCompleto}` : 'Agregar a Lista Negra'
	);

	let showSeveridadDropdown = $state(false);
	let showEmpresaDropdown = $state(false);
	let showEmpresaModal = $state(false);
	let searchResetKey = $state(0);

	// Estado para crear empresa
	let creatingEmpresa = $state(false);
	let nuevaEmpresaNombre = $state('');

	// --- SUPERFORMS SETUP ---
	const initialValues: AddToListaNegraForm = {
		cedula: '',
		nombre: '',
		segundoNombre: '',
		apellido: '',
		segundoApellido: '',
		empresaId: '',
		empresaNombre: '',
		nivelSeveridad: 'MEDIO',
		motivoBloqueo: ''
	};

	const { form, errors, constraints, enhance, reset, formId, tainted, message } =
		superForm<AddToListaNegraForm>(initialValues, {
			SPA: true,
			validators: zod4(AddToListaNegraSchema),
			resetForm: false, // Control manual
			onUpdate: async ({ form: f }) => {
				if (!f.valid) return;

				const usuario = get(currentUser);
				const bloqueadoPor = usuario ? `${usuario.nombre} ${usuario.apellido}` : 'Sistema';

				const input: AddToListaNegraInput = {
					...f.data,
					bloqueadoPor,
					// Asegurar campos opcionales
					segundoNombre: f.data.segundoNombre || undefined,
					segundoApellido: f.data.segundoApellido || undefined,
					empresaId: f.data.empresaId || undefined,
					empresaNombre: f.data.empresaNombre || undefined,
					motivoBloqueo: f.data.motivoBloqueo || undefined
				};

				const success = await onSave(input);
				if (success) {
					onClose();
				}
			}
		});

	// Estado de selección
	let selectedPersona = $state<any>(null); // Usamos any para flexibilidad con PersonaFinder

	// Cargar datos en modo edición o resetear
	$effect(() => {
		if (show) {
			if (bloqueado) {
				// Modo Edición
				reset({
					data: {
						cedula: bloqueado.cedula || '',
						nombre: bloqueado.nombre || '',
						segundoNombre: bloqueado.segundoNombre || '',
						apellido: bloqueado.apellido || '',
						segundoApellido: bloqueado.segundoApellido || '',
						empresaId: bloqueado.empresaId || '',
						empresaNombre: bloqueado.empresaNombre || '',
						nivelSeveridad: (bloqueado.nivelSeveridad as any) || 'MEDIO',
						motivoBloqueo: bloqueado.motivoBloqueo || ''
					}
				});
				selectedPersona = null;
			} else {
				// Modo Creación (Reset)
				reset();
				selectedPersona = null;
			}
		}
	});

	import * as contratistaService from '$lib/logic/contratista/contratistaService';
	import * as userService from '$lib/logic/user/userService';
	import * as proveedorService from '$lib/logic/proveedor/proveedorService';
	import { fetchEmpresaPorId } from '$lib/api/empresa';

	// Handler para selección desde PersonaFinder
	async function handlePersonaSelect(event: CustomEvent) {
		const { id, type, data } = event.detail;

		selectedPersona = data;
		searchResetKey++; // Reset search input visual state

		// 1. Poblado inicial rápido (Fallback)
		$form.cedula = data.cedula || '';
		$form.empresaNombre = data.empresaNombre || data.empresa_nombre || '';

		// 2. Fetch de datos completos para precisión (segundos nombres, etc.)
		try {
			loading = true;
			let fullData: any = null;

			if (type === 'contratista') {
				const res = await contratistaService.fetchContratistaById(id);
				if (res.ok) fullData = res.data;
			} else if (type === 'user' || type === 'usuario') {
				// Handle both just in case
				const res = await userService.fetchUserById(id);
				if (res.ok) fullData = res.data;
			} else if (type === 'proveedor') {
				const res = await proveedorService.fetchProveedorById(id);
				if (res.ok) fullData = res.data;
			}

			if (fullData) {
				// Mapeo preciso desde entidad completa
				$form.nombre = fullData.nombre || '';
				$form.segundoNombre = fullData.segundoNombre || fullData.segundo_nombre || '';
				$form.apellido = fullData.apellido || '';
				$form.segundoApellido = fullData.segundoApellido || fullData.segundo_apellido || '';

				// Empresa: Manejar objeto o string o campos planos (DTOs nuevos)
				let empIdString = '';

				if (fullData.empresa) {
					if (typeof fullData.empresa === 'object') {
						// Caso objeto (puede ser {id:..., nombre:...} o {tb:..., id:...})
						if (fullData.empresa.id) {
							empIdString = fullData.empresa.id.toString();
						}
						if (fullData.empresa.nombre && !$form.empresaNombre) {
							$form.empresaNombre = fullData.empresa.nombre;
						}
					} else {
						// Caso string directo
						empIdString = fullData.empresa;
					}
				} else if (fullData.empresaId) {
					// Caso respuesta plana (ContratistaResponse / ProveedorResponse)
					empIdString = fullData.empresaId;
				}

				if (empIdString) {
					$form.empresaId = empIdString;

					// Si tenemos ID y el nombre no se ha seteado (o queremos asegurarlo)
					if (!$form.empresaNombre && fullData.empresaNombre) {
						$form.empresaNombre = fullData.empresaNombre;
					}

					// Fallback fetch si aun no tenemos nombre
					if (!$form.empresaNombre) {
						try {
							const emp = await fetchEmpresaPorId(empIdString);
							if (emp) {
								$form.empresaNombre = emp.nombre;
							}
						} catch (e) {
							console.error('Error fetching empresa details:', e);
						}
					}
				}
			} else {
				// Fallback: Intentar parsear nombre completo si no se pudo hacer fetch
				fillFromSearchResult(data);
			}
		} catch (e) {
			console.error('Error fetching full details:', e);
			fillFromSearchResult(data);
		} finally {
			loading = false;
		}
	}

	function fillFromSearchResult(data: any) {
		// Intento heurístico de separar nombres
		const fullName = data.nombre_completo || data.nombreCompleto;
		if (fullName) {
			const parts = fullName.split(' ');
			$form.nombre = parts[0] || '';
			// Asumir que el resto son apellidos es arriesgado pero mejor que nada en fallback
			if (parts.length > 2) {
				$form.apellido = parts[1];
				$form.segundoApellido = parts.slice(2).join(' ');
			} else {
				$form.apellido = parts.slice(1).join(' ') || '';
			}
		}
	}

	function clearSelection() {
		selectedPersona = null;
		// Limpiar campos de identidad pero mantener severidad/motivo si ya se escribieron
		$form.cedula = '';
		$form.nombre = '';
		$form.segundoNombre = '';
		$form.apellido = '';
		$form.segundoApellido = '';
		$form.empresaId = '';
		$form.empresaNombre = '';
	}

	// --- STANDARD STYLES (ui-patterns.md) ---
	const inputClass =
		'w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[34px] text-sm text-white placeholder:text-gray-500 focus:outline-none focus:!border-blue-500/50 focus:!ring-1 focus:!ring-blue-500/20 disabled:opacity-50 transition-all';
	const labelClass = 'block text-xs font-medium text-secondary mb-1';
	const errorClass = 'text-xs text-red-500 mt-0.5';

	// Helper de Validación
	function getFieldStateClass(field: keyof AddToListaNegraForm) {
		if ($errors[field]) {
			return '!border-red-500/50 !ring-1 !ring-red-500/20';
		}
		return '';
	}

	// Severidad Options
	const severidadOptions = [
		{ value: 'ALTO', label: '🔴 ALTO - Crítico' },
		{ value: 'MEDIO', label: '🟡 MEDIO - Moderado' },
		{ value: 'BAJO', label: '🟢 BAJO - Bajo riesgo' }
	];

	// Handler para Ctrl+S
	function handleKeydown(e: KeyboardEvent) {
		if (!show || loading) return;
		if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 's') {
			e.preventDefault();
			const form = document.querySelector('form') as HTMLFormElement;
			if (form) {
				form.requestSubmit();
			}
		}
	}

	// Handler para crear nueva empresa (Inline)
	async function handleCrearEmpresa() {
		if (!nuevaEmpresaNombre.trim()) return;

		creatingEmpresa = true;
		const res = await submitCreateEmpresa(nuevaEmpresaNombre);

		if (res.ok && res.empresa) {
			await empresaStore.refresh(); // Recargar lista
			// Seleccionar la nueva empresa
			$form.empresaId = res.empresa.id;
			$form.empresaNombre = res.empresa.nombre;
			showEmpresaModal = false;
			nuevaEmpresaNombre = ''; // Reset
		} else if (!res.ok) {
			console.error('Error creando empresa:', res.error);
		}
		creatingEmpresa = false;
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if show}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
		transition:fade={{ duration: 150 }}
	>
		<!-- Backdrop click handler -->
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div class="absolute inset-0" onclick={onClose}></div>

		<!-- Modal Content: Compact CRUD Modal Style -->
		<div
			class="relative z-10 w-full max-w-[420px] max-h-[95vh] flex flex-col overflow-hidden rounded-xl bg-surface-2 shadow-2xl border border-surface"
			transition:fly={{ y: 20, duration: 200 }}
		>
			<!-- Header Estándar -->
			<div
				class="flex-none flex items-center justify-between px-3 py-3 bg-surface-2 border-b border-surface"
			>
				<h2 class="text-lg font-semibold text-primary ml-1">
					{modalTitle}
				</h2>
				<button
					onclick={onClose}
					class="p-1.5 rounded-lg text-secondary hover:text-primary hover:bg-surface-3 transition-colors"
				>
					<X size={18} />
				</button>
			</div>

			<!-- Form Content Container -->
			<form method="POST" use:enhance class="flex flex-col flex-1 overflow-hidden">
				<!-- Scrollable Area -->
				<div class="flex-1 overflow-y-auto">
					<div class="p-5 space-y-4">
						<!-- Búsqueda de persona (solo en creación/limpio) -->
						{#if !isEditMode}
							<div class="mb-4">
								<!-- Componente PersonaFinder (Siempre visible) -->
								{#key searchResetKey}
									<PersonaFinder on:select={handlePersonaSelect} autoFocus={true} />
								{/key}

								<!-- Botón para limpiar selección manual si ya se eligió alguien -->
								{#if selectedPersona}
									<div class="flex justify-end mt-1">
										<button
											type="button"
											onclick={clearSelection}
											class="text-[10px] text-red-400 hover:text-red-300 transition-colors flex items-center gap-1"
										>
											<X size={12} /> Limpiar formulario
										</button>
									</div>
								{/if}
							</div>
						{/if}

						<!-- Input Card Container -->
						<div class="bg-surface-1 rounded-lg border border-surface p-5 space-y-4">
							<!-- Cédula (Full Width) -->
							<div>
								<label for="cedula" class={labelClass}
									>Cédula <span class="text-red-500">*</span></label
								>
								<input
									id="cedula"
									name="cedula"
									type="text"
									bind:value={$form.cedula}
									disabled={loading || isEditMode || !!selectedPersona}
									class="{inputClass} {getFieldStateClass('cedula')}"
									placeholder={selectedPersona ? '' : '1-1234-5678'}
									{...$constraints.cedula}
								/>
								{#if $errors.cedula}<p class={errorClass}>{$errors.cedula}</p>{/if}
							</div>

							<!-- Row: Nombre + Segundo Nombre -->
							<div class="grid grid-cols-2 gap-3">
								<div>
									<label for="nombre" class={labelClass}
										>Nombre <span class="text-red-500">*</span></label
									>
									<input
										id="nombre"
										name="nombre"
										type="text"
										bind:value={$form.nombre}
										disabled={loading || !!selectedPersona}
										class="{inputClass} {getFieldStateClass('nombre')}"
										placeholder={selectedPersona ? '' : 'Juan'}
										{...$constraints.nombre}
									/>
									{#if $errors.nombre}<p class={errorClass}>{$errors.nombre}</p>{/if}
								</div>
								<div>
									<label for="segundoNombre" class={labelClass}>Segundo Nombre</label>
									<input
										id="segundoNombre"
										name="segundoNombre"
										type="text"
										bind:value={$form.segundoNombre}
										disabled={loading || !!selectedPersona}
										class="{inputClass} {getFieldStateClass('segundoNombre')}"
										placeholder="Opcional"
										{...$constraints.segundoNombre}
									/>
								</div>
							</div>

							<!-- Row: Apellido + Segundo Apellido -->
							<div class="grid grid-cols-2 gap-3">
								<div>
									<label for="apellido" class={labelClass}
										>Apellido <span class="text-red-500">*</span></label
									>
									<input
										id="apellido"
										name="apellido"
										type="text"
										bind:value={$form.apellido}
										disabled={loading || !!selectedPersona}
										class="{inputClass} {getFieldStateClass('apellido')}"
										placeholder={selectedPersona ? '' : 'Pérez'}
										{...$constraints.apellido}
									/>
									{#if $errors.apellido}<p class={errorClass}>{$errors.apellido}</p>{/if}
								</div>
								<div>
									<label for="segundoApellido" class={labelClass}>Segundo Apellido</label>
									<input
										id="segundoApellido"
										name="segundoApellido"
										type="text"
										bind:value={$form.segundoApellido}
										disabled={loading || !!selectedPersona}
										class="{inputClass} {getFieldStateClass('segundoApellido')}"
										placeholder="Opcional"
										{...$constraints.segundoApellido}
									/>
								</div>
							</div>

							<!-- Empresa (Full Width, Dropdown + Add Button) -->
							<div class="relative">
								<label for="empresaId" class={labelClass}>Empresa</label>
								<div class="flex gap-2 relative">
									<!-- Custom Dropdown Trigger -->
									<div class="relative flex-1">
										<button
											type="button"
											onclick={() => (showEmpresaDropdown = !showEmpresaDropdown)}
											disabled={loading || !!selectedPersona}
											class="{inputClass} flex items-center justify-between cursor-pointer w-full text-left {showEmpresaDropdown
												? '!border-blue-500/50 !ring-1 !ring-blue-500/20'
												: getFieldStateClass(
														'empresaId'
													)} disabled:cursor-not-allowed disabled:opacity-60"
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
										{#if showEmpresaDropdown && !selectedPersona}
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
																$form.empresaNombre = empresa.nombre; // Sync name too
																showEmpresaDropdown = false;
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
									{#if !isEditMode && !selectedPersona}
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
								{#if $errors.empresaId}<p class={errorClass}>{$errors.empresaId}</p>{/if}
							</div>

							<!-- Nivel de Severidad CUSTOM DROPDOWN (Full Width) -->
							<div class="relative">
								<label for="nivelSeveridad" class={labelClass}
									>Nivel de Severidad <span class="text-red-500">*</span></label
								>
								<button
									type="button"
									onclick={() => (showSeveridadDropdown = !showSeveridadDropdown)}
									class="{inputClass} flex items-center justify-between cursor-pointer w-full text-left {getFieldStateClass(
										'nivelSeveridad'
									)} 
                                    {$form.nivelSeveridad === 'ALTO'
										? '!text-red-400 !border-red-500/50 !ring-1 !ring-red-500/20'
										: $form.nivelSeveridad === 'MEDIO'
											? '!text-yellow-400 !border-yellow-500/50 !ring-1 !ring-yellow-500/20'
											: '!text-green-400 !border-green-500/50 !ring-1 !ring-green-500/20'}"
								>
									<span class="truncate">
										{severidadOptions.find((o) => o.value === $form.nivelSeveridad)?.label ||
											'Seleccionar...'}
									</span>
									<ChevronDown size={16} class="text-secondary" />
								</button>

								<select
									name="nivelSeveridad"
									class="hidden"
									bind:value={$form.nivelSeveridad}
									{...$constraints.nivelSeveridad}
								>
									{#each severidadOptions as opt}
										<option value={opt.value}>{opt.label}</option>
									{/each}
								</select>

								{#if showSeveridadDropdown}
									<!-- Backdrop -->
									<!-- svelte-ignore a11y_click_events_have_key_events -->
									<!-- svelte-ignore a11y_no_static_element_interactions -->
									<div
										class="fixed inset-0 z-40"
										onclick={() => (showSeveridadDropdown = false)}
									></div>

									<div
										class="absolute z-50 w-full mt-1 bg-[#1c2128] border border-white/10 rounded-lg shadow-xl overflow-hidden p-1 origin-top"
										transition:fly={{ y: -5, duration: 200 }}
									>
										{#each severidadOptions as option}
											<button
												type="button"
												onclick={() => {
													$form.nivelSeveridad = option.value as any;
													showSeveridadDropdown = false;
												}}
												class="w-full text-left px-3 py-2 text-sm text-gray-300 hover:bg-white/10 rounded-md transition-colors flex items-center justify-between group"
											>
												<span>{option.label}</span>
												{#if $form.nivelSeveridad === option.value}
													<CheckCircle size={14} class="text-white" />
												{/if}
											</button>
										{/each}
									</div>
								{/if}
								{#if $errors.nivelSeveridad}<p class={errorClass}>{$errors.nivelSeveridad}</p>{/if}
							</div>

							<!-- Motivo - Container Pattern -->
							<div>
								<label for="motivoBloqueo" class={labelClass}>Motivo del Bloqueo</label>
								<div
									class="w-full bg-black/20 border border-white/10 rounded-lg transition-all outline-none focus-within:!border-blue-500/50 focus-within:!ring-1 focus-within:!ring-blue-500/20 {getFieldStateClass(
										'motivoBloqueo'
									)
										? '!border-red-500/50'
										: ''}"
								>
									<textarea
										id="motivoBloqueo"
										name="motivoBloqueo"
										bind:value={$form.motivoBloqueo}
										disabled={loading}
										class="w-full bg-transparent px-3 py-2 text-sm text-white placeholder:text-gray-500 resize-none focus:outline-none !outline-none !border-none !ring-0 !shadow-none appearance-none h-[80px]"
										rows="3"
										placeholder="Describa el motivo del bloqueo..."
										{...$constraints.motivoBloqueo}
									></textarea>
								</div>
								{#if $errors.motivoBloqueo}<p class={errorClass}>{$errors.motivoBloqueo}</p>{/if}
							</div>
						</div>
					</div>
				</div>

				<!-- Footer de Acciones -->
				<div
					class="flex-none flex items-center justify-end gap-3 px-5 py-4 border-t border-surface bg-surface-1"
				>
					<!-- Cancelar -->
					<button
						type="button"
						onclick={onClose}
						class="flex items-center gap-2 px-3 py-2 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-white/60 hover:text-white/80 text-xs"
					>
						Cancelar
					</button>

					<!-- Guardar -->
					<button
						type="submit"
						disabled={loading}
						class="flex items-center gap-2 px-5 py-2 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-red-500 hover:text-red-500 text-xs disabled:opacity-50 disabled:cursor-not-allowed"
					>
						{loading ? 'Guardando...' : 'Bloquear'}
					</button>
				</div>
			</form>
		</div>
	</div>

	<!-- Modal Inline para Crear Empresa -->
	{#if showEmpresaModal}
		<div
			class="fixed inset-0 z-[60] flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
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
{/if}

<style>
	/* Autofill Dark fix */
	input:-webkit-autofill,
	textarea:-webkit-autofill {
		-webkit-text-fill-color: white !important;
		-webkit-box-shadow: 0 0 0px 1000px #00000000 inset !important;
		transition: background-color 5000s ease-in-out 0s;
	}
</style>
