import { writable } from 'svelte/store';
import { browser } from '$app/environment';
import type { ParticleConfig } from '../components/visual/types';

// =============================================================================
// TYPES
// =============================================================================

export interface BokehConfig {
    enabled: boolean;
    count: number;
    minSize: number;
    maxSize: number;
    minSpeed: number;
    maxSpeed: number;
    minOpacity: number;
    maxOpacity: number;
    colorSaturation: number;
}

export interface WeatherConfigs {
    winter: ParticleConfig;
    spring: ParticleConfig;
    summer: ParticleConfig & { nightVariant?: ParticleConfig };
    autumn: ParticleConfig;
}

export interface ParticleSettings {
    bokeh: BokehConfig;
    weather: WeatherConfigs;
}

// =============================================================================
// DEFAULTS
// =============================================================================

const DEFAULT_BOKEH_CONFIG: BokehConfig = {
    enabled: true,
    count: 25,
    minSize: 30,
    maxSize: 70,
    minSpeed: 0.03,
    maxSpeed: 0.1,
    minOpacity: 0.1,
    maxOpacity: 0.3,
    colorSaturation: 0.8,
};

const DEFAULT_WEATHER_CONFIGS: WeatherConfigs = {
    winter: {
        count: 80,
        colors: ['#ffffff', '#e8f4ff', '#d0e8ff'],
        sizeRange: [3, 7],
        speedRange: [0.5, 1.5],
        rotates: false,
        glows: false,
    },
    spring: {
        count: 40,
        colors: ['#ffb7c5', '#ffc0cb', '#ffe4e8', '#ffffff'],
        sizeRange: [6, 12],
        speedRange: [0.3, 0.8],
        rotates: true,
        glows: false,
    },
    summer: {
        // Day
        count: 30,
        colors: ['#fffdd0', '#fff8c0', '#ffffff'],
        sizeRange: [2, 4],
        speedRange: [0.5, 1.5],
        rotates: false,
        glows: false,
        // Night
        nightVariant: {
            count: 25,
            colors: ['#ffff88', '#aaffaa', '#ffffff', '#e0ffe0', '#ffffe0'],
            sizeRange: [3, 5],
            speedRange: [0.01, 0.03],
            rotates: false,
            glows: true,
        },
    },
    autumn: {
        count: 50,
        colors: ['#cd5c5c', '#d2691e', '#daa520', '#8b4513', '#ff6347'],
        sizeRange: [8, 15],
        speedRange: [0.5, 1.2],
        rotates: true,
        glows: false,
    },
};

const DEFAULT_SETTINGS: ParticleSettings = {
    bokeh: DEFAULT_BOKEH_CONFIG,
    weather: DEFAULT_WEATHER_CONFIGS,
};

// =============================================================================
// STORE
// =============================================================================

const STORAGE_KEY = 'brisas-particle-settings';

function loadFromStorage(): ParticleSettings {
    if (!browser) return DEFAULT_SETTINGS;

    try {
        const stored = localStorage.getItem(STORAGE_KEY);
        if (!stored) return DEFAULT_SETTINGS;

        const parsed = JSON.parse(stored);

        // Deep merge to ensure new keys exist
        return {
            ...DEFAULT_SETTINGS,
            ...parsed,
            bokeh: { ...DEFAULT_SETTINGS.bokeh, ...(parsed.bokeh || {}) },
            weather: {
                ...DEFAULT_SETTINGS.weather,
                ...(parsed.weather || {}),
                // Ensure nested objects are merged correctly if needed
                summer: {
                    ...DEFAULT_SETTINGS.weather.summer,
                    ...(parsed.weather?.summer || {}),
                }
            }
        };
    } catch (e) {
        console.warn('Failed to load particle settings:', e);
        return DEFAULT_SETTINGS;
    }
}

function createParticleSettingsStore() {
    const initial = loadFromStorage();
    const { subscribe, set, update } = writable<ParticleSettings>(initial);

    if (browser) {
        subscribe((value) => {
            try {
                localStorage.setItem(STORAGE_KEY, JSON.stringify(value));
            } catch (e) {
                console.warn('Failed to save particle settings:', e);
            }
        });
    }

    return {
        subscribe,
        set,
        update,
        reset: () => set(DEFAULT_SETTINGS),

        updateBokeh: (changes: Partial<BokehConfig>) =>
            update(s => ({ ...s, bokeh: { ...s.bokeh, ...changes } })),

        updateWeather: (season: 'winter' | 'spring' | 'summer' | 'autumn' | 'summerNight', changes: Partial<ParticleConfig>) =>
            update(s => {
                const newWeather = { ...s.weather };
                if (season === 'summerNight') {
                    if (newWeather.summer.nightVariant) {
                        newWeather.summer = {
                            ...newWeather.summer,
                            nightVariant: { ...newWeather.summer.nightVariant, ...changes }
                        };
                    }
                } else {
                    newWeather[season] = { ...newWeather[season], ...changes };
                }
                return { ...s, weather: newWeather };
            }),
    };
}

export const particleSettings = createParticleSettingsStore();
