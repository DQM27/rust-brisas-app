<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { toastService } from '$lib/services/toastService';
	import { UserPlus, RefreshCw, X, Calendar, Search, LogIn, Pencil, Trash2 } from 'lucide-svelte';
	import type { ColumnDefinition } from 'tabulator-tables';

	import TabulatorWrapper from '$lib/components/tabulator/TabulatorWrapper.svelte';
	import GridToolbar from '$lib/components/tabulator/GridToolbar.svelte';
	import PreRegistroFormModal from '$lib/components/visita/PreRegistroFormModal.svelte';
	import { preRegistroVisitaService } from '$lib/services/preRegistroVisitaService';
	import { defaultTabulatorOptions } from '$lib/logic/tabulator/tabulatorController';
	import type { PreRegistroVisita } from '$lib/types/ingreso-nuevos';
	import IngresoVisitaFormModal from '$lib/components/ingreso/IngresoVisitaFormModal.svelte';
	import { shortcutRegistry, setActiveContext } from '$lib/shortcuts';

	interface Props {
		showHeader?: boolean;
		showTitle?: boolean;
	}

	let { showHeader = true, showTitle = true }: Props = $props();

	let preRegistros = $state<PreRegistroVisita[]>([]);
	let filteredPreRegistros = $derived.by(() => {
		if (!searchTerm) return preRegistros;
		const term = searchTerm.toLowerCase();
		return preRegistros.filter((p) => {
			const fullName =
				`${p.nombre} ${p.apellido} ${p.segundoNombre || ''} ${p.segundoApellido || ''}`.toLowerCase();
			return (
				p.cedula?.toLowerCase().includes(term) ||
				fullName.includes(term) ||
				p.empresaNombre?.toLowerCase().includes(term) ||
				(p as any).empresa_nombre?.toLowerCase().includes(term) ||
				p.anfitrion?.toLowerCase().includes(term) ||
				p.motivo?.toLowerCase().includes(term)
			);
		});
	});
	let loading = $state(false);
	let searchTerm = $state('');
	let gridWrapper = $state<any>(null);
	let selectedRows = $state<any[]>([]);

	// Modal state
	let showCreateModal = $state(false);
	let showIngresoModal = $state(false);
	let editingPreRegistro = $state<PreRegistroVisita | null>(null);
	let selectedPreRegistroForIngreso = $state<PreRegistroVisita | null>(null);

	const columns: ColumnDefinition[] = [
		{ title: 'Cédula', field: 'cedula', width: 120 },
		{
			title: 'Visitante',
			field: 'nombre',
			formatter: (cell: any) => {
				const data = cell.getData();
				return `${data.nombre} ${data.apellido} ${data.segundoNombre || ''} ${data.segundoApellido || ''}`.trim();
			}
		},
		{
			title: 'Empresa',
			field: 'empresaNombre',
			width: 150,
			formatter: (cell: any) => {
				const data = cell.getData();
				return data.empresaNombre || data.empresa_nombre || '';
			}
		},
		{ title: 'Anfitrión', field: 'anfitrion', width: 140 },
		{
			title: 'Área Visitada',
			field: 'areaVisitada',
			width: 140,
			formatter: (cell: any) => {
				const data = cell.getData();
				return data.areaVisitada || data.area_visitada || '-';
			}
		},
		{
			title: 'Motivo',
			field: 'motivo',
			width: 160,
			formatter: (cell: any) => cell.getValue() || '-'
		},
		{
			title: 'Fecha Esperada',
			field: 'fechaEsperada',
			width: 140,
			formatter: (cell: any) => {
				const data = cell.getData();
				const val = data.fechaEsperada || data.fecha_esperada;
				if (!val) return '';
				// Si ya viene formateada como YYYY-MM-DD
				const bits = val.split('T')[0].split('-');
				if (bits.length === 3) return `${bits[2]}/${bits[1]}/${bits[0]}`;
				return new Date(val).toLocaleDateString();
			}
		},
		{
			title: 'Hora',
			field: 'horaEsperada',
			width: 80,
			formatter: (cell: any) => {
				const data = cell.getData();
				return data.horaEsperada || data.hora_esperada || '';
			}
		},
		{
			title: 'Estado',
			field: 'estado',
			width: 100,
			hozAlign: 'center' as 'center',
			formatter: (cell: any) => {
				const val = cell.getValue();
				let color = 'bg-gray-500';
				if (val === 'PENDIENTE') color = 'bg-yellow-500/20 text-yellow-500';
				if (val === 'COMPLETADO') color = 'bg-green-500/20 text-green-500';
				if (val === 'CANCELADO') color = 'bg-red-500/20 text-red-500';
				return `<span class="px-2 py-0.5 rounded text-xs font-semibold ${color}">${val}</span>`;
			}
		},
		{
			title: 'Acciones',
			width: 120,
			hozAlign: 'center' as 'center',
			headerSort: false,
			formatter: (cell: any) => {
				return `
					<div class="flex items-center justify-center gap-2">
						<button class="p-1.5 text-blue-400 hover:text-blue-300 transition-colors ingreso-btn" title="Ingresar">
							<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"/><polyline points="10 17 15 12 10 7"/><line x1="15" y1="12" x2="3" y2="12"/></svg>
						</button>
						<button class="p-1.5 text-red-400 hover:text-red-300 transition-colors cancel-btn" title="Cancelar">
							<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6 6 18"/><path d="m6 6 18 18"/></svg>
						</button>
					</div>
				`;
			},
			cellClick: (e: any, cell: any) => {
				e.stopPropagation();
				const target = e.target as HTMLElement;
				const btn = target.closest('button');
				if (btn?.classList.contains('ingreso-btn')) {
					handleIngreso(cell.getRow().getData());
				} else if (btn?.classList.contains('cancel-btn')) {
					handleCancelPreRegistro(cell.getRow().getData().id);
				}
			}
		}
	];

	// Grid State
	let toolbarColumns = $state<any[]>([]);
	let showHeaderFilters = $state(
		typeof window !== 'undefined'
			? localStorage.getItem('tabulator-header-filters') === 'true'
			: false
	);

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

	export async function loadData() {
		loading = true;
		try {
			preRegistros = await preRegistroVisitaService.getPendientes();
		} catch (error) {
			console.error(error);
			toastService.error('Error al cargar pre-registros');
		} finally {
			loading = false;
		}
	}

	// Expose methods if needed by parent
	export function setSearchTerm(term: string) {
		searchTerm = term;
	}

	// Normalizar ID de empresa para SurrealDB (tb:id)
	const normalizeId = (id: any): string => {
		if (!id) return '';
		if (typeof id === 'string') return id;
		if (typeof id === 'object' && id.tb && id.id) {
			const innerId =
				typeof id.id === 'object' ? id.id.String || id.id.id || JSON.stringify(id.id) : id.id;
			return `${id.tb}:${innerId}`;
		}
		return id.toString();
	};

	async function handleCancelPreRegistro(idRaw: any) {
		const id = normalizeId(idRaw);
		if (!id || !confirm('¿Estás seguro de cancelar este pre-registro?')) return;
		try {
			await preRegistroVisitaService.cancel(id);
			toastService.success('Pre-registro cancelado');
			loadData();
		} catch (e) {
			console.error(e);
			toastService.error('Error al cancelar');
		}
	}

	async function handleCancelMultiple(selection: any[]) {
		if (!confirm(`¿Estás seguro de cancelar ${selection.length} pre-registros?`)) return;
		const toastId = toastService.loading('Cancelando pre-registros...');
		let errors = 0;
		for (const p of selection) {
			try {
				const id = normalizeId(p.id);
				await preRegistroVisitaService.cancel(id);
			} catch (e) {
				errors++;
			}
		}
		if (errors === 0) {
			toastService.success('Pre-registros cancelados', { id: toastId });
		} else {
			toastService.error(`Error en ${errors} cancelaciones`, { id: toastId });
		}
		loadData();
		gridWrapper?.deselectAll();
	}

	function handleRowDblClick(e: any, row: any) {
		editingPreRegistro = row.getData();
		showCreateModal = true;
	}

	function handleIngreso(preRegistro: PreRegistroVisita) {
		selectedPreRegistroForIngreso = preRegistro;
		showIngresoModal = true;
	}

	onMount(() => {
		loadData();
		// Activar scope de shortcuts
		shortcutRegistry.setScope('list');
		setActiveContext('preregistro-list');
	});

	// Auto-update toolbar columns when grid is ready or changes
	// This is handled by binding in TabulatorWrapper
