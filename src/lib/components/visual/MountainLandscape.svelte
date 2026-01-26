<script lang="ts">
	import { generalSettings } from '$lib/stores/settingsStore';
	import { currentSeason } from '$lib/utils/season';
	import { MOUNTAIN_THEMES, getMountainBrightness } from './constants';
	import type { Season } from './types';

	// Reactive values (Svelte 5 syntax)
	let effectiveHour = $derived($generalSettings.overrideHour ?? new Date().getHours());
	let season = $derived(($generalSettings.overrideSeason ?? $currentSeason) as Season);

	// Get mountain colors based on season
	let theme = $derived(MOUNTAIN_THEMES[season]);

	// Get brightness based on time of day
	let brightness = $derived(getMountainBrightness(effectiveHour));
</script>

<div class="absolute inset-0 w-full h-full overflow-hidden pointer-events-none">
	<!-- Background Mountains (Farthest) -->
	<svg
		class="absolute bottom-0 w-full h-[60%] transition-all duration-1000 ease-in-out"
		viewBox="0 0 1440 320"
		preserveAspectRatio="none"
		style="filter: brightness({brightness}); color: {theme[0]}"
	>
		<path
			fill="currentColor"
			fill-opacity="0.85"
			d="M0,224L48,224C96,224,192,224,288,208C384,192,480,160,576,170.7C672,181,768,235,864,240C960,245,1056,203,1152,186.7C1248,171,1344,181,1392,186.7L1440,192L1440,320L1392,320C1344,320,1248,320,1152,320C1056,320,960,320,864,320C768,320,672,320,576,320C480,320,384,320,288,320C192,320,96,320,48,320L0,320Z"
		/>
	</svg>

	<!-- Midground Mountains -->
	<svg
		class="absolute bottom-0 w-full h-[50%] transition-all duration-1000 ease-in-out"
		viewBox="0 0 1440 320"
		preserveAspectRatio="none"
		style="filter: brightness({brightness}); color: {theme[1]}"
	>
		<path
			fill="currentColor"
			fill-opacity="0.9"
			d="M0,96L48,112C96,128,192,160,288,186.7C384,213,480,235,576,213.3C672,192,768,128,864,128C960,128,1056,192,1152,213.3C1248,235,1344,213,1392,202.7L1440,192L1440,320L1392,320C1344,320,1248,320,1152,320C1056,320,960,320,864,320C768,320,672,320,576,320C480,320,384,320,288,320C192,320,96,320,48,320L0,320Z"
		/>
	</svg>

	<!-- Foreground Mountains (Closest) -->
	<svg
		class="absolute bottom-0 w-full h-[35%] transition-all duration-1000 ease-in-out"
		viewBox="0 0 1440 320"
		preserveAspectRatio="none"
		style="filter: brightness({brightness}); color: {theme[2]}"
	>
		<path
			fill="currentColor"
			fill-opacity="1"
			d="M0,192L48,197.3C96,203,192,213,288,229.3C384,245,480,267,576,250.7C672,235,768,181,864,160C960,139,1056,149,1152,149.3C1248,149,1344,139,1392,133.3L1440,128L1440,320L1392,320C1344,320,1248,320,1152,320C1056,320,960,320,864,320C768,320,672,320,576,320C480,320,384,320,288,320C192,320,96,320,48,320L0,320Z"
		/>
	</svg>
</div>
