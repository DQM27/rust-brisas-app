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
		Key
	} from 'lucide-svelte';
	import type { ColDef } from '@ag-grid-community/core';

	// Components
	import AGGridWrapper from '$lib/components/grid/AGGridWrapper.svelte';

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
	import type { CustomToolbarButton } from '$lib/types/agGrid';

	// Logic
	import { BackupColumns } from '$lib/logic/backup/backupColumns';

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

	// Selección
	let selectedRows = $state<BackupEntry[]>([]);

	// Modal de contraseña para backups portables
	let showPasswordModal = $state(false);
	let passwordInput = $state('');
	let passwordMode = $state<'create' | 'restore'>('create');
	let pendingRestoreEntry = $state<BackupEntry | null>(null);
	let isCreatingPortable = $state(false);

	// Permisos
	const canUpdate = $derived($currentUser && can($currentUser, 'UPDATE_SETTINGS_BACKUP'));

	// ==========================================
	// COLUMNAS AG GRID
	// ==========================================
	const columnDefs: ColDef<BackupEntry>[] = BackupColumns.getColumns({
		onRestore: handleRestore,
		onDelete: handleDelete
	});

	// ==========================================
	// BOTONES TOOLBAR POR CONTEXTO
	// ==========================================
	const customButtons = $derived.by(() => {
		const selected = selectedRows[0];

		const defaultBtns: CustomToolbarButton[] = [
			{
				id: 'backup-now',
				label: 'Crear Backup',
				icon: Database,
				onClick: handleBackupNow,
				variant: 'success',
				tooltip: 'Crear backup inmediato al directorio automático'
			},
			{
				id: 'backup-portable',
				label: 'Portable',
				icon: Key,
				onClick: handlePortableBackup,
				variant: 'primary',
				tooltip: 'Crear backup encriptado con contraseña (portable a otra máquina)'
			},
			{
				id: 'backup-manual',
				label: 'Exportar',
				icon: Download,
				onClick: handleBackupManual,
				variant: 'default',
				tooltip: 'Exportar a ubicación personalizada'
			},
			{
				id: 'restore-file',
				label: 'Importar',
				icon: Upload,
				onClick: handleRestoreFromFile,
				variant: 'default',
				tooltip: 'Restaurar desde archivo externo'
			}
		];

		const singleSelectBtns: CustomToolbarButton[] = [
			{
				id: 'restore',
				label: 'Restaurar',
				icon: ArchiveRestore,
				onClick: () => {
					if (selected) handleRestore(selected);
				},
				variant: 'primary',
				tooltip: 'Restaurar este backup'
			},
			{
				id: 'delete',
				label: 'Eliminar',
				icon: Trash2,
				onClick: () => {
					if (selected) handleDelete(selected);
				},
				variant: 'danger',
				tooltip: 'Eliminar backup'
			}
		];

		const multiSelectBtns: CustomToolbarButton[] = [
			{
				id: 'delete-multi',
				label: `Eliminar (${selectedRows.length})`,
				icon: Trash2,
				onClick: handleDeleteMultiple,
				variant: 'danger',
				tooltip: 'Eliminar backups seleccionados'
			}
		];

		return {
			default: defaultBtns,
			singleSelect: singleSelectBtns,
			multiSelect: multiSelectBtns
		};
	});

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

<div class="flex h-full flex-col relative bg-[#1e1e1e]">
	<!-- Header -->
	<div class="border-b border-white/10 px-6 py-4 bg-[#252526]">
		<div class="flex items-center gap-4">
			<div>
				<h2 class="text-xl font-semibold text-gray-100 flex items-center gap-2">
					<Database class="w-5 h-5 text-purple-500" />
					Copias de Seguridad
				</h2>
				<p class="mt-1 text-sm text-gray-400">
					Gestión de backups automáticos y manuales • Usa el botón "Configuración" en la toolbar
					para ajustar auto-backup
				</p>
			</div>
		</div>
	</div>

	<!-- Content -->
	<div class="flex-1 overflow-hidden relative bg-[#1e1e1e]">
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
		{:else if loading}
			<div class="flex h-full items-center justify-center">
				<div class="text-center">
					<svg class="mx-auto h-8 w-8 animate-spin text-purple-500" fill="none" viewBox="0 0 24 24">
						<circle
							class="opacity-25"
							cx="12"
							cy="12"
							r="10"
							stroke="currentColor"
							stroke-width="4"
						/>
						<path
							class="opacity-75"
							fill="currentColor"
							d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
						/>
					</svg>
					<p class="mt-4 text-sm text-gray-400">Cargando backups...</p>
				</div>
			</div>
		{:else if backups.length === 0}
			<div class="flex h-full items-center justify-center">
				<div class="text-center">
					<Database size={48} class="mx-auto text-gray-400" />
					<p class="mt-4 text-lg font-medium text-gray-300">No hay backups</p>
					<p class="mt-2 text-sm text-gray-400">
						Crea tu primer backup usando el botón "Crear Backup"
					</p>
					{#if canUpdate}
						<button
							onclick={handleBackupNow}
							class="mt-4 px-4 py-2 bg-purple-600 text-white rounded-md hover:bg-purple-700 transition-colors"
						>
							Crear Backup Ahora
						</button>
					{/if}
				</div>
			</div>
		{:else}
			<AGGridWrapper
				gridId="backup-list"
				{columnDefs}
				rowData={backups}
				{customButtons}
				getRowId={(params) => params.data.ruta}
				persistenceKey="backup-list-columns"
				onSelectionChanged={(rows) => (selectedRows = rows)}
				onRefresh={loadBackups}
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
