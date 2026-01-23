<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { CreatePreRegistroInput, PreRegistroVisita } from '$lib/types/ingreso-nuevos';
	import { preRegistroVisitaService } from '$lib/services/preRegistroVisitaService';
	import type { SearchResult } from '$lib/types/search.types';
	import { slide, fly, fade, scale } from 'svelte/transition';

	import {
		X,
		CalendarDays,
		Save,
		PersonStanding,
		Car,
		Bike,
		Loader2,
		Check,
		User as UserIcon,
		RefreshCw,
		Plus,
		ChevronDown
	} from 'lucide-svelte';
	import PersonaFinder from '$lib/components/ingreso/shared/persona/PersonaFinder.svelte';
	import { empresaStore } from '$lib/stores/empresaStore.svelte';
	import { submitCreateEmpresa } from '$lib/logic/empresa/empresaService';
	import { vehiculos } from '$lib/api/vehiculos';
	import type { VehiculoResponse } from '$lib/types/vehiculo';
	import { onMount, untrack } from 'svelte';

	let {
		show = $bindable(false),
		initialData = null,
		onClose
	}: { show: boolean; initialData?: PreRegistroVisita | null; onClose: () => void } = $props();

	const dispatch = createEventDispatcher();

	let loading = $state(false);
	let error = $state('');

	// State for ID tracking
	let editingId = $state<string | null>(null);

	$effect(() => {
		if (show) {
			if (initialData) {
				untrack(() => populateForm(initialData!));
			} else {
				untrack(() => resetForm());
			}
		}
	});

	function populateForm(data: PreRegistroVisita) {
		try {
			// @ts-ignore - Svelte 5 rune for snapshotting state
			console.log('[snapshot] Populating form with:', $state.snapshot(data));
			if (!data) return;

			// Store ID for update
			if (data.id) {
				// Handle both string and object IDs
				if (typeof data.id === 'object') {
					// @ts-ignore
					if (data.id.tb && data.id.id && data.id.id.String) {
						// @ts-ignore
						editingId = `${data.id.tb}:${data.id.id.String}`;
					} else {
						editingId = (data.id as any).toString();
					}
				} else {
					editingId = data.id.toString();
				}
			}

			cedula = data.cedula || '';
			nombre = data.nombre || '';
			segundoNombre = data.segundoNombre || (data as any).segundo_nombre || '';
			apellido = data.apellido || '';
			segundoApellido = data.segundoApellido || (data as any).segundo_apellido || '';

			empresaId = normalizeId(data.empresaId || (data as any).empresa_id);
			empresaNombre = data.empresaNombre || (data as any).empresa_nombre || '';

			// Si tenemos nombre pero no ID (sucede a veces en el catálogo), intentar buscarlo
			if (!empresaId && empresaNombre) {
				const matched = empresaStore.empresas.find(
					(e) => e.nombre?.toLowerCase() === empresaNombre.toLowerCase()
				);
				if (matched) empresaId = matched.id;
			}

			// Cargar vehículos si el visitante está vinculado
			const vId = (data as any).visitante || (data as any).visitante_id;
			if (vId) {
				loadVehiculos(normalizeId(vId));
			}

			anfitrion = data.anfitrion || '';
			areaVisitada = data.areaVisitada || (data as any).area_visitada || '';
			motivo = data.motivo || '';

			const rawFecha = data.fechaEsperada || (data as any).fecha_esperada;
			if (rawFecha) {
				try {
					fechaEsperadaDisplay = formatDateForDisplay(rawFecha);
				} catch (e) {
					console.warn('Error formatting date:', e);
					fechaEsperadaDisplay = ''; // Fallback
				}
			} else {
				fechaEsperadaDisplay = '';
			}

			horaEsperada = data.horaEsperada || (data as any).hora_esperada || '';

			if (data.placa || (data as any).placa) {
				vehiculoPlaca = data.placa || (data as any).placa;
			}

			// If we have data, we assume visitor is selected (locks fields)
			visitorSelected = true;
		} catch (err) {
			console.error('Error populating form:', err);
			error = 'Error cargando datos del registro';
		}
	}

	// Form data
	let cedula = $state('');
	let nombre = $state('');
	let segundoNombre = $state('');
	let apellido = $state('');
	let segundoApellido = $state('');
	let empresaNombre = $state('');
	// Empresa State (Dropdown)
	let empresaId = $state('');
	let showEmpresaDropdown = $state(false);
	let showEmpresaModal = $state(false);
	let nuevaEmpresaNombre = $state('');
	let creatingEmpresa = $state(false);
	let empresaError = $state('');

	let fechaEsperadaDisplay = $state(formatDateForDisplay(new Date().toISOString().split('T')[0]));
	let horaEsperada = $state('');
	let anfitrion = $state('');
	let areaVisitada = $state('');
	let motivo = $state('');

	// Mode & Vehicle State
	let vehiculosList = $state<VehiculoResponse[]>([]);

	let groupedVehiculos = $derived(
		vehiculosList.reduce(
			(acc, v) => {
				const key = v.tipoVehiculoDisplay;
				if (!acc[key]) acc[key] = [];
				acc[key].push(v);
				return acc;
			},
			{} as Record<string, VehiculoResponse[]>
		)
	);

	let loadingVehiculos = $state(false);
	let showVehiculoDropdown = $state(false);
	let showVehiculoForm = $state(false); // Mini modal

	// Vehicle Form Data (para creación inline)
	let vehiculoTipo = $state('');
	let vehiculoPlaca = $state('');
	let vehiculoMarca = $state('');
	let vehiculoModelo = $state('');
	let vehiculoColor = $state('');

	// Search logic
	let visitorSelected = $state(false);
	let searchResetKey = $state(0);

	// Clases estándar
	const inputClass =
		'w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[34px] text-sm text-white placeholder:text-gray-500 focus:outline-none focus:!border-blue-500/50 focus:!ring-1 focus:!ring-blue-500/20 disabled:opacity-50 transition-all';
	const selectClass =
		'w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[34px] text-sm text-white focus:outline-none disabled:opacity-50 transition-all cursor-pointer appearance-none';
	const labelClass = 'block text-xs font-medium text-secondary mb-1 uppercase tracking-wide';

	// Helpers para formato de fecha (DD/MM/YYYY ↔ YYYY-MM-DD)
	function formatDateForDisplay(isoDate: string): string {
		if (!isoDate) return '';
		const [year, month, day] = isoDate.split('T')[0].split('-');
		return `${day}/${month}/${year}`;
	}

	// Normalizar ID de empresa para SurrealDB (tb:id)
	function normalizeId(id: any): string {
		if (!id) return '';
		if (typeof id === 'string') return id;
		if (typeof id === 'object' && id.tb && id.id) {
			const innerId =
				typeof id.id === 'object' ? id.id.String || id.id.id || JSON.stringify(id.id) : id.id;
			return `${id.tb}:${innerId}`;
		}
		return id.toString();
	}

	function formatDateForBackend(displayDate: string): string {
		if (!displayDate || displayDate.length !== 10) return '';
		const [day, month, year] = displayDate.split('/');
		return `${year}-${month}-${day}`;
	}

	async function handleSubmit() {
		if (
			!cedula ||
			!nombre ||
			!apellido ||
			!empresaId ||
			!anfitrion ||
			!areaVisitada ||
			!motivo ||
			!horaEsperada
		) {
			error = 'Por favor complete los campos obligatorios (*)';
			return;
		}

		// Inferir modo ingreso
		const modoIngreso = vehiculoPlaca ? 'vehiculo' : 'caminando';

		loading = true;
		error = '';

		try {
			// Construir input
			const input: CreatePreRegistroInput = {
				cedula,
				nombre,
				segundoNombre,
				apellido,
				segundoApellido,

				// Lógica Empresa:
				empresaNombre: empresaId
					? empresaStore.empresas.find((e) => e.id === empresaId)?.nombre
					: undefined,
				empresaId: empresaId || undefined,

				fechaEsperada: formatDateForBackend(fechaEsperadaDisplay),
				horaEsperada,
				anfitrion,
				areaVisitada,
				motivo,
				modoIngreso,

				// Lógica Vehículo
				placa: vehiculoPlaca || undefined
			};

			if (editingId) {
				await preRegistroVisitaService.update(editingId, input);
				// TODO: Check if update returns new data, but generally we just need success
			} else {
				await preRegistroVisitaService.create(input);
			}

			dispatch('success');
			close();
		} catch (e: any) {
			error = e.message || 'Error al guardar pre-registro';
			console.error(e);
		} finally {
			loading = false;
		}
	}

	function close() {
		onClose();
	}

	function resetForm() {
		editingId = null;
		cedula = '';
		nombre = '';
		segundoNombre = '';
		apellido = '';
		segundoApellido = '';

		empresaId = '';
		empresaNombre = '';

		resetVehiculoForm();

		horaEsperada = '';
		error = '';
		visitorSelected = false;
		searchResetKey++;
		fechaEsperadaDisplay = formatDateForDisplay(new Date().toISOString().split('T')[0]);
	}

	function resetVehiculoForm() {
		vehiculoTipo = '';
		vehiculoPlaca = '';
		vehiculoMarca = '';
		vehiculoModelo = '';
		vehiculoColor = '';
		vehiculosList = []; // Clear list
		showVehiculoDropdown = false;
		showVehiculoForm = false;
	}

	function selectVisitor(result: SearchResult) {
		cedula = result.cedula || '';
		nombre = result.nombre || '';
		segundoNombre = result.segundoNombre || '';
		apellido = result.apellido || '';
		segundoApellido = result.segundoApellido || '';

		// Map empresa info
		if (result.empresaId) {
			empresaId = result.empresaId;
		} else if (result.empresaNombre) {
			const found = empresaStore.empresas.find(
				(e) => e.nombre?.toLowerCase() === result.empresaNombre?.toLowerCase()
			);
			if (found) empresaId = found.id;
		}

		// Load vehicles for this specific visitor
		if (result.id) {
			loadVehiculos(result.id);
		} else {
			vehiculosList = [];
		}

		visitorSelected = true;
	}

	function handlePersonaSelect(event: CustomEvent) {
		const { data } = event.detail;
		selectVisitor(data);
	}

	function clearSearch() {
		visitorSelected = false;
		// Don't fully reset, just clear visual selection state if needed, or allow editing fields
		// Actually, clearSearch usually means "I want to type manually"
		// so we just unlock fields
		cedula = '';
		nombre = '';
		apellido = '';
		// ... etc if they want to clear
		// But usually "Limpiar y editar manual" means "Unlock me"
		// For now let's reuse logic
		resetForm();
	}

	async function handleCrearEmpresa() {
		if (!nuevaEmpresaNombre.trim()) return;
		creatingEmpresa = true;
		empresaError = '';
		const result = await submitCreateEmpresa(nuevaEmpresaNombre);
		if (result.ok) {
			empresaStore.add(result.empresa);
			empresaId = result.empresa.id;
			nuevaEmpresaNombre = '';
			showEmpresaModal = false;
		} else {
			empresaError = result.error;
		}
		creatingEmpresa = false;
	}

	// Init Data
	onMount(async () => {
		await empresaStore.init();
		// No cargamos vehículos globales para evitar mostrar datos de otros visitantes
		// loadVehiculos();
	});

	async function loadVehiculos(propietarioId?: string) {
		if (!propietarioId) {
			vehiculosList = [];
			return;
		}

		loadingVehiculos = true;
		try {
			vehiculosList = await vehiculos.getByPropietario(propietarioId);
		} catch (e) {
			console.error('Error loading vehicles for visitor:', e);
			vehiculosList = [];
		} finally {
			loadingVehiculos = false;
		}
	}

	function handleDateInput(e: Event) {
		const input = e.target as HTMLInputElement;
		let value = input.value.replace(/[^\d/]/g, ''); // Solo números y /
		if (value.length >= 3 && value[2] !== '/') {
			value = value.slice(0, 2) + '/' + value.slice(2);
		}
		if (value.length >= 6 && value[5] !== '/') {
			value = value.slice(0, 5) + '/' + value.slice(5);
		}
		value = value.slice(0, 10);
		fechaEsperadaDisplay = value;
		input.value = value;
	}
