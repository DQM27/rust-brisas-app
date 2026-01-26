<script lang="ts">
	import { onMount } from 'svelte';
	import { fade, fly } from 'svelte/transition';
	import { toast } from 'svelte-5-french-toast';
	import {
		AlertCircle,
		Database,
		Download,
		Upload,
		Trash2,
		ArchiveRestore,
		Key,
		X
	} from 'lucide-svelte';
	// Components
	import TabulatorWrapper from '$lib/components/tabulator/TabulatorWrapper.svelte';
	import GridToolbar from '$lib/components/tabulator/GridToolbar.svelte';

	// Services
	import {
		listBackups,
		deleteBackup,
		restoreFromAutoBackup,
		restorePortableBackup,
		cleanupOldBackups,
		getBackupConfig,
		updateBackupConfig,
		backupDatabaseAuto,
		backupDatabasePortable,
		backupDatabase,
		restoreDatabase
	} from '$lib/services/backupService';
	import { message, confirm } from '@tauri-apps/plugin-dialog';

	// Types
	import type { BackupEntry, BackupConfig } from '$lib/types/backup';

	// Logic
	import { getBackupColumns } from '$lib/logic/backup/backupColumns';
	import { defaultTabulatorOptions } from '$lib/logic/tabulator/tabulatorController';

	// Stores
	import { currentUser } from '$lib/stores/auth';
	import { can } from '$lib/logic/permissions';

	// ==========================================
	// ESTADO LOCAL
	// ==========================================
	let backups = $state<BackupEntry[]>([]);
	let loading = $state(false);
	let error = $state('');

	// Config
	let config = $state<BackupConfig | null>(null);
	let configEnabled = $state(false);
	let configHora = $state('02:00');
	let configDiasRetencion = $state(30);
	// Grid State
	let gridWrapper = $state<any>(null);
	let toolbarColumns = $state<any[]>([]);
	let searchTerm = $state('');
	// Selección
	let selectedRows = $state<BackupEntry[]>([]);
	// Filtros Header
	let showHeaderFilters = $state(false);

	// Modal de contraseña para backups portables
	let showPasswordModal = $state(false);
	let passwordInput = $state('');
	let passwordMode = $state<'create' | 'restore'>('create');
	let pendingRestoreEntry = $state<BackupEntry | null>(null);
	let isCreatingPortable = $state(false);

	// Permisos
	const canUpdate = $derived($currentUser && can($currentUser, 'UPDATE_SETTINGS_BACKUP'));

	// ==========================================
	// COLUMNAS
	// ==========================================
	const columns = $derived(
		getBackupColumns({
			onRestore: handleRestore,
			onDelete: handleDelete
		})
	);

	function handleToggleFilters() {
		showHeaderFilters = !showHeaderFilters;
		if (gridWrapper) {
			setTimeout(() => gridWrapper.redraw(true), 50);
		}
	}

	// ==========================================
	// HANDLERS - DATA
	// ==========================================
	async function loadBackups() {
		loading = true;
		error = '';
		try {
			const [backupList, backupConfig] = await Promise.all([listBackups(), getBackupConfig()]);
			backups = backupList;
			config = backupConfig;

			if (config) {
				configEnabled = config.enabled;
				configHora = config.hora;
				configDiasRetencion = config.diasRetencion;
			}
		} catch (err) {
			console.error('Error loading backups:', err);
			error = String(err);
		}
		loading = false;
	}

	// ==========================================
	// HANDLERS - BACKUP
	// ==========================================
	async function handleBackupNow() {
		const toastId = toast.loading('Creando backup...');
		try {
			const filename = await backupDatabaseAuto();
			toast.success(`Backup creado: ${filename}`, { id: toastId });
			await loadBackups();
		} catch (err) {
			console.error('Error creating backup:', err);
			toast.error(`Error: ${err}`, { id: toastId });
		}
	}

	async function handleBackupManual() {
		try {
			await backupDatabase();
		} catch (err) {
			console.error('Error in manual backup:', err);
		}
	}

	async function handleRestoreFromFile() {
		try {
			await restoreDatabase();
		} catch (err) {
			console.error('Error restoring from file:', err);
		}
	}

	// ==========================================
	// HANDLERS - BACKUP PORTABLE
	// ==========================================
	function handlePortableBackup() {
		passwordMode = 'create';
		passwordInput = '';
		showPasswordModal = true;
	}

	async function handlePasswordSubmit() {
		if (passwordInput.length < 8) {
			toast.error('La contraseña debe tener al menos 8 caracteres');
			return;
		}

		showPasswordModal = false;

		if (passwordMode === 'create') {
			isCreatingPortable = true;
			const toastId = toast.loading('Creando backup portable...');
			try {
				const filename = await backupDatabasePortable(passwordInput);
				toast.success(`Backup portable creado: ${filename}`, { id: toastId });
				await message(
					'⚠️ Guarda la contraseña en un lugar seguro.\nSin ella no podrás restaurar este backup.',
					{
						title: 'Backup Portable Creado',
						kind: 'warning'
					}
				);
				await loadBackups();
			} catch (err) {
				console.error('Error creating portable backup:', err);
				toast.error(`Error: ${err}`, { id: toastId });
			} finally {
				isCreatingPortable = false;
			}
		} else if (passwordMode === 'restore' && pendingRestoreEntry) {
			const toastId = toast.loading('Restaurando backup portable...');
			try {
				await restorePortableBackup(pendingRestoreEntry.nombre, passwordInput);
			} catch (err) {
				console.error('Error restoring portable backup:', err);
				toast.error(`Error: ${err}`, { id: toastId });
			} finally {
				pendingRestoreEntry = null;
			}
		}

		passwordInput = '';
	}

	// ==========================================
	// HANDLERS - RESTORE FROM GRID
	// ==========================================
	async function handleRestore(entry: BackupEntry) {
		// Si es portable, pedir contraseña primero
		if (entry.encryptionType === 'portable') {
			pendingRestoreEntry = entry;
			passwordMode = 'restore';
			passwordInput = '';
			showPasswordModal = true;
			return;
		}

		try {
			await restoreFromAutoBackup(entry.nombre);
		} catch (err) {
			console.error('Error restoring backup:', err);
			await message(`Error al restaurar: ${err}`, {
				title: 'Error',
				kind: 'error'
			});
		}
	}

	// ==========================================
	// HANDLERS - DELETE
	// ==========================================
	async function handleDelete(entry: BackupEntry) {
		const confirmed = await confirm(
			`¿Eliminar "${entry.nombre}"?\n\nEsta acción no se puede deshacer.`,
			{ title: 'Confirmar Eliminación', kind: 'warning' }
		);
		if (!confirmed) return;

		const toastId = toast.loading('Eliminando...');
		try {
			await deleteBackup(entry.nombre);
			toast.success('Backup eliminado', { id: toastId });
			await loadBackups();
		} catch (err) {
			console.error('Error deleting backup:', err);
			toast.error(`Error: ${err}`, { id: toastId });
		}
	}

	async function handleDeleteMultiple() {
		const confirmed = await confirm(
			`¿Eliminar ${selectedRows.length} backups?\n\nEsta acción no se puede deshacer.`,
			{ title: 'Confirmar Eliminación', kind: 'warning' }
		);
		if (!confirmed) return;

		const toastId = toast.loading('Eliminando...');
		let errors = 0;
		for (const entry of selectedRows) {
			try {
				await deleteBackup(entry.nombre);
			} catch {
				errors++;
			}
		}

		if (errors === 0) {
			toast.success('Backups eliminados', { id: toastId });
		} else {
			toast.error(`${errors} errores`, { id: toastId });
		}
		await loadBackups();
	}

	// ==========================================
	// LIFECYCLE
	// ==========================================
	onMount(() => {
		loadBackups();
	});
