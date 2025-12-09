/**
 * LANDSCAPE DATA
 * API unificada para el sistema de paisajes
 */

import type { Season } from "$lib/stores/settingsStore";
import {
    type LandscapeType,
    type LandscapeTheme,
    type BiomeData,
    LANDSCAPE_TYPES,
    mountainsBiome,
    forestBiome,
    cityBiome,
    desertBiome,
    beachBiome,
    moonBiome,
} from './biomes';

export type { LandscapeType, LandscapeTheme, BiomeData };
export { LANDSCAPE_TYPES };

// =============================================================================
// REGISTRO DE BIOMAS
// =============================================================================

const BIOME_REGISTRY: Record<LandscapeType, BiomeData> = {
    mountains: mountainsBiome,
    forest: forestBiome,
    city: cityBiome,
    desert: desertBiome,
    beach: beachBiome,
    moon: moonBiome,
};

// =============================================================================
// API PÚBLICA
// =============================================================================

export function getLandscapeTheme(type: LandscapeType, season: Season): LandscapeTheme {
    const biome = BIOME_REGISTRY[type] ?? BIOME_REGISTRY.mountains;
    return biome.themes[season] ?? biome.themes.winter;
}

export function getLandscapePaths(type: LandscapeType): [string, string, string] {
    const biome = BIOME_REGISTRY[type] ?? BIOME_REGISTRY.mountains;
    return biome.paths;
}

export function getBiomeData(type: LandscapeType): BiomeData {
    return BIOME_REGISTRY[type] ?? BIOME_REGISTRY.mountains;
}

export function isValidLandscapeType(type: string): type is LandscapeType {
    return type in BIOME_REGISTRY;
}

// Compatibilidad con código existente
export const LANDSCAPE_PATHS: Record<LandscapeType, [string, string, string]> = {
    mountains: mountainsBiome.paths,
    forest: forestBiome.paths,
    city: cityBiome.paths,
    desert: desertBiome.paths,
    beach: beachBiome.paths,
    moon: moonBiome.paths,
};
