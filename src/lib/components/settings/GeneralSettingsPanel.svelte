<script lang="ts">
	import { generalSettings } from '$lib/stores/settingsStore';
	import { scale } from 'svelte/transition';
	import { Check, X, Power, Volume2, Music, Upload, Bell, BellOff } from 'lucide-svelte';
	import { open } from '@tauri-apps/plugin-dialog';
	import { onMount } from 'svelte';
	import { toastService } from '$lib/services/toastService';
	import { can } from '$lib/logic/permissions';
	import { currentUser } from '$lib/stores/auth';
	import { configService } from '$lib/logic/system/configService';

	// Permisos
	const canUpdate = $derived($currentUser && can($currentUser, 'settings_general:update'));

	// Estado de audio
	let alertSound = $state('Hand');
	let useCustomSound = $state(false);
	let customSoundPath = $state<string | null>(null);
	let uploadingSound = $state(false);

	// Cargar configuración al montar
	onMount(async () => {
		const res = await configService.getAppConfig();
		if (res.ok) {
			const config = res.data;
			alertSound = config?.audio?.alert_sound ?? 'Hand';
			useCustomSound = config?.audio?.use_custom ?? false;
			customSoundPath = config?.audio?.custom_sound_path ?? null;
		} else {
			console.warn('No se pudo cargar config de la aplicación:', res.error);
		}
	});

	async function saveAudioConfig() {
		const res = await configService.updateAudioConfig(alertSound);
		if (res.ok) {
			toastService.success('Sonido de sistema actualizado', { icon: '🔔' });
		} else {
			console.error('Error saving audio config:', res.error);
			toastService.error('Error al guardar configuración de audio');
		}
	}

	async function toggleCustomSound() {
		if (useCustomSound && !customSoundPath) {
			toastService.error('Primero selecciona un archivo de sonido');
			useCustomSound = false;
			return;
		}
		const res = await configService.setUseCustomSound(useCustomSound);
		if (res.ok) {
			toastService.success(useCustomSound ? 'Usando sonido personalizado' : 'Usando sonido nativo');
		} else {
			console.error('Error toggling custom sound:', res.error);
			toastService.error('Error al cambiar tipo de sonido');
		}
	}

	async function pickCustomSound() {
		try {
			const selected = await open({
				multiple: false,
				filters: [
					{
						name: 'Audio',
						extensions: ['wav', 'mp3']
					}
				]
			});

			if (selected && typeof selected === 'string') {
				uploadingSound = true;
				const res = await configService.uploadCustomSound(selected);
				if (res.ok) {
					customSoundPath = res.data;
					useCustomSound = true;
					toastService.success('Sonido personalizado cargado correctamente');
				} else {
					toastService.error(`Error: ${res.error}`);
				}
			}
		} catch (e) {
			console.error('Error picking sound:', e);
			toastService.error('Error al cargar sonido');
		} finally {
			uploadingSound = false;
		}
	}

	async function testSound() {
		const res = await configService.playAlertSound();
		if (!res.ok) {
			console.error('Error playing test sound:', res.error);
			toastService.error('No se pudo reproducir el sonido de prueba');
		}
	}

	// ==========================================================================
	// Toggle Component (reusable)
	// ==========================================================================
</script>

<!-- Reusable Toggle Switch -->
{#snippet toggleSwitch(
	checked: boolean,
	onChange: () => void,
	srLabel: string,
	disabled: boolean = false
)}
	<button
		onclick={onChange}
		{disabled}
		class="relative inline-flex h-7 w-12 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus-visible:ring-2 focus-visible:ring-accent focus-visible:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed
    {checked ? 'bg-green-500' : 'bg-gray-300 dark:bg-gray-700'}"
	>
		<span class="sr-only">{srLabel}</span>
		<span
			class="pointer-events-none inline-flex h-6 w-6 items-center justify-center transform rounded-full bg-white shadow-lg ring-0 transition duration-200 ease-in-out
      {checked ? 'translate-x-5' : 'translate-x-0'}"
		>
			{#if checked}
				<Check size={12} class="text-green-600" strokeWidth={3} />
			{:else}
				<X size={12} class="text-gray-400" strokeWidth={3} />
			{/if}
		</span>
	</button>
{/snippet}

<!-- Setting Row -->
{#snippet settingRow(
	Icon: any,
	iconBg: string,
	iconColor: string,
	label: string,
	checked: boolean,
	onChange: () => void,
	disabled: boolean = false
)}
	<div class="flex items-center justify-between py-3">
		<div class="flex items-center gap-3">
			<div class="p-2 rounded-md {iconBg}">
				<Icon size={18} class={iconColor} />
			</div>
			<span class="text-secondary font-medium">{label}</span>
		</div>
		{@render toggleSwitch(checked, onChange, label, disabled)}
	</div>
{/snippet}

<div
	class="flex h-full flex-col bg-surface-1 p-6 overflow-y-auto"
	in:scale={{ duration: 300, start: 0.95 }}
