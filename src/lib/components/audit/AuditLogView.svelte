<script lang="ts">
	import { onMount, tick } from 'svelte';
	import { auditService, type SysLogEntry } from '$lib/services/auditService';
	import TabulatorWrapper from '$lib/components/tabulator/TabulatorWrapper.svelte';
	import { RefreshCw, Search } from 'lucide-svelte';

	let tableData: SysLogEntry[] = [];
	let isLoading = false;
	let searchText = '';

	// Configuración de columnas Tabulator
	const columns = [
		{
			title: 'Fecha',
			field: 'access_date',
			width: 180,
			sorter: 'datetime',
			formatter: 'datetime',
			formatterParams: {
				outputFormat: 'DD/MM/YYYY hh:mm a',
				invalidPlaceholder: '(Fecha Inválida)'
			}
		},
		{
			title: 'Usuario',
			field: 'user_name',
			headerFilter: 'input',
			widthGrow: 1
		},
		{
			title: 'Evento',
			field: 'event_type',
			width: 120,
			formatter: (cell: any) => {
				const val = cell.getValue();
				let color = 'bg-gray-500';
				if (val === 'LOGIN') color = 'bg-green-600';
				if (val === 'LOGOUT') color = 'bg-orange-500';
				if (val === 'TIMEOUT') color = 'bg-red-600';
				if (val === 'FAILED_LOGIN') color = 'bg-red-800';
				return `<span class="badge ${color} text-white px-2 py-1 rounded text-xs font-bold">${val}</span>`;
			},
			hozAlign: 'center'
		},
		{
			title: 'Terminal',
			field: 'terminal_name',
			width: 150,
			tooltip: (cell: any) => cell.getData().terminal_id
		},
		{
			title: 'Duración',
			field: 'duration',
			width: 100,
			hozAlign: 'right'
		},
		{
			title: 'IP',
			field: 'ip_address',
			width: 120
		},
		{
			title: 'Detalles',
			field: 'details',
			widthGrow: 2,
			formatter: 'textarea'
		}
	] as any[];

	async function loadLogs() {
		isLoading = true;
		try {
			// Cargar últimos 500 registros por defecto
			tableData = await auditService.getLogs(500, 0);
		} catch (error) {
			console.error('Error loading logs:', error);
		} finally {
			isLoading = false;
		}
	}

	onMount(() => {
		loadLogs();
	});

	// Filtro reactivo simple (si TabulatorWrapper no lo maneja nativamente)
	// En este caso, TabulatorWrapper suele manejar sus propios filtros si se configuran,
	// pero aquí usaremos el reload para simplicidad o filtrado local si la data es pequeña.
</script>

<div class="h-full flex flex-col p-4 space-y-4">
	<!-- Header -->
	<div
		class="flex justify-between items-center bg-surface-1 p-4 rounded-lg shadow-sm border border-surface-2"
	>
		<div>
			<h2 class="text-xl font-bold text-primary">Auditoría del Sistema</h2>
			<p class="text-sm text-secondary">Historial de accesos y eventos de seguridad</p>
		</div>

		<div class="flex space-x-2">
			<button
				class="btn-icon variant-ghost-surface"
				on:click={loadLogs}
				title="Actualizar"
				disabled={isLoading}
			>
				<RefreshCw size={20} class={isLoading ? 'animate-spin' : ''} />
			</button>
		</div>
	</div>

	<!-- Table Container -->
	<div
		class="flex-1 bg-surface-1 rounded-lg shadow-sm border border-surface-2 overflow-hidden relative"
	>
		{#if isLoading && tableData.length === 0}
			<div class="absolute inset-0 flex items-center justify-center bg-surface-1/50 z-10">
				<RefreshCw class="animate-spin text-primary" size={32} />
			</div>
		{/if}

		<TabulatorWrapper
			data={tableData}
			{columns}
			layout="fitColumns"
			height="100%"
			pagination={true}
			paginationSize={50}
			options={{
				initialSort: [{ column: 'access_date', dir: 'desc' }]
			}}
		/>
	</div>
</div>

<style>
	/* Estilos adicionales si fueran necesarios */
</style>
