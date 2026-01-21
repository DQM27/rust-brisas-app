<script lang="ts">
	import { onMount } from 'svelte';
	import { preRegistroVisitaService } from '$lib/services/preRegistroVisitaService';
	import type { PreRegistroVisita } from '$lib/types/ingreso-nuevos';
	import PreRegistroFormModal from './PreRegistroFormModal.svelte';

	let pendientes: PreRegistroVisita[] = [];
	let loading = true;
	let showCreateModal = false;

	onMount(() => {
		loadPendientes();
	});

	async function loadPendientes() {
		loading = true;
		try {
			pendientes = await preRegistroVisitaService.getPendientes();
		} catch (error) {
			console.error(error);
		} finally {
			loading = false;
		}
	}

	async function handleCancel(id: string) {
		if (!confirm('¿Estás seguro de cancelar esta visita esperada?')) return;
		try {
			await preRegistroVisitaService.cancel(id);
			await loadPendientes();
		} catch (error) {
			console.error(error);
			alert('Error al cancelar');
		}
	}
</script>

<div class="flex flex-col h-full bg-surface-1 rounded-xl border border-surface-3 overflow-hidden">
	<!-- Toolbar -->
	<div class="p-4 border-b border-surface-3 bg-surface-2 flex justify-between items-center">
		<h3 class="font-bold text-lg text-primary flex items-center gap-2">
			📅 Visitas Esperadas (Pre-Registros)
			<span class="bg-surface-3 text-sm px-2 py-0.5 rounded-full text-secondary"
				>{pendientes.length}</span
			>
		</h3>
		<button
			on:click={() => (showCreateModal = true)}
			class="px-4 py-2 bg-primary text-primary-content text-sm font-bold rounded-lg hover:bg-primary-hover hover:scale-[1.02] active:scale-[0.98] transition-all shadow-lg shadow-primary/20 flex items-center gap-2"
		>
			➕ Nuevo Pre-Registro
		</button>
	</div>

	<!-- Lista -->
	<div class="flex-1 overflow-y-auto p-4">
		{#if loading}
			<div class="flex justify-center items-center h-full text-secondary">Cargando...</div>
		{:else if pendientes.length === 0}
			<div class="flex flex-col justify-center items-center h-full text-secondary gap-2 opacity-60">
				<span class="text-4xl">📭</span>
				<p>No hay visitas esperadas pendientes.</p>
			</div>
		{:else}
			<div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
				{#each pendientes as p (p.id)}
					<div
						class="bg-surface-2 p-4 rounded-lg border border-surface-3 hover:border-primary/50 transition-colors flex justify-between items-start group relative overflow-hidden"
					>
						<!-- Decoración de fecha -->
						<div class="absolute left-0 top-0 bottom-0 w-1 bg-accent"></div>

						<div class="pl-2">
							<div class="flex items-baseline gap-2 mb-1">
								<span class="font-bold text-lg text-white">{p.nombre} {p.apellido}</span>
								<span class="text-xs px-1.5 py-0.5 bg-surface-3 rounded text-secondary font-mono"
									>{p.cedula}</span
								>
							</div>

							{#if p.empresaNombre}
								<div class="text-sm text-secondary mb-2 flex items-center gap-1">
									🏢 {p.empresaNombre}
								</div>
							{/if}

							<div class="grid grid-cols-2 gap-x-6 gap-y-1 text-xs text-secondary/80 mt-3">
								<div>
									<span class="uppercase text-[10px] font-bold tracking-wider opacity-70"
										>Fecha</span
									>
									<div class="text-primary font-medium">
										{new Date(p.fechaEsperada).toLocaleDateString()}
									</div>
								</div>
								<div>
									<span class="uppercase text-[10px] font-bold tracking-wider opacity-70"
										>Anfitrión</span
									>
									<div class="text-white">{p.anfitrion}</div>
								</div>
								<div class="col-span-2 mt-1">
									<span class="uppercase text-[10px] font-bold tracking-wider opacity-70"
										>Motivo / Área</span
									>
									<div class="text-white">{p.motivo} en {p.areaVisitada}</div>
								</div>
							</div>
						</div>

						<div class="flex flex-col gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
							<button
								on:click={() => handleCancel(p.id)}
								title="Cancelar Pre-Registro"
								class="p-2 hover:bg-error/20 text-error rounded transition-colors"
							>
								🗑️
							</button>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</div>
</div>

<PreRegistroFormModal
	isOpen={showCreateModal}
	on:close={() => (showCreateModal = false)}
	on:success={loadPendientes}
/>