</script>

<div class="flex flex-col h-full bg-surface-1">
	<GridToolbar
		bind:searchTerm
		hasSelection={selectedRows.length > 0}
		selectionCount={selectedRows.length}
		onAutoSizeColumns={() => gridWrapper?.autoSizeColumns()}
		onFitColumns={() => gridWrapper?.fitColumns()}
		onToggleColumn={(field) => gridWrapper?.toggleColumn(field)}
		onToggleFreeze={(field) => gridWrapper?.toggleFreeze(field)}
		onToggleFilters={() => handleToggleFilters()}
		columns={toolbarColumns}
	>
		{#snippet primaryActions()}
			{#if selectedRows.length > 0}
				<div class="flex items-center gap-2 animate-in fade-in slide-in-from-left-2 duration-200">
					<button
						onclick={() => gridWrapper?.deselectAll()}
						class="flex items-center gap-1.5 px-3 py-1.5 bg-surface-3 text-secondary border border-surface rounded-md hover:bg-surface-4 hover:text-primary text-sm font-medium transition-colors"
						title="Cancelar selección"
					>
						<X size={14} /> Cancelar
					</button>

					{#if selectedRows.length === 1}
						<button
							onclick={() => handleIngreso(selectedRows[0])}
							class="flex items-center gap-1.5 px-3 py-1.5 bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 rounded-md hover:bg-emerald-500/20 text-sm font-medium transition-colors"
							title="Registrar Ingreso"
						>
							<LogIn size={14} /> Ingresar
						</button>

						<button
							onclick={() => {
								editingPreRegistro = selectedRows[0];
								showCreateModal = true;
							}}
							class="flex items-center gap-1.5 px-3 py-1.5 bg-amber-500/10 text-amber-400 border border-amber-500/20 rounded-md hover:bg-amber-500/20 text-sm font-medium transition-colors"
							title="Editar Pre-Registro"
						>
							<Pencil size={14} /> Editar
						</button>
					{/if}

					<button
						onclick={() => handleCancelMultiple(selectedRows)}
						class="flex items-center gap-1.5 px-3 py-1.5 bg-red-500/10 text-red-400 border border-red-500/20 rounded-md hover:bg-red-500/20 text-sm font-medium transition-colors"
						title="Cancelar Pre-Registros"
					>
						<Trash2 size={14} />
						{selectedRows.length === 1 ? 'Cancelar' : `Cancelar (${selectedRows.length})`}
					</button>
				</div>
			{:else}
				<button
					onclick={() => {
						editingPreRegistro = null;
						showCreateModal = true;
					}}
					class="flex items-center gap-1.5 px-3 py-1.5 bg-blue-500/10 text-blue-400 border border-blue-500/20 rounded-md hover:bg-blue-500/20 text-sm font-medium transition-colors"
				>
					<UserPlus size={14} />
					{showTitle ? 'Nuevo Pre-Registro' : 'Nuevo'}
				</button>
			{/if}
		{/snippet}
	</GridToolbar>

	<div class="flex-1 overflow-hidden p-0 relative {showHeaderFilters ? '' : 'hide-filters'}">
		{#if loading && preRegistros.length === 0}
			<div
				class="absolute inset-0 flex items-center justify-center bg-surface-1/50 z-10 backdrop-blur-[1px]"
			>
				<span class="loading loading-spinner loading-md text-primary"></span>
			</div>
		{/if}

		<TabulatorWrapper
			bind:this={gridWrapper}
			bind:toolbarColumns
			data={filteredPreRegistros}
			{columns}
			class="h-full"
			pagination={true}
			layout="fitColumns"
			onRowDblClick={handleRowDblClick}
			withCheckboxSelection={true}
			onRowSelectionChanged={(data) => (selectedRows = data)}
			persistenceID="pre-registros-grid-v5"
			options={{
				...defaultTabulatorOptions,
				placeholder: 'No hay pre-registros pendientes'
			}}
		/>
	</div>
</div>

<PreRegistroFormModal
	bind:show={showCreateModal}
	initialData={editingPreRegistro}
	onClose={() => (showCreateModal = false)}
	on:success={() => {
		loadData();
		showCreateModal = false;
	}}
/>

<IngresoVisitaFormModal
	bind:show={showIngresoModal}
	initialPerson={selectedPreRegistroForIngreso}
	onComplete={() => {
		loadData();
		showIngresoModal = false;
	}}
/>

<style>
	:global(.hide-filters .tabulator-header-filter) {
		display: none !important;
	}
</style>