>
	<div class="mb-6">
		<h2 class="text-2xl font-bold text-primary">Ajustes Generales</h2>
		<p class="text-secondary mt-1">Configura las preferencias del sistema e interfaz de usuario.</p>
	</div>

	<div class="grid gap-4 max-w-3xl pb-8">
		<!-- ================================================================== -->
		<!-- SYSTEM CARD -->
		<!-- ================================================================== -->
		<div class="card-base p-5">
			<div class="flex items-center gap-4 mb-4">
				<div
					class="p-3 rounded-lg bg-slate-100 text-slate-600 dark:bg-slate-800 dark:text-slate-400"
				>
					<Power size={22} />
				</div>
				<div>
					<h3 class="text-lg font-semibold text-primary">Sistema</h3>
					<p class="text-sm text-secondary">Configuraciones de arranque y sistema.</p>
				</div>
			</div>

			<div class="divide-y divide-emphasis">
				{@render settingRow(
					Power,
					'bg-gray-50 dark:bg-gray-900/20',
					'text-gray-500',
					'Deshabilitar Setup Wizard',
					$generalSettings.disableSetupWizard,
					() => generalSettings.toggleSetupWizard(),
					!canUpdate
				)}
			</div>
		</div>

		<!-- ================================================================== -->
		<!-- AUDIO ALERTS CARD -->
		<!-- ================================================================== -->
		<div class="card-base p-5">
			<div class="flex items-center gap-4 mb-4">
				<div
					class="p-3 rounded-lg bg-orange-100 text-orange-600 dark:bg-orange-900/30 dark:text-orange-400"
				>
					<Volume2 size={22} />
				</div>
				<div>
					<h3 class="text-lg font-semibold text-primary">Alertas de Audio</h3>
					<p class="text-sm text-secondary">Configura los sonidos de notificación del sistema.</p>
				</div>
			</div>

			<div class="space-y-6">
				<!-- Selector Nativo -->
				<div class="flex items-center justify-between py-2">
					<div class="flex-1">
						<span class="text-secondary font-medium block">Sonido del Sistema</span>
						<p class="text-xs text-secondary/70">Usa los sonidos nativos de Windows.</p>
					</div>

					<div class="flex items-center gap-2">
						<select
							bind:value={alertSound}
							onchange={saveAudioConfig}
							disabled={useCustomSound || !canUpdate}
							class="rounded-md border border-emphasis bg-surface-2 px-3 py-1.5 text-sm text-primary focus:outline-none focus:ring-2 focus:ring-accent disabled:opacity-50"
						>
							<option value="Hand">Crítico (Hand)</option>
							<option value="Exclamation">Exclamación</option>
							<option value="Beep">Beep Sugerido</option>
							<option value="Question">Pregunta/Aviso</option>
							<option value="Asterisk">Informativo</option>
						</select>

						<button
							onclick={testSound}
							class="p-2 rounded-md bg-surface-hover hover:bg-emphasis text-primary transition-colors"
							title="Probar Sonido"
						>
							<Volume2 size={16} />
						</button>
					</div>
				</div>

				<div class="border-t border-emphasis pt-4">
					<div class="flex items-center justify-between">
						<div class="flex items-center gap-3">
							<div class="p-2 rounded-md bg-blue-50 dark:bg-blue-900/20 text-blue-500">
								<Music size={18} />
							</div>
							<div class="flex flex-col">
								<span class="text-secondary font-medium">Sonido Personalizado</span>
								<span class="text-xs text-secondary/60">
									{customSoundPath
										? customSoundPath.split(/[\\/]/).pop()
										: 'Ningún archivo seleccionado'}
								</span>
							</div>
						</div>

						<div class="flex items-center gap-4">
							<button
								onclick={pickCustomSound}
								disabled={uploadingSound || !canUpdate}
								class="btn-secondary py-1.5 px-3 text-xs flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed"
							>
								{#if uploadingSound}
									<span class="animate-spin text-accent">⏳</span>
								{:else}
									<Upload size={14} />
								{/if}
								{customSoundPath ? 'Cambiar archivo' : 'Subir archivo .wav'}
							</button>

							{@render toggleSwitch(
								useCustomSound,
								() => {
									useCustomSound = !useCustomSound;
									toggleCustomSound();
								},
								'Usar personalizado',
								!customSoundPath || !canUpdate
							)}
						</div>
					</div>
				</div>
			</div>
		</div>

		<!-- ================================================================== -->
		<!-- NOTIFICATIONS CARD -->
		<!-- ================================================================== -->
		<div class="card-base p-5">
			<div class="flex items-center gap-4 mb-4">
				<div
					class="p-3 rounded-lg bg-blue-100 text-blue-600 dark:bg-blue-900/30 dark:text-blue-400"
				>
					<Bell size={22} />
				</div>
				<div>
					<h3 class="text-lg font-semibold text-primary">Notificaciones</h3>
					<p class="text-sm text-secondary">Controla los avisos visuales y sonoros.</p>
				</div>
			</div>

			<div class="divide-y divide-emphasis">
				{@render settingRow(
					$generalSettings.showToasts ? Bell : BellOff,
					'bg-blue-50 dark:bg-blue-900/20',
					'text-blue-500',
					'Activar Notificaciones Visuales',
					$generalSettings.showToasts,
					() => generalSettings.toggleToasts(),
					!canUpdate
				)}
				{@render settingRow(
					Volume2,
					'bg-orange-50 dark:bg-orange-900/20',
					'text-orange-500',
					'Activar Sonidos de Alerta',
					$generalSettings.enableNotificationSounds,
					() => generalSettings.toggleNotificationSounds(),
					!canUpdate
				)}
			</div>
		</div>

		<!-- ================================================================== -->
		<!-- RESET BUTTON -->
		<!-- ================================================================== -->
		<div class="flex justify-end pt-2">
			<button
				class="btn-base bg-red-100 hover:bg-red-200 dark:bg-red-900/30 dark:hover:bg-red-900/50 text-red-600 dark:text-red-400 text-sm disabled:opacity-50 disabled:cursor-not-allowed"
				disabled={!canUpdate}
				onclick={() => {
					if (confirm('¿Restaurar todas las configuraciones a sus valores por defecto?')) {
						generalSettings.reset();
					}
				}}
			>
				Restaurar Todo
			</button>
		</div>
	</div>
</div>
