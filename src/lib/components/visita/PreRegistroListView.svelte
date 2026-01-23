<script lang="ts">
	import { onMount } from 'svelte';
	import { preRegistroVisitaService } from '$lib/services/preRegistroVisitaService';
	import type { PreRegistroVisita } from '$lib/types/ingreso-nuevos';
	import PreRegistroFormModal from './PreRegistroFormModal.svelte';
	import IngresoVisitaFormModal from '../ingreso/IngresoVisitaFormModal.svelte';
	import {
		CalendarClock,
		Plus,
		Trash2,
		Building2,
		User,
		MapPin,
		MailOpen,
		LogIn
	} from 'lucide-svelte';

	let pendientes: PreRegistroVisita[] = [];
	let loading = true;
	let showCreateModal = false;

	// Ingreso Modal State
	let showIngresoModal = false;
	let selectedPreRegistro: PreRegistroVisita | null = null;

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

	function handleIngreso(preRegistro: PreRegistroVisita) {
		selectedPreRegistro = preRegistro;
		showIngresoModal = true;
	}
</script>

<div class="flex flex-col h-full bg-surface-1 rounded-xl border border-surface overflow-hidden">
	<!-- Toolbar -->
	<div class="px-6 py-4 border-b border-surface bg-surface-2 flex justify-between items-center">
		<h3 class="font-bold text-lg text-primary flex items-center gap-2.5">
			<div class="p-2 rounded-lg bg-surface-3 text-accent">
				<CalendarClock size={20} />
			</div>
			Visitas Esperadas
			<span
				class="bg-surface-3 text-xs font-mono px-2 py-0.5 rounded-full text-secondary border border-surface shadow-sm"
				>{pendientes.length}</span
			>
		</h3>
		<button
			onclick={() => (showCreateModal = true)}
			class="flex items-center gap-2 px-4 py-2 bg-primary/10 text-primary border border-primary/20 rounded-lg hover:bg-primary/20 hover:border-primary/40 transition-all font-medium text-sm shadow-sm"
		>
			<Plus size={16} />
			<span>Nuevo Pre-Registro</span>
		</button>
	</div>

	<!-- Lista -->
	<div class="flex-1 overflow-y-auto p-6 bg-black/20">
		{#if loading}
			<div class="flex justify-center items-center h-full text-secondary gap-2">
				<div class="loading loading-spinner loading-md"></div>
				<span class="text-sm">Cargando datos...</span>
			</div>
		{:else if pendientes.length === 0}
			<div class="flex flex-col justify-center items-center h-full text-secondary gap-4 opacity-60">
				<div class="p-6 rounded-full bg-surface-2 border border-surface">
					<MailOpen size={48} strokeWidth={1} />
				</div>
				<p class="text-sm font-medium">No hay visitas esperadas pendientes.</p>
			</div>
		{:else}
			<div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
				{#each pendientes as p (p.id)}
					<div
						class="bg-surface-2 p-5 rounded-xl border border-surface hover:border-accent/50 hover:shadow-lg hover:shadow-accent/5 transition-all flex justify-between items-start group relative overflow-hidden"
					>
						<!-- Decoración de fecha -->
						<div
							class="absolute left-0 top-0 bottom-0 w-1 bg-gradient-to-b from-accent to-purple-500 opacity-80"
						></div>

						<div class="pl-3 flex-1">
							<div class="flex items-center gap-2 mb-2">
								<span class="font-bold text-lg text-white tracking-tight"
									>{p.nombre} {p.apellido}</span
								>
								<span
									class="text-[10px] px-1.5 py-0.5 bg-surface-3 rounded border border-surface text-secondary font-mono tracking-wide"
									>{p.cedula}</span
								>
							</div>

							{#if p.empresaNombre}
								<div class="text-xs text-secondary mb-3 flex items-center gap-1.5 font-medium">
									<Building2 size={12} class="text-tertiary" />
									{p.empresaNombre}
								</div>
							{/if}

							<div
								class="grid grid-cols-2 gap-x-4 gap-y-3 text-xs text-secondary/80 mt-2 border-t border-surface/50 pt-3"
							>
								<div>
									<span
										class="uppercase text-[9px] font-bold tracking-wider opacity-60 flex items-center gap-1 mb-0.5"
									>
										<CalendarClock size={10} /> Fecha
									</span>
									<div class="text-accent font-medium text-sm">
										{new Date(p.fechaEsperada).toLocaleDateString(undefined, {
											weekday: 'short',
											day: 'numeric',
											month: 'short'
										})}
									</div>
								</div>
								<div>
									<span
										class="uppercase text-[9px] font-bold tracking-wider opacity-60 flex items-center gap-1 mb-0.5"
									>
										<User size={10} /> Anfitrión
									</span>
									<div class="text-white font-medium">{p.anfitrion}</div>
								</div>
								<div class="col-span-2">
									<span
										class="uppercase text-[9px] font-bold tracking-wider opacity-60 flex items-center gap-1 mb-0.5"
									>
										<MapPin size={10} /> Área / Motivo
									</span>
									<div class="text-gray-300">
										<span class="text-white">{p.areaVisitada}</span>
										<span class="text-surface mx-1">•</span>
										<span class="italic">{p.motivo}</span>
									</div>
								</div>
							</div>
						</div>

						<div
							class="flex flex-col gap-2 opacity-0 group-hover:opacity-100 transition-opacity absolute top-3 right-3"
						>
							<button
								onclick={() => handleIngreso(p)}
								title="Dar Ingreso"
								class="p-2 bg-accent/10 hover:bg-accent text-accent hover:text-white border border-accent/20 rounded-lg transition-all shadow-sm"
							>
								<LogIn size={16} />
							</button>
							<button
								onclick={() => handleCancel(p.id)}
								title="Cancelar Pre-Registro"
								class="p-2 hover:bg-error/10 text-secondary hover:text-error border border-transparent hover:border-error/20 rounded-lg transition-all"
							>
								<Trash2 size={16} />
							</button>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</div>
</div>

<PreRegistroFormModal
	bind:isOpen={showCreateModal}
	on:close={() => (showCreateModal = false)}
	on:success={loadPendientes}
/>

<IngresoVisitaFormModal
	bind:show={showIngresoModal}
	initialPerson={selectedPreRegistro}
	onComplete={() => {
		showIngresoModal = false;
		loadPendientes();
	}}
/>
