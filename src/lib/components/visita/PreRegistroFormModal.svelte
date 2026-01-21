<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { CreatePreRegistroInput } from '$lib/types/ingreso-nuevos';
	import { preRegistroVisitaService } from '$lib/services/preRegistroVisitaService';
	import { slide } from 'svelte/transition';

	export let isOpen = false;
	const dispatch = createEventDispatcher();

	let loading = false;
	let error = '';

	// Form data
	let cedula = '';
	let nombre = '';
	let apellido = '';
	let empresaNombre = '';
	let fechaEsperada = new Date().toISOString().split('T')[0]; // Hoy por defecto
	let anfitrion = '';
	let areaVisitada = '';
	let motivo = '';
	let modoIngreso = 'caminando';
	let observaciones = '';

	// TODO: Implementar búsqueda en catálogo para autocompletar nombre/empresa

	async function handleSubmit() {
		if (!cedula || !nombre || !apellido || !anfitrion || !areaVisitada || !motivo) {
			error = 'Por favor complete los campos obligatorios (*)';
			return;
		}

		loading = true;
		error = '';

		try {
			const input: CreatePreRegistroInput = {
				cedula,
				nombre,
				apellido,
				empresaNombre,
				fechaEsperada,
				anfitrion,
				areaVisitada,
				motivo,
				modoIngreso,
				observaciones
			};

			await preRegistroVisitaService.create(input);
			dispatch('success');
			close();
		} catch (e: any) {
			error = e.message || 'Error al crear pre-registro';
			console.error(e);
		} finally {
			loading = false;
		}
	}

	function close() {
		resetForm();
		dispatch('close');
	}

	function resetForm() {
		cedula = '';
		nombre = '';
		apellido = '';
		empresaNombre = '';
		// Mantener fecha, anfitrión, área y motivo puede ser útil si vas a registrar varios seguidos del mismo correo
		// fechaEsperada = ...
		// anfitrion = ...
		error = '';
	}
</script>

