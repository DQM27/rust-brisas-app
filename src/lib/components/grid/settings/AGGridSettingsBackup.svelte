<script lang="ts">
	import { onMount } from 'svelte';
	import { Clock, Calendar, Settings, Save, RefreshCw, Trash2 } from 'lucide-svelte';
	import { toast } from 'svelte-5-french-toast';
	import {
		getBackupConfig,
		updateBackupConfig,
		cleanupOldBackups
	} from '$lib/services/backupService';

	// Estados
	let isLoading = $state(true);
	let isSaving = $state(false);
	let isCleaning = $state(false);

	// Config form
	let configEnabled = $state(false);
	let configHora = $state('02:00');
	let configDiasRetencion = $state(30);
	let lastBackup = $state<string | null>(null);

	onMount(async () => {
		await loadConfig();
	});

	async function loadConfig() {
		isLoading = true;
		try {
			const config = await getBackupConfig();
			if (config) {
				configEnabled = !!config.enabled;
				configHora = config.hora || '02:00';
				configDiasRetencion = config.diasRetencion || 30;
				lastBackup = config.ultimoBackup;
			}
		} catch (err) {
			console.error('Error loading backup config:', err);
		} finally {
			isLoading = false;
		}
	}

	async function handleSave() {
		isSaving = true;
		try {
			await updateBackupConfig(configEnabled, configHora, configDiasRetencion);
			toast.success('Configuración de backup guardada');
		} catch (err) {
			console.error('Error saving backup config:', err);
			toast.error(`Error: ${err}`);
		} finally {
			isSaving = false;
		}
	}

	async function handleCleanup() {
		isCleaning = true;
		try {
			const count = await cleanupOldBackups();
			toast.success(`${count} backup(s) antiguo(s) eliminado(s)`);
		} catch (err) {
			console.error('Error cleaning up:', err);
			toast.error(`Error: ${err}`);
		} finally {
			isCleaning = false;
		}
	}

	const sectionClass = 'space-y-4 p-1';
	const labelClass = 'block text-xs font-medium text-zinc-400 mb-1.5 ml-0.5';
	const inputClass =
		'w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[34px] text-sm text-white placeholder:text-gray-500 focus:outline-none focus:border-green-500/50 focus:ring-1 focus:ring-green-500/20 transition-all';
	const checkboxClass =
		'w-4 h-4 rounded border-zinc-600 bg-black/20 accent-green-600 cursor-pointer flex-shrink-0';
</script>

<div class={sectionClass}>
	<!-- Header -->
	<div class="px-1">
		<h3 class="text-sm font-semibold text-white mb-0.5">Backup Automático</h3>
		<p class="text-xs text-zinc-500">Configura las copias de seguridad automáticas del sistema</p>
	</div>

	{#if isLoading}
		<div class="flex items-center justify-center py-8">
			<RefreshCw class="w-5 h-5 text-zinc-500 animate-spin" />
		</div>
	{:else}
		<!-- Config Card -->
		<div class="space-y-4 p-4 rounded-lg bg-black/10 border border-white/5">
			<!-- Toggle -->
			<label class="flex items-start gap-3 cursor-pointer group">
				<input type="checkbox" bind:checked={configEnabled} class={checkboxClass} />
				<div class="flex flex-col -mt-0.5">
					<span class="text-sm font-medium text-zinc-200 group-hover:text-white transition-colors"
						>Activar Backup Automático</span
					>
					<span class="text-xs text-zinc-500">
						Se ejecutará diariamente si la app está abierta
					</span>
				</div>
			</label>

			<div class="h-px bg-white/5 my-3"></div>

			<!-- Inputs Grid -->
			<div class="grid grid-cols-2 gap-5">
				<div>
					<label for="hora" class={labelClass}>Hora de ejecución</label>
					<div class="relative">
						<input
							id="hora"
							type="time"
							bind:value={configHora}
							class={inputClass}
							disabled={!configEnabled}
						/>
						<Clock size={14} class="absolute right-3 top-2.5 text-zinc-500 pointer-events-none" />
					</div>
				</div>
				<div>
					<label for="dias" class={labelClass}>Retención (Días)</label>
					<input
						id="dias"
						type="number"
						min="1"
						max="365"
						bind:value={configDiasRetencion}
						class={inputClass}
						disabled={!configEnabled}
					/>
				</div>
			</div>

			<p class="text-[10px] text-zinc-500 flex items-center gap-1.5 mt-2">
				<Settings size={10} />
				Las copias antiguas se eliminarán automáticamente tras
				<strong class="text-zinc-400">{configDiasRetencion || '...'}</strong> días.
			</p>
		</div>

		<!-- Info & Actions -->
		<div class="space-y-4 pt-1">
			{#if lastBackup}
				<div
					class="px-3 py-2 bg-black/20 rounded-lg border border-white/5 flex items-center justify-between"
				>
					<div class="flex items-center gap-3">
						<div class="p-1.5 bg-green-500/10 rounded-md">
							<Calendar size={14} class="text-green-500" />
						</div>
						<div>
							<p class="text-[10px] text-zinc-500 uppercase tracking-wider">Último backup</p>
							<p class="text-xs text-white font-medium">
								{new Date(lastBackup).toLocaleString('es-MX')}
							</p>
						</div>
					</div>
					<div class="flex items-center gap-1.5">
						<span class="relative flex h-2 w-2">
							<span
								class="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75"
							></span>
							<span class="relative inline-flex rounded-full h-2 w-2 bg-green-500"></span>
						</span>
						<span class="text-[10px] text-green-500 font-medium">Exitoso</span>
					</div>
				</div>
			{/if}

			<div class="flex gap-3 pt-2">
				<button
					onclick={handleCleanup}
					disabled={isCleaning}
					class="flex-1 flex items-center justify-center gap-2 px-4 py-2.5 rounded-lg border-2 border-white/10 text-zinc-400 font-medium transition-all duration-200 hover:border-red-500 hover:text-red-500 disabled:opacity-50 h-[40px] text-xs hover:bg-transparent"
				>
					{#if isCleaning}
						<RefreshCw size={14} class="animate-spin" />
					{:else}
						<Trash2 size={14} />
					{/if}
					Limpiar Antiguos
				</button>

				<button
					onclick={handleSave}
					disabled={isSaving}
					class="flex-1 flex items-center justify-center gap-2 px-4 py-2.5 rounded-lg border-2 border-white/10 text-zinc-400 font-medium transition-all duration-200 hover:border-green-500 hover:text-green-500 disabled:opacity-50 h-[40px] text-xs hover:bg-transparent"
				>
					{#if isSaving}
						<RefreshCw size={14} class="animate-spin" />
					{:else}
						<Save size={14} />
					{/if}
					Guardar Configuración
				</button>
			</div>
		</div>
	{/if}
</div>
