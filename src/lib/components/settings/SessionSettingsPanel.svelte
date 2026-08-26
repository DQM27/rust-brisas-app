<script lang="ts">
	import { RotateCcw } from '@lucide/svelte';
	import { sessionSettings } from '$lib/stores/sessionSettingsStore';
	import { can } from '$lib/logic/permissions';
	import { currentUser } from '$lib/stores/auth';

	// Permisos
	const canUpdate = $derived($currentUser && can($currentUser, 'settings_sessions:update'));

	// Local state for editing
	let screensaverMinutes = $state($sessionSettings.screensaverTimeoutMinutes);
	let completeMinutes = $state($sessionSettings.completeTimeoutMinutes);

	// Update local state when store changes (external changes)
	$effect(() => {
		screensaverMinutes = $sessionSettings.screensaverTimeoutMinutes;
		completeMinutes = $sessionSettings.completeTimeoutMinutes;
	});

	// Save changes to store (debounced by the store itself)
	function updateScreensaverTimeout() {
		sessionSettings.setScreensaverTimeout(screensaverMinutes);
	}

	function updateCompleteTimeout() {
		sessionSettings.setCompleteTimeout(completeMinutes);
	}

	function handleReset() {
		if (confirm('¿Restaurar configuración de sesión a valores por defecto?')) {
			sessionSettings.reset();
		}
	}

	// Format minutes for display
	function formatTime(minutes: number): string {
		if (minutes < 60) {
			return `${minutes} min`;
		}
		const hours = Math.floor(minutes / 60);
		const mins = minutes % 60;
		if (mins === 0) {
			return `${hours} h`;
		}
		return `${hours} h ${mins} min`;
	}
</script>