{#if isOpen}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm p-4"
		transition:slide|local
	>
		<div
			class="bg-surface-1 w-full max-w-2xl rounded-xl shadow-2xl overflow-hidden border border-surface-3 flex flex-col max-h-[90vh]"
		>
			<!-- Header -->
			<div class="p-4 border-b border-surface-3 bg-surface-2 flex justify-between items-center">
				<h2 class="text-lg font-bold text-primary flex items-center gap-2">
					📅 Nuevo Pre-Registro de Visita
				</h2>
				<button on:click={close} class="text-secondary hover:text-primary transition-colors">
					✕
				</button>
			</div>

			<!-- Body -->
			<div class="p-6 overflow-y-auto flex-1">
				{#if error}
					<div class="p-3 mb-4 bg-error/10 border border-error/20 text-error rounded-lg text-sm">
						{error}
					</div>
				{/if}

				<form on:submit|preventDefault={handleSubmit} class="space-y-4">
					<!-- Fila 1: Datos Personales -->
					<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
						<div class="space-y-1">
							<label class="text-xs font-medium text-secondary uppercase">Cédula *</label>
							<input
								type="text"
								bind:value={cedula}
								class="w-full bg-surface-2 border border-surface-3 rounded-lg px-3 py-2 text-sm focus:border-primary focus:ring-1 focus:ring-primary outline-none transition-all"
								placeholder="ID Visitante"
								autofocus
							/>
						</div>
						<div class="space-y-1">
							<label class="text-xs font-medium text-secondary uppercase">Nombre *</label>
							<input
								type="text"
								bind:value={nombre}
								class="w-full bg-surface-2 border border-surface-3 rounded-lg px-3 py-2 text-sm focus:border-primary focus:ring-1 focus:ring-primary outline-none transition-all"
								placeholder="Nombre"
							/>
						</div>
						<div class="space-y-1">
							<label class="text-xs font-medium text-secondary uppercase">Apellido *</label>
							<input
								type="text"
								bind:value={apellido}
								class="w-full bg-surface-2 border border-surface-3 rounded-lg px-3 py-2 text-sm focus:border-primary focus:ring-1 focus:ring-primary outline-none transition-all"
								placeholder="Apellido"
							/>
						</div>
					</div>

					<!-- Fila 2: Empresa y Fecha -->
					<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
						<div class="space-y-1">
							<label class="text-xs font-medium text-secondary uppercase">Empresa (Opcional)</label>
							<input
								type="text"
								bind:value={empresaNombre}
								class="w-full bg-surface-2 border border-surface-3 rounded-lg px-3 py-2 text-sm focus:border-primary focus:ring-1 focus:ring-primary outline-none transition-all"
								placeholder="Empresa visitante"
							/>
						</div>
						<div class="space-y-1">
							<label class="text-xs font-medium text-secondary uppercase">Fecha Esperada *</label>
							<input
								type="date"
								bind:value={fechaEsperada}
								class="w-full bg-surface-2 border border-surface-3 rounded-lg px-3 py-2 text-sm focus:border-primary focus:ring-1 focus:ring-primary outline-none transition-all"
							/>
						</div>
					</div>

					<div class="border-t border-surface-3 my-4"></div>
					<h3 class="text-sm font-semibold text-primary mb-2">Datos de la Visita</h3>

					<!-- Fila 3: Detalles Visita -->
					<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
						<div class="space-y-1">
							<label class="text-xs font-medium text-secondary uppercase">Anfitrión *</label>
							<input
								type="text"
								bind:value={anfitrion}
								class="w-full bg-surface-2 border border-surface-3 rounded-lg px-3 py-2 text-sm focus:border-primary focus:ring-1 focus:ring-primary outline-none transition-all"
								placeholder="¿A quién visita?"
							/>
						</div>
						<div class="space-y-1">
							<label class="text-xs font-medium text-secondary uppercase">Área Visitada *</label>
							<input
								type="text"
								bind:value={areaVisitada}
								class="w-full bg-surface-2 border border-surface-3 rounded-lg px-3 py-2 text-sm focus:border-primary focus:ring-1 focus:ring-primary outline-none transition-all"
								placeholder="Ej. Administración, Planta..."
							/>
						</div>
					</div>

					<!-- Fila 4: Motivo y Modo -->
					<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
						<div class="space-y-1">
							<label class="text-xs font-medium text-secondary uppercase">Motivo *</label>
							<input
								type="text"
								bind:value={motivo}
								class="w-full bg-surface-2 border border-surface-3 rounded-lg px-3 py-2 text-sm focus:border-primary focus:ring-1 focus:ring-primary outline-none transition-all"
								placeholder="Ej. Reunión, Entrega..."
							/>
						</div>
						<div class="space-y-1">
							<label class="text-xs font-medium text-secondary uppercase">Modo Ingreso</label>
							<select
								bind:value={modoIngreso}
								class="w-full bg-surface-2 border border-surface-3 rounded-lg px-3 py-2 text-sm focus:border-primary focus:ring-1 focus:ring-primary outline-none transition-all appearance-none"
							>
								<option value="caminando">🚶 Caminando</option>
								<option value="vehiculo">🚗 Vehículo</option>
							</select>
						</div>
					</div>
				</form>
			</div>

			<!-- Footer -->
			<div class="p-4 border-t border-surface-3 bg-surface-2 flex justify-end gap-3">
				<button
					on:click={close}
					class="px-4 py-2 text-sm font-medium text-secondary hover:text-primary transition-colors"
				>
					Cancelar
				</button>
				<button
					on:click={handleSubmit}
					disabled={loading}
					class="px-6 py-2 bg-primary text-primary-content text-sm font-bold rounded-lg hover:bg-primary-hover transition-colors shadow-lg shadow-primary/20 disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
				>
					{#if loading}
						<span class="animate-spin">⏳</span> Guardando...
					{:else}
						💾 Guardar Pre-Registro
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}
