<script lang="ts">
	import { fade, fly } from 'svelte/transition';

	import {
		X,
		Database,
		ArchiveRestore,
		RefreshCw,
		AlertTriangle,
		Download,
		Upload,
		Clock,
		Calendar,
		Settings,
		Trash2,
		Key
	} from 'lucide-svelte';
	import {
		backupDatabase,
		restoreDatabase,
		listBackups,
		deleteBackup,
		restoreFromAutoBackup,
		restorePortableBackup,
		cleanupOldBackups,
		getBackupConfig,
		updateBackupConfig,
		backupDatabaseAuto,
		backupDatabasePortable
	} from '$lib/services/backupService';
	import { message, confirm } from '@tauri-apps/plugin-dialog';
	import type { BackupEntry, BackupConfig } from '$lib/types/backup';

	// AG Grid
	import { AgGrid } from 'ag-grid-svelte5-extended';
	import { ClientSideRowModelModule } from '@ag-grid-community/client-side-row-model';
	import type { ColDef, GridApi, GridOptions } from '@ag-grid-community/core';
	import { themeQuartz, colorSchemeDark } from '@ag-grid-community/theming';
	import { BackupColumns } from '$lib/logic/backup/backupColumns';

	interface Props {
		show: boolean;
		onClose: () => void;
	}

	let { show, onClose }: Props = $props();

	// Estados
	let isBackingUp = $state(false);
	let isRestoring = $state(false);
	let isLoading = $state(false);
	let isSavingConfig = $state(false);
	let isCleaningUp = $state(false);
	let isCreatingPortable = $state(false);

	// Datos
	let backups = $state<BackupEntry[]>([]);
	let config = $state<BackupConfig | null>(null);

	// Config form
	let configEnabled = $state(false);
	let configHora = $state('02:00');

	// Portable backup
	let showPasswordModal = $state(false);
	let passwordInput = $state('');
	let passwordMode = $state<'create' | 'restore'>('create');
	let pendingRestoreEntry = $state<BackupEntry | null>(null);
	let configDiasRetencion = $state(30);

	// AG Grid

	const modules = [ClientSideRowModelModule];

	const columnDefs: ColDef<BackupEntry>[] = BackupColumns.getColumns({
		onRestore: handleRestoreFromGrid,
		onDelete: handleDeleteFromGrid
	});

	const myTheme = themeQuartz.withPart(colorSchemeDark).withParams({
		backgroundColor: 'rgb(13 17 23)',
		foregroundColor: 'rgb(255 255 255)',
		browserColorScheme: 'dark',
		headerBackgroundColor: 'rgb(22 27 34)',
		headerTextColor: 'rgb(209 213 219)',
		oddRowBackgroundColor: 'rgb(13 17 23)',
		chromeBackgroundColor: 'rgb(22 27 34)',
		rowHoverColor: 'rgba(255, 255, 255, 0.05)',
		selectedRowBackgroundColor: 'rgba(147, 51, 234, 0.15)',
		fontSize: 12,
		headerFontSize: 11,
		spacing: 3,
		cellHorizontalPadding: 8
	});

	const gridOptions: GridOptions<BackupEntry> = {
		columnDefs,
		defaultColDef: {
			resizable: true,
			suppressMovable: true
		},
		theme: myTheme as any,
		rowHeight: 36,
		headerHeight: 32,
		animateRows: true,
		suppressCellFocus: true,
		onGridReady: (params) => {
			params.api.sizeColumnsToFit();
		}
	};

	// Cargar datos cuando se abre
	$effect(() => {
		if (show) {
			loadData();
		}
	});

	async function loadData() {
		isLoading = true;
		try {
			[backups, config] = await Promise.all([listBackups(), getBackupConfig()]);
			if (config) {
				configEnabled = config.enabled;
				configHora = config.hora;
				configDiasRetencion = config.diasRetencion;
			}
		} catch (error) {
			console.error('Error loading backup data:', error);
		} finally {
			isLoading = false;
		}
	}

	async function handleBackup() {
		isBackingUp = true;
		try {
			await backupDatabase();
		} finally {
			isBackingUp = false;
		}
	}

	async function handleAutoBackup() {
		isBackingUp = true;
		try {
			const filename = await backupDatabaseAuto();
			await message(`Backup creado: ${filename}`, {
				title: 'Backup Exitoso',
				kind: 'info'
			});
			await loadData();
		} catch (error) {
			console.error('Error creating auto backup:', error);
			await message(`Error: ${error}`, { title: 'Error', kind: 'error' });
		} finally {
			isBackingUp = false;
		}
	}

	async function handleRestore() {
		isRestoring = true;
		try {
			await restoreDatabase();
		} finally {
			isRestoring = false;
		}
	}

	async function handleRestoreFromGrid(entry: BackupEntry) {
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
		} catch (error) {
			console.error('Error restoring backup:', error);
			await message(`Error al restaurar: ${error}`, {
				title: 'Error',
				kind: 'error'
			});
		}
	}

	async function handlePortableBackup() {
		passwordMode = 'create';
		passwordInput = '';
		showPasswordModal = true;
	}

	async function handlePasswordSubmit() {
		if (passwordInput.length < 8) {
			await message('La contraseña debe tener al menos 8 caracteres.', {
				title: 'Contraseña muy corta',
				kind: 'warning'
			});
			return;
		}

		showPasswordModal = false;

		if (passwordMode === 'create') {
			isCreatingPortable = true;
			try {
				const filename = await backupDatabasePortable(passwordInput);
				await message(
					`Backup portable creado: ${filename}\n\n⚠️ Guarda la contraseña en un lugar seguro. Sin ella no podrás restaurar este backup.`,
					{
						title: 'Backup Portable Creado',
						kind: 'info'
					}
				);
				await loadData();
			} catch (error) {
				console.error('Error creating portable backup:', error);
				await message(`Error: ${error}`, { title: 'Error', kind: 'error' });
			} finally {
				isCreatingPortable = false;
			}
		} else if (passwordMode === 'restore' && pendingRestoreEntry) {
			try {
				await restorePortableBackup(pendingRestoreEntry.nombre, passwordInput);
			} catch (error) {
				console.error('Error restoring portable backup:', error);
				await message(`Error al restaurar: ${error}`, {
					title: 'Error',
					kind: 'error'
				});
			} finally {
				pendingRestoreEntry = null;
			}
		}

		passwordInput = '';
	}

	async function handleDeleteFromGrid(entry: BackupEntry) {
		const confirmed = await confirm(
			`¿Eliminar el backup "${entry.nombre}"?\n\nEsta acción no se puede deshacer.`,
			{ title: 'Confirmar Eliminación', kind: 'warning' }
		);
		if (!confirmed) return;

		try {
			await deleteBackup(entry.nombre);
			await message('Backup eliminado correctamente.', {
				title: 'Eliminado',
				kind: 'info'
			});
			await loadData();
		} catch (error) {
			console.error('Error deleting backup:', error);
			await message(`Error al eliminar: ${error}`, {
				title: 'Error',
				kind: 'error'
			});
		}
	}

	async function handleCleanup() {
		const confirmed = await confirm(`¿Eliminar backups con más de ${configDiasRetencion} días?`, {
			title: 'Confirmar Limpieza',
			kind: 'warning'
		});
		if (!confirmed) return;

		isCleaningUp = true;
		try {
			const count = await cleanupOldBackups();
			await message(`Se eliminaron ${count} backup(s) antiguo(s).`, {
				title: 'Limpieza Completa',
				kind: 'info'
			});
			await loadData();
		} catch (error) {
			console.error('Error cleaning up backups:', error);
			await message(`Error en limpieza: ${error}`, {
				title: 'Error',
				kind: 'error'
			});
		} finally {
			isCleaningUp = false;
		}
	}

	async function handleSaveConfig() {
		isSavingConfig = true;
		try {
			config = await updateBackupConfig(configEnabled, configHora, configDiasRetencion);
			await message('Configuración guardada.', {
				title: 'Guardado',
				kind: 'info'
			});
		} catch (error) {
			console.error('Error saving config:', error);
			await message(`Error: ${error}`, { title: 'Error', kind: 'error' });
		} finally {
			isSavingConfig = false;
		}
	}
