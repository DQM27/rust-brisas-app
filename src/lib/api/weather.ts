// src/lib/api/weather.ts

import { invoke } from '@tauri-apps/api/core';

export interface WeatherData {
    temperature: number;
    weather_code: number;
    wind_speed: number;
    is_day: number; // 1 = Day, 0 = Night
    condition_text: string;
}

export const weatherApi = {
    getWeather: async (lat?: number, lon?: number): Promise<WeatherData> => {
        try {
            return await invoke<WeatherData>('get_weather_data', { lat, lon });
        } catch (e) {
            console.error('Error fetching weather data:', e);
            // Fallback en caso de error (offline)
            return {
                temperature: 25.0,
                weather_code: 0, // Despejado
                wind_speed: 10.0,
                is_day: 1,
                condition_text: 'Desconocido (Offline)'
            };
        }
    }
};
