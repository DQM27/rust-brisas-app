// src/lib/api/astro.ts

import { invoke } from '@tauri-apps/api/core';

export interface AstroData {
    moon_phase: string;
    illumination: number;
    age_days: number;
    location: string;
}

export const astroApi = {
    getAstroData: async (lat?: number, lon?: number): Promise<AstroData> => {
        try {
            // Si no se pasan coordenadas, el backend usa las de San José por defecto
            return await invoke<AstroData>('get_astro_data', { lat, lon });
        } catch (e) {
            console.error('Error fetching astro data:', e);
            // Fallback seguro en caso de error
            return {
                moon_phase: 'full',
                illumination: 1.0,
                age_days: 15,
                location: 'Error fallback'
            };
        }
    }
};
