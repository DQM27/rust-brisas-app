<script lang="ts">
	import type { GridId, SettingsTab } from '$lib/types/agGrid';
	import type { GridApi } from '@ag-grid-community/core';
	import { X, RotateCcw } from 'lucide-svelte';
	import { slide, fade } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { agGridSettings } from '$lib/stores/agGridSettings.svelte';

	import AGGridSettingsAppearance from './settings/AGGridSettingsAppearance.svelte';
	import AGGridSettingsColumns from './settings/AGGridSettingsColumns.svelte';
	import AGGridSettingsToolbar from './settings/AGGridSettingsToolbar.svelte';
	import AGGridSettingsData from './settings/AGGridSettingsData.svelte';
	import AGGridSettingsAdvanced from './settings/AGGridSettingsAdvanced.svelte';
	import AGGridSettingsBackup from './settings/AGGridSettingsBackup.svelte';
	import { Palette, Columns, Wrench, Database, Settings, HardDrive } from 'lucide-svelte';

	interface Props {
		gridId: GridId;
		gridApi: GridApi | null;
		customButtons?: {
			default?: any[];
			singleSelect?: any[];
			multiSelect?: any[];
		};
		onClose: () => void;
	}

	let { gridId, gridApi, customButtons, onClose }: Props = $props();

	// For backup-list grid, start on backup tab
	// svelte-ignore state_referenced_locally
	const isBackupGrid = gridId === 'backup-list';
	let activeTab = $state<SettingsTab>(isBackupGrid ? 'data' : 'appearance');
	let isClosing = $state(false);
	let showResetConfirm = $state(false);

	// Dynamic tabs based on gridId
	const baseTabs: { id: SettingsTab; label: string; icon: any }[] = [
		{ id: 'appearance', label: 'Apariencia', icon: Palette },
		{ id: 'columns', label: 'Columnas', icon: Columns },
		{ id: 'toolbar', label: 'Toolbar', icon: Wrench },
		{ id: 'data', label: 'Datos', icon: Database },
		{ id: 'advanced', label: 'Avanzado', icon: Settings }
	];

	// For backup grid, replace "data" with "backup" tab
	const tabs = $derived(
		isBackupGrid
			? [
					{ id: 'data' as SettingsTab, label: 'Backup', icon: HardDrive },
					...baseTabs.filter((t) => t.id !== 'data')
				]
			: baseTabs
	);

	function handleClose() {
		isClosing = true;
		setTimeout(onClose, 150);
	}

	function handleBackdropClick(e: MouseEvent) {
		if (e.target === e.currentTarget) {
			handleClose();
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			if (showResetConfirm) {
				showResetConfirm = false;
			} else {
				handleClose();
			}
		}
	}

	function handleResetAll() {
		agGridSettings.resetToDefaults(gridId);
		if (gridApi) {
			gridApi.resetColumnState();
			gridApi.setFilterModel(null);
		}
		showResetConfirm = false;
	}
</script>

<svelte:window on:keydown={handleKeydown} />

<div
	class="fixed inset-0 z-50 flex items-center justify-center p-4
    {isClosing ? 'opacity-0' : 'opacity-100'} transition-opacity duration-150"
	onclick={handleBackdropClick}
	role="dialog"
	aria-modal="true"
	tabindex="-1"
	onkeydown={(e) => e.key === 'Escape' && handleClose()}
