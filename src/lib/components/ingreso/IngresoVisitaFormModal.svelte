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
		ChevronRight,
		RefreshCw
	} from 'lucide-svelte';
	import { shortcutRegistry, shortcutCommand, clearCommand } from '$lib/shortcuts';
	import PersonaFinder from '$lib/components/ingreso/shared/persona/PersonaFinder.svelte';

	// Superforms & Zod v4
	import { superForm } from 'sveltekit-superforms';
	import { zod4 } from 'sveltekit-superforms/adapters';
	import { ingresoVisitaSchema, type IngresoVisitaFormData } from '$lib/schemas/visitaSchema';

	// Logic
	import { ingresoVisitaService } from '$lib/services/ingresoVisitaService';
	import { preRegistroVisitaService } from '$lib/services/preRegistroVisitaService';
	import { getVisitanteByCedula } from '$lib/logic/visitante/visitanteService';
	import { empresaStore } from '$lib/stores/empresaStore.svelte';
	import { submitCreateEmpresa } from '$lib/logic/empresa/empresaService';
	import { currentUser } from '$lib/stores/auth';
	import { invoke } from '@tauri-apps/api/core';
	import { vehiculos } from '$lib/api/vehiculos';
	import type { VehiculoResponse } from '$lib/types/vehiculo';
	import { PersonStanding, Car, Bike, Check } from 'lucide-svelte';

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
	let foundPreRegistroId = $state<string | undefined>(undefined);
	let visitorSelected = $state(false);
	let searchResetKey = $state(0);

	// Derived
	const isPreRegistroMode = $derived(!!foundPreRegistroId);

	// UI State
	let showEmpresaDropdown = $state(false);
	let showEmpresaModal = $state(false);
	let nuevaEmpresaNombre = $state('');
	let creatingEmpresa = $state(false);
	let showObservaciones = $state(false);
	let checkTimeout: ReturnType<typeof setTimeout>;

	// Vehicle State (Refined)
	let vehiculosList = $state<VehiculoResponse[]>([]);
	let loadingVehiculos = $state(false);
	let showVehiculoDropdown = $state(false);
	let showVehiculoForm = $state(false);

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

	// Vehicle Creation State
	let vehiculoTipo = $state('');
	let vehiculoPlaca = $state('');
	let vehiculoMarca = $state('');
	let vehiculoModelo = $state('');
	let vehiculoColor = $state('');

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
		observaciones: '',
		placaVehiculo: '',
		modoIngreso: 'caminando'
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

						// Convertir gafete a número si existe
						const gafeteNum = f.data.gafete ? parseInt(f.data.gafete, 10) : undefined;

						await ingresoVisitaService.createIngreso({
							cedula: f.data.cedula.trim(),
							preRegistroId:
								typeof foundPreRegistroId === 'object'
									? (foundPreRegistroId as any).id?.String ||
										(foundPreRegistroId as any).id ||
										JSON.stringify(foundPreRegistroId)
									: foundPreRegistroId,
							nombre: f.data.nombre.trim(),
							apellido: f.data.apellido.trim(),
							segundoNombre: f.data.segundoNombre?.trim() || undefined,
							segundoApellido: f.data.segundoApellido?.trim() || undefined,
							empresaNombre:
								selectedEmpresa?.nombre ||
								initialPerson?.empresaNombre ||
								initialPerson?.empresa_nombre ||
								undefined,
							anfitrion: f.data.anfitrion.trim(),
							areaVisitada: f.data.areaVisitada.trim(),
							motivo: f.data.motivo.trim(),
							modoIngreso: f.data.modoIngreso,
							placaVehiculo: f.data.placaVehiculo?.trim() || undefined,
							gafeteNumero: !isNaN(Number(gafeteNum)) ? gafeteNum : undefined,
							observaciones: f.data.observaciones?.trim() || undefined
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
			shortcutRegistry.pushScope('modal');
			empresaStore.init();
			if (initialPerson) {
				fillPersonData(initialPerson);
			} else {
				reset();
				validationResult = null;
				showObservaciones = false;
				foundPreRegistroId = undefined;
			}
		} else {
			shortcutRegistry.popScope();
		}
	});

	// Handle global shortcuts
	$effect(() => {
		const cmd = $shortcutCommand;
		if (show && cmd?.command === 'cancel') {
			handleClose();
			clearCommand();
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
		// Determine if this is a PreRegistro object
		// PreRegistro has specific fields like 'fechaEsperada' or 'anfitrion' populated
		// Backend returns snake_case, so we check both
		const isPreRegistro = !!person.fechaEsperada || !!person.fecha_esperada;

		if (isPreRegistro) {
			// Normalizar ID para evitar objetos RecordId enviados al backend como string
			if (person.id && typeof person.id === 'object') {
				foundPreRegistroId = person.id.id?.String || person.id.id || person.id.toString();
			} else {
				foundPreRegistroId = person.id;
			}
		}

		const data = {
			cedula: person.cedula || '',
			nombre: person.nombre || '',
			segundoNombre: person.segundoNombre || person.segundo_nombre || '',
			apellido: person.apellido || '',
			segundoApellido: person.segundoApellido || person.segundo_apellido || '',
			empresaId: '', // Set below
			anfitrion: person.anfitrion || '',
			areaVisitada: person.areaVisitada || person.area_visitada || '',
			motivo: person.motivo || '',
			gafete: '',
			observaciones: person.observaciones || '',
			placaVehiculo: person.placaVehiculo || person.placa_vehiculo || person.placa || '',
			modoIngreso:
				person.modoIngreso ||
				person.modo_ingreso ||
				(person.placaVehiculo || person.placa_vehiculo || person.placa ? 'vehiculo' : 'caminando')
		};

		// Normalize empresaId from RecordId object if needed
		let rawEmpresaId = person.empresaId || person.empresa_id;
		if (rawEmpresaId && typeof rawEmpresaId === 'object' && rawEmpresaId.tb && rawEmpresaId.id) {
			data.empresaId = `${rawEmpresaId.tb}:${rawEmpresaId.id}`;
		} else if (typeof rawEmpresaId === 'string') {
			data.empresaId = rawEmpresaId;
		}

		// Fallback: Try to match enterprise name to ID for the dropdown if ID is missing
		if (!data.empresaId && person.empresa_nombre) {
			const matched = empresaStore.empresas.find(
				(e) => e.nombre.toLowerCase() === person.empresa_nombre.toLowerCase()
			);
			if (matched) data.empresaId = matched.id;
		} else if (!data.empresaId && person.empresaNombre) {
			// Handle camelCase variant often found in frontend types
			const matched = empresaStore.empresas.find(
				(e) => e.nombre.toLowerCase() === person.empresaNombre.toLowerCase()
			);
			if (matched) data.empresaId = matched.id;
		}

		reset({ data });

		// Auto-show observaciones if present
		if (data.observaciones) {
			showObservaciones = true;
		}

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
				// 1. Check Pre-Registro (Cita)
				const preReg = await preRegistroVisitaService.checkByCedula(val);

				if (preReg) {
					// Cargar datos de la cita
					$form.nombre = preReg.nombre;
					$form.apellido = preReg.apellido;
					$form.segundoNombre = preReg.segundoNombre || '';
					$form.segundoApellido = preReg.segundoApellido || '';
					$form.anfitrion = preReg.anfitrion;
					$form.areaVisitada = preReg.areaVisitada;
					$form.motivo = preReg.motivo;
					$form.observaciones = preReg.observaciones || '';
					$form.placaVehiculo = preReg.placa || '';
					$form.modoIngreso = preReg.modo_ingreso || (preReg.placa ? 'vehiculo' : 'caminando');
					foundPreRegistroId = preReg.id;

					// Map company
					if (preReg.empresa_id) {
						let eid = preReg.empresa_id;
						if (eid && typeof eid === 'object' && eid.tb) {
							$form.empresaId = `${eid.tb}:${eid.id}`;
						} else {
							$form.empresaId = eid;
						}
					} else if (preReg.empresaNombre) {
						const matched = empresaStore.empresas.find(
							(e) => e.nombre.toLowerCase() === preReg.empresaNombre!.toLowerCase()
						);
						if (matched) $form.empresaId = matched.id;
					}

					toast.success('📅 Cita encontrada: Datos cargados');
				} else {
					// 2. Si no hay cita, buscar en historial (Catálogo)
					foundPreRegistroId = undefined;
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
						toast.success('Visitante encontrado en historial');
					}
				}

				await validarAcceso(val);
			} catch (e) {
				console.error(e);
			} finally {
				searchingPerson = false;
			}
		}, 500);
	}

	function handleGafeteInput(event: Event) {
		const input = event.target as HTMLInputElement;
		// Solo números
		let val = input.value.replace(/[^0-9]/g, '');
		// Eliminar ceros a la izquierda
		val = val.replace(/^0+/, '');

		$form.gafete = val;
		// Sincronizar el valor del input para evitar que se muestren los ceros borrados
		input.value = val;
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

	function resetVehiculoForm() {
		vehiculoTipo = '';
		vehiculoPlaca = '';
		vehiculoMarca = '';
		vehiculoModelo = '';
		vehiculoColor = '';
		showVehiculoDropdown = false;
		showVehiculoForm = false;
	}

	async function handlePersonaSelect(event: CustomEvent) {
		const { data } = event.detail;
		visitorSelected = true;

		// Rellenar formulario
		$form.cedula = data.cedula || '';
		$form.nombre = data.nombre || '';
		$form.segundoNombre = data.segundoNombre || '';
		$form.apellido = data.apellido || '';
		$form.segundoApellido = data.segundoApellido || '';

		// Empresa
		if (data.empresaId) {
			$form.empresaId = data.empresaId;
		} else if (data.empresaNombre) {
			const matched = empresaStore.empresas.find(
				(e) => e.nombre.toLowerCase() === data.empresaNombre.toLowerCase()
			);
			if (matched) $form.empresaId = matched.id;
		}

		// Vehículo
		if (data.placaVehiculo || data.placa) {
			$form.placaVehiculo = data.placaVehiculo || data.placa || '';
			$form.modoIngreso = 'vehiculo';
		} else {
			$form.placaVehiculo = '';
			$form.modoIngreso = 'caminando';
		}

		// Cargar vehículos del visitante
		if (data.id) {
			loadVehiculos(data.id);
		} else {
			vehiculosList = [];
		}

		// Validar acceso
		if ($form.cedula) {
			await validarAcceso($form.cedula);
		}

		// Foco en gafete
		setTimeout(() => {
			const gafeteInput = document.getElementById('gafete') as HTMLInputElement;
			if (gafeteInput) gafeteInput.focus();
		}, 100);
	}

	function clearSearch() {
		visitorSelected = false;
		reset();
		validationResult = null;
		foundPreRegistroId = undefined;
		searchResetKey++;
	}

	function handleClose() {
		if (!loading) {
			show = false;
			reset();
			visitorSelected = false;
			searchResetKey++;
		}
	}

	// UI Helpers
	// Usando clases centralizadas de theme.css
	// inputClass, labelClass, errorClass reemplazados por form-input, form-label, form-error

	function getFieldStateClass(field: keyof IngresoVisitaFormData, value?: any) {
		if ($errors[field]) return 'is-error';
		if (value && String(value).trim() !== '') return 'is-valid';
		return '';
	}
</script>

<svelte:window
	onkeydown={(e) => {
		if (show) {
			if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 's') {
				e.preventDefault();
				const f = document.querySelector('form[method="POST"]') as HTMLFormElement;
				if (f) f.requestSubmit();
			}
		}
	}}
