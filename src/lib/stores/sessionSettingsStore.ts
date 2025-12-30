import { writable, type Writable } from 'svelte/store';
import { browser } from '$app/environment';

// =============================================================================
// TYPES
// =============================================================================

export interface SessionSettings {
    enableAppLock: boolean;
    appLockTimeoutMinutes: number;
    enableScreensaver: boolean;
    screensaverTimeoutMinutes: number;
    screensaverRequiresPassword: boolean;
    enableCompleteTimeout: boolean;
    completeTimeoutMinutes: number;
}

// =============================================================================
// DEFAULTS
// =============================================================================

const DEFAULT_SETTINGS: SessionSettings = {
    enableAppLock: false,
    appLockTimeoutMinutes: 20,
    enableScreensaver: true,
    screensaverTimeoutMinutes: 20,
    screensaverRequiresPassword: true,
    enableCompleteTimeout: true,
    completeTimeoutMinutes: 60,
};

// =============================================================================
// STORAGE - DUAL WRITE (localStorage + Tauri Store)
// =============================================================================

const STORAGE_KEY = 'brisas-session-settings';

function loadFromLocalStorage(): SessionSettings {
    if (!browser) return DEFAULT_SETTINGS;
    try {
        const stored = localStorage.getItem(STORAGE_KEY);
        if (!stored) return DEFAULT_SETTINGS;
        return { ...DEFAULT_SETTINGS, ...JSON.parse(stored) };
    } catch {
        return DEFAULT_SETTINGS;
    }
}

async function loadFromTauriStore(): Promise<SessionSettings> {
    try {
        const { getSetting } = await import('$lib/services/storeService');
        const stored = await getSetting<SessionSettings>(STORAGE_KEY, DEFAULT_SETTINGS);
        return { ...DEFAULT_SETTINGS, ...stored };
    } catch {
        return loadFromLocalStorage();
    }
}

async function saveToStorage(settings: SessionSettings): Promise<void> {
    if (!browser) return;
    localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
    try {
        const { setSetting } = await import('$lib/services/storeService');
        await setSetting(STORAGE_KEY, settings);
    } catch {
        // localStorage already saved
    }
}

// =============================================================================
// VALIDATION
// =============================================================================

function validateSettings(settings: SessionSettings): SessionSettings {
    const validated = { ...settings };
    validated.appLockTimeoutMinutes = Math.max(1, Math.floor(settings.appLockTimeoutMinutes));
    validated.screensaverTimeoutMinutes = Math.max(1, Math.floor(settings.screensaverTimeoutMinutes));
    validated.completeTimeoutMinutes = Math.max(1, Math.floor(settings.completeTimeoutMinutes));

    if (validated.enableScreensaver && validated.enableCompleteTimeout) {
        if (validated.completeTimeoutMinutes <= validated.screensaverTimeoutMinutes) {
            validated.completeTimeoutMinutes = validated.screensaverTimeoutMinutes + 10;
        }
    }
    return validated;
}

// =============================================================================
// STORE
// =============================================================================

export interface SessionSettingsStore extends Writable<SessionSettings> {
    reset: () => void;
    toggleAppLock: () => void;
    toggleScreensaver: () => void;
    toggleCompleteTimeout: () => void;
    toggleScreensaverPassword: () => void;
    setAppLockTimeout: (minutes: number) => void;
    setScreensaverTimeout: (minutes: number) => void;
    setCompleteTimeout: (minutes: number) => void;
}

function createSessionSettingsStore(): SessionSettingsStore {
    const initial = loadFromLocalStorage();
    const { subscribe, set, update } = writable<SessionSettings>(initial);

    // Load from Tauri Store after hydration
    if (browser) {
        loadFromTauriStore().then(settings => set(validateSettings(settings)));
    }

    let saveTimeout: ReturnType<typeof setTimeout>;
    subscribe((value) => {
        clearTimeout(saveTimeout);
        saveTimeout = setTimeout(() => saveToStorage(value), 300);
    });

    return {
        subscribe,
        set: (value: SessionSettings) => set(validateSettings(value)),
        update: (fn: (value: SessionSettings) => SessionSettings) => {
            update((current) => validateSettings(fn(current)));
        },
        reset: () => set(DEFAULT_SETTINGS),
        toggleAppLock: () => update((s) => ({ ...s, enableAppLock: !s.enableAppLock })),
        toggleScreensaver: () => update((s) => ({ ...s, enableScreensaver: !s.enableScreensaver })),
        toggleCompleteTimeout: () => update((s) => ({ ...s, enableCompleteTimeout: !s.enableCompleteTimeout })),
        toggleScreensaverPassword: () => update((s) => ({ ...s, screensaverRequiresPassword: !s.screensaverRequiresPassword })),
        setAppLockTimeout: (minutes: number) => update((s) => ({ ...s, appLockTimeoutMinutes: minutes })),
        setScreensaverTimeout: (minutes: number) => update((s) => ({ ...s, screensaverTimeoutMinutes: minutes })),
        setCompleteTimeout: (minutes: number) => update((s) => ({ ...s, completeTimeoutMinutes: minutes })),
    };
}

export const sessionSettings = createSessionSettingsStore();
