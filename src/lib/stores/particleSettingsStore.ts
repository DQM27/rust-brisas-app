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
}

export const DEFAULT_PARTICLE_SETTINGS: ParticleSettings = {
    // Tuned defaults matching current aesthetics
    bokehCount: 25,
    bokehMinSize: 30,
    bokehMaxSize: 70,
    bokehMaxOpacity: 0.3,
    bokehSpeedMultiplier: 1.0,

    globalSpeedMultiplier: 1.0,
};

function createParticleSettingsStore() {
    const { subscribe, set, update } = writable<ParticleSettings>(DEFAULT_PARTICLE_SETTINGS);

    return {
        subscribe,
        set,
        update,
        reset: () => set(DEFAULT_PARTICLE_SETTINGS),

        // Fine-tuning actions
        updateBokehCount: (count: number) => update(s => ({ ...s, bokehCount: count })),
        updateBokehOpacity: (opacity: number) => update(s => ({ ...s, bokehMaxOpacity: opacity })),
        updateBokehSize: (min: number, max: number) => update(s => ({ ...s, bokehMinSize: min, bokehMaxSize: max })),
    };
}

export const particleSettings = createParticleSettingsStore();
