<script lang="ts">
	import { spotlightSettings } from '$lib/stores/spotlightStore';
	import { scale } from 'svelte/transition';
	import { Check, X, Search, Database, List, Layers, Type, History } from 'lucide-svelte';
	import { currentUser } from '$lib/stores/auth';
	import { can } from '$lib/logic/permissions';

	// Permisos
	const canUpdate = $derived($currentUser && can($currentUser, 'settings_general:update'));

	// ==========================================================================
	// Snippets reutilizables (estilo GeneralSettingsPanel)
	// ==========================================================================
</script>

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

{#snippet settingRow(
	Icon: any,
	iconBg: string,
	iconColor: string,
	label: string,
	description: string,
	checked: boolean,
	onChange: () => void,
	disabled: boolean = false
)}
	<div class="flex items-center justify-between py-3">
		<div class="flex items-center gap-3">
			<div class="p-2 rounded-md {iconBg}">
				<Icon size={18} class={iconColor} />
			</div>
			<div class="flex flex-col">
				<span class="text-secondary font-medium">{label}</span>
				<span class="text-xs text-secondary/60">{description}</span>
			</div>
		</div>
		{@render toggleSwitch(checked, onChange, label, disabled)}
	</div>
{/snippet}

<div
	class="flex h-full flex-col bg-surface-1 p-6 overflow-y-auto"
	in:scale={{ duration: 300, start: 0.95 }}
>
	<div class="mb-6">
		<h2 class="text-2xl font-bold text-primary">Ajustes de Spotlight</h2>
		<p class="text-secondary mt-1">
			Personaliza la experiencia y potencia del buscador inteligente.
		</p>
	</div>

	<div class="grid gap-4 max-w-3xl pb-8">
		<!-- ================================================================== -->
		<!-- VISIBILIDAD DE CONTENIDO -->
		<!-- ================================================================== -->
		<div class="card-base p-5">
			<div class="flex items-center gap-4 mb-4">
				<div
					class="p-3 rounded-lg bg-indigo-100 text-indigo-600 dark:bg-indigo-900/30 dark:text-indigo-400"
				>
					<Layers size={22} />
				</div>
				<div>
					<h3 class="text-lg font-semibold text-primary">Visibilidad</h3>
					<p class="text-sm text-secondary">
						Elige qué tipos de elementos mostrar en los resultados.
					</p>
				</div>
			</div>

			<div class="divide-y divide-emphasis">
				{@render settingRow(
					List,
					'bg-blue-50 dark:bg-blue-900/20',
					'text-blue-500',
					'Mostrar Listas y Vistas',
					'Acceso directo a listas, catálogos e informes.',
					$spotlightSettings.showModules,
					() => spotlightSettings.toggleModules(),
					!canUpdate
				)}
				{@render settingRow(
					Search,
					'bg-orange-50 dark:bg-orange-900/20',
					'text-orange-500',
					'Mostrar Atajos de Creación',
					'Formularios de nuevos ingresos y creación de registros.',
					$spotlightSettings.showActions,
					() => spotlightSettings.toggleActions(),
					!canUpdate
				)}
				{@render settingRow(
					History,
					'bg-purple-50 dark:bg-purple-900/20',
					'text-purple-500',
					'Mostrar Búsquedas Recientes',
					'Muestra tus últimas acciones al abrir el buscador.',
					$spotlightSettings.showRecent,
					() => spotlightSettings.toggleRecent(),
					!canUpdate
				)}
				{@render settingRow(
					Type,
					'bg-emerald-50 dark:bg-emerald-900/20',
					'text-emerald-500',
					'Mostrar Descripciones',
					'Muestra información adicional bajo el título del resultado.',
					$spotlightSettings.showDescriptions,
					() => spotlightSettings.toggleDescriptions(),
					!canUpdate
				)}
			</div>
		</div>

		<!-- ================================================================== -->
		<!-- BÚSQUEDA PROFUNDA (TANTIVY) -->
		<!-- ================================================================== -->
		<div class="card-base p-5">
			<div class="flex items-center gap-4 mb-4">
				<div
					class="p-3 rounded-lg bg-amber-100 text-amber-600 dark:bg-amber-900/30 dark:text-amber-400"
				>
					<Database size={22} />
				</div>
				<div>
					<h3 class="text-lg font-semibold text-primary">Búsqueda Profunda</h3>
					<p class="text-sm text-secondary">
						Conecta Spotlight con el motor Tantivy para buscar datos reales.
					</p>
				</div>
			</div>

			<div class="divide-y divide-emphasis">
				{@render settingRow(
					Database,
					'bg-amber-50 dark:bg-amber-900/20',
					'text-amber-500',
					'Activar Tantivy Search',
					'Permite buscar por Cédula, Nombre o Empresa directamente.',
					$spotlightSettings.enableTantivySearch,
					() => spotlightSettings.toggleTantivy(),
					!canUpdate
				)}

				<div class="flex items-center justify-between py-4">
					<div class="flex items-center gap-3">
						<div class="p-2 rounded-md bg-gray-50 dark:bg-gray-800 text-secondary">
							<List size={18} />
						</div>
						<div class="flex flex-col">
							<span class="text-secondary font-medium">Límite de resultados de datos</span>
							<span class="text-xs text-secondary/60"
								>Cuántos resultados de Tantivy mostrar por búsqueda.</span
							>
						</div>
					</div>
					<select
						bind:value={$spotlightSettings.tantivyLimit}
						disabled={!$spotlightSettings.enableTantivySearch || !canUpdate}
						class="rounded-md border border-emphasis bg-surface-2 px-3 py-1.5 text-sm text-primary focus:outline-none focus:ring-2 focus:ring-accent disabled:opacity-50"
					>
						<option value={3}>3 resultados</option>
						<option value={5}>5 resultados</option>
						<option value={10}>10 resultados</option>
					</select>
				</div>
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
					if (confirm('¿Restaurar configuración de Spotlight?')) {
						spotlightSettings.reset();
					}
				}}
			>
				Restaurar Todo
			</button>
		</div>
	</div>
</div>
