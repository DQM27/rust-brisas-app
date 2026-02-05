import { writable } from 'svelte/store';
import { browser } from '$app/environment';

// =============================================================================
// TYPES
// =============================================================================

export interface ParticleSettings {
	bokehCount: number;
	bokehMinSize: number;
	bokehMaxSize: number;
	bokehMaxOpacity: number;
	bokehSpeedMultiplier: number;
	globalSpeedMultiplier: number;
	weatherDensityMultiplier: number;
	weatherSpeedMultiplier: number;
	weatherSizeMultiplier: number;
	weatherWindInfluence: number;
	weatherTurbulence: number;
	moonPhase: string;
	sunStyle: string;
	sunrise: number;
	sunset: number;
	temperature: number;
	conditionText: string;
	starCountMultiplier: number;
	starTwinkleSpeed: number;
	shootingStarFrequency: number;
	shootingStarSpeed: number;
	meteorShowerEnabled: boolean;
	cloudStyle: 'cartoon' | 'soft';
	cloudOpacity: number;
	cloudCount: number;
	cloudWindSpeed: number;
	cloudTurbulence: number;
	autoWeather: boolean;
}

export const DEFAULT_PARTICLE_SETTINGS: ParticleSettings = {
	bokehCount: 25,
	bokehMinSize: 30,
	bokehMaxSize: 70,
	bokehMaxOpacity: 0.3,
	bokehSpeedMultiplier: 1.0,
	globalSpeedMultiplier: 1.0,
	weatherDensityMultiplier: 1.0,
	weatherSpeedMultiplier: 1.0,
	weatherSizeMultiplier: 1.0,
	weatherWindInfluence: 1.0,
	weatherTurbulence: 1.0,
	moonPhase: 'full',
	sunStyle: 'normal',
	sunrise: 6.0,
	sunset: 18.0,
	temperature: 25.0,
	conditionText: '',
	starCountMultiplier: 1.0,
	starTwinkleSpeed: 1.0,
	shootingStarFrequency: 1.0,
	shootingStarSpeed: 1.0,
	meteorShowerEnabled: false,
	cloudStyle: 'cartoon',
	cloudOpacity: 0.9,
	cloudCount: 5,
	cloudWindSpeed: 1.0,
	cloudTurbulence: 0.0,
	autoWeather: true
};

// =============================================================================
// STORAGE - DUAL WRITE
// =============================================================================

const STORAGE_KEY = 'particleSettings';

function loadFromLocalStorage(): ParticleSettings {
	if (!browser) return DEFAULT_PARTICLE_SETTINGS;
	try {
		const stored = localStorage.getItem(STORAGE_KEY);
		if (!stored) return DEFAULT_PARTICLE_SETTINGS;
		const parsed = JSON.parse(stored);
		// Migration: Clear "..." if present
		if (parsed.conditionText === '...') {
			parsed.conditionText = '';
		}
		return { ...DEFAULT_PARTICLE_SETTINGS, ...parsed };
	} catch {
		return DEFAULT_PARTICLE_SETTINGS;
	}
}

async function loadFromTauriStore(): Promise<ParticleSettings> {
	try {
		const { getSetting } = await import('../api/store');
		const stored = await getSetting<ParticleSettings>(STORAGE_KEY, DEFAULT_PARTICLE_SETTINGS);
		// Migration: Clear "..." if present
		if (stored && stored.conditionText === '...') {
			stored.conditionText = '';
		}
		return { ...DEFAULT_PARTICLE_SETTINGS, ...stored };
	} catch {
		return loadFromLocalStorage();
	}
}

async function saveToStorage(settings: ParticleSettings): Promise<void> {
	if (!browser) return;
	localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
	try {
		const { setSetting } = await import('../api/store');
		await setSetting(STORAGE_KEY, settings);
	} catch {
		// localStorage already saved
	}
}

// =============================================================================
// STORE
// =============================================================================

