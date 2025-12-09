import { writable, get, type Writable } from 'svelte/store';
import { browser } from '$app/environment';

// =============================================================================
// TYPES
// =============================================================================

export type Season = 'winter' | 'spring' | 'summer' | 'autumn';

export interface GeneralSettings {
  // === Visual Background ===
  showBackground: boolean;        // Mostrar paisaje de montañas
  showClouds: boolean;            // Mostrar nubes animadas
  showStars: boolean;             // Mostrar estrellas de noche

  showCelestial: boolean;         // Mostrar sol/luna
  landscapeType: 'mountains' | 'forest' | 'city' | 'desert' | 'beach' | 'moon'; // Tipo de paisaje

  // === Weather Effects ===
  enableWeatherEffects: boolean;  // Partículas climáticas (nieve, hojas, etc.)
  showBokeh: boolean;             // Efecto "Flent" (partículas desenfocadas)

  // === UI Elements ===
  showWelcomeCards: boolean;      // Mostrar tarjetas de módulos
  showWelcomeText: boolean;       // Mostrar texto de bienvenida (nombre y saludo)

  // === Debug/Preview Overrides ===
  overrideHour: number | null;    // null = tiempo real, 0-23 = hora fija
  overrideSeason: Season | null;  // null = estación real, string = estación fija
  overrideBirthday: boolean;      // Forzar modo cumpleaños para testing
}

// =============================================================================
// DEFAULTS
// =============================================================================

const DEFAULT_SETTINGS: GeneralSettings = {
  // Visual - all enabled by default for best experience
  showBackground: true,
  showClouds: true,
  showStars: true,
  showCelestial: true,
  landscapeType: 'mountains',

  // Weather
  enableWeatherEffects: true,
  showBokeh: true, // New "Flent" effect enabled by default

  // UI
  showWelcomeCards: true,
  showWelcomeText: true,

  // Overrides - null means "use real values"
  overrideHour: null,
  overrideSeason: null,
  overrideBirthday: false,
};

// =============================================================================
// STORAGE
// =============================================================================

const STORAGE_KEY = 'brisas-general-settings';

function loadFromStorage(): GeneralSettings {
  if (!browser) return DEFAULT_SETTINGS;

  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (!stored) return DEFAULT_SETTINGS;

    const parsed = JSON.parse(stored);

    // Merge with defaults to handle new properties added in updates
    return {
      ...DEFAULT_SETTINGS,
      ...parsed,
    };
  } catch (e) {
    console.warn('Failed to load settings from storage:', e);
    return DEFAULT_SETTINGS;
  }
}

function saveToStorage(settings: GeneralSettings): void {
  if (!browser) return;

  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
  } catch (e) {
    console.warn('Failed to save settings to storage:', e);
  }
}

// =============================================================================
// STORE
// =============================================================================



export interface GeneralSettingsStore extends Writable<GeneralSettings> {
  reset: () => void;
  resetOverrides: () => void;
  toggleWeather: () => void;
  toggleClouds: () => void;
  toggleStars: () => void;
  toggleCelestial: () => void;
  toggleBackground: () => void;
  toggleCards: () => void;
  toggleWelcomeText: () => void;
  toggleBokeh: () => void;

  toggleBirthdayTest: () => void;
  setLandscapeType: (type: 'mountains' | 'forest' | 'city' | 'desert' | 'beach' | 'moon') => void;
}

function createGeneralSettingsStore(): GeneralSettingsStore {
  const initial = loadFromStorage();
  const { subscribe, set, update } = writable<GeneralSettings>(initial);

  // Auto-save on changes
  let saveTimeout: ReturnType<typeof setTimeout>;

  subscribe((value) => {
    // Debounce saves to avoid excessive writes
    clearTimeout(saveTimeout);
    saveTimeout = setTimeout(() => {
      saveToStorage(value);
    }, 300);
  });

  return {
    subscribe,
    set,
    update,

    // Convenience methods
    reset: () => {
      set(DEFAULT_SETTINGS);
    },

    // Reset only the preview/debug overrides
    resetOverrides: () => {
      update(s => ({
        ...s,
        overrideHour: null,
        overrideSeason: null,
        overrideBirthday: false,
      }));
    },

    // Toggle helpers
    toggleWeather: () => update(s => ({ ...s, enableWeatherEffects: !s.enableWeatherEffects })),
    toggleClouds: () => update(s => ({ ...s, showClouds: !s.showClouds })),
    toggleStars: () => update(s => ({ ...s, showStars: !s.showStars })),
    toggleCelestial: () => update(s => ({ ...s, showCelestial: !s.showCelestial })),
    toggleBackground: () => update(s => ({ ...s, showBackground: !s.showBackground })),
    toggleCards: () => update(s => ({ ...s, showWelcomeCards: !s.showWelcomeCards })),
    toggleWelcomeText: () => update(s => ({ ...s, showWelcomeText: !s.showWelcomeText })),
    toggleBokeh: () => update(s => ({ ...s, showBokeh: !s.showBokeh })),

    toggleBirthdayTest: () => update(s => ({ ...s, overrideBirthday: !s.overrideBirthday })),
    setLandscapeType: (type) => update(s => ({ ...s, landscapeType: type })),
  };
}

export const generalSettings = createGeneralSettingsStore();

// =============================================================================
// DERIVED HELPERS (optional - for convenience in components)
// =============================================================================

// Example: Check if any visual effects are enabled
export function hasVisualEffects(): boolean {
  const settings = get(generalSettings);
  return settings.showBackground ||
    settings.showClouds ||
    settings.showStars ||
    settings.showCelestial ||
    settings.enableWeatherEffects;
}
