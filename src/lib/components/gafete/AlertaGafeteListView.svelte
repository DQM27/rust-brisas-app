<script lang="ts">
	import { onMount } from 'svelte';
	import { toast } from 'svelte-5-french-toast';
	import { AlertCircle, History, CheckCircle, Search, Filter } from 'lucide-svelte';
	import type { AlertaGafeteResponse } from '$lib/types/ingreso';
	import { getAllAlertas, resolverAlerta } from '$lib/logic/alertaGafete/alertaGafeteService';
	import { getAlertaGafeteColumns } from '$lib/logic/alertaGafete/alertaGafeteColumns';
	import TabulatorWrapper from '$lib/components/tabulator/TabulatorWrapper.svelte';
	import { defaultTabulatorOptions } from '$lib/logic/tabulator/tabulatorController';

	let alerts = $state<AlertaGafeteResponse[]>([]);
	let loading = $state(false);
	let error = $state<string | null>(null);
	let showResolved = $state(false);
	let gridWrapper = $state<any>(null);

	async function loadAlerts() {
		loading = true;
		error = null;
		try {
			// If not showing resolved, only pull pending (false)
			// If showing resolved, pull all (undefined)
			const res = await getAllAlertas(showResolved ? undefined : false);
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

	async function handleResolve(alerta: AlertaGafeteResponse) {
		const notas = prompt('Notas de resolución (ej. Devolvió gafete, Pagó multa):', '');
		if (notas === null) return;

		loading = true;
		try {
			const res = await resolverAlerta(alerta.id, notas);
			if (res.ok) {
				toast.success('Alerta resuelta correctamente');
				loadAlerts();
			} else {
				toast.error(res.error);
			}
		} finally {
			loading = false;
		}
	}

	let columns = $derived(
		getAlertaGafeteColumns({
			onResolve: handleResolve
		})
	);

	onMount(() => {
		loadAlerts();
	});

	$effect(() => {
		if (showResolved !== undefined) {
			loadAlerts();
		}
	});
</script>

<div class="flex h-full flex-col bg-surface-1">
	<div class="p-4 border-b border-surface bg-surface-2 flex items-center justify-between gap-4">
		<div class="flex items-center gap-2">
			<div class="p-2 bg-red-500/10 rounded-lg">
				<AlertCircle class="text-red-400" size={20} />
			</div>
			<div>
				<h3 class="text-base font-bold text-primary">Gestión de Alertas de Gafetes</h3>
				<p class="text-xs text-secondary">Control de incidencias, pérdidas y retornos pendientes</p>
			</div>
		</div>

		<div class="flex items-center gap-3">
			<div class="flex items-center p-1 bg-surface-3 rounded-lg border border-surface">
				<button
					onclick={() => (showResolved = false)}
					class="px-3 py-1.5 text-xs font-bold uppercase rounded-md transition-all {!showResolved
						? 'bg-surface-1 text-primary shadow-sm'
						: 'text-secondary hover:text-primary'}"
				>
					Pendientes
				</button>
				<button
					onclick={() => (showResolved = true)}
					class="px-3 py-1.5 text-xs font-bold uppercase rounded-md transition-all {showResolved
						? 'bg-surface-1 text-primary shadow-sm'
						: 'text-secondary hover:text-primary'}"
				>
					Todas / Historial
				</button>
			</div>

			<button
				onclick={loadAlerts}
				class="p-2 text-secondary hover:text-primary transition-colors"
				title="Refrescar datos"
			>
				<History size={18} />
			</button>
		</div>
	</div>

	<div class="flex-1 overflow-hidden relative">
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
