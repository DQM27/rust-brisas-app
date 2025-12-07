import { persisted } from 'svelte-persisted-store';

export type Season = 'winter' | 'spring' | 'summer' | 'autumn';

export interface GeneralSettings {
    enableWeatherEffects: boolean;
    overrideSeason?: Season | null;
    overrideHour?: number | null; // 0-23
    showWelcomeCards: boolean;
    showBackground: boolean;
}

export const generalSettings = persisted<GeneralSettings>('brisas-general-settings', {
    enableWeatherEffects: true,
    overrideSeason: null,
    overrideHour: null,
    showWelcomeCards: true,
    showBackground: true,
});
