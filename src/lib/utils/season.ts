import type { Season } from "$lib/stores/settingsStore";
import { generalSettings } from "$lib/stores/settingsStore";
import { derived } from "svelte/store";

export function getSeason(): Season {
    const month = new Date().getMonth(); // 0-11

    // Winter: December (Christmas winds, holiday feel)
    if (month === 11) return "winter";

    // Summer: January to March (Peak Dry Season)
    if (month >= 0 && month <= 2) return "summer";

    // Spring: April (Transition, first rains, blooming)
    if (month === 3) return "spring";

    // Autumn: November (Transition, end of heavy rains, cooling)
    if (month === 10) return "autumn";

    // Rain: May to October (Core Green Season)
    return "rain";
}

// Store derived from settings that resolves the effective season (override or automatic)
export const currentSeason = derived(generalSettings, ($settings) => {
    return $settings.overrideSeason || getSeason();
});
