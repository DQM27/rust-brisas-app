import type { Season } from "$lib/stores/settingsStore";
import { generalSettings } from "$lib/stores/settingsStore";
import { derived } from "svelte/store";

export function getSeason(): Season {
    const month = new Date().getMonth();
    if (month >= 2 && month <= 4) return "spring";
    if (month >= 5 && month <= 7) return "summer";
    if (month >= 8 && month <= 10) return "autumn";
    return "winter";
}

// Store derived from settings that resolves the effective season (override or automatic)
export const currentSeason = derived(generalSettings, ($settings) => {
    return $settings.overrideSeason || getSeason();
});
