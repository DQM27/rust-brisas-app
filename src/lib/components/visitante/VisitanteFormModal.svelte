<!-- src/lib/components/visitante/VisitanteFormModal.svelte -->
<script lang="ts">
	import { fade, fly } from 'svelte/transition';
	import { X } from 'lucide-svelte';
	import { onMount } from 'svelte';
	import type {
		VisitanteResponse,
		CreateVisitanteInput,
		UpdateVisitanteInput
	} from '$lib/types/visitante';
	import { submitFetchActiveEmpresas } from '$lib/logic/empresa/empresaService';

	interface Props {
		show: boolean;
		visitante?: VisitanteResponse | null;
		loading?: boolean;
		onSave: (data: CreateVisitanteInput | UpdateVisitanteInput) => Promise<boolean | void>;
		onClose: () => void;
	}

	let { show, visitante = null, loading = false, onSave, onClose }: Props = $props();

	const isEditMode = $derived(!!visitante);
	const modalTitle = $derived(
		isEditMode ? `Editar: ${visitante?.nombre} ${visitante?.apellido}` : 'Nuevo Visitante'
	);

	let formData = $state({
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
	});

	let empresas = $state<{ id: string; nombre: string }[]>([]);
	let loadingEmpresas = $state(false);
	const isFormValid = $derived(
		formData.cedula.trim() &&
			formData.nombre.trim() &&
			formData.apellido.trim() &&
			(!formData.hasVehicle || (formData.tipoVehiculo && formData.placa.trim()))
	);

	onMount(async () => {
		await loadEmpresas();
	});

	async function loadEmpresas() {
		loadingEmpresas = true;
		const res = await submitFetchActiveEmpresas();
		if (res.ok) {
			empresas = res.empresas;
		}
		loadingEmpresas = false;
	}

	$effect(() => {
		if (show && visitante) {
			formData = {
				cedula: visitante.cedula,
				nombre: visitante.nombre,
				segundoNombre: visitante.segundoNombre || '',
				apellido: visitante.apellido,
				segundoApellido: visitante.segundoApellido || '',
				empresaId: visitante.empresaId || '',
				hasVehicle: visitante.hasVehicle,
				tipoVehiculo: (visitante as any).tipoVehiculo || '', // Assuming these might come from backend if we update response
				placa: (visitante as any).placa || '', // We might need to handle this properly if response doesn't have it explicitly mapped yet
				marca: (visitante as any).marca || '',
				modelo: (visitante as any).modelo || '',
				color: (visitante as any).color || ''
			};
		} else if (show && !visitante) {
			resetForm();
		}
	});

	function resetForm() {
		formData = {
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
	}

	async function handleSubmit(e: Event) {
		e.preventDefault();
		if (!isFormValid) return;

		const payload: CreateVisitanteInput = {
			cedula: formData.cedula,
			nombre: formData.nombre,
			apellido: formData.apellido,
			segundoNombre: formData.segundoNombre || undefined,
			segundoApellido: formData.segundoApellido || undefined,
			empresaId: formData.empresaId || undefined,
			hasVehicle: formData.hasVehicle,
			tipoVehiculo: formData.hasVehicle ? formData.tipoVehiculo : undefined,
			placa: formData.hasVehicle ? formData.placa : undefined,
			marca: formData.hasVehicle ? formData.marca : undefined,
			modelo: formData.hasVehicle ? formData.modelo : undefined,
			color: formData.hasVehicle ? formData.color : undefined
		};

		const success = await onSave(payload);
		if (success) {
			onClose();
		}
	}

	function handleClose() {
		if (!loading) onClose();
	}

	const labelClass = 'text-xs font-medium text-gray-700 dark:text-gray-300';
	const inputClass =
		'w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] px-3 py-2 text-sm text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e] focus:border-transparent focus:outline-none transition-all placeholder-gray-400';
</script>

{#if show}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center p-4 text-gray-100"
		transition:fade={{ duration: 200 }}
	>
		<button
			type="button"
			class="absolute inset-0 bg-black/60 w-full h-full cursor-default"
			onclick={handleClose}
			aria-label="Cerrar modal"
		></button>

		<div
			class="relative z-10 w-full max-w-md overflow-hidden rounded-lg bg-[#0d1117] shadow-2xl border border-gray-700"
			transition:fly={{ y: 20, duration: 300 }}
		>
			<!-- Header -->
			<div
				class="flex items-center justify-between px-6 py-4 border-b border-gray-700 bg-[#161b22]"
			>
				<h2 class="text-lg font-semibold">{modalTitle}</h2>
				<button onclick={handleClose} class="text-gray-400 hover:text-white">
					<X class="w-5 h-5" />
				</button>
			</div>

			<form onsubmit={handleSubmit} class="p-6 space-y-4">
				<div class="space-y-1">
					<label class={labelClass} for="cedula">Cédula *</label>
					<input
						id="cedula"
						bind:value={formData.cedula}
						class={inputClass}
						disabled={isEditMode}
					/>
				</div>

				<div class="grid grid-cols-2 gap-4">
					<div class="space-y-1">
						<label class={labelClass} for="nombre">Nombre *</label>
						<input id="nombre" bind:value={formData.nombre} class={inputClass} />
					</div>
					<div class="space-y-1">
						<label class={labelClass} for="apellido">Apellido *</label>
						<input id="apellido" bind:value={formData.apellido} class={inputClass} />
					</div>
				</div>

				<div class="grid grid-cols-2 gap-4">
					<div class="space-y-1">
						<label class={labelClass} for="segundoNombre">Segundo Nombre</label>
						<input id="segundoNombre" bind:value={formData.segundoNombre} class={inputClass} />
					</div>
					<div class="space-y-1">
						<label class={labelClass} for="segundoApellido">Segundo Apellido</label>
						<input id="segundoApellido" bind:value={formData.segundoApellido} class={inputClass} />
					</div>
				</div>

				<div class="space-y-1">
					<label class={labelClass} for="empresaId">Empresa (Opcional)</label>
					<select id="empresaId" bind:value={formData.empresaId} class={inputClass}>
						<option value="">Ninguna / Independiente</option>
						{#each empresas as emp}
							<option value={emp.id}>{emp.nombre}</option>
						{/each}
					</select>
				</div>

				<div class="flex items-center gap-2 pt-2">
					<input
						type="checkbox"
						id="hasVehicle"
						bind:checked={formData.hasVehicle}
						class="checkbox checkbox-primary checkbox-sm"
					/>
					<label for="hasVehicle" class="text-sm cursor-pointer">¿Tiene vehículo?</label>
				</div>

				{#if formData.hasVehicle}
					<div class="p-4 rounded border border-gray-700 bg-black/20 space-y-3">
						<div class="grid grid-cols-2 gap-2">
							<button
								type="button"
								onclick={() => (formData.tipoVehiculo = 'motocicleta')}
								class="py-2 px-3 rounded border text-sm font-medium transition-all {formData.tipoVehiculo ===
								'motocicleta'
									? 'border-[#2da44e] bg-[#2da44e]/10 text-[#2da44e]'
									: 'border-gray-600 text-gray-400 hover:border-gray-500'}"
							>
								🏍️ Moto
							</button>
							<button
								type="button"
								onclick={() => (formData.tipoVehiculo = 'automovil')}
								class="py-2 px-3 rounded border text-sm font-medium transition-all {formData.tipoVehiculo ===
								'automovil'
									? 'border-[#2da44e] bg-[#2da44e]/10 text-[#2da44e]'
									: 'border-gray-600 text-gray-400 hover:border-gray-500'}"
							>
								🚗 Auto
							</button>
						</div>

						<div class="space-y-1">
							<label class={labelClass} for="placa">Placa *</label>
							<input id="placa" bind:value={formData.placa} class="{inputClass} uppercase" />
						</div>

						<div class="grid grid-cols-2 gap-3">
							<div class="space-y-1">
								<label class={labelClass} for="marca">Marca</label>
								<input id="marca" bind:value={formData.marca} class={inputClass} />
							</div>
							<div class="space-y-1">
								<label class={labelClass} for="modelo">Modelo</label>
								<input id="modelo" bind:value={formData.modelo} class={inputClass} />
							</div>
						</div>

						<div class="space-y-1">
							<label class={labelClass} for="color">Color</label>
							<input id="color" bind:value={formData.color} class={inputClass} />
						</div>
					</div>
				{/if}

				<div class="flex justify-end gap-3 pt-4">
					<button
						type="button"
						onclick={handleClose}
						class="px-4 py-2 rounded border border-gray-600 hover:bg-gray-800">Cancelar</button
					>
					<button
						type="submit"
						disabled={!isFormValid || loading}
						class="px-4 py-2 rounded bg-[#2da44e] hover:bg-[#2c974b] text-white disabled:opacity-50"
					>
						{loading ? 'Guardando...' : isEditMode ? 'Actualizar' : 'Crear'}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}
