<script lang="ts">
	import { onMount } from 'svelte';
	import { toast } from 'svelte-5-french-toast';
	import { AlertCircle, History, CheckCircle, Search, Filter } from 'lucide-svelte';
	import { shortcutRegistry, setActiveContext } from '$lib/shortcuts';
	import type { AlertaGafeteResponse } from '$lib/types/ingreso';
	import { getAllAlertas, resolverAlerta } from '$lib/logic/alertaGafete/alertaGafeteService';
	import { getAlertaGafeteColumns } from '$lib/logic/alertaGafete/alertaGafeteColumns';
	import TabulatorWrapper from '$lib/components/tabulator/TabulatorWrapper.svelte';
	import GridToolbar from '$lib/components/tabulator/GridToolbar.svelte';
	import ResolveAlertModal from './modals/ResolveAlertModal.svelte';
	import { defaultTabulatorOptions } from '$lib/logic/tabulator/tabulatorController';

	let alerts = $state<AlertaGafeteResponse[]>([]);
	let loading = $state(false);
	let error = $state<string | null>(null);
	let showResolved = $state(false);
	let gridWrapper = $state<any>(null);

	// Modal State
	let showResolveModal = $state(false);
	let selectedAlerta = $state<AlertaGafeteResponse | null>(null);
	let formLoading = $state(false);

	// Grid Toolbar State
	let searchTerm = $state('');
	let toolbarColumns = $state<any[]>([]);
	let showHeaderFilters = $state(
		typeof window !== 'undefined'
			? localStorage.getItem('tabulator-header-filters') === 'true'
			: false
	);

	async function loadAlerts() {
		loading = true;
		error = null;
		try {
			// If not showing resolved, only pull pending (false)
			// If showing resolved, pull all (undefined)
			const res = await getAllAlertas(showResolved ? true : false);
			if (res.ok) {
				alerts = res.data;
				if (gridWrapper) gridWrapper.replaceData(alerts);
			} else {
				error = res.error;
				toast.error(res.error);
			}
		} finally {
			loading = false;
		}
	}

	function handleResolve(alerta: AlertaGafeteResponse) {
		selectedAlerta = alerta;
		showResolveModal = true;
	}

	async function handleResolveSubmit(notas: string) {
		if (!selectedAlerta) return;

		loading = true;
		formLoading = true;
		try {
			const res = await resolverAlerta(selectedAlerta.id, notas);
			if (res.ok) {
				toast.success('Alerta resuelta correctamente');
				showResolveModal = false;
				selectedAlerta = null;
				loadAlerts();
			} else {
				toast.error(res.error);
			}
		} finally {
			loading = false;
			formLoading = false;
		}
	}

	function handleSearch(term: string) {
		searchTerm = term;
		if (gridWrapper) {
			const table = gridWrapper.getTable();
			if (term) {
				table.setFilter([
					[
						{ field: 'nombreCompleto', type: 'like', value: term },
						{ field: 'cedula', type: 'like', value: term },
						{ field: 'gafeteNumero', type: 'like', value: term }
					]
				]);
			} else {
				table.clearFilter();
			}
		}
	}

	function handleToggleFilters() {
		showHeaderFilters = !showHeaderFilters;
		if (typeof window !== 'undefined') {
			localStorage.setItem('tabulator-header-filters', String(showHeaderFilters));
		}
		if (gridWrapper) {
			setTimeout(() => {
				gridWrapper.redraw(true);
			}, 50);
		}
	}

	let columns = $derived(
		getAlertaGafeteColumns({
			onResolve: handleResolve,
			hideActions: showResolved
		})
	);

	onMount(() => {
		loadAlerts();
		// Activar scope al montar
		shortcutRegistry.setScope('list');
		setActiveContext('alerta-gafete-list');
	});

	$effect(() => {
		if (showResolved !== undefined) {
			loadAlerts();
		}
	});
	function getTipoAsignacion(alerta: AlertaGafeteResponse): string {
		if (alerta.ingresoContratistaId) return 'Contratista';
		if (alerta.ingresoProveedorId) return 'Proveedor';
		if (alerta.ingresoVisitaId) return 'Visita';
		return 'General';
	}