</script>

{#if show}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center p-4"
		transition:fade={{ duration: 150 }}
	>
		<!-- Backdrop -->
		<button
			class="absolute inset-0 bg-black/60 backdrop-blur-sm border-0 cursor-default"
			onclick={onClose}
			aria-label="Cerrar modal"
		></button>

		<!-- Modal Content - Más ancho para la grid -->
		<div
			class="relative z-10 w-full max-w-3xl max-h-[90vh] overflow-hidden rounded-lg bg-[#0d1117] shadow-2xl border border-gray-700 flex flex-col"
			transition:fly={{ y: 20, duration: 200 }}
		>
			<!-- Header -->
			<div class="flex items-center justify-between px-6 py-4 border-b border-gray-700">
				<div class="flex items-center gap-2">
					<Database class="w-5 h-5 text-purple-500" />
					<h2 class="text-lg font-semibold text-white">Copias de Seguridad</h2>
				</div>
				<button
					onclick={onClose}
					class="p-1 rounded-full text-gray-400 hover:text-gray-200 hover:bg-gray-800 transition-colors"
					aria-label="Cerrar"
				>
					<X size={20} />
				</button>
			</div>

			<!-- Body - Scrollable -->
			<div class="flex-1 overflow-y-auto p-6 space-y-5">
				<!-- Config Section -->
				<div class="bg-[#161b22] p-4 rounded-md border border-gray-700">
					<div class="flex items-center gap-2 mb-3">
						<Settings class="w-4 h-4 text-gray-400" />
						<h3 class="text-sm font-semibold text-white">Backup Automático</h3>
					</div>

					<div class="flex items-center gap-4 mb-4">
						<!-- Toggle -->
						<label class="relative inline-flex items-center cursor-pointer">
							<input type="checkbox" bind:checked={configEnabled} class="sr-only peer" />
							<div
								class="w-9 h-5 bg-gray-700 rounded-full peer peer-checked:bg-purple-600 peer-checked:after:translate-x-full after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:rounded-full after:h-4 after:w-4 after:transition-all"
							></div>
						</label>
						<span class="text-sm text-gray-300">{configEnabled ? 'Activado' : 'Desactivado'}</span>

						<!-- Hora -->
						<div class="flex items-center gap-1">
							<Clock class="w-3 h-3 text-gray-400" />
							<input
								type="time"
								bind:value={configHora}
								class="px-2 py-1 text-xs rounded bg-[#0d1117] border border-gray-600 text-white w-20"
							/>
						</div>

						<!-- Días -->
						<div class="flex items-center gap-1">
							<Calendar class="w-3 h-3 text-gray-400" />
							<input
								type="number"
								min="1"
								max="365"
								bind:value={configDiasRetencion}
								class="px-2 py-1 text-xs rounded bg-[#0d1117] border border-gray-600 text-white w-14"
							/>
							<span class="text-xs text-gray-400">días</span>
						</div>

						<button
							class="ml-auto px-3 py-1 text-xs font-medium rounded bg-purple-600 hover:bg-purple-700 text-white disabled:opacity-50"
							onclick={handleSaveConfig}
							disabled={isSavingConfig}
						>
							{isSavingConfig ? '...' : 'Guardar'}
						</button>
					</div>

					<div class="flex gap-2">
						<button
							class="inline-flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium rounded-md bg-emerald-600 hover:bg-emerald-700 text-white disabled:opacity-50"
							onclick={handleAutoBackup}
							disabled={isBackingUp}
						>
							{#if isBackingUp}
								<RefreshCw class="w-3 h-3 animate-spin" />
							{:else}
								<Database class="w-3 h-3" />
							{/if}
							Crear Backup Ahora
						</button>

						<button
							class="inline-flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium rounded-md bg-purple-600 hover:bg-purple-700 text-white disabled:opacity-50"
							onclick={handlePortableBackup}
							disabled={isCreatingPortable}
							title="Backup encriptado con contraseña, portable a otra máquina"
						>
							{#if isCreatingPortable}
								<RefreshCw class="w-3 h-3 animate-spin" />
							{:else}
								<Key class="w-3 h-3" />
							{/if}
							Portable
						</button>
						<button
							class="inline-flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium rounded-md bg-gray-700 hover:bg-gray-600 text-gray-200 disabled:opacity-50"
							onclick={handleCleanup}
							disabled={isCleaningUp}
						>
							<Trash2 class="w-3 h-3" />
							Limpiar Antiguos
						</button>
					</div>
				</div>

				<!-- Backups Grid -->
				<div class="rounded-md border border-gray-700 overflow-hidden">
					<div
						class="bg-[#161b22] px-4 py-2 border-b border-gray-700 flex items-center justify-between"
					>
						<span class="text-sm font-medium text-white flex items-center gap-2">
							<Database class="w-4 h-4 text-gray-400" />
							Backups Disponibles ({backups.length})
						</span>
						<button
							class="text-xs text-gray-400 hover:text-white flex items-center gap-1"
							onclick={loadData}
							disabled={isLoading}
						>
							<RefreshCw class="w-3 h-3 {isLoading ? 'animate-spin' : ''}" />
							Actualizar
						</button>
					</div>

					<div class="h-[200px]">
						<AgGrid {gridOptions} rowData={backups} {modules} />
					</div>
				</div>

				<!-- Manual Backup & Restore -->
				<div class="grid grid-cols-2 gap-4">
					<!-- Manual Backup -->
					<div class="bg-[#161b22] p-4 rounded-md border border-gray-700">
						<h3 class="text-sm font-semibold text-white flex items-center gap-2 mb-2">
							<Download class="w-4 h-4 text-gray-400" />
							Backup Manual
						</h3>
						<p class="text-xs text-gray-400 mb-3">Exportar a ubicación personalizada</p>
						<button
							class="w-full inline-flex items-center justify-center gap-2 px-3 py-1.5 text-xs font-medium rounded-md bg-emerald-600 hover:bg-emerald-700 text-white disabled:opacity-50"
							onclick={handleBackup}
							disabled={isBackingUp}
						>
							{#if isBackingUp}
								<RefreshCw class="w-3.5 h-3.5 animate-spin" />
							{:else}
								<Download class="w-3.5 h-3.5" />
							{/if}
							Exportar Archivo
						</button>
					</div>

					<!-- Restore -->
					<div class="bg-[#161b22] p-4 rounded-md border border-gray-700">
						<h3 class="text-sm font-semibold text-white flex items-center gap-2 mb-2">
							<Upload class="w-4 h-4 text-gray-400" />
							Restaurar
						</h3>
						<p class="text-xs text-amber-400 mb-3 flex items-center gap-1">
							<AlertTriangle class="w-3 h-3" />
							Sobrescribirá datos actuales
						</p>
						<button
							class="w-full inline-flex items-center justify-center gap-2 px-3 py-1.5 text-xs font-medium rounded-md bg-gray-700 hover:bg-gray-600 text-gray-200 border border-gray-600 disabled:opacity-50"
							onclick={handleRestore}
							disabled={isRestoring}
						>
							{#if isRestoring}
								<RefreshCw class="w-3.5 h-3.5 animate-spin" />
							{:else}
								<ArchiveRestore class="w-3.5 h-3.5" />
							{/if}
							Seleccionar Archivo
						</button>
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}

{#if showPasswordModal}
	<div
		class="fixed inset-0 z-[60] flex items-center justify-center p-4"
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
			class="relative z-10 w-full max-w-sm p-6 rounded-lg bg-[#161b22] border border-gray-600 shadow-2xl"
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
					<span class="text-amber-400">⚠️ No podrás recuperar el backup sin esta contraseña.</span>
				{:else}
					Ingresa la contraseña para desencriptar el backup.
				{/if}
			</p>

			<input
				type="password"
				bind:value={passwordInput}
				placeholder="Contraseña (mín. 8 caracteres)"
				class="w-full px-3 py-2 text-sm rounded-md bg-[#0d1117] border border-gray-600 text-white placeholder-gray-500 focus:border-purple-500 focus:outline-none mb-4"
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
