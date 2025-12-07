import { persisted } from 'svelte-persisted-store';

export type Season = 'winter' | 'spring' | 'summer' | 'autumn';

export interface GeneralSettings {
    enableWeatherEffects: boolean;
    overrideSeason?: Season | null;
}

export const generalSettings = persisted<GeneralSettings>('brisas-general-settings', {
    enableWeatherEffects: true,
    overrideSeason: null,
});