</script>

{#if show}
	<!-- Backdrop -->
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4"
		transition:slide|local
	>
		<!-- Modal Container -->
		<div
			class="bg-surface-2 w-full max-w-[700px] rounded-xl shadow-2xl border border-surface overflow-hidden flex flex-col max-h-[95vh]"
			transition:fly={{ y: 20, duration: 200 }}
		>
			<!-- Header Estándar -->
			<div
				class="flex-none flex items-center justify-between px-5 py-4 bg-surface-2 border-b border-surface"
			>
				<h2 class="text-xl font-semibold text-primary flex items-center gap-2">
					<CalendarDays size={20} class="text-accent" />
					{#if editingId}
						Editar Pre-Registro
					{:else}
						Nuevo Pre-Registro de Visita
					{/if}
				</h2>
				<button
					onclick={close}
					class="p-1.5 rounded-lg text-secondary hover:text-primary hover:bg-surface-3 transition-colors"
				>
					<X size={20} />
				</button>
			</div>

			<!-- Body con Card de Inputs -->
			<div class="flex-1 p-6 space-y-4 overflow-y-auto">
				{#if error}
					<div class="p-3 bg-error/10 border border-error/20 text-error rounded-lg text-sm">
						{error}
					</div>
				{/if}

				<!-- Card de Inputs -->
				<div class="bg-surface-1 rounded-lg border border-surface p-6 space-y-6">
					<!-- Buscador Unificado -->
					{#if !editingId}
						<div>
							<div class="flex items-center justify-end mb-2">
								{#if visitorSelected}
									<button
										type="button"
										onclick={clearSearch}
										class="text-[10px] text-red-400 hover:text-red-300 transition-colors flex items-center gap-1.5 bg-red-400/5 px-2 py-1 rounded-md border border-red-400/10"
									>
										<RefreshCw size={10} /> Limpiar y editar manual
									</button>
								{/if}
							</div>
							{#key searchResetKey}
								<PersonaFinder scope="all" on:select={handlePersonaSelect} autoFocus={true} />
							{/key}
						</div>
					{/if}

					<form
						onsubmit={(e) => {
							e.preventDefault();
							handleSubmit();
						}}
						class="space-y-6"
					>
						<!-- Fila 1: Captura de Identidad -->
						<div class="space-y-5">
							<div class="relative">
								<label for="cedula" class={labelClass}
									>Cédula <span class="text-red-500">*</span></label
								>
								<div class="relative">
									<input
										id="cedula"
										type="text"
										bind:value={cedula}
										class="{inputClass} {visitorSelected ? 'opacity-70 bg-white/5' : ''}"
										placeholder="Ingrese ID (Cédula)"
										readonly={visitorSelected}
									/>
									{#if visitorSelected}
										<div class="absolute right-3 top-1/2 -translate-y-1/2 text-accent">
											<Check size={14} />
										</div>
									{/if}
								</div>
							</div>

							<div class="grid grid-cols-1 lg:grid-cols-2 gap-5">
								<div class="grid grid-cols-2 gap-3">
									<div>
										<label for="nombre" class={labelClass}
											>Primer Nombre <span class="text-red-500">*</span></label
										>
										<input
											id="nombre"
											type="text"
											bind:value={nombre}
											class="{inputClass} {visitorSelected ? 'opacity-70 bg-white/5' : ''}"
											placeholder="Nombre"
											readonly={visitorSelected}
										/>
									</div>
									<div>
										<label for="segundoNombre" class={labelClass}>Segundo (Opc)</label>
										<input
											id="segundoNombre"
											type="text"
											bind:value={segundoNombre}
											class="{inputClass} {visitorSelected ? 'opacity-70 bg-white/5' : ''}"
											placeholder="Nombre"
											readonly={visitorSelected}
										/>
									</div>
								</div>
								<div class="grid grid-cols-2 gap-3">
									<div>
										<label for="apellido" class={labelClass}
											>Primer Apellido <span class="text-red-500">*</span></label
										>
										<input
											id="apellido"
											type="text"
											bind:value={apellido}
											class="{inputClass} {visitorSelected ? 'opacity-70 bg-white/5' : ''}"
											placeholder="Apellido"
											readonly={visitorSelected}
										/>
									</div>
									<div>
										<label for="segundoApellido" class={labelClass}>Segundo (Opc)</label>
										<input
											id="segundoApellido"
											type="text"
											bind:value={segundoApellido}
											class="{inputClass} {visitorSelected ? 'opacity-70 bg-white/5' : ''}"
											placeholder="Apellido"
											readonly={visitorSelected}
										/>
									</div>
								</div>
							</div>
						</div>

						<!-- Fila 2: Empresa y Vehículo (Unificado) -->
						<div class="grid grid-cols-1 lg:grid-cols-2 gap-5">
							<!-- Selector Empresa -->
							<div>
								<label for="empresaId" class={labelClass}
									>Empresa <span class="text-red-500">*</span></label
								>
								<div class="flex gap-2 relative">
									<!-- Custom Dropdown Trigger -->
									<div class="relative flex-1">
										<button
											type="button"
											disabled={loading || empresaStore.loading}
											onclick={() => (showEmpresaDropdown = !showEmpresaDropdown)}
											class="{selectClass} flex items-center justify-between"
										>
											<span class="truncate">
												{#if empresaStore.loading}
													Cargando...
												{:else}
													{empresaStore.empresas.find((e) => e.id === empresaId)?.nombre ||
														'Seleccione empresa'}
												{/if}
											</span>
											{#if !visitorSelected}
												<ChevronDown size={14} class="text-secondary" />
											{/if}
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
												class="form-dropdown absolute top-full left-0 right-0 mt-1 z-50 max-h-60 overflow-y-auto bg-[#1c2128] border border-white/10 rounded-lg shadow-xl"
												transition:fly={{ y: -5, duration: 200 }}
											>
												{#if empresaStore.empresas.length === 0}
													<div class="px-3 py-2 text-sm text-secondary">No hay empresas</div>
												{:else}
													{#each empresaStore.empresas as empresa}
														<button
															type="button"
															onclick={() => {
																empresaId = empresa.id;
																showEmpresaDropdown = false;
															}}
															class="flex w-full items-center justify-between px-3 py-2 text-sm text-gray-300 hover:bg-white/10 transition-colors"
														>
															<span>{empresa.nombre}</span>
															{#if empresaId === empresa.id}
																<Check size={14} class="text-blue-500" />
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
										class="px-3 py-1.5 rounded-lg border border-white/10 bg-black/20 text-secondary hover:text-white hover:border-white/30 transition-colors"
										title="Añadir nueva empresa"
									>
										<Plus size={16} />
									</button>
								</div>
							</div>

							<!-- Selector Vehículo (Simplificado - Unificado) -->
							<div>
								<label for="vehiculoPlaca" class={labelClass}>Vehículo / Medio Ingreso</label>
								<div class="flex gap-2 relative">
									<!-- Custom Dropdown Trigger -->
									<div class="relative flex-1">
										<button
											type="button"
											disabled={loading || loadingVehiculos}
											onclick={() => (showVehiculoDropdown = !showVehiculoDropdown)}
											class="{selectClass} flex items-center justify-between {vehiculoPlaca
												? '!border-blue-500/50 !ring-1 !ring-blue-500/20'
												: ''}"
										>
											<span class="truncate flex items-center gap-2">
												{#if loadingVehiculos}
													Cargando...
												{:else if vehiculoPlaca}
													<span class="font-mono font-bold bg-white/10 px-1.5 rounded text-xs"
														>{vehiculoPlaca}</span
													>
													{#if vehiculoMarca}
														<span class="text-xs opacity-70">
															{vehiculoMarca}
															{vehiculoModelo}
														</span>
													{/if}
												{:else}
													<span class="opacity-50 flex items-center gap-2">
														<PersonStanding size={14} />
														Caminando (Sin vehículo)
													</span>
												{/if}
											</span>
											<ChevronDown size={14} class="text-secondary" />
										</button>

										<!-- Dropdown Options -->
										{#if showVehiculoDropdown}
											<div
												class="fixed inset-0 z-40"
												onclick={() => (showVehiculoDropdown = false)}
												role="presentation"
												aria-hidden="true"
											></div>

											<div
												class="absolute top-full left-0 right-0 mt-1 z-50 max-h-60 overflow-y-auto bg-[#1c2128] border border-white/10 rounded-lg shadow-xl"
												transition:fly={{ y: -5, duration: 200 }}
											>
												<!-- Opción Caminando (Limpiar) -->
												<button
													type="button"
													onclick={() => {
														resetVehiculoForm(); // Limpia placa -> Modo caminando
														showVehiculoDropdown = false;
													}}
													class="flex w-full items-center justify-between px-3 py-2 text-sm text-gray-300 hover:bg-white/10 transition-colors border-b border-white/5"
												>
													<div class="flex items-center gap-2">
														<PersonStanding size={16} class="opacity-70" />
														<span>Caminando</span>
													</div>
													{#if !vehiculoPlaca}
														<Check size={14} class="text-blue-500" />
													{/if}
												</button>

												{#if vehiculosList.length > 0}
													{#each Object.entries(groupedVehiculos) as [tipo, list]}
														<div
															class="px-3 py-1 text-xs text-secondary/50 font-medium bg-black/20"
														>
															{tipo}
														</div>
														{#each list as v}
															<button
																type="button"
																onclick={() => {
																	vehiculoTipo = v.tipoVehiculo;
																	vehiculoPlaca = v.placa;
																	vehiculoMarca = v.marca || '';
																	vehiculoModelo = v.modelo || '';
																	vehiculoColor = v.color || '';
																	showVehiculoDropdown = false;
																}}
																class="flex w-full items-center justify-between px-3 py-2 text-sm text-gray-300 hover:bg-white/10 transition-colors group"
															>
																<div class="flex items-center gap-2">
																	{#if v.tipoVehiculo === 'motocicleta'}
																		<Bike size={14} class="opacity-70" />
																	{:else}
																		<Car size={14} class="opacity-70" />
																	{/if}
																	<span class="font-mono font-bold">{v.placa}</span>
																</div>
																{#if vehiculoPlaca === v.placa}
																	<Check size={14} class="text-blue-500" />
																{/if}
															</button>
														{/each}
													{/each}
												{:else}
													<div class="px-3 py-4 text-xs text-secondary opacity-50 text-center">
														No hay vehículos recientes
													</div>
												{/if}
											</div>
										{/if}
									</div>

									<!-- Add Button -->
									<button
										type="button"
										onclick={() => (showVehiculoForm = true)}
										disabled={loading}
										class="px-3 py-1.5 rounded-lg border border-white/10 bg-black/20 text-secondary hover:text-white hover:border-white/30 transition-colors"
										title="Registrar nuevo vehículo"
									>
										<Plus size={16} />
									</button>
								</div>
							</div>
						</div>

						<!-- Fila 3: Detalles Visita -->
						<div class="grid grid-cols-1 lg:grid-cols-2 gap-5">
							<div>
								<label for="anfitrion" class={labelClass}
									>Anfitrión <span class="text-red-500">*</span></label
								>
								<input
									id="anfitrion"
									type="text"
									bind:value={anfitrion}
									class={inputClass}
									placeholder="¿A quién visita?"
								/>
							</div>
							<div>
								<label for="areaVisitada" class={labelClass}
									>Área Visitada <span class="text-red-500">*</span></label
								>
								<input
									id="areaVisitada"
									type="text"
									bind:value={areaVisitada}
									class={inputClass}
									placeholder="Ej. Administración, Planta..."
								/>
							</div>
						</div>

						<!-- Fila 4: Motivo y Fecha -->
						<div class="grid grid-cols-1 lg:grid-cols-2 gap-5">
							<div>
								<label for="motivo" class={labelClass}
									>Motivo <span class="text-red-500">*</span></label
								>
								<input
									id="motivo"
									type="text"
									bind:value={motivo}
									class={inputClass}
									placeholder="Ej. Reunión, Entrega..."
								/>
							</div>
							<div class="grid grid-cols-2 gap-3">
								<div>
									<label for="fechaEsperada" class={labelClass}
										>Fecha Esperada <span class="text-red-500">*</span></label
									>
									<input
										id="fechaEsperada"
										type="text"
										value={fechaEsperadaDisplay}
										oninput={handleDateInput}
										class={inputClass}
										placeholder="DD/MM/YYYY"
										maxlength="10"
									/>
								</div>
								<div>
									<label for="horaEsperada" class={labelClass}
										>Hora <span class="text-red-500">*</span></label
									>
									<input
										id="horaEsperada"
										type="time"
										bind:value={horaEsperada}
										class="{inputClass} [color-scheme:dark]"
									/>
								</div>
							</div>
						</div>
					</form>
				</div>
			</div>

			<!-- Footer Estándar con Botones Reactivos -->
			<div
				class="flex-none flex items-center justify-end gap-3 px-6 py-4 border-t border-surface bg-surface-1"
			>
				<!-- Cancelar -->
				<button
					onclick={close}
					class="px-4 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-white/60 hover:text-white/80 text-sm"
				>
					Cancelar
				</button>

				<!-- Guardar -->
				<button
					onclick={handleSubmit}
					disabled={loading}
					class="px-6 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-success hover:text-success text-sm disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
				>
					{#if loading}
						<Loader2 size={16} class="animate-spin" />
						<span>Guardando...</span>
					{:else}
						<Save size={16} />
						<span
							>{#if editingId}Actualizar{:else}Guardar{/if}</span
						>
					{/if}
				</button>
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
					<div class="p-3 bg-error/10 border border-error/20 text-error rounded-lg text-sm">
						{empresaError}
					</div>
				{/if}
				<input
					type="text"
					bind:value={nuevaEmpresaNombre}
					placeholder="Nombre de la empresa"
					class={inputClass}
				/>
			</div>

			<div
				class="flex items-center justify-end gap-3 px-5 py-4 border-t border-surface bg-surface-1"
			>
				<button
					onclick={() => (showEmpresaModal = false)}
					class="px-3 py-1.5 rounded-lg border border-surface text-secondary hover:text-white transition-colors text-sm"
				>
					Cancelar
				</button>
				<button
					onclick={handleCrearEmpresa}
					disabled={creatingEmpresa}
					class="px-3 py-1.5 rounded-lg bg-primary text-white hover:bg-primary/90 transition-colors text-sm"
				>
					{creatingEmpresa ? 'Guardando...' : 'Guardar'}
				</button>
			</div>
		</div>
	</div>
{/if}
