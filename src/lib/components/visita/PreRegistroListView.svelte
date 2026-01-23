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
		LogIn,
		GanttChart,
		Table2
	} from 'lucide-svelte';
	import SvarGanttView from './SvarGanttView.svelte';
	import SvarGridView from './SvarGridView.svelte';

	interface Props {
		showHeader?: boolean;
		viewMode?: 'grid' | 'focus' | 'svar-gantt' | 'svar-grid';
	}
	let { showHeader = true, viewMode = $bindable('svar-grid') }: Props = $props();

	let pendientes: PreRegistroVisita[] = $state([]);
	let loading = $state(true);
	let showCreateModal = $state(false);

	// Ingreso Modal State
	let showIngresoModal = $state(false);
	let selectedPreRegistro: PreRegistroVisita | null = $state(null);

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

	export function openCreateModal() {
		showCreateModal = true;
	}

	export function setViewMode(mode: typeof viewMode) {
		viewMode = mode;
	}

	export const getViewMode = () => viewMode;
</script>

<div
	class="flex flex-col h-full bg-surface-1 {showHeader
		? 'rounded-xl border border-surface'
		: ''} overflow-hidden"
>
	<!-- Toolbar -->
	{#if showHeader}
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
			<div class="flex items-center gap-3">
				<div
					class="flex items-center bg-surface-3 rounded-lg p-1 border border-surface shadow-inner mr-2"
				>
					<button
						onclick={() => (viewMode = 'svar-gantt')}
						class="p-1.5 rounded-md transition-all {viewMode === 'svar-gantt'
							? 'bg-primary text-white shadow-md'
							: 'text-secondary hover:bg-surface-1'}"
						title="SVAR Gantt"
					>
						<GanttChart size={16} />
					</button>
					<button
						onclick={() => (viewMode = 'svar-grid')}
						class="p-1.5 rounded-md transition-all {viewMode === 'svar-grid'
							? 'bg-primary text-white shadow-md'
							: 'text-secondary hover:bg-surface-1'}"
						title="SVAR Data Grid"
					>
						<Table2 size={16} />
					</button>
				</div>
				<button
					onclick={() => (showCreateModal = true)}
					class="flex items-center gap-2 px-4 py-2 bg-primary/10 text-primary border border-primary/20 rounded-lg hover:bg-primary/20 hover:border-primary/40 transition-all font-medium text-sm shadow-sm"
				>
					<Plus size={16} />
					<span>Nuevo Pre-Registro</span>
				</button>
			</div>
		</div>
	{/if}

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
		{:else if viewMode === 'svar-gantt'}
			<SvarGanttView {pendientes} />
		{:else if viewMode === 'svar-grid'}
			<SvarGridView {pendientes} onIngreso={handleIngreso} onCancel={handleCancel} />
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