</script>

<div class="flex h-full flex-col bg-surface-1">
	<!-- Grid Toolbar -->
	<GridToolbar
		bind:searchTerm
		onSearch={handleSearch}
		hasSelection={false}
		onAutoSizeColumns={() => gridWrapper?.autoSizeColumns()}
		onFitColumns={() => gridWrapper?.fitColumns()}
		onToggleColumn={(field) => gridWrapper?.toggleColumn(field)}
		onToggleFreeze={(field) => gridWrapper?.toggleFreeze(field)}
		onToggleFilters={handleToggleFilters}
		columns={toolbarColumns}
	>
		{#snippet secondaryActions()}
			<div class="flex items-center p-1 bg-surface-3 rounded-lg border border-surface ml-2">
				<button
					onclick={() => (showResolved = false)}
					class="px-3 py-1 text-[10px] font-bold uppercase rounded-md transition-all {!showResolved
						? 'bg-surface-1 text-primary shadow-sm'
						: 'text-secondary hover:text-primary'}"
				>
					Pendientes
				</button>
				<button
					onclick={() => (showResolved = true)}
					class="px-3 py-1 text-[10px] font-bold uppercase rounded-md transition-all {showResolved
						? 'bg-surface-1 text-primary shadow-sm'
						: 'text-secondary hover:text-primary'}"
				>
					Historial
				</button>
			</div>
		{/snippet}
	</GridToolbar>

	<div class="flex-1 overflow-hidden relative {showHeaderFilters ? '' : 'hide-filters'}">
		{#if loading && alerts.length === 0}
			<div
				class="absolute inset-0 flex items-center justify-center bg-surface-1/50 backdrop-blur-sm z-10"
			>
				<div class="loading loading-spinner loading-lg text-primary"></div>
			</div>
		{/if}

		{#if error}
			<div class="p-8 text-center">
				<p class="text-red-400 font-bold mb-2">Error al cargar alertas</p>
				<p class="text-secondary text-sm">{error}</p>
				<button
					onclick={loadAlerts}
					class="mt-4 px-4 py-2 bg-surface-2 hover:bg-surface-3 text-primary border border-surface rounded-md text-sm transition-colors"
				>
					Reintentar
				</button>
			</div>
		{:else}
			<TabulatorWrapper
				bind:this={gridWrapper}
				bind:toolbarColumns
				data={alerts}
				{columns}
				options={{
					...defaultTabulatorOptions,
					placeholder: showResolved
						? 'No hay historial de alertas registrado'
						: 'No hay alertas pendientes de gafetes',
					height: '100%',
					layout: 'fitColumns'
				}}
				persistenceID="gafete-alerts-grid-v1"
			/>
		{/if}
	</div>
</div>

{#if showResolveModal && selectedAlerta}
	<ResolveAlertModal
		show={showResolveModal}
		gafeteNumero={selectedAlerta.gafeteNumero.toString()}
		nombrePersona={selectedAlerta.nombreCompleto}
		cedulaPersona={selectedAlerta.cedula}
		empresaPersona={selectedAlerta.empresaNombre}
		fechaReporte={selectedAlerta.fechaReporte}
		tipoAsignacion={getTipoAsignacion(selectedAlerta)}
		reportadoPor={selectedAlerta.reportadoPorNombre}
		loading={formLoading}
		onResolve={handleResolveSubmit}
		onCancel={() => {
			showResolveModal = false;
			selectedAlerta = null;
		}}
	/>
{/if}

<style>
	:global(.hide-filters .tabulator-header-filter) {
		display: none !important;
	}
</style>
