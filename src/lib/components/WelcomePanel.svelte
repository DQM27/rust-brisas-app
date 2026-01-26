<script lang="ts">
	import { Users, ShieldAlert, LogIn, HardHat, FileText, Settings } from 'lucide-svelte';
	import { fade, fly } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
	import { currentUser } from '$lib/stores/auth';
	import { totalPersonasAdentro } from '$lib/stores/ingresoStore';
	import { onMount } from 'svelte';
	import { ingresoStore } from '$lib/stores/ingresoStore';
	import { generalSettings } from '$lib/stores/settingsStore';
	import { currentSeason } from '$lib/utils/season';
	import { getTimeOfDay } from '$lib/components/visual/constants';

	// Visual System Components
	import SceneRenderer from '$lib/components/visual/SceneRenderer.svelte';

	import Landscape from '$lib/components/visual/Landscape.svelte';
	import BirthdayCelebration from '$lib/components/visual/BirthdayCelebration.svelte';
	import { getCurrentWindow } from '@tauri-apps/api/window';

	// Load ingreso data on mount
	let wakeLock: WakeLockSentinel | null = null;

	async function requestWakeLock() {
		try {
			if ('wakeLock' in navigator) {
				wakeLock = await navigator.wakeLock.request('screen');
			}
		} catch (err) {
			const e = err as Error;
			console.error(`${e.name}, ${e.message}`);
		}
	}

	// Handle visibility change (tab switching releases lock, so request again when back)
	function handleVisibilityChange() {
		if (wakeLock !== null && document.visibilityState === 'visible') {
			requestWakeLock();
		}
	}

	async function toggleFullscreen() {
		try {
			const appWindow = getCurrentWindow();
			const current = await appWindow.isFullscreen();
			await appWindow.setFullscreen(!current);
			generalSettings.update((s) => ({ ...s, isKioskMode: !current }));
		} catch {
			if (!document.fullscreenElement) {
				document.documentElement.requestFullscreen().catch((err) => console.log(err));
				generalSettings.update((s) => ({ ...s, isKioskMode: true }));
			} else {
				document.exitFullscreen().catch((err) => console.log(err));
				generalSettings.update((s) => ({ ...s, isKioskMode: false }));
			}
		}
	}

	// Sync state on mount and listen for external changes (like Esc key)
	onMount(() => {
		(async () => {
			ingresoStore.load();
			requestWakeLock();
			document.addEventListener('visibilitychange', handleVisibilityChange);

			try {
				const appWindow = getCurrentWindow();
				const isFull = await appWindow.isFullscreen();
				generalSettings.update((s) => ({ ...s, isKioskMode: isFull }));
			} catch (e) {
				console.warn('Error fetching window fullscreen status:', e);
			}
		})();

		// Listen for Escape key to exit kiosk mode
		const handleKeydown = async (e: KeyboardEvent) => {
			if (e.key === 'Escape') {
				try {
					const appWindow = getCurrentWindow();
					if (await appWindow.isFullscreen()) {
						await appWindow.setFullscreen(false);
						generalSettings.update((s) => ({ ...s, isKioskMode: false }));
					}
				} catch {
					if (document.fullscreenElement) {
						document.exitFullscreen();
						generalSettings.update((s) => ({ ...s, isKioskMode: false }));
					}
				}
			}
		};
		window.addEventListener('keydown', handleKeydown);
		return () => window.removeEventListener('keydown', handleKeydown);
	});

	// Birthday Logic 🎂 - Check real birthday OR override from settings
	function checkBirthday(dateString?: string | null): boolean {
		if (!dateString) return false;
		try {
			if (dateString.match(/^\d{4}-\d{2}-\d{2}$/)) {
				const [month, day] = dateString.split('-').slice(1).map(Number);
				const today = new Date();
				return today.getMonth() + 1 === month && today.getDate() === day;
			}
			const birth = new Date(dateString);
			const today = new Date();
			return birth.getMonth() === today.getMonth() && birth.getDate() === today.getDate();
		} catch (e) {
			console.error('Error checking birthday', e);
			return false;
		}
	}

	let isBirthday = $derived(
		$generalSettings.overrideBirthday || checkBirthday($currentUser?.fechaNacimiento)
	);

	const modules = [
		{
			icon: LogIn,
			title: 'Control de Acceso',
			description: 'Registro de ingresos y salidas de contratistas',
			stat: totalPersonasAdentro,
			statLabel: 'Adentro',
			color: 'text-green-500',
			delay: 0
		},
		{
			icon: HardHat,
			title: 'Contratistas',
			description: 'Gestión de personal, empresas y vehículos',
			delay: 50
		},
		{
			icon: ShieldAlert,
			title: 'Seguridad',
			description: 'Listas negras, alertas de gafetes y bloqueos',
			delay: 100
		},
		{
			icon: Users,
			title: 'Usuarios',
			description: 'Administración de usuarios y permisos del sistema',
			delay: 150
		},
		{
			icon: FileText,
			title: 'Reportes',
			description: 'Exportación de datos y generación de informes PDF',
			delay: 200
		},
		{
			icon: Settings,
			title: 'Configuración',
			description: 'Ajustes generales del sistema y preferencias globales',
			delay: 250
		}
	];

	// Time-based text styling
	let effectiveHour = $derived($generalSettings.overrideHour ?? new Date().getHours());
	let timeOfDay = $derived(getTimeOfDay(effectiveHour));
	let isDay = $derived(timeOfDay === 'day' || timeOfDay === 'morning');

	// Use dark text only if it's daytime AND NOT winter (winter mountains are dark) AND background is shown
	let useDarkText = $derived(
		$generalSettings.showBackground && isDay && $currentSeason !== 'winter'
	);

	let textColorClass = $derived(
		isBirthday
			? 'text-white drop-shadow-xl tracking-wide'
			: useDarkText
				? 'text-slate-800 drop-shadow-sm'
				: 'text-white drop-shadow-md'
	);

	// Dynamic greeting based on time
	let currentHour = $derived(new Date().getHours());
	let greeting = $derived(
		isBirthday
			? '¡Feliz Cumpleaños!'
			: currentHour < 6
				? 'Feliz madrugada'
				: currentHour < 12
					? 'Buenos días'
					: currentHour < 18
						? 'Buenas tardes'
						: 'Buenas noches'
	);