function createParticleSettingsStore() {
	const initial = loadFromLocalStorage();
	const { subscribe, set, update } = writable<ParticleSettings>({ ...initial, autoWeather: true });

	// ... storage logic ...

	return {
		subscribe,
		set,
		update,
		reset: () => set({ ...DEFAULT_PARTICLE_SETTINGS, autoWeather: true }),


		// Manual overrides disable auto-weather
		updateCloudStyle: (style: 'cartoon' | 'soft') => update((s) => ({ ...s, cloudStyle: style, autoWeather: false })),
		updateCloudOpacity: (opacity: number) => update((s) => ({ ...s, cloudOpacity: opacity, autoWeather: false })),
		updateCloudCount: (count: number) => update((s) => ({ ...s, cloudCount: count, autoWeather: false })),
		updateCloudWindSpeed: (speed: number) => update((s) => ({ ...s, cloudWindSpeed: speed, autoWeather: false })),
		updateCloudTurbulence: (turbulence: number) => update((s) => ({ ...s, cloudTurbulence: turbulence, autoWeather: false })),

		updateMoonPhase: (phase: string) => update((s) => ({ ...s, moonPhase: phase, autoWeather: false })),
		updateSunTimes: (sunrise: number, sunset: number) => update((s) => ({ ...s, sunrise, sunset, autoWeather: false })),
		updateSunStyle: (style: string) => update((s) => ({ ...s, sunStyle: style, autoWeather: false })),

		updateBokehCount: (count: number) => update((s) => ({ ...s, bokehCount: count, autoWeather: false })),
		updateBokehOpacity: (opacity: number) => update((s) => ({ ...s, bokehMaxOpacity: opacity, autoWeather: false })),

		updateWeatherDensity: (density: number) => update((s) => ({ ...s, weatherDensityMultiplier: density, autoWeather: false })),
		updateWeatherSpeed: (speed: number) => update((s) => ({ ...s, weatherSpeedMultiplier: speed, autoWeather: false })),
		updateWeatherSize: (size: number) => update((s) => ({ ...s, weatherSizeMultiplier: size, autoWeather: false })),
		updateWeatherWind: (wind: number) => update((s) => ({ ...s, weatherWindInfluence: wind, autoWeather: false })),
		updateWeatherTurbulence: (turbulence: number) => update((s) => ({ ...s, weatherTurbulence: turbulence, autoWeather: false })),

		toggleMeteorShower: () => update((s) => ({ ...s, meteorShowerEnabled: !s.meteorShowerEnabled, autoWeather: false })),
		updateStarCount: (count: number) => update((s) => ({ ...s, starCountMultiplier: count, autoWeather: false })),
		updateStarTwinkle: (speed: number) => update((s) => ({ ...s, starTwinkleSpeed: speed, autoWeather: false })),
		updateShootingStarFreq: (freq: number) => update((s) => ({ ...s, shootingStarFrequency: freq, autoWeather: false })),
		updateShootingStarSpeed: (speed: number) => update((s) => ({ ...s, shootingStarSpeed: speed, autoWeather: false })),

		applyWeatherConditions: (data: { code: number; temp: number; wind: number; text: string }) =>
			update((s) => {
				// Always update data
				const newState = {
					...s,
					temperature: data.temp,
					conditionText: data.text,
					// Always update wind influence from real weather? Maybe kept separate.
				};

				// Only apply visual overrides if autoWeather is ON
				if (s.autoWeather) {
					let cloudStyle: 'cartoon' | 'soft' = 'cartoon';
					let cloudCount = 5;
					let cloudOpacity = 0.9;
					let sunStyle = 'normal';

					if (data.code <= 1) {
						cloudCount = 3; cloudOpacity = 0.7;
					} else if (data.code <= 3) {
						cloudCount = 12; cloudOpacity = 0.95; cloudStyle = 'soft'; sunStyle = 'cloudy';
					} else if (data.code >= 51) {
						cloudCount = 20; cloudOpacity = 1.0; cloudStyle = 'soft'; sunStyle = 'cloudy';
					}

					newState.cloudStyle = cloudStyle;
					newState.cloudCount = cloudCount;
					newState.cloudOpacity = cloudOpacity;
					newState.sunStyle = sunStyle;
				}

				return newState;
			}),
		// ... rest of actions
	};
}

export const particleSettings = createParticleSettingsStore();
