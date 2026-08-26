<script lang="ts">
	import { generalSettings } from '$lib/stores/settingsStore';
	import { scale } from 'svelte/transition';
	import { Check, X, Power, Bell, BellOff } from '@lucide/svelte';
	import { can } from '$lib/logic/permissions';
	import { currentUser } from '$lib/stores/auth';

	const canUpdate = $derived($currentUser && can($currentUser, 'settings_general:update'));
</script>

{#snippet toggleSwitch(checked: boolean, onChange: () => void, label: string, disabled = false)}
	<button onclick={onChange} {disabled} aria-label={label} class="relative h-7 w-12 rounded-full disabled:opacity-50 {checked ? 'bg-green-500' : 'bg-gray-300 dark:bg-gray-700'}">
		<span class="absolute left-0 top-0 inline-flex h-7 w-7 items-center justify-center rounded-full bg-white shadow transition-transform {checked ? 'translate-x-5' : ''}">
			{#if checked}<Check size={12} class="text-green-600" />{:else}<X size={12} class="text-gray-400" />{/if}
		</span>
	</button>
{/snippet}

{#snippet settingRow(Icon: any, label: string, checked: boolean, onChange: () => void)}
	<div class="flex items-center justify-between py-3">
		<div class="flex items-center gap-3"><div class="rounded-md bg-blue-50 p-2 text-blue-500 dark:bg-blue-900/20"><Icon size={18} /></div><span class="font-medium text-secondary">{label}</span></div>
		{@render toggleSwitch(checked, onChange, label, !canUpdate)}
	</div>
{/snippet}

<div class="flex h-full flex-col overflow-y-auto bg-surface-1 p-6" in:scale={{ duration: 300, start: 0.95 }}>
	<div class="mb-6"><h2 class="text-2xl font-bold text-primary">Ajustes Generales</h2><p class="mt-1 text-secondary">Configura las preferencias del sistema e interfaz.</p></div>
	<div class="grid max-w-3xl gap-4 pb-8">
		<div class="card-base p-5">
			<div class="mb-4 flex items-center gap-4"><div class="rounded-lg bg-slate-100 p-3 text-slate-600 dark:bg-slate-800 dark:text-slate-400"><Power size={22} /></div><div><h3 class="text-lg font-semibold text-primary">Sistema</h3><p class="text-sm text-secondary">Configuraciones de arranque y sistema.</p></div></div>
			{@render settingRow(Power, 'Deshabilitar Setup Wizard', $generalSettings.disableSetupWizard, () => generalSettings.toggleSetupWizard())}
		</div>
		<div class="card-base p-5">
			<div class="mb-4 flex items-center gap-4"><div class="rounded-lg bg-blue-100 p-3 text-blue-600 dark:bg-blue-900/30 dark:text-blue-400"><Bell size={22} /></div><div><h3 class="text-lg font-semibold text-primary">Notificaciones</h3><p class="text-sm text-secondary">Controla los avisos visuales.</p></div></div>
			{@render settingRow($generalSettings.showToasts ? Bell : BellOff, 'Activar Notificaciones Visuales', $generalSettings.showToasts, () => generalSettings.toggleToasts())}
			<div class="flex items-center justify-between border-t border-emphasis py-3"><span class="font-medium text-secondary">Posición de las notificaciones</span><select value={$generalSettings.toastPosition} onchange={(e) => generalSettings.setToastPosition(e.currentTarget.value as any)} disabled={!canUpdate} class="rounded-md border border-emphasis bg-surface-2 px-3 py-1.5 text-sm text-primary disabled:opacity-50"><option value="top-left">Superior Izquierda</option><option value="top-center">Superior Centro</option><option value="top-right">Superior Derecha</option><option value="bottom-left">Inferior Izquierda</option><option value="bottom-center">Inferior Centro</option><option value="bottom-right">Inferior Derecha</option></select></div>
		</div>
		<div class="flex justify-end"><button class="btn-base bg-red-100 text-sm text-red-600 disabled:opacity-50 dark:bg-red-900/30 dark:text-red-400" disabled={!canUpdate} onclick={() => { if (confirm('¿Restaurar todas las configuraciones?')) generalSettings.reset(); }}>Restaurar Todo</button></div>
	</div>
</div>