</script>

<div
	class="relative flex h-full items-center justify-center bg-surface-1 px-6 overflow-hidden select-none transition-all duration-300 {$generalSettings.isKioskMode
		? 'fixed inset-0 z-[9999]'
		: ''}"
	ondblclick={toggleFullscreen}
	role="presentation"
>
	<!-- Visual Background System -->
	{#if isBirthday}
		<BirthdayCelebration name={$currentUser?.nombre || 'Usuario'} />
		<div class="absolute inset-0 bg-black/10 pointer-events-none z-[3]"></div>
	{:else if $generalSettings.showBackground}
		<SceneRenderer isBirthday={false}>
			<Landscape />
		</SceneRenderer>
	{:else}
		<!-- Minimal mode: just celestial cycle and weather (no mountains) -->
		<SceneRenderer isBirthday={false} />
	{/if}

	<!-- Content -->
	<div class="relative z-10 w-full max-w-5xl text-center">
		<!-- Header with animations (solo si showWelcomeText está activo) -->
		{#if $generalSettings.showWelcomeText}
			<div
				in:fly={{ y: -30, duration: 800, easing: quintOut }}
				class="flex flex-col items-center {$generalSettings.showWelcomeCards
					? 'mb-4 md:mb-6'
					: 'mb-12 md:mb-16'}"
			>
				{#if isBirthday}
					<!-- Special Birthday Design 🎂 -->
					<div class="flex flex-col items-center gap-2">
						<h2
							class="{$generalSettings.showWelcomeCards
								? 'text-3xl md:text-4xl mb-2'
								: 'text-5xl md:text-6xl mb-4'} font-black text-white drop-shadow-lg tracking-widest uppercase"
						>
							{greeting}
						</h2>
						<h1
							class="{$generalSettings.showWelcomeCards
								? 'text-5xl md:text-7xl'
								: 'text-7xl md:text-9xl'} font-black text-transparent bg-clip-text bg-gradient-to-r from-yellow-200 via-amber-200 to-yellow-400 drop-shadow-xl animate-pulse"
							style="filter: drop-shadow(0 4px 6px rgba(0,0,0,0.3));"
						>
							{$currentUser?.nombre || 'Usuario'}
						</h1>
					</div>
				{:else}
					<!-- Normal Greeting -->
					<h2
						class="tracking-wide transition-colors duration-1000 {$generalSettings.showWelcomeCards
							? 'text-xl md:text-2xl mb-0'
							: 'text-3xl md:text-4xl mb-1'} font-medium {useDarkText
							? 'text-slate-700/80'
							: 'text-white/90 drop-shadow-md'}"
					>
						{greeting}
					</h2>

					<!-- Elegant Adaptive Name -->
					<h1
						class="tracking-tight transition-all duration-1000 font-bold
            {$generalSettings.showWelcomeCards ? 'text-4xl md:text-5xl' : 'text-6xl md:text-7xl'}
            {useDarkText
							? 'text-transparent bg-clip-text bg-gradient-to-br from-slate-800 via-slate-600 to-slate-800 drop-shadow-sm'
							: 'text-transparent bg-clip-text bg-gradient-to-b from-white via-white to-blue-50 drop-shadow-lg'}"
						style={useDarkText ? '' : 'filter: drop-shadow(0 2px 4px rgba(0,0,0,0.2));'}
					>
						{$currentUser?.nombre || 'Usuario'}
					</h1>
				{/if}

				<p
					class="transition-colors duration-1000 {textColorClass} {isBirthday
						? 'text-xl font-bold mt-2'
						: $generalSettings.showWelcomeCards
							? 'text-sm mt-2 opacity-80'
							: 'text-xl font-medium tracking-wide opacity-90 mt-6'}"
				>
					{isBirthday
						? 'Te desea el equipo de Mega Brisas 🎉'
						: 'Mega Brisas - Sistema Integral de Control de Acceso'}
				</p>
			</div>
		{/if}

		<!-- Modules Grid -->
		{#if $generalSettings.showWelcomeCards}
			<div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3" transition:fade={{ duration: 300 }}>
				{#each modules as module, i (i)}
					{@const Icon = module.icon}
					<div
						in:fly={{
							y: 50,
							duration: 600,
							delay: module.delay,
							easing: quintOut
						}}
						class="group relative overflow-hidden rounded-lg border border-emphasis bg-surface-2 p-6 text-left transition-all duration-300 hover:-translate-y-2 hover:border-accent hover:shadow-lg hover:shadow-accent/20"
					>
						<!-- Shine effect on hover -->
						<div
							class="absolute inset-0 -translate-x-full bg-gradient-to-r from-transparent via-surface-1/5 to-transparent transition-transform duration-700 group-hover:translate-x-full"
						></div>

						<!-- Content -->
						<div class="relative z-10 flex flex-col h-full justify-between">
							<div>
								<!-- Icon with scale animation -->
								<div
									class="mb-4 inline-flex rounded-lg bg-surface-1 p-3 {module.color ||
										'text-accent'} transition-all duration-300 group-hover:scale-110 group-hover:bg-accent/10"
								>
									<Icon size={28} strokeWidth={2} />
								</div>

								<h3
									class="mb-2 text-lg font-semibold text-primary transition-colors duration-300 group-hover:text-accent"
								>
									{module.title}
								</h3>

								<p class="text-sm leading-relaxed text-secondary">
									{module.description}
								</p>
							</div>

							<!-- Optional stat -->
							{#if module.stat}
								<div class="mt-4 flex items-center gap-2 border-t border-emphasis pt-3">
									<span class="text-2xl font-bold text-primary">{$totalPersonasAdentro}</span>
									<span class="text-xs font-medium text-secondary uppercase tracking-wider"
										>{module.statLabel}</span
									>
									<div class="ml-auto h-2 w-2 animate-pulse rounded-full bg-green-500"></div>
								</div>
							{/if}
						</div>

						<!-- Animated bottom border -->
						<div
							class="absolute bottom-0 left-0 h-1 w-0 bg-gradient-to-r from-[#007acc] to-[#0098ff] transition-all duration-500 group-hover:w-full"
						></div>
					</div>
				{/each}
			</div>
		{/if}
	</div>
</div>
