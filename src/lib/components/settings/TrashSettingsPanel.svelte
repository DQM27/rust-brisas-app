<!-- src/lib/components/settings/TrashSettingsPanel.svelte -->
<script lang="ts">
	import { Trash2, HardHat, Package, User } from 'lucide-svelte';
	import TrashListView from '$lib/components/trash/TrashListView.svelte';
	import { contratistas as contratistaService } from '$lib/api/contratista';
	import * as proveedorService from '$lib/api/proveedor';
	import * as visitanteService from '$lib/api/visitante';
	import { ContratistaColumns } from '$lib/logic/contratista/contratistaColumns';
	import { ProveedorColumns } from '$lib/logic/proveedor/proveedorColumns';
	import { VisitanteColumns } from '$lib/logic/visitante/visitanteColumns';
	import { slide } from 'svelte/transition';

	// Estado de pestañas
	let activeTab = $state<'contratista' | 'proveedor' | 'visitante'>('contratista');

	const tabs = [
		{
			id: 'contratista',
			label: 'Contratistas',
			icon: HardHat,
			color: 'text-orange-500'
		},
		{
			id: 'proveedor',
			label: 'Proveedores',
			icon: Package,
			color: 'text-blue-500'
		},
		{
			id: 'visitante',
			label: 'Visitantes',
			icon: User,
			color: 'text-green-500'
		}
	] as const;
</script>

<div class="h-full flex flex-col">
	<!-- Header & Tabs Section -->
	<div class="flex flex-col space-y-4 p-6 pb-4">
		<!-- Header -->
		<div class="flex items-center gap-3">
			<div class="p-2 bg-red-500/10 rounded-lg">
				<Trash2 class="w-6 h-6 text-red-500" />
			</div>
			<div>
				<h2 class="text-xl font-semibold text-gray-900 dark:text-gray-100">
					Papelera de Reciclaje
				</h2>
				<p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
					Gestiona y restaura elementos eliminados del sistema
				</p>
			</div>
		</div>

		<!-- Tabs -->
		<div class="flex gap-1 bg-gray-100 dark:bg-[#161b22] p-1 rounded-lg w-fit">
			{#each tabs as tab}
				<button
					onclick={() => (activeTab = tab.id)}
					class="flex items-center gap-2 px-4 py-2 text-sm font-medium rounded-md transition-all
            {activeTab === tab.id
						? 'bg-white dark:bg-[#21262d] text-gray-900 dark:text-gray-100 shadow-sm'
						: 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200'}"
				>
					<tab.icon size={16} class={activeTab === tab.id ? tab.color : ''} />
					{tab.label}
				</button>
			{/each}
		</div>
	</div>

	<!-- Content Area (Full Width/Height) -->
	<div
		class="flex-1 bg-white dark:bg-[#0d1117] border-t border-gray-200 dark:border-gray-700 overflow-hidden relative"
	>
		{#if activeTab === 'contratista'}
			<div class="absolute inset-0" transition:slide={{ axis: 'x', duration: 200 }}>
				<TrashListView
					entityName="Contratista"
					service={contratistaService}
					columnDefs={ContratistaColumns.getTrashColumns()}
					gridId="trash-contratista"
					onBack={() => {}}
				/>
			</div>
		{:else if activeTab === 'proveedor'}
			<div class="absolute inset-0" transition:slide={{ axis: 'x', duration: 200 }}>
				<TrashListView
					entityName="Proveedor"
					service={proveedorService.proveedor}
					columnDefs={ProveedorColumns.getTrashColumns()}
					gridId="trash-proveedor"
					onBack={() => {}}
				/>
			</div>
		{:else if activeTab === 'visitante'}
			<div class="absolute inset-0" transition:slide={{ axis: 'x', duration: 200 }}>
				<TrashListView
					entityName="Visitante"
					service={visitanteService.visitante}
					columnDefs={VisitanteColumns.getTrashColumns()}
					gridId="trash-visitante"
					onBack={() => {}}
				/>
			</div>
		{/if}
	</div>
</div>
