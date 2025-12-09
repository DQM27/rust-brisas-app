import { writable } from 'svelte/store';

// Interface for fine-tuning visual effects
export interface ParticleSettings {
    // Bokeh Effect Controls
    bokehCount: number;
    bokehMinSize: number;
    bokehMaxSize: number;
    bokehMaxOpacity: number; // Max opacity for pulsation
    bokehSpeedMultiplier: number;

    // General Particle Controls
    globalSpeedMultiplier: number;

    // Weather Controls (New)
    weatherDensityMultiplier: number;
    weatherSpeedMultiplier: number;
    weatherSizeMultiplier: number;
    weatherWindInfluence: number; // 0 to 2
    weatherTurbulence: number; // 0 to 2

    // Celestial Controls
    moonPhase: string;
    sunStyle: string;

    // Star Controls
    starCountMultiplier: number; // 0 to 2
    starTwinkleSpeed: number; // 0 to 3
    shootingStarFrequency: number; // 0 to 5
    shootingStarSpeed: number; // 0.5 to 3
    meteorShowerEnabled: boolean;

    // Cloud settings
    cloudStyle: 'cartoon' | 'soft';
    cloudOpacity: number; // 0.1 to 1.0
    cloudCount: number; // 3 to 10
    cloudWindSpeed: number; // 0.0 to 3.0
    cloudTurbulence: number; // 0.0 to 1.0
}

export const DEFAULT_PARTICLE_SETTINGS: ParticleSettings = {
    // Tuned defaults matching current aesthetics
    bokehCount: 25,
    bokehMinSize: 30,
    bokehMaxSize: 70,
    bokehMaxOpacity: 0.3,
    bokehSpeedMultiplier: 1.0,

    globalSpeedMultiplier: 1.0,

    // Weather defaults
    weatherDensityMultiplier: 1.0,
    weatherSpeedMultiplier: 1.0,
    weatherSizeMultiplier: 1.0,
    weatherWindInfluence: 1.0,
    weatherTurbulence: 1.0,

    // Celestial Controls
    moonPhase: 'full', // 'new', 'waxing-crescent', 'first-quarter', 'waxing-gibbous', 'full', 'waning-gibbous', 'last-quarter', 'waning-crescent'
    sunStyle: 'normal', // 'normal', 'cloudy'

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

import { browser } from '$app/environment';

function createParticleSettingsStore() {
    // Load from localStorage if available
    const savedSettings = browser ? localStorage.getItem('particleSettings') : null;
    const initialSettings: ParticleSettings = savedSettings
        ? { ...DEFAULT_PARTICLE_SETTINGS, ...JSON.parse(savedSettings) } // Merge to ensure new fields (like clouds) are present if missing in old save
        : DEFAULT_PARTICLE_SETTINGS;

    const { subscribe, set, update } = writable<ParticleSettings>(initialSettings);

    // Subscribe to changes and save to localStorage
    if (browser) {
        subscribe(settings => {
            localStorage.setItem('particleSettings', JSON.stringify(settings));
        });
    }

    return {
        subscribe,
        set,
        update,
        reset: () => set(DEFAULT_PARTICLE_SETTINGS),

        // Fine-tuning actions
        updateBokehCount: (count: number) => update(s => ({ ...s, bokehCount: count })),
        updateBokehOpacity: (opacity: number) => update(s => ({ ...s, bokehMaxOpacity: opacity })),
        updateBokehSize: (min: number, max: number) => update(s => ({ ...s, bokehMinSize: min, bokehMaxSize: max })),

        // Weather actions
        updateWeatherDensity: (m: number) => update(s => ({ ...s, weatherDensityMultiplier: m })),
        updateWeatherSpeed: (m: number) => update(s => ({ ...s, weatherSpeedMultiplier: m })),
        updateWeatherSize: (m: number) => update(s => ({ ...s, weatherSizeMultiplier: m })),
        updateWeatherWind: (m: number) => update(s => ({ ...s, weatherWindInfluence: m })),
        updateWeatherTurbulence: (m: number) => update(s => ({ ...s, weatherTurbulence: m })),

        // Celestial actions
        updateMoonPhase: (phase: string) => update(s => ({ ...s, moonPhase: phase })),
        updateSunStyle: (style: string) => update(s => ({ ...s, sunStyle: style })),

        // Star actions
        updateStarCount: (m: number) => update(s => ({ ...s, starCountMultiplier: m })),
        updateStarTwinkle: (m: number) => update(s => ({ ...s, starTwinkleSpeed: m })),
        updateShootingStarFreq: (m: number) => update(s => ({ ...s, shootingStarFrequency: m })),
        updateShootingStarSpeed: (m: number) => update(s => ({ ...s, shootingStarSpeed: m })),
        toggleMeteorShower: () => update(s => ({ ...s, meteorShowerEnabled: !s.meteorShowerEnabled })),

        // Cloud updates
        updateCloudStyle: (style: 'cartoon' | 'soft') => update(s => ({ ...s, cloudStyle: style })),
        updateCloudOpacity: (opacity: number) => update(s => ({ ...s, cloudOpacity: opacity })),
        updateCloudCount: (count: number) => update(s => ({ ...s, cloudCount: count })),
        updateCloudWindSpeed: (speed: number) => update(s => ({ ...s, cloudWindSpeed: speed })),
        updateCloudTurbulence: (turb: number) => update(s => ({ ...s, cloudTurbulence: turb })),
    };
}

export const particleSettings = createParticleSettingsStore();
