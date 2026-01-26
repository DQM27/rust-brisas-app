<script lang="ts">
	import { generalSettings, type Season } from '$lib/stores/settingsStore';
	import { scale, slide } from 'svelte/transition';
	import VisualCustomizationPanel from './VisualCustomizationPanel.svelte';
	import {
		CloudRain,
		Check,
		X,
		Mountain,
		Sun,
		Moon,
		Cloud,
		Sparkles,
		Star,
		Cake,
		RotateCcw,
		Eye,
		Sliders,
		Trees,
		Building2,
		Umbrella,
		Type,
		Layout
	} from 'lucide-svelte';
	import { LANDSCAPE_TYPES } from '$lib/components/visual/systems/landscapeData';
	import { can } from '$lib/logic/permissions';
	import { currentUser } from '$lib/stores/auth';

	// Permisos
	const canUpdate = $derived($currentUser && can($currentUser, 'UPDATE_SETTINGS_VISUAL'));

	let showAdvancedCustomization = $state(false);
	let showWeatherCustomization = $state(false);
	let showCelestialCustomization = $state(false);
	let showStarCustomization = $state(false);
	let showCloudCustomization = $state(false);
	let showLandscapeCustomization = $state(false);

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
	icon: any,
	iconBg: string,
	iconColor: string,
	label: string,
	checked: boolean,
	onChange: () => void,
	disabled: boolean = false
)}
	{@const Icon = icon}
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
		<h2 class="text-2xl font-bold text-primary">Ajustes Gráficos</h2>
		<p class="text-secondary mt-1">
			Configura las preferencias visuales del sistema (paisajes, clima, efectos).
		</p>
	</div>

	<div class="grid gap-4 max-w-3xl pb-8">
		<!-- ================================================================== -->
		<!-- UI ELEMENTS CARD -->
		<!-- ================================================================== -->
		<div class="card-base p-5">
			<div class="flex items-center gap-4 mb-4">
				<div
					class="p-3 rounded-lg bg-violet-100 text-violet-600 dark:bg-violet-900/30 dark:text-violet-400"
				>
					<Layout size={22} />
				</div>
				<div>
					<h3 class="text-lg font-semibold text-primary">Interfaz</h3>
					<p class="text-sm text-secondary">Configura los elementos de la interfaz.</p>
				</div>
			</div>

			<div class="divide-y divide-emphasis">
				{@render settingRow(
					Type,
					'bg-cyan-50 dark:bg-cyan-900/20',
					'text-cyan-500',
					'Texto de Bienvenida',
					$generalSettings.showWelcomeText,
					() => generalSettings.toggleWelcomeText(),
					!canUpdate
				)}

				{@render settingRow(
					Layout,
					'bg-violet-50 dark:bg-violet-900/20',
					'text-violet-500',
					'Tarjetas de Módulos',
					$generalSettings.showWelcomeCards,
					() => generalSettings.toggleCards(),
					!canUpdate
				)}
			</div>
		</div>
		<!-- ================================================================== -->
		<!-- VISUAL ELEMENTS CARD -->
		<!-- ================================================================== -->

		<div class="card-base p-5">
			<div class="flex items-center gap-4 mb-4">
				<div
					class="p-3 rounded-lg bg-purple-100 text-purple-600 dark:bg-purple-900/30 dark:text-purple-400"
				>
					<Eye size={22} />
				</div>
				<div>
					<h3 class="text-lg font-semibold text-primary">Elementos Visuales</h3>
					<p class="text-sm text-secondary">Activa o desactiva cada capa del paisaje.</p>
				</div>
			</div>

			<div class="divide-y divide-emphasis">
				{@render settingRow(
					Mountain,
					'bg-emerald-50 dark:bg-emerald-900/20',
					'text-emerald-500',
					'Paisaje',
					$generalSettings.showBackground,
					() => generalSettings.toggleBackground(),
					!canUpdate
				)}

				{#if $generalSettings.showBackground}
					<div class="pr-4 pt-1 pb-4" transition:slide={{ duration: 200 }}>
						<button
							class="flex items-center gap-2 text-sm font-medium text-blue-500 hover:text-blue-600 hover:underline transition-colors w-fit ml-1 mb-2"
							onclick={() => (showLandscapeCustomization = !showLandscapeCustomization)}
						>
							<Sliders size={14} />
							{showLandscapeCustomization ? 'Ocultar' : 'Personalizar'}
						</button>

						{#if showLandscapeCustomization}
							<div class="pl-12 pt-2 relative" transition:slide={{ duration: 200 }}>
								{#if !canUpdate}
									<div
										class="absolute inset-0 z-10 bg-white/50 dark:bg-black/50 cursor-not-allowed"
									></div>
								{/if}
								<div class="grid grid-cols-3 gap-2">
									{#each LANDSCAPE_TYPES as biome}
										<button
											class="flex flex-col items-center justify-center gap-1.5 p-2 rounded-lg border transition-all duration-200
                      {$generalSettings.landscapeType === biome.id
												? 'bg-blue-500/10 border-blue-500 text-blue-500'
												: 'bg-surface-2 border-white/5 text-secondary hover:bg-surface-3 hover:border-white/10'}"
											onclick={() => generalSettings.setLandscapeType(biome.id)}
											title={biome.name}
										>
											<!-- Icon -->
											{#if biome.icon === 'Mountain'}
												<Mountain size={16} />
											{:else if biome.icon === 'Trees'}
												<Trees size={16} />
											{:else if biome.icon === 'Building2'}
												<Building2 size={16} />
											{:else if biome.icon === 'Sun'}
												<Sun size={16} />
											{:else if biome.icon === 'Umbrella'}
												<Umbrella size={16} />
											{:else if biome.icon === 'Moon'}
												<Moon size={16} />
											{/if}

											<span class="text-[10px] font-medium">{biome.name}</span>
										</button>
									{/each}
								</div>
							</div>
						{/if}
					</div>
				{/if}

				{@render settingRow(
					Cloud,
					'bg-sky-50 dark:bg-sky-900/20',
					'text-sky-500',
					'Nubes Animadas',
					$generalSettings.showClouds,
					() => generalSettings.toggleClouds(),
					!canUpdate
				)}

				{#if $generalSettings.showClouds}
					<div class="pr-4 pt-1 pb-4" transition:slide={{ duration: 200 }}>
						<button
							class="flex items-center gap-2 text-sm font-medium text-blue-500 hover:text-blue-600 hover:underline transition-colors w-fit ml-1"
							onclick={() => (showCloudCustomization = !showCloudCustomization)}
						>
							<Sliders size={14} />
							{showCloudCustomization ? 'Ocultar' : 'Personalizar'}
						</button>

						{#if showCloudCustomization}
							<div class="relative">
								{#if !canUpdate}
									<div
										class="absolute inset-0 z-10 bg-white/50 dark:bg-black/50 cursor-not-allowed"
									></div>
								{/if}
								<VisualCustomizationPanel embedded={true} mode="clouds" />
							</div>
						{/if}
					</div>
				{/if}

				{@render settingRow(
					Star,
					'bg-indigo-50 dark:bg-indigo-900/20',
					'text-indigo-400',
					'Estrellas (Noche)',
					$generalSettings.showStars,
					() => generalSettings.toggleStars(),
					!canUpdate
				)}

				{#if $generalSettings.showStars}
					<div class="pr-4 pt-1 pb-4" transition:slide={{ duration: 200 }}>
						<button
							class="flex items-center gap-2 text-sm font-medium text-blue-500 hover:text-blue-600 hover:underline transition-colors w-fit ml-1"
							onclick={() => (showStarCustomization = !showStarCustomization)}
						>
							<Sliders size={14} />
							{showStarCustomization ? 'Ocultar' : 'Personalizar'}
						</button>

						{#if showStarCustomization}
							<div class="relative">
								{#if !canUpdate}
									<div
										class="absolute inset-0 z-10 bg-white/50 dark:bg-black/50 cursor-not-allowed"
									></div>
								{/if}
								<VisualCustomizationPanel embedded={true} mode="stars" />
							</div>
						{/if}
					</div>
				{/if}

				{@render settingRow(
					Sun,
					'bg-amber-50 dark:bg-amber-900/20',
					'text-amber-500',
					'Sol / Luna',
					$generalSettings.showCelestial,
					() => generalSettings.toggleCelestial(),
					!canUpdate
				)}

				{#if $generalSettings.showCelestial}
					<div class="pr-4 pt-1 pb-4" transition:slide={{ duration: 200 }}>
						<button
							class="flex items-center gap-2 text-sm font-medium text-blue-500 hover:text-blue-600 hover:underline transition-colors w-fit ml-1"
							onclick={() => (showCelestialCustomization = !showCelestialCustomization)}
						>
							<Sliders size={14} />
							{showCelestialCustomization ? 'Ocultar' : 'Personalizar'}
						</button>

						{#if showCelestialCustomization}
							<div class="relative">
								{#if !canUpdate}
									<div
										class="absolute inset-0 z-10 bg-white/50 dark:bg-black/50 cursor-not-allowed"
									></div>
								{/if}
								<VisualCustomizationPanel embedded={true} mode="celestial" />
							</div>
						{/if}
					</div>
				{/if}

				{@render settingRow(
					CloudRain,
					'bg-blue-50 dark:bg-blue-900/20',
					'text-blue-500',
					'Efectos Climáticos',
					$generalSettings.enableWeatherEffects,
					() => generalSettings.toggleWeather(),
					!canUpdate
				)}

				{#if $generalSettings.enableWeatherEffects}
					<div class="pr-4 pt-1 pb-4" transition:slide={{ duration: 200 }}>
						<button
							class="flex items-center gap-2 text-sm font-medium text-blue-500 hover:text-blue-600 hover:underline transition-colors w-fit ml-1"
							onclick={() => (showWeatherCustomization = !showWeatherCustomization)}
						>
							<Sliders size={14} />
							{showWeatherCustomization ? 'Ocultar' : 'Personalizar'}
						</button>

						{#if showWeatherCustomization}
							<div class="relative">
								{#if !canUpdate}
									<div
										class="absolute inset-0 z-10 bg-white/50 dark:bg-black/50 cursor-not-allowed"
									></div>
								{/if}
								<VisualCustomizationPanel embedded={true} mode="weather" />
							</div>
						{/if}
					</div>
				{/if}

				{@render settingRow(
					Sparkles,
					'bg-pink-50 dark:bg-pink-900/20',
					'text-pink-500',
					'Efecto Bokeh',
					$generalSettings.showBokeh,
					() => generalSettings.toggleBokeh(),
					!canUpdate
				)}

				{#if $generalSettings.showBokeh}
					<div class="pr-4 pt-1 pb-4" transition:slide={{ duration: 200 }}>
						<button
							class="flex items-center gap-2 text-sm font-medium text-blue-500 hover:text-blue-600 hover:underline transition-colors w-fit ml-1"
							onclick={() => (showAdvancedCustomization = !showAdvancedCustomization)}
						>
							<Sliders size={14} />
							{showAdvancedCustomization ? 'Ocultar' : 'Personalizar'}
						</button>

						{#if showAdvancedCustomization}
							<div class="relative">
								{#if !canUpdate}
									<div
										class="absolute inset-0 z-10 bg-white/50 dark:bg-black/50 cursor-not-allowed"
									></div>
								{/if}
								<VisualCustomizationPanel embedded={true} />
							</div>
						{/if}
					</div>
				{/if}
			</div>
		</div>

		<!-- ================================================================== -->
		<!-- TIME CONTROL CARD -->
		<!-- ================================================================== -->
		<div class="card-base p-5">
			<div class="flex items-center justify-between mb-4">
				<div class="flex items-center gap-4">
					<div
						class="p-3 rounded-lg bg-orange-100 text-orange-600 dark:bg-orange-900/30 dark:text-orange-400"
					>
						{#if $generalSettings.overrideHour !== null && $generalSettings.overrideHour >= 6 && $generalSettings.overrideHour < 18}
							<Sun size={22} />
						{:else}
							<Moon size={22} />
						{/if}
					</div>
					<div>
						<h3 class="text-lg font-semibold text-primary">Ciclo Día/Noche</h3>
						<p class="text-sm text-secondary">Controla la hora manualmente.</p>
					</div>
				</div>

				{#if $generalSettings.overrideHour !== null}
					<button
						class="flex items-center gap-1 text-xs text-accent hover:underline disabled:opacity-50 disabled:cursor-not-allowed"
						onclick={() => ($generalSettings.overrideHour = null)}
						disabled={!canUpdate}
					>
						<RotateCcw size={12} />
						Auto
					</button>
				{/if}
			</div>

			<div class="space-y-3">
				<!-- Time Display -->
				<div class="flex items-center justify-between">
					<span class="text-sm text-secondary">Hora simulada:</span>
					<span class="font-mono text-lg font-semibold text-primary">
						{#if $generalSettings.overrideHour !== null}
							{Math.floor($generalSettings.overrideHour).toString().padStart(2, '0')}:{Math.round(
								($generalSettings.overrideHour % 1) * 60
							)
								.toString()
								.padStart(2, '0')}
						{:else}
							<span class="text-accent">Tiempo Real</span>
						{/if}
					</span>
				</div>

				<!-- Time Slider -->
				<div class="flex items-center gap-3">
					<Moon size={16} class="text-indigo-400" />
					<input
						type="range"
						min="0"
						max="24"
						step="0.25"
						disabled={!canUpdate}
						class="flex-1 h-2 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-accent disabled:opacity-50"
						value={$generalSettings.overrideHour ??
							new Date().getHours() + new Date().getMinutes() / 60}
						oninput={(e) =>
							($generalSettings.overrideHour = parseFloat(e.currentTarget.value) % 24)}
					/>
					<Sun size={16} class="text-amber-400" />
				</div>

				<!-- Quick Time Buttons -->
				<div class="flex gap-2 pt-2">
					{#each [{ label: '🌅 6:00', hour: 6 }, { label: '☀️ 12:00', hour: 12 }, { label: '🌆 18:00', hour: 18 }, { label: '🌙 0:00', hour: 0 }] as preset}
						<button
							class="flex-1 py-1.5 px-2 text-xs rounded-md transition-colors disabled:opacity-50 disabled:cursor-not-allowed
                {$generalSettings.overrideHour === preset.hour
								? 'bg-accent text-white'
								: 'bg-surface-2 hover:bg-surface-hover text-secondary'}"
							onclick={() => ($generalSettings.overrideHour = preset.hour)}
							disabled={!canUpdate}
						>
							{preset.label}
						</button>
					{/each}
				</div>
			</div>
		</div>

		<!-- ================================================================== -->
		<!-- SEASON PREVIEW CARD -->
		<!-- ================================================================== -->
		<div class="card-base p-5">
			<div class="flex items-center justify-between mb-4">
				<div class="flex items-center gap-4">
					<div
						class="p-3 rounded-lg bg-pink-100 text-pink-600 dark:bg-pink-900/30 dark:text-pink-400"
					>
						<Sparkles size={22} />
					</div>
					<div>
						<h3 class="text-lg font-semibold text-primary">Estación</h3>
						<p class="text-sm text-secondary">Previsualiza efectos estacionales.</p>
					</div>
				</div>

				{#if $generalSettings.overrideSeason !== null}
					<button
						class="flex items-center gap-1 text-xs text-accent hover:underline disabled:opacity-50 disabled:cursor-not-allowed"
						onclick={() => ($generalSettings.overrideSeason = null)}
						disabled={!canUpdate}
					>
						<RotateCcw size={12} />
						Auto
					</button>
				{/if}
			</div>

			<div class="grid grid-cols-6 gap-2">
				{#each [{ label: 'Auto', value: null, icon: '🔄', bg: 'bg-gray-500' }, { label: 'Invierno', value: 'winter' as Season, icon: '❄️', bg: 'bg-blue-500' }, { label: 'Primavera', value: 'spring' as Season, icon: '🌸', bg: 'bg-pink-400' }, { label: 'Verano', value: 'summer' as Season, icon: '✨', bg: 'bg-yellow-500' }, { label: 'Otoño', value: 'autumn' as Season, icon: '🍂', bg: 'bg-orange-500' }, { label: 'Lluvia', value: 'rain' as Season, icon: '🌧️', bg: 'bg-slate-500' }] as season}
					<button
						class="flex flex-col items-center gap-1 py-2 px-1 rounded-lg transition-all
                {$generalSettings.overrideSeason === season.value
							? `${season.bg} text-white scale-105 shadow-md`
							: 'bg-surface-2 hover:bg-surface-hover text-secondary hover:scale-102'}"
						onclick={() => ($generalSettings.overrideSeason = season.value)}
					>
						<span class="text-lg">{season.icon}</span>
						<span class="text-xs font-medium">{season.label}</span>
					</button>
				{/each}
			</div>
		</div>

		<!-- ================================================================== -->
		<!-- BIRTHDAY TEST CARD -->
		<!-- ================================================================== -->
		<div class="card-base p-5">
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-4">
					<div
						class="p-3 rounded-lg bg-rose-100 text-rose-600 dark:bg-rose-900/30 dark:text-rose-400"
					>
						<Cake size={22} />
					</div>
					<div>
						<h3 class="text-lg font-semibold text-primary">Modo Cumpleaños</h3>
						<p class="text-sm text-secondary">Prueba la celebración de cumpleaños.</p>
					</div>
				</div>

				{@render toggleSwitch(
					$generalSettings.overrideBirthday,
					() => generalSettings.toggleBirthdayTest(),
					'Activar modo cumpleaños',
					!canUpdate
				)}
			</div>

			{#if $generalSettings.overrideBirthday}
				<div
					class="mt-3 p-3 rounded-md bg-rose-50 dark:bg-rose-900/20 text-rose-700 dark:text-rose-300 text-sm"
					transition:slide={{ duration: 150 }}
				>
					🎉 ¡Modo cumpleaños activado! Revisa la pantalla de bienvenida.
				</div>
			{/if}
		</div>

		<!-- ================================================================== -->
		<!-- RESET BUTTON -->
		<!-- ================================================================== -->
		<div class="flex justify-end gap-2 pt-2">
			<button
				class="btn-base bg-surface-2 hover:bg-surface-hover text-secondary text-sm disabled:opacity-50 disabled:cursor-not-allowed"
				onclick={() => generalSettings.resetOverrides()}
				disabled={!canUpdate}
			>
				<RotateCcw size={14} />
				Resetear Previews
			</button>

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