<div class="flex flex-col gap-3 h-full overflow-hidden">
	<!-- Header Compacto -->
	<div class="flex items-center justify-between pb-2 border-b border-surface">
		<div>
			<h3 class="text-base font-semibold text-primary">Seguridad de Sesión</h3>
			<p class="text-xs text-tertiary">Timeouts y desconexión automática</p>
		</div>
		<button
			type="button"
			onclick={handleReset}
			disabled={!canUpdate}
			class="flex items-center gap-1.5 px-2 py-1 text-xs rounded border border-surface-tertiary text-secondary hover:bg-surface-3 transition-colors disabled:opacity-50"
			title="Restaurar valores por defecto"
		>
			<RotateCcw size={12} />
			Restaurar
		</button>
	</div>

	<div class="flex-1 overflow-y-auto pr-1 space-y-3">
		<!-- App Lock Settings -->
		<div class="rounded border border-emphasis bg-surface-1 p-3">
			<div class="flex items-center justify-between mb-2">
				<div>
					<h4 class="text-sm font-medium text-primary">Bloqueo por Inactividad (App)</h4>
					<p class="text-[11px] text-secondary">Bloquea la interfaz sin cerrar sesión</p>
				</div>
				<label class="relative inline-flex items-center cursor-pointer">
					<input
						type="checkbox"
						checked={$sessionSettings.enableAppLock}
						onchange={() => sessionSettings.toggleAppLock()}
						disabled={!canUpdate}
						class="sr-only peer"
					/>
					<div
						class="w-9 h-5 bg-gray-600/50 rounded-full peer peer-checked:bg-amber-500 peer-focus:outline-none transition-colors after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-4 after:w-4 after:transition-transform peer-checked:after:translate-x-full"
					></div>
				</label>
			</div>

			<div class="text-xs text-secondary/80 leading-relaxed mb-3">
				Este ajuste oscurece la pantalla de la aplicación y bloquea los controles después del tiempo
				establecido. Es ideal para pausas cortas, protegiendo tu trabajo de miradas curiosas sin
				cerrar tus pestañas activas. Requiere volver a ingresar (con PIN o contraseña) para
				continuar.
			</div>

			{#if $sessionSettings.enableAppLock}
				<div class="mt-2 pt-2 border-t border-surface flex items-center gap-4">
					<label for="app-lock-timeout" class="text-xs text-secondary whitespace-nowrap">
						Tiempo: <span class="text-amber-500 font-bold"
							>{formatTime($sessionSettings.appLockTimeoutMinutes)}</span
						>
					</label>
					<input
						id="app-lock-timeout"
						type="range"
						min="1"
						max="120"
						step="1"
						value={$sessionSettings.appLockTimeoutMinutes}
						oninput={(e) =>
							sessionSettings.setAppLockTimeout(Number((e.target as HTMLInputElement).value))}
						disabled={!canUpdate}
						class="flex-1 h-1.5 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-amber-500"
					/>
				</div>
			{/if}
		</div>

		<!-- Screensaver Settings -->
		<div class="rounded border border-emphasis bg-surface-1 p-3">
			<div class="flex items-center justify-between mb-2">
				<div>
					<h4 class="text-sm font-medium text-primary">Protector de Pantalla</h4>
					<p class="text-[11px] text-secondary">Panel de bienvenida en pantalla completa</p>
				</div>
				<label class="relative inline-flex items-center cursor-pointer">
					<input
						type="checkbox"
						checked={$sessionSettings.enableScreensaver}
						onchange={() => sessionSettings.toggleScreensaver()}
						disabled={!canUpdate}
						class="sr-only peer"
					/>
					<div
						class="w-9 h-5 bg-gray-600/50 rounded-full peer peer-checked:bg-blue-500 peer-focus:outline-none transition-colors after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-4 after:w-4 after:transition-transform peer-checked:after:translate-x-full"
					></div>
				</label>
			</div>

			<div class="text-xs text-secondary/80 leading-relaxed mb-3">
				Muestra el panel de bienvenida con la hora y fecha ocupando toda la pantalla, ocultando
				información sensible de transeúntes. Si habilitas "Solicitar contraseña al volver",
				funcionará como un bloqueo de sesión de seguridad media.
			</div>

			{#if $sessionSettings.enableScreensaver}
				<div class="mt-2 space-y-3 pt-2 border-t border-surface">
					<div class="flex items-center gap-4">
						<label for="screensaver-timeout" class="text-xs text-secondary whitespace-nowrap">
							Tiempo: <span class="text-blue-500 font-bold">{formatTime(screensaverMinutes)}</span>
						</label>
						<input
							id="screensaver-timeout"
							type="range"
							min="1"
							max="120"
							step="1"
							bind:value={screensaverMinutes}
							oninput={updateScreensaverTimeout}
							disabled={!canUpdate}
							class="flex-1 h-1.5 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-blue-500"
						/>
					</div>

					<div class="flex items-center justify-between">
						<span class="text-xs text-secondary">Solicitar contraseña al volver</span>
						<label class="relative inline-flex items-center cursor-pointer scale-90 origin-right">
							<input
								type="checkbox"
								checked={$sessionSettings.screensaverRequiresPassword}
								onchange={() => sessionSettings.toggleScreensaverPassword()}
								disabled={!canUpdate}
								class="sr-only peer"
							/>
							<div
								class="w-9 h-5 bg-gray-600/50 rounded-full peer peer-checked:bg-blue-500 transition-colors after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-4 after:w-4 after:transition-transform peer-checked:after:translate-x-full"
							></div>
						</label>
					</div>
				</div>
			{/if}
		</div>

		<!-- Complete Logout Settings -->
		<div class="rounded border border-emphasis bg-surface-1 p-3">
			<div class="flex items-center justify-between mb-2">
				<div>
					<h4 class="text-sm font-medium text-primary">Cierre de Sesión Automático</h4>
					<p class="text-[11px] text-secondary">Desconexión total tras inactividad prolongada</p>
				</div>
				<label class="relative inline-flex items-center cursor-pointer">
					<input
						type="checkbox"
						checked={$sessionSettings.enableCompleteTimeout}
						onchange={() => sessionSettings.toggleCompleteTimeout()}
						disabled={!canUpdate}
						class="sr-only peer"
					/>
					<div
						class="w-9 h-5 bg-gray-600/50 rounded-full peer peer-checked:bg-red-500 peer-focus:outline-none transition-colors after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-4 after:w-4 after:transition-transform peer-checked:after:translate-x-full"
					></div>
				</label>
			</div>

			<div class="text-xs text-secondary/80 leading-relaxed mb-3">
				Por máxima seguridad, el sistema cerrará tu sesión completamente tras este periodo.
				Importante: Se cerrarán todas las pestañas y podrías perder cambios no guardados en
				formularios abiertos. Recomendado para equipos compartidos.
			</div>

			{#if $sessionSettings.enableCompleteTimeout}
				<div class="mt-2 pt-2 border-t border-surface flex items-center gap-4">
					<label for="complete-timeout" class="text-xs text-secondary whitespace-nowrap">
						Tiempo: <span class="text-red-500 font-bold">{formatTime(completeMinutes)}</span>
					</label>
					<input
						id="complete-timeout"
						type="range"
						min="5"
						max="240"
						step="5"
						bind:value={completeMinutes}
						oninput={updateCompleteTimeout}
						disabled={!canUpdate}
						class="flex-1 h-1.5 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-red-500"
					/>
				</div>
				{#if $sessionSettings.enableScreensaver && completeMinutes <= screensaverMinutes}
					<div class="mt-2 text-[10px] text-amber-500 bg-amber-500/10 px-2 py-1 rounded">
						⚠ Se ajustará automáticamente para ser mayor al protector.
					</div>
				{/if}
			{/if}
		</div>

		<!-- Quick Access Settings -->
		<div class="rounded border border-emphasis bg-surface-1 p-3">
			<div class="flex items-center justify-between mb-2">
				<div>
					<h4 class="text-sm font-medium text-primary">Accesos Rápidos</h4>
					<p class="text-[11px] text-secondary">Atajos de teclado y menú</p>
				</div>
				<!-- No master toggle for this section, just a header placeholder or individual controls -->
			</div>

			<div class="space-y-3 pt-2">
				<!-- Quick Session Switch -->
				<div class="flex items-center justify-between">
					<div>
						<div class="text-xs text-primary font-medium">Cambio Rápido de Sesión</div>
						<div class="text-[10px] text-secondary">
							Permite cambiar de usuario rápidamente con <span
								class="font-mono bg-surface-3 px-1 rounded">Ctrl+Shift+U</span
							>
						</div>
					</div>
					<label class="relative inline-flex items-center cursor-pointer">
						<input
							type="checkbox"
							checked={$sessionSettings.enableQuickSessionSwitch}
							onchange={() => sessionSettings.toggleQuickSessionSwitch()}
							disabled={!canUpdate}
							class="sr-only peer"
						/>
						<div
							class="w-9 h-5 bg-gray-600/50 rounded-full peer peer-checked:bg-purple-500 peer-focus:outline-none transition-colors after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-4 after:w-4 after:transition-transform peer-checked:after:translate-x-full"
						></div>
					</label>
				</div>

				<!-- Logout Shortcut -->
				<div class="flex items-center justify-between border-t border-surface pt-2">
					<div>
						<div class="text-xs text-primary font-medium">Atajo de Cierre de Sesión</div>
						<div class="text-[10px] text-secondary">
							Habilita <span class="font-mono bg-surface-3 px-1 rounded">Ctrl+Q</span> para cerrar sesión
							inmediatamente
						</div>
					</div>
					<label class="relative inline-flex items-center cursor-pointer">
						<input
							type="checkbox"
							checked={$sessionSettings.enableLogoutShortcut}
							onchange={() => sessionSettings.toggleLogoutShortcut()}
							disabled={!canUpdate}
							class="sr-only peer"
						/>
						<div
							class="w-9 h-5 bg-gray-600/50 rounded-full peer peer-checked:bg-purple-500 peer-focus:outline-none transition-colors after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-4 after:w-4 after:transition-transform peer-checked:after:translate-x-full"
						></div>
					</label>
				</div>
			</div>
		</div>

		<!-- Enhancement Settings -->
		<div class="rounded border border-emphasis bg-surface-1 p-3">
			<div class="flex items-center justify-between mb-2">
				<div>
					<h4 class="text-sm font-medium text-primary">Experiencia y Auditoría</h4>
					<p class="text-[11px] text-secondary">Ajustes avanzados de seguridad y UX</p>
				</div>
			</div>

			<div class="space-y-3 pt-2">
				<!-- Pre-Logout Warning -->
				<div class="flex items-center justify-between">
					<div>
						<div class="text-xs text-primary font-medium">Alerta de Pre-Desconexión</div>
						<div class="text-[10px] text-secondary">
							Avisa 60s antes de cerrar sesión por inactividad
						</div>
					</div>
					<label class="relative inline-flex items-center cursor-pointer">
						<input
							type="checkbox"
							checked={$sessionSettings.enablePreLogoutWarning}
							onchange={() => sessionSettings.togglePreLogoutWarning()}
							disabled={!canUpdate}
							class="sr-only peer"
						/>
						<div
							class="w-9 h-5 bg-gray-600/50 rounded-full peer peer-checked:bg-teal-500 peer-focus:outline-none transition-colors after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-4 after:w-4 after:transition-transform peer-checked:after:translate-x-full"
						></div>
					</label>
				</div>

				<!-- Grace Period -->
				<div class="flex items-center justify-between border-t border-surface pt-2">
					<div>
						<div class="text-xs text-primary font-medium">Periodo de Gracia</div>
						<div class="text-[10px] text-secondary">
							Permite cancelar bloqueo accidentalmente (2s)
						</div>
					</div>
					<label class="relative inline-flex items-center cursor-pointer">
						<input
							type="checkbox"
							checked={$sessionSettings.enableGracePeriod}
							onchange={() => sessionSettings.toggleGracePeriod()}
							disabled={!canUpdate}
							class="sr-only peer"
						/>
						<div
							class="w-9 h-5 bg-gray-600/50 rounded-full peer peer-checked:bg-teal-500 peer-focus:outline-none transition-colors after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-4 after:w-4 after:transition-transform peer-checked:after:translate-x-full"
						></div>
					</label>
				</div>

				<!-- Audit Log -->
				<div class="flex items-center justify-between border-t border-surface pt-2">
					<div>
						<div class="text-xs text-primary font-medium">Auditoría de Cierres</div>
						<div class="text-[10px] text-secondary">Registra eventos de desconexión en el log</div>
					</div>
					<label class="relative inline-flex items-center cursor-pointer">
						<input
							type="checkbox"
							checked={$sessionSettings.enableSessionAudit}
							onchange={() => sessionSettings.toggleSessionAudit()}
							disabled={!canUpdate}
							class="sr-only peer"
						/>
						<div
							class="w-9 h-5 bg-gray-600/50 rounded-full peer peer-checked:bg-teal-500 peer-focus:outline-none transition-colors after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-4 after:w-4 after:transition-transform peer-checked:after:translate-x-full"
						></div>
					</label>
				</div>
			</div>
		</div>

		<!-- Info Footnote (Compact) -->
		<div
			class="px-3 py-2 bg-blue-500/5 border border-blue-500/10 rounded text-[10px] text-secondary leading-tight"
		>
			<strong>Nota:</strong> Los cambios se guardan automáticamente. Interactuar con teclado/mouse reinicia
			los contadores.
		</div>
	</div>
</div>
```