>
	<!-- Backdrop -->
	<div class="absolute inset-0 bg-black/60"></div>

	<!-- Modal (GitHub Style) -->
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div
		class="relative w-full max-w-3xl max-h-[85vh] flex flex-col
      bg-zinc-900 border border-zinc-800 rounded-lg shadow-2xl
      {isClosing ? 'scale-95' : 'scale-100'} transition-transform duration-150"
		onclick={(e) => e.stopPropagation()}
		role="document"
		tabindex="-1"
		onkeydown={(e) => e.stopPropagation()}
	>
		<!-- Header -->
		<div class="flex items-center justify-between px-4 py-3 border-b border-zinc-800">
			<h2 class="text-sm font-semibold text-white">Configuración de Grid</h2>
			<button
				onclick={handleClose}
				class="p-1 text-zinc-400 hover:text-white rounded transition-colors"
				aria-label="Cerrar"
			>
				<X size={18} />
			</button>
		</div>

		<!-- Content -->
		<div class="flex flex-1 overflow-hidden">
			<!-- Sidebar Navigation -->
			<nav class="w-52 border-r border-zinc-800 py-2 overflow-y-auto">
				{#each tabs as tab}
					<button
						onclick={() => (activeTab = tab.id)}
						class="w-full flex items-center gap-3 px-4 py-2 text-left text-sm transition-colors
              {activeTab === tab.id
							? 'bg-zinc-800 text-white border-l-2 border-l-blue-500'
							: 'text-zinc-400 hover:bg-zinc-800/50 hover:text-white border-l-2 border-l-transparent'}"
					>
						<tab.icon size={16} class={activeTab === tab.id ? 'text-blue-500' : 'text-zinc-500'} />
						{tab.label}
					</button>
				{/each}

				<!-- Reset Button in Sidebar -->
				<div class="mt-4 px-3">
					<button
						onclick={() => (showResetConfirm = true)}
						class="w-full flex items-center justify-center gap-2 px-3 py-2 text-xs
              text-red-500 hover:bg-red-500/10 border border-red-500/30 rounded-md transition-colors"
					>
						<RotateCcw size={14} />
						Restaurar todo
					</button>
				</div>
			</nav>

			<!-- Tab Content -->
			<div class="flex-1 overflow-y-auto p-6 bg-zinc-950">
				{#if activeTab === 'appearance'}
					<AGGridSettingsAppearance {gridId} {gridApi} />
				{:else if activeTab === 'columns'}
					<AGGridSettingsColumns {gridId} {gridApi} />
				{:else if activeTab === 'toolbar'}
					<AGGridSettingsToolbar {gridId} {gridApi} {customButtons} />
				{:else if activeTab === 'data'}
					{#if isBackupGrid}
						<AGGridSettingsBackup />
					{:else}
						<AGGridSettingsData {gridId} {gridApi} />
					{/if}
				{:else if activeTab === 'advanced'}
					<AGGridSettingsAdvanced {gridId} {gridApi} />
				{/if}
			</div>
		</div>

		<!-- Footer -->
		<div class="flex items-center justify-between px-4 py-3 border-t border-zinc-800 bg-zinc-900">
			<p class="text-xs text-zinc-500 w-full text-center">Los cambios se aplican automáticamente</p>
		</div>
	</div>

	<!-- Reset Confirmation (GitHub Style) -->
	{#if showResetConfirm}
		<div
			transition:fade={{ duration: 100 }}
			class="absolute inset-0 z-10 flex items-center justify-center bg-black/70"
			onclick={() => (showResetConfirm = false)}
			role="button"
			tabindex="0"
			onkeydown={(e) => e.key === 'Escape' && (showResetConfirm = false)}
		>
			<div
				transition:slide={{ duration: 150, easing: cubicOut }}
				class="bg-zinc-900 border border-zinc-800 rounded-lg p-4 max-w-sm mx-4 shadow-2xl"
				onclick={(e) => e.stopPropagation()}
				role="dialog"
				aria-modal="true"
				tabindex="-1"
				onkeydown={(e) => e.stopPropagation()}
			>
				<h3 class="text-sm font-semibold text-white mb-2">¿Restaurar configuración?</h3>
				<p class="text-xs text-zinc-400 mb-4">
					Esto restablecerá todas las opciones de esta grid a sus valores por defecto. Esta acción
					no se puede deshacer.
				</p>
				<div class="flex justify-end gap-2">
					<button
						onclick={() => (showResetConfirm = false)}
						class="px-3 py-1.5 text-sm text-zinc-400 hover:text-white transition-colors"
					>
						Cancelar
					</button>
					<button
						onclick={handleResetAll}
						class="px-3 py-1.5 text-sm font-medium text-white bg-red-600 hover:bg-red-500
              rounded-md transition-colors"
					>
						Restaurar
					</button>
				</div>
			</div>
		</div>
	{/if}
</div>
