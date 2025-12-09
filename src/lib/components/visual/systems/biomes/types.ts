import type { Season } from "$lib/stores/settingsStore";

export type LandscapeType = 'mountains' | 'forest' | 'city' | 'desert' | 'beach' | 'moon';

export interface LandscapeTheme {
    colors: [string, string, string];
    accent?: string;
}

export interface BiomeData {
    themes: Record<Season, LandscapeTheme>;
    paths: [string, string, string];
}

export const LANDSCAPE_TYPES: { id: LandscapeType; name: string; icon: string }[] = [
    { id: 'mountains', name: 'Montañas', icon: 'Mountain' },
    { id: 'forest', name: 'Bosque', icon: 'Trees' },
    { id: 'city', name: 'Ciudad', icon: 'Building2' },
    { id: 'desert', name: 'Desierto', icon: 'Sun' },
    { id: 'beach', name: 'Playa', icon: 'Umbrella' },
    { id: 'moon', name: 'Luna', icon: 'Moon' },
];
