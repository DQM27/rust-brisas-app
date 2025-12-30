import { writable, get, type Writable } from 'svelte/store';
import { browser } from '$app/environment';

// =============================================================================
// TYPES
// =============================================================================

export type Season = 'winter' | 'spring' | 'summer' | 'autumn' | 'rain';

export interface GeneralSettings {
  // === Visual Background ===
  showBackground: boolean;
  showClouds: boolean;
  showStars: boolean;
  showCelestial: boolean;
  landscapeType: 'mountains' | 'forest' | 'city' | 'desert' | 'beach' | 'moon';

  // === Weather Effects ===
  enableWeatherEffects: boolean;
  showBokeh: boolean;

  // === UI Elements ===
  showWelcomeCards: boolean;
  showWelcomeText: boolean;

  // === Debug/Preview Overrides ===
  overrideHour: number | null;
  overrideSeason: Season | null;
  overrideBirthday: boolean;
  isKioskMode: boolean;
  disableSetupWizard: boolean;
}

// =============================================================================
// DEFAULTS
// =============================================================================

const DEFAULT_SETTINGS: GeneralSettings = {
  showBackground: true,
  showClouds: true,
  showStars: true,
  showCelestial: true,
  landscapeType: 'mountains',
  enableWeatherEffects: true,
  showBokeh: true,
  showWelcomeCards: true,
  showWelcomeText: true,
  overrideHour: null,
  overrideSeason: null,
  overrideBirthday: false,
  isKioskMode: false,
  disableSetupWizard: false,
};

// =============================================================================
// STORAGE - DUAL WRITE (localStorage + Tauri Store)
// =============================================================================

const STORAGE_KEY = 'brisas-general-settings';

function loadFromLocalStorage(): GeneralSettings {
  if (!browser) return DEFAULT_SETTINGS;
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (!stored) return DEFAULT_SETTINGS;
    return { ...DEFAULT_SETTINGS, ...JSON.parse(stored) };
  } catch {
    return DEFAULT_SETTINGS;
  }
}

async function loadFromTauriStore(): Promise<GeneralSettings> {
  try {
    const { getSetting } = await import('$lib/services/storeService');
    const stored = await getSetting<GeneralSettings>(STORAGE_KEY, DEFAULT_SETTINGS);
    return { ...DEFAULT_SETTINGS, ...stored };
  } catch {
    return loadFromLocalStorage();
  }
}

async function saveToStorage(settings: GeneralSettings): Promise<void> {
  if (!browser) return;

  // Sync save to localStorage (fast, for next page load)
  localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));

  // Async save to Tauri Store
  try {
    const { setSetting } = await import('$lib/services/storeService');
    await setSetting(STORAGE_KEY, settings);
  } catch {
    // localStorage already saved as fallback
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
  setLandscapeType: (type: GeneralSettings['landscapeType']) => void;
  toggleSetupWizard: () => void;
}

function createGeneralSettingsStore(): GeneralSettingsStore {
  const initial = loadFromLocalStorage();
  const { subscribe, set, update } = writable<GeneralSettings>(initial);

  // Load from Tauri Store after hydration
  if (browser) {
    loadFromTauriStore().then(settings => {
      set(settings);
    });
  }

  // Auto-save on changes (debounced)
  let saveTimeout: ReturnType<typeof setTimeout>;
  subscribe((value) => {
    clearTimeout(saveTimeout);
    saveTimeout = setTimeout(() => {
      saveToStorage(value);
    }, 300);
  });

  return {
    subscribe,
    set,
    update,
    reset: () => set(DEFAULT_SETTINGS),
    resetOverrides: () => update(s => ({
      ...s,
      overrideHour: null,
      overrideSeason: null,
      overrideBirthday: false,
    })),
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
    toggleSetupWizard: () => update(s => ({ ...s, disableSetupWizard: !s.disableSetupWizard })),
  };
}

export const generalSettings = createGeneralSettingsStore();

// =============================================================================
// DERIVED HELPERS
// =============================================================================

export function hasVisualEffects(): boolean {
  const settings = get(generalSettings);
  return settings.showBackground ||
    settings.showClouds ||
    settings.showStars ||
    settings.showCelestial ||
    settings.enableWeatherEffects;
}
