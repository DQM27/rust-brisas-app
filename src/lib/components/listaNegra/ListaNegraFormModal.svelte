<!-- src/lib/components/listaNegra/ListaNegraFormModal.svelte -->
<!-- Modal para agregar/editar personas en lista negra (Validación Zod + Superforms) -->
<script lang="ts">
	import { fade, fly } from 'svelte/transition';
	import { X, AlertTriangle, User, Building2 } from 'lucide-svelte';
	import { get } from 'svelte/store';
	import { currentUser } from '$lib/stores/auth';
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

	const { form, errors, constraints, enhance, reset } = superForm<AddToListaNegraForm>(
		initialValues,
		{
			SPA: true,
			validators: zod4(AddToListaNegraSchema),
			resetForm: false, // Control manual
			onUpdate: async ({ form: f }) => {
				if (!f.valid) return;

				const usuario = get(currentUser); // Use get() to ensure we have the value
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
		}
	);

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

	// ... imports existing ...

	// Handler para selección desde PersonaFinder
	async function handlePersonaSelect(event: CustomEvent) {
		const { id, type, data } = event.detail;

		selectedPersona = data;

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
					// Normalizar si es necesario (agregar prefijo si falta) pero generalmente viene con prefijo
					// La normalización real ocurre en el backend o en Zod si es estricto
					$form.empresaId = empIdString;

					// Si tenemos ID y el nombre no se ha seteado (o queremos asegurarlo)
					if (!$form.empresaNombre && fullData.empresaNombre) {
						$form.empresaNombre = fullData.empresaNombre;
					}

					// Fallback fetch si aun no tenemos nombre
					if (!$form.empresaNombre) {
						// ... fetch logic existing
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

	// Styles
	const inputClass =
		'w-full rounded-md border border-gray-600 bg-[#0d1117] px-3 py-2 text-sm text-gray-100 placeholder:text-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-60 disabled:cursor-not-allowed';
	const labelClass = 'block text-sm font-medium text-gray-300 mb-1';
	const errorClass = 'text-xs text-red-400 mt-1';

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
</script>

<svelte:window onkeydown={handleKeydown} />

{#if show}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center p-4"
		transition:fade={{ duration: 150 }}
	>
		<!-- Backdrop -->
		<button
			class="absolute inset-0 bg-black/60 backdrop-blur-sm border-0 cursor-default"
			onclick={onClose}
			aria-label="Cerrar modal"
		></button>

		<!-- Modal Content -->
		<div
			class="relative z-10 w-full max-w-2xl max-h-[90vh] overflow-auto rounded-lg bg-[#0d1117] shadow-2xl border border-gray-700"
			transition:fly={{ y: 20, duration: 200 }}
		>
			<!-- Header -->
			<div
				class="sticky top-0 z-20 flex items-center justify-between px-6 py-4 bg-[#0d1117] border-b border-gray-700"
			>
				<div class="flex items-center gap-3">
					<div class="p-2 rounded-lg bg-red-500/10">
						<AlertTriangle size={20} class="text-red-400" />
					</div>
					<h2 class="text-xl font-semibold text-white">{modalTitle}</h2>
				</div>
				<button
					onclick={onClose}
					class="p-1 rounded-full text-gray-400 hover:text-gray-200 hover:bg-gray-800 transition-colors"
					aria-label="Cerrar"
				>
					<X size={20} />
				</button>
			</div>

			<form method="POST" use:enhance class="p-6 space-y-5">
				<!-- Búsqueda de persona (solo en creación) -->
				{#if !isEditMode}
					<div class="space-y-2">
						<span class="{labelClass} mb-2">Buscar persona</span>

						{#if selectedPersona}
							<!-- Persona seleccionada -->
							<div
								class="flex items-center gap-3 p-3 rounded-lg bg-blue-500/10 border border-blue-500/30"
							>
								<User size={20} class="text-blue-400" />
								<div class="flex-1">
									<div class="font-medium text-white">
										{selectedPersona.nombre_completo ||
											selectedPersona.nombreCompleto ||
											'Desconocido'}
									</div>
									<div class="text-sm text-gray-400">
										{selectedPersona.cedula || 'S/C'} •
										{selectedPersona.tipo || 'Persona'}
										{#if selectedPersona.empresa_nombre || selectedPersona.empresaNombre}
											• {selectedPersona.empresa_nombre || selectedPersona.empresaNombre}
										{/if}
									</div>
								</div>
								<button
									type="button"
									onclick={clearSelection}
									class="p-1 text-gray-400 hover:text-white"
								>
									<X size={16} />
								</button>
							</div>
						{:else}
							<!-- Componente PersonaFinder -->
							<PersonaFinder on:select={handlePersonaSelect} autoFocus={true} />

							<!-- Entrada manual hint -->
							<p class="text-xs text-gray-500 mt-2">
								O ingresa los datos manualmente si la persona no está registrada
							</p>
						{/if}
					</div>
				{/if}

				<!-- Datos de la persona -->
				<div class="grid grid-cols-2 gap-4">
					<div>
						<label for="cedula" class={labelClass}>Cédula *</label>
						<input
							id="cedula"
							name="cedula"
							type="text"
							bind:value={$form.cedula}
							disabled={loading || isEditMode || !!selectedPersona}
							class={inputClass}
							placeholder={selectedPersona ? '' : '1-1234-5678'}
							{...$constraints.cedula}
						/>
						{#if $errors.cedula}<p class={errorClass}>{$errors.cedula}</p>{/if}
					</div>

					<div>
						<label for="empresaNombre" class={labelClass}>Empresa</label>
						<div class="relative">
							<Building2 size={16} class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500" />
							<input
								id="empresaNombre"
								name="empresaNombre"
								type="text"
								bind:value={$form.empresaNombre}
								disabled={loading || !!selectedPersona}
								class="{inputClass} pl-9"
								placeholder={selectedPersona ? '' : 'Nombre de empresa'}
								{...$constraints.empresaNombre}
							/>
						</div>
						{#if $errors.empresaNombre}<p class={errorClass}>
								{$errors.empresaNombre}
							</p>{/if}
					</div>

					<div>
						<label for="nombre" class={labelClass}>Nombre *</label>
						<input
							id="nombre"
							name="nombre"
							type="text"
							bind:value={$form.nombre}
							disabled={loading || !!selectedPersona}
							class={inputClass}
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
							class={inputClass}
							placeholder={selectedPersona ? '' : 'Carlos'}
							{...$constraints.segundoNombre}
						/>
						{#if $errors.segundoNombre}<p class={errorClass}>
								{$errors.segundoNombre}
							</p>{/if}
					</div>

					<div>
						<label for="apellido" class={labelClass}>Apellido *</label>
						<input
							id="apellido"
							name="apellido"
							type="text"
							bind:value={$form.apellido}
							disabled={loading || !!selectedPersona}
							class={inputClass}
							placeholder={selectedPersona ? '' : 'Pérez'}
							{...$constraints.apellido}
						/>
						{#if $errors.apellido}<p class={errorClass}>
								{$errors.apellido}
							</p>{/if}
					</div>

					<div>
						<label for="segundoApellido" class={labelClass}>Segundo Apellido</label>
						<input
							id="segundoApellido"
							name="segundoApellido"
							type="text"
							bind:value={$form.segundoApellido}
							disabled={loading || !!selectedPersona}
							class={inputClass}
							placeholder={selectedPersona ? '' : 'González'}
							{...$constraints.segundoApellido}
						/>
						{#if $errors.segundoApellido}<p class={errorClass}>
								{$errors.segundoApellido}
							</p>{/if}
					</div>
				</div>

				<!-- Nivel de Severidad (Dropdown compacto) -->
				<div>
					<label for="nivelSeveridad" class={labelClass}>Nivel de Severidad *</label>
					<select
						id="nivelSeveridad"
						name="nivelSeveridad"
						bind:value={$form.nivelSeveridad}
						disabled={loading}
						class="{inputClass} {$form.nivelSeveridad === 'ALTO'
							? 'border-red-500/50 text-red-400'
							: $form.nivelSeveridad === 'MEDIO'
								? 'border-yellow-500/50 text-yellow-400'
								: 'border-gray-500/50 text-gray-400'}"
						{...$constraints.nivelSeveridad}
					>
						<option value="ALTO">🔴 ALTO - Crítico</option>
						<option value="MEDIO">🟡 MEDIO - Moderado</option>
						<option value="BAJO">⚪ BAJO - Bajo riesgo</option>
					</select>
					{#if $errors.nivelSeveridad}<p class={errorClass}>
							{$errors.nivelSeveridad}
						</p>{/if}
				</div>

				<!-- Motivo (Opcional) -->
				<div>
					<label for="motivoBloqueo" class={labelClass}>Motivo del Bloqueo</label>
					<textarea
						id="motivoBloqueo"
						name="motivoBloqueo"
						bind:value={$form.motivoBloqueo}
						disabled={loading}
						class={inputClass}
						rows="2"
						placeholder="Describa el motivo del bloqueo (opcional)"
						{...$constraints.motivoBloqueo}
					></textarea>
					{#if $errors.motivoBloqueo}<p class={errorClass}>
							{$errors.motivoBloqueo}
						</p>{/if}
				</div>

				<!-- Buttons -->
				<div class="flex gap-3 pt-4 border-t border-gray-700">
					<button
						type="button"
						onclick={onClose}
						class="flex-1 py-2.5 px-4 rounded-md border border-gray-600 text-gray-300 hover:bg-gray-800 transition-colors"
					>
						Cancelar
					</button>
					<button
						type="submit"
						disabled={loading}
						class="flex-1 py-2.5 px-4 rounded-md bg-red-600 text-white font-medium hover:bg-red-700 disabled:opacity-50 transition-colors"
					>
						{loading ? 'Guardando...' : isEditMode ? 'Guardar Cambios' : 'Agregar a Lista Negra'}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}