/>

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
			class="relative z-10 w-full max-w-[700px] max-h-[95vh] overflow-hidden rounded-xl bg-surface-2 shadow-2xl border border-surface flex flex-col"
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
						{#if validationResult && !validationResult.puedeIngresar}
							<div
								class="p-4 rounded-xl border flex items-start gap-3 transition-all bg-red-500/10 border-red-500/20"
								transition:fade
							>
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
							</div>
						{/if}

						{#if isPreRegistroMode}
							<!-- ========================================== -->
							<!-- VISTA SIMPLIFICADA (Tarjeta de Verificación) -->
							<!-- ========================================== -->
							<div class="space-y-6 px-1">
								<!-- Info Card -->
								<div class="bg-surface-1 rounded-xl border border-surface overflow-hidden relative">
									<!-- Decoración lateral -->
									<div class="absolute left-0 top-0 bottom-0 w-1 bg-accent"></div>

									<div class="p-5 grid grid-cols-2 gap-y-4 gap-x-6">
										<div class="col-span-2 flex items-center gap-3 pb-4 border-b border-surface/50">
											<div class="p-2.5 rounded-full bg-surface-2 text-primary">
												<ChevronRight size={20} />
											</div>
											<div>
												<h3 class="text-xl font-bold text-white tracking-tight">
													{$form.nombre}
													{$form.apellido}
												</h3>
												<div class="flex items-center gap-2 text-secondary text-sm font-mono">
													<span>{$form.cedula}</span>
													{#if $form.placaVehiculo}
														<span class="px-1.5 py-0.5 rounded bg-surface-3 text-xs"
															>{$form.placaVehiculo}</span
														>
													{/if}
												</div>
											</div>
										</div>

										<!-- Sección 1: Quién y De Dónde -->
										<div>
											<span class="text-[10px] uppercase font-bold text-secondary tracking-wider"
												>Anfitrión</span
											>
											<div class="text-sm font-medium text-white">{$form.anfitrion}</div>
										</div>

										<div>
											<span class="text-[10px] uppercase font-bold text-secondary tracking-wider"
												>Empresa</span
											>
											<div class="text-sm font-medium text-white">
												{empresaStore.empresas.find((e) => e.id === $form.empresaId)?.nombre ||
													initialPerson?.empresa_nombre ||
													initialPerson?.empresaNombre ||
													'Sin Empresa'}
											</div>
										</div>

										<!-- Sección 2: Dónde y Por Qué -->
										<div>
											<span class="text-[10px] uppercase font-bold text-secondary tracking-wider"
												>Área Visitada</span
											>
											<div class="text-sm font-medium text-white">{$form.areaVisitada}</div>
										</div>

										<div>
											<span class="text-[10px] uppercase font-bold text-secondary tracking-wider"
												>Motivo</span
											>
											<div class="text-sm font-medium text-white">{$form.motivo}</div>
										</div>

										<!-- Sección 3: Cuándo y Cómo -->
										<div>
											<span class="text-[10px] uppercase font-bold text-secondary tracking-wider"
												>Fecha / Hora Cita</span
											>
											<div class="text-sm font-medium text-white">
												{(() => {
													const f = initialPerson?.fecha_esperada || initialPerson?.fechaEsperada;
													if (!f) return 'No definida';
													const h =
														initialPerson?.hora_esperada || initialPerson?.horaEsperada || '';
													const bits = f.split('T')[0].split('-');
													const dateStr =
														bits.length === 3 ? `${bits[2]}/${bits[1]}/${bits[0]}` : f;
													return `${dateStr} ${h}`;
												})()}
											</div>
										</div>

										<div>
											<span class="text-[10px] uppercase font-bold text-secondary tracking-wider"
												>Vehículo</span
											>
											<div class="text-sm font-medium text-white flex items-center gap-1.5">
												{#if $form.modoIngreso === 'vehiculo'}
													<span class="text-accent">🚘 {$form.placaVehiculo || 'Sin placa'}</span>
												{:else}
													<span>🚶 Caminando</span>
												{/if}
											</div>
										</div>
									</div>
								</div>

								<!-- Inputs de Confirmación -->
								<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
									<div class="md:col-span-2">
										<label class="form-label" for="gafete_verify">
											Asignar Gafete <span class="text-error">*</span>
										</label>
										<!-- svelte-ignore a11y_autofocus -->
										<input
											id="gafete_verify"
											name="gafete"
											type="text"
											bind:value={$form.gafete}
											placeholder="Escanee o ingrese #"
											class="form-input text-center text-lg font-mono tracking-widest bg-surface-1 border-accent/50 focus:border-accent ring-accent/20 h-12"
											autocomplete="off"
											disabled={loading}
											autofocus
										/>
									</div>

									<div class="md:col-span-2">
										<label class="form-label" for="observaciones_verify">
											Observaciones de Ingreso
										</label>
										<textarea
											id="observaciones_verify"
											class="form-input resize-none bg-surface-1"
											rows="2"
											placeholder="Nota opcional..."
											bind:value={$form.observaciones}
											disabled={loading}
										></textarea>
									</div>
								</div>
							</div>
						{:else}
							<!-- ========================================== -->
							<!-- VISTA COMPLETA (Formulario Manual)       -->
							<!-- ========================================== -->
							<!-- grid-cols-4 and spacing adapted to reference -->
							<div
								class="bg-surface-1 rounded-lg border border-surface p-5 grid grid-cols-4 gap-x-4 gap-y-3"
							>
								<!-- Buscador Unificado (Full Width) -->
								<div class="col-span-4 mb-1">
									{#key searchResetKey}
										<PersonaFinder scope="all" on:select={handlePersonaSelect} autoFocus={true} />
									{/key}
								</div>

								<!-- Identity Row -->
								<div class="col-span-1">
									<label for="cedula" class="form-label">
										Cédula <span class="text-error">*</span>
									</label>
									<div class="relative">
										<input
											id="cedula"
											name="cedula"
											class="form-input {getFieldStateClass('cedula', $form.cedula)}"
											bind:value={$form.cedula}
											oninput={handleCedulaInput}
											placeholder="Ej: 1-1122-0333"
											disabled={loading || visitorSelected}
											readonly={visitorSelected}
											{...$constraints.cedula}
										/>
										{#if searchingPerson}
											<div class="absolute right-3 top-2.5">
												<div
													class="w-3.5 h-3.5 border-2 border-accent border-t-transparent rounded-full animate-spin"
												></div>
											</div>
										{/if}
									</div>
									{#if $errors.cedula}<p class="form-error">{$errors.cedula}</p>{/if}
								</div>

								<div class="col-span-1">
									<label class="form-label" for="gafete"> Gafete </label>
									<input
										id="gafete"
										name="gafete"
										type="text"
										bind:value={$form.gafete}
										oninput={handleGafeteInput}
										placeholder="Ej: 123"
										class="form-input text-center font-mono tracking-widest bg-surface-1 border-accent/20"
										autocomplete="off"
										disabled={loading}
									/>
								</div>

								<div class="col-span-1">
									<label for="anfitrion" class="form-label"
										>Anfitrión <span class="text-error">*</span></label
									>
									<input
										id="anfitrion"
										name="anfitrion"
										class="form-input {getFieldStateClass('anfitrion', $form.anfitrion)}"
										bind:value={$form.anfitrion}
										oninput={() => validate('anfitrion')}
										placeholder="¿A quién visita?"
										disabled={loading}
									/>
									{#if $errors.anfitrion}<p class="form-error">{$errors.anfitrion}</p>{/if}
								</div>

								<div class="col-span-1">
									<label for="motivo" class="form-label"
										>Motivo <span class="text-error">*</span></label
									>
									<input
										id="motivo"
										name="motivo"
										class="form-input {getFieldStateClass('motivo', $form.motivo)}"
										bind:value={$form.motivo}
										oninput={() => validate('motivo')}
										placeholder="Ej. Entrevista..."
										disabled={loading}
									/>
									{#if $errors.motivo}<p class="form-error">{$errors.motivo}</p>{/if}
								</div>

								<!-- Names Row (4 Cols) -->
								<div class="col-span-1">
									<label for="nombre" class="form-label"
										>Nombre <span class="text-error">*</span></label
									>
									<input
										id="nombre"
										name="nombre"
										class="form-input {getFieldStateClass('nombre', $form.nombre)}"
										bind:value={$form.nombre}
										oninput={() => validate('nombre')}
										placeholder="Ej: Juan"
										disabled={loading || searchingPerson || visitorSelected}
										readonly={visitorSelected}
									/>
								</div>

								<div class="col-span-1">
									<label for="segundoNombre" class="form-label">Segundo Nombre</label>
									<input
										id="segundoNombre"
										name="segundoNombre"
										class="form-input"
										bind:value={$form.segundoNombre}
										placeholder="Ej: Carlos"
										disabled={loading || searchingPerson || visitorSelected}
										readonly={visitorSelected}
									/>
								</div>

								<!-- Metadata row moved up -->
								<div class="col-span-1">
									<label for="empresaId" class="form-label"
										>Empresa <span class="text-error">*</span></label
									>
									<div class="relative">
										<button
											type="button"
											onclick={() => (showEmpresaDropdown = !showEmpresaDropdown)}
											disabled={loading ||
												searchingPerson ||
												empresaStore.loading ||
												visitorSelected}
											class="form-select w-full h-[34px] pr-8 text-left {showEmpresaDropdown
												? 'border-accent ring-1 ring-accent/20'
												: getFieldStateClass('empresaId', $form.empresaId)}"
										>
											<span class="truncate">
												{#if empresaStore.loading}
													...
												{:else}
													{empresaStore.empresas.find((e) => e.id === $form.empresaId)?.nombre ||
														'Seleccionar'}
												{/if}
											</span>
											<ChevronDown
												size={14}
												class="absolute right-8 top-1/2 -translate-y-1/2 text-secondary"
											/>
										</button>
										<button
											type="button"
											onclick={() => (showEmpresaModal = true)}
											disabled={loading || searchingPerson || visitorSelected}
											class="absolute right-0 top-0 h-[34px] w-8 flex items-center justify-center border-l border-surface text-secondary hover:text-primary transition-colors bg-surface-2 rounded-r-lg"
											title="Añadir"
										>
											<Plus size={14} />
										</button>
										{#if showEmpresaDropdown}
											<button
												type="button"
												class="fixed inset-0 z-40 w-full h-full bg-transparent border-none cursor-default"
												onclick={() => (showEmpresaDropdown = false)}
												aria-label="Cerrar desplegable"
											></button>
											<div
												class="form-dropdown absolute top-full left-0 right-0 mt-1 z-50 max-h-60 overflow-y-auto"
												transition:fly={{ y: -5, duration: 200 }}
											>
												{#if !empresaStore.empresas || empresaStore.empresas.length === 0}
													<div class="px-3 py-2 text-sm text-secondary">Vacío</div>
												{:else}
													{#each empresaStore.empresas as empresa}
														<button
															type="button"
															onclick={() => {
																$form.empresaId = empresa.id;
																showEmpresaDropdown = false;
																validate('empresaId');
															}}
															class="form-dropdown-item justify-between group"
														>
															<span class="truncate">{empresa.nombre}</span>
															{#if $form.empresaId === empresa.id}
																<Check size={14} class="text-primary" />
															{/if}
														</button>
													{/each}
												{/if}
											</div>
										{/if}
									</div>
								</div>

								<div class="col-span-1">
									<label for="vehiculoPlaca" class="form-label">Vehículo</label>
									<div class="relative">
										<button
											type="button"
											disabled={loading || loadingVehiculos}
											onclick={() => (showVehiculoDropdown = !showVehiculoDropdown)}
											class="form-select w-full h-[34px] pr-8 text-left {$form.placaVehiculo
												? 'border-accent ring-1 ring-accent/10 text-xs'
												: 'text-xs opacity-70'}"
										>
											<span class="truncate">
												{#if loadingVehiculos}
													...
												{:else if $form.placaVehiculo}
													<span class="font-mono font-bold bg-surface-3 px-1 rounded"
														>{$form.placaVehiculo}</span
													>
												{:else}
													Caminando
												{/if}
											</span>
											<ChevronDown
												size={14}
												class="absolute right-8 top-1/2 -translate-y-1/2 text-secondary"
											/>
										</button>
										<button
											type="button"
											onclick={() => (showVehiculoForm = true)}
											disabled={loading}
											class="absolute right-0 top-0 h-[34px] w-8 flex items-center justify-center border-l border-surface text-secondary hover:text-primary transition-colors bg-surface-2 rounded-r-lg"
										>
											<Plus size={14} />
										</button>
										{#if showVehiculoDropdown}
											<button
												type="button"
												class="fixed inset-0 z-40 w-full h-full bg-transparent border-none cursor-default"
												onclick={() => (showVehiculoDropdown = false)}
												aria-label="Cerrar desplegable"
											></button>
											<div
												class="form-dropdown absolute top-full left-0 right-0 mt-1 z-50 max-h-60 overflow-y-auto"
												transition:fly={{ y: -5, duration: 200 }}
											>
												<button
													type="button"
													onclick={() => {
														$form.placaVehiculo = '';
														$form.modoIngreso = 'caminando';
														showVehiculoDropdown = false;
													}}
													class="form-dropdown-item text-xs"
												>
													Caminando
												</button>
												{#each Object.entries(groupedVehiculos) as [tipo, list]}
													<div
														class="px-2 py-1 text-[9px] uppercase font-bold bg-surface-3 opacity-50"
													>
														{tipo}
													</div>
													{#each list as v}
														<button
															type="button"
															onclick={() => {
																$form.placaVehiculo = v.placa;
																$form.modoIngreso = 'vehiculo';
																showVehiculoDropdown = false;
															}}
															class="form-dropdown-item text-xs"
														>
															{v.placa} ({v.marca || ''})
														</button>
													{/each}
												{/each}
											</div>
										{/if}
									</div>
								</div>

								<!-- Names Row Continued (Moved down) -->
								<div class="col-span-1">
									<label for="apellido" class="form-label"
										>Apellido <span class="text-error">*</span></label
									>
									<input
										id="apellido"
										name="apellido"
										class="form-input {getFieldStateClass('apellido', $form.apellido)}"
										bind:value={$form.apellido}
										oninput={() => validate('apellido')}
										placeholder="Ej: Pérez"
										disabled={loading || searchingPerson || visitorSelected}
										readonly={visitorSelected}
									/>
								</div>

								<div class="col-span-1">
									<label for="segundoApellido" class="form-label">Segundo Apellido</label>
									<input
										id="segundoApellido"
										name="segundoApellido"
										class="form-input"
										bind:value={$form.segundoApellido}
										placeholder="Ej: González"
										disabled={loading || searchingPerson || visitorSelected}
										readonly={visitorSelected}
									/>
								</div>

								<div class="col-span-1">
									<label for="area" class="form-label">Área <span class="text-error">*</span></label
									>
									<input
										id="area"
										name="areaVisitada"
										class="form-input"
										bind:value={$form.areaVisitada}
										placeholder="Ej: Piso 2"
										disabled={loading}
									/>
								</div>

								<div class="col-span-1">
									<div class="form-label opacity-0" aria-hidden="true">-</div>
									<button
										type="button"
										onclick={() => (showObservaciones = !showObservaciones)}
										class="flex items-center justify-center w-full h-[34px] gap-1.5 text-secondary hover:text-primary transition-colors text-xs border border-surface rounded-lg bg-surface-1"
									>
										{#if showObservaciones}
											<ChevronDown size={14} />
										{:else}
											<ChevronRight size={14} />
										{/if}
										<span>Observaciones</span>
									</button>
								</div>

								<!-- Moved Gafete upwards next to Cédula -->

								<!-- Observaciones Section (Full Width if open) -->
								{#if showObservaciones}
									<div class="col-span-4" transition:slide>
										<div
											class="obs-container w-full bg-surface-2 border border-surface rounded-lg focus-within:border-accent focus-within:ring-1 focus-within:ring-accent/20 transition-all outline-none"
										>
											<textarea
												class="w-full bg-transparent px-3 py-2 text-sm text-primary placeholder:text-secondary/50 resize-none focus:outline-none outline-none border-none appearance-none ring-0"
												rows="2"
												placeholder="Notas adicionales..."
												bind:value={$form.observaciones}
												disabled={loading}
											></textarea>
										</div>
									</div>
								{/if}
							</div>
						{/if}
					</div>

					<!-- Footer Actions -->
					<div
						class="flex-none flex items-center justify-end gap-3 px-6 py-4 border-t border-surface bg-surface-1"
					>
						<button
							type="button"
							onclick={handleClose}
							disabled={loading}
							class="form-btn-outline-secondary"
						>
							Cancelar
						</button>
						<button type="submit" disabled={loading} class="form-btn-outline-success">
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

<!-- Modal para Crear Vehículo -->
{#if showVehiculoForm}
	<div
		class="fixed inset-0 z-[110] flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
		transition:fade={{ duration: 150 }}
	>
		<button
			type="button"
			class="absolute inset-0 w-full h-full bg-transparent border-none cursor-default"
			onclick={() => (showVehiculoForm = false)}
			aria-label="Cerrar modal"
		></button>

		<div
			class="relative w-full max-w-[320px] bg-surface-2 rounded-xl shadow-2xl border border-surface overflow-hidden"
			transition:fly={{ y: 10, duration: 200 }}
		>
			<div class="px-5 py-4 space-y-4">
				<h3 class="text-sm font-semibold text-white">Nuevo Vehículo</h3>

				<div class="space-y-3">
					<div>
						<label class="form-label" for="vt">Tipo</label>
						<select id="vt" class="form-select text-xs h-9 bg-surface-1" bind:value={vehiculoTipo}>
							<option value="automovil">Automóvil</option>
							<option value="motocicleta">Motocicleta</option>
							<option value="camion">Camión / Pesado</option>
							<option value="otro">Otro</option>
						</select>
					</div>

					<div>
						<label class="form-label" for="vp">Placa</label>
						<input
							id="vp"
							type="text"
							class="form-input text-xs h-9 font-mono"
							placeholder="ABC-123"
							bind:value={vehiculoPlaca}
						/>
					</div>

					<div class="grid grid-cols-2 gap-2">
						<div>
							<label class="form-label" for="vm">Marca</label>
							<input
								id="vm"
								type="text"
								class="form-input text-xs h-9"
								placeholder="Toyota"
								bind:value={vehiculoMarca}
							/>
						</div>
						<div>
							<label class="form-label" for="vmo">Modelo</label>
							<input
								id="vmo"
								type="text"
								class="form-input text-xs h-9"
								placeholder="Yaris"
								bind:value={vehiculoModelo}
							/>
						</div>
					</div>
				</div>
			</div>

			<div class="flex justify-end gap-2 px-5 py-3 border-t border-surface bg-surface-1">
				<button
					type="button"
					onclick={() => (showVehiculoForm = false)}
					class="px-3 py-1.5 text-xs font-medium rounded-lg border-2 border-surface text-secondary transition-all duration-200 hover:border-white/60 hover:text-white/80"
				>
					Cancelar
				</button>
				<button
					type="button"
					disabled={!vehiculoPlaca.trim()}
					onclick={async () => {
						$form.placaVehiculo = vehiculoPlaca;
						$form.modoIngreso = 'vehiculo';
						showVehiculoForm = false;
						toast.success('Vehículo asignado a este ingreso');
					}}
					class="px-3 py-1.5 text-xs font-medium rounded-lg border-2 border-surface text-secondary transition-all duration-200 hover:border-blue-500 hover:text-blue-500"
				>
					Usar Placa
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

	/* Observaciones container */
	.obs-container,
	.obs-container *:focus {
		outline: none !important;
		box-shadow: none !important;
	}

	.obs-container:focus-within {
		border-color: var(--color-accent) !important;
		box-shadow: 0 0 0 1px var(--color-accent-bg) !important;
	}
</style>
