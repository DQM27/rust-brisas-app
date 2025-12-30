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
        return { ...DEFAULT_PARTICLE_SETTINGS, ...JSON.parse(stored) };
    } catch {
        return DEFAULT_PARTICLE_SETTINGS;
    }
}

async function loadFromTauriStore(): Promise<ParticleSettings> {
    try {
        const { getSetting } = await import('$lib/services/storeService');
        const stored = await getSetting<ParticleSettings>(STORAGE_KEY, DEFAULT_PARTICLE_SETTINGS);
        return { ...DEFAULT_PARTICLE_SETTINGS, ...stored };
    } catch {
        return loadFromLocalStorage();
    }
}

async function saveToStorage(settings: ParticleSettings): Promise<void> {
    if (!browser) return;
    localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
    try {
        const { setSetting } = await import('$lib/services/storeService');
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
    const { subscribe, set, update } = writable<ParticleSettings>(initial);

    // Load from Tauri Store after hydration
    if (browser) {
        loadFromTauriStore().then(set);
    }

    // Auto-save on changes
    if (browser) {
        subscribe(settings => saveToStorage(settings));
    }

    return {
        subscribe,
        set,
        update,
        reset: () => set(DEFAULT_PARTICLE_SETTINGS),
        updateBokehCount: (count: number) => update(s => ({ ...s, bokehCount: count })),
        updateBokehOpacity: (opacity: number) => update(s => ({ ...s, bokehMaxOpacity: opacity })),
        updateBokehSize: (min: number, max: number) => update(s => ({ ...s, bokehMinSize: min, bokehMaxSize: max })),
        updateWeatherDensity: (m: number) => update(s => ({ ...s, weatherDensityMultiplier: m })),
        updateWeatherSpeed: (m: number) => update(s => ({ ...s, weatherSpeedMultiplier: m })),
        updateWeatherSize: (m: number) => update(s => ({ ...s, weatherSizeMultiplier: m })),
        updateWeatherWind: (m: number) => update(s => ({ ...s, weatherWindInfluence: m })),
        updateWeatherTurbulence: (m: number) => update(s => ({ ...s, weatherTurbulence: m })),
        updateMoonPhase: (phase: string) => update(s => ({ ...s, moonPhase: phase })),
        updateSunStyle: (style: string) => update(s => ({ ...s, sunStyle: style })),
        updateStarCount: (m: number) => update(s => ({ ...s, starCountMultiplier: m })),
        updateStarTwinkle: (m: number) => update(s => ({ ...s, starTwinkleSpeed: m })),
        updateShootingStarFreq: (m: number) => update(s => ({ ...s, shootingStarFrequency: m })),
        updateShootingStarSpeed: (m: number) => update(s => ({ ...s, shootingStarSpeed: m })),
        toggleMeteorShower: () => update(s => ({ ...s, meteorShowerEnabled: !s.meteorShowerEnabled })),
        updateCloudStyle: (style: 'cartoon' | 'soft') => update(s => ({ ...s, cloudStyle: style })),
        updateCloudOpacity: (opacity: number) => update(s => ({ ...s, cloudOpacity: opacity })),
        updateCloudCount: (count: number) => update(s => ({ ...s, cloudCount: count })),
        updateCloudWindSpeed: (speed: number) => update(s => ({ ...s, cloudWindSpeed: speed })),
        updateCloudTurbulence: (turb: number) => update(s => ({ ...s, cloudTurbulence: turb })),
    };
}

export const particleSettings = createParticleSettingsStore();
