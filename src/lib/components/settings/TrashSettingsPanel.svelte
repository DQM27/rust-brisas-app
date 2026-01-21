<!-- src/lib/components/settings/TrashSettingsPanel.svelte -->
<script lang="ts">
	import { Trash2, HardHat, Package, User } from 'lucide-svelte';
	import TrashListView from '$lib/components/trash/TrashListView.svelte';
	import { contratistas as contratistaService } from '$lib/api/contratista';
	import * as proveedorService from '$lib/api/proveedor';
	import * as visitanteService from '$lib/api/visitante';
	import { getContratistaTrashColumns } from '$lib/logic/contratista/contratistaColumns';
	import { getProveedorTrashColumns } from '$lib/logic/proveedor/proveedorColumns';
	import { getVisitanteTrashColumns } from '$lib/logic/visitante/visitanteColumns';

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

<div class="h-full flex flex-col bg-surface-1">
	<!-- Header Minimalista con Toggle a la Derecha -->
	<div class="border-b border-surface px-6 py-4 bg-surface-2">
		<div class="flex items-center justify-between">
			<div>
				<h2 class="text-xl font-semibold text-primary">Papelera de Reciclaje</h2>
				<p class="mt-1 text-sm text-secondary">
					Gestiona y restaura elementos eliminados del sistema
				</p>
			</div>

			<!-- Toggle de Pestañas (Derecha) -->
			<div class="relative flex items-center bg-surface-3 p-1 rounded-lg">
				{#each tabs as tab}
					<button
						onclick={() => (activeTab = tab.id)}
						class="flex items-center gap-2 px-4 py-2 rounded-md text-sm font-medium transition-colors {activeTab ===
						tab.id
							? 'bg-surface-1 text-primary shadow-sm'
							: 'text-secondary hover:text-primary'}"
					>
						<tab.icon size={16} class={activeTab === tab.id ? tab.color : ''} />
						{tab.label}
					</button>
				{/each}
			</div>
		</div>
	</div>

	<!-- Content Area (Full Width/Height) -->
	<div class="flex-1 overflow-hidden relative">
		{#if activeTab === 'contratista'}
			<div class="absolute inset-0">
				<TrashListView
					entityName="Contratista"
					service={contratistaService}
					columnDefs={getContratistaTrashColumns()}
					gridId="trash-contratista"
					onBack={() => {}}
				/>
			</div>
		{:else if activeTab === 'proveedor'}
			<div class="absolute inset-0">
				<TrashListView
					entityName="Proveedor"
					service={proveedorService.proveedor}
					columnDefs={getProveedorTrashColumns()}
					gridId="trash-proveedor"
					onBack={() => {}}
				/>
			</div>
		{:else if activeTab === 'visitante'}
			<div class="absolute inset-0">
				<TrashListView
					entityName="Visitante"
					service={visitanteService.visitante}
					columnDefs={getVisitanteTrashColumns()}
					gridId="trash-visitante"
					onBack={() => {}}
				/>
			</div>
		{/if}
	</div>
</div>