</script>

<div class="flex h-full flex-col relative bg-surface-1">
	<!-- Header -->
	<div class="border-b border-surface px-6 py-4 bg-surface-2">
		<div class="flex items-center gap-4">
			<div>
				<h2 class="text-xl font-semibold text-primary flex items-center gap-2">
					<Database class="w-5 h-5 text-purple-500" />
					Copias de Seguridad
				</h2>
				<p class="mt-1 text-sm text-secondary">
					Gestión de backups automáticos y manuales • Usa la toolbar para gestionar tus archivos
				</p>
			</div>
		</div>
	</div>

	<!-- Toolbar -->
	<GridToolbar
		bind:searchTerm
		hasSelection={selectedRows.length > 0}
		selectionCount={selectedRows.length}
		onAutoSizeColumns={() => gridWrapper?.autoSizeColumns()}
		onFitColumns={() => gridWrapper?.fitColumns()}
		onToggleColumn={(field) => gridWrapper?.toggleColumn(field)}
		onToggleFreeze={(field) => gridWrapper?.toggleFreeze(field)}
		onToggleFilters={handleToggleFilters}
		columns={toolbarColumns}
	>
		{#snippet primaryActions()}
			{#if selectedRows.length > 0}
				<button
					onclick={() => gridWrapper?.deselectAll()}
					class="flex items-center gap-1.5 px-3 py-1.5 bg-surface-3 text-secondary border border-surface rounded-md hover:bg-surface-4 hover:text-primary text-sm font-medium transition-colors"
				>
					<X size={14} /> Cancelar
				</button>
				<button
					onclick={handleDeleteMultiple}
					class="flex items-center gap-1.5 px-3 py-1.5 bg-red-500/10 text-red-400 border border-red-500/20 rounded-md hover:bg-red-500/20 text-sm font-medium transition-colors"
				>
					<Trash2 size={14} /> Eliminar seleccionado
				</button>
			{:else}
				<button
					onclick={handleBackupNow}
					class="flex items-center gap-1.5 px-3 py-1.5 bg-green-500/10 text-green-400 border border-green-500/20 rounded-md hover:bg-green-500/20 text-sm font-medium transition-colors"
				>
					<Database size={14} /> Crear Backup
				</button>
				<button
					onclick={handlePortableBackup}
					class="flex items-center gap-1.5 px-3 py-1.5 bg-blue-500/10 text-blue-400 border border-blue-500/20 rounded-md hover:bg-blue-500/20 text-sm font-medium transition-colors"
				>
					<Key size={14} /> Portable
				</button>
				<button
					onclick={handleBackupManual}
					class="flex items-center gap-1.5 px-3 py-1.5 bg-surface-3 text-secondary border border-surface rounded-md hover:bg-surface-4 hover:text-primary text-sm font-medium transition-colors"
				>
					<Download size={14} /> Exportar
				</button>
				<button
					onclick={handleRestoreFromFile}
					class="flex items-center gap-1.5 px-3 py-1.5 bg-surface-3 text-secondary border border-surface rounded-md hover:bg-surface-4 hover:text-primary text-sm font-medium transition-colors"
				>
					<Upload size={14} /> Importar
				</button>
			{/if}
		{/snippet}
	</GridToolbar>

	<!-- Content -->
	<div
		class="flex-1 overflow-hidden relative bg-surface-1 {showHeaderFilters ? '' : 'hide-filters'}"
	>
		{#if error}
			<div class="p-6">
				<div
					class="flex items-center gap-3 rounded-lg border border-red-500/20 bg-red-500/10 p-4 text-red-400"
					transition:fade
				>
					<AlertCircle size={20} />
					<div>
						<div class="font-medium">Error al cargar backups</div>
						<div class="text-sm opacity-90">{error}</div>
					</div>
				</div>
			</div>
		{:else if loading && backups.length === 0}
			<div class="flex h-full items-center justify-center">
				<div class="loading loading-spinner loading-lg text-primary opacity-20"></div>
			</div>
		{:else if backups.length === 0}
			<div class="flex h-full items-center justify-center">
				<div class="text-center">
					<Database size={48} class="mx-auto text-secondary opacity-20" />
					<p class="mt-4 text-lg font-medium text-secondary">No hay backups</p>
					<p class="mt-2 text-sm text-secondary opacity-60">
						Crea tu primer backup usando los botones de arriba
					</p>
				</div>
			</div>
		{:else}
			<TabulatorWrapper
				bind:this={gridWrapper}
				bind:toolbarColumns
				data={backups}
				{columns}
				withCheckboxSelection={true}
				onRowSelectionChanged={(data) => (selectedRows = data)}
				persistenceID="backup-list-v2"
				options={{
					...defaultTabulatorOptions,
					layout: 'fitData',
					placeholder: 'No se encontraron backups'
				}}
			/>
		{/if}
	</div>
</div>

{#if showPasswordModal}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center p-4"
		transition:fade={{ duration: 100 }}
	>
		<button
			class="absolute inset-0 bg-black/70 backdrop-blur-sm border-0 cursor-default"
			onclick={() => {
				showPasswordModal = false;
				passwordInput = '';
			}}
			aria-label="Cerrar"
		></button>

		<div
			class="relative z-10 w-full max-w-sm p-6 rounded-lg bg-[#252526] border border-white/10 shadow-2xl"
			transition:fly={{ y: 10, duration: 150 }}
		>
			<div class="flex items-center gap-2 mb-4">
				<Key class="w-5 h-5 text-purple-500" />
				<h3 class="text-base font-semibold text-white">
					{passwordMode === 'create' ? 'Crear Backup Portable' : 'Restaurar Backup'}
				</h3>
			</div>

			<p class="text-xs text-gray-400 mb-4">
				{#if passwordMode === 'create'}
					Ingresa una contraseña de al menos 8 caracteres para proteger el backup.
					<span class="text-amber-400 block mt-1"
						>⚠️ No podrás recuperar el backup sin esta contraseña.</span
					>
				{:else}
					Ingresa la contraseña del backup "{pendingRestoreEntry?.nombre}".
				{/if}
			</p>

			<input
				type="password"
				bind:value={passwordInput}
				placeholder="Contraseña (mín. 8 caracteres)"
				class="w-full px-3 py-2 text-sm rounded-md bg-[#1e1e1e] border border-white/10 text-white placeholder-gray-500 focus:border-purple-500 focus:outline-none mb-4"
				onkeydown={(e) => {
					if (e.key === 'Enter') handlePasswordSubmit();
				}}
			/>

			<div class="flex gap-2 justify-end">
				<button
					class="px-3 py-1.5 text-xs font-medium rounded-md bg-gray-700 hover:bg-gray-600 text-gray-200"
					onclick={() => {
						showPasswordModal = false;
						passwordInput = '';
					}}
				>
					Cancelar
				</button>
				<button
					class="px-3 py-1.5 text-xs font-medium rounded-md bg-purple-600 hover:bg-purple-700 text-white disabled:opacity-50"
					onclick={handlePasswordSubmit}
					disabled={passwordInput.length < 8}
				>
					{passwordMode === 'create' ? 'Crear Backup' : 'Restaurar'}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	:global(.hide-filters .tabulator-header-filter) {
		display: none !important;
	}
</style>
