import { writable, get, type Writable } from 'svelte/store';
import { browser } from '$app/environment';

// =============================================================================
// TYPES
// =============================================================================

export interface SessionSettings {
    // App Lock (cuando app pierde foco o está minimizada)
    enableAppLock: boolean;                 // Bloquear cuando app no tiene foco
    appLockTimeoutMinutes: number;          // Timeout para bloqueo de app

    // System Screensaver (cuando PC está inactiva)
    enableScreensaver: boolean;             // Activar screensaver del sistema
    screensaverTimeoutMinutes: number;      // Timeout para screensaver
    screensaverRequiresPassword: boolean;   // Requiere password para salir

    // Complete Logout
    enableCompleteTimeout: boolean;         // Logout completo
    completeTimeoutMinutes: number;         // Timeout para logout completo
}

// =============================================================================
// DEFAULTS
// =============================================================================

const DEFAULT_SETTINGS: SessionSettings = {
    // App Lock: deshabilitado por defecto (usuario elige)
    enableAppLock: false,
    appLockTimeoutMinutes: 20,

    // System Screensaver: habilitado por defecto para seguridad
    enableScreensaver: true,
    screensaverTimeoutMinutes: 20,
    screensaverRequiresPassword: true,

    // Complete Logout: habilitado por defecto
    enableCompleteTimeout: true,
    completeTimeoutMinutes: 60,
};

// =============================================================================
// STORAGE
// =============================================================================

const STORAGE_KEY = 'brisas-session-settings';

function loadFromStorage(): SessionSettings {
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
        console.warn('Failed to load session settings from storage:', e);
        return DEFAULT_SETTINGS;
    }
}

function saveToStorage(settings: SessionSettings): void {
    if (!browser) return;

    try {
        localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
    } catch (e) {
        console.warn('Failed to save session settings to storage:', e);
    }
}

// =============================================================================
// VALIDATION
// =============================================================================

/**
 * Validates session settings and returns validated/corrected settings
 */
function validateSettings(settings: SessionSettings): SessionSettings {
    const validated = { ...settings };

    // Ensure all timeouts are positive numbers
    validated.appLockTimeoutMinutes = Math.max(1, Math.floor(settings.appLockTimeoutMinutes));
    validated.screensaverTimeoutMinutes = Math.max(1, Math.floor(settings.screensaverTimeoutMinutes));
    validated.completeTimeoutMinutes = Math.max(1, Math.floor(settings.completeTimeoutMinutes));

    // If screensaver and complete logout are both enabled, ensure complete > screensaver
    if (validated.enableScreensaver && validated.enableCompleteTimeout) {
        if (validated.completeTimeoutMinutes <= validated.screensaverTimeoutMinutes) {
            validated.completeTimeoutMinutes = validated.screensaverTimeoutMinutes + 10;
            console.warn(
                `Complete logout timeout adjusted to ${validated.completeTimeoutMinutes} minutes`
            );
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
    const initial = loadFromStorage();
    const { subscribe, set, update } = writable<SessionSettings>(initial);

    // Auto-save on changes (with debouncing)
    let saveTimeout: ReturnType<typeof setTimeout>;

    subscribe((value) => {
        clearTimeout(saveTimeout);
        saveTimeout = setTimeout(() => {
            saveToStorage(value);
        }, 300);
    });

    return {
        subscribe,
        set: (value: SessionSettings) => {
            const validated = validateSettings(value);
            set(validated);
        },
        update: (fn: (value: SessionSettings) => SessionSettings) => {
            update((current) => {
                const updated = fn(current);
                return validateSettings(updated);
            });
        },

        // Convenience methods
        reset: () => {
            set(DEFAULT_SETTINGS);
        },

        toggleAppLock: () => {
            update((s) => ({ ...s, enableAppLock: !s.enableAppLock }));
        },

        toggleScreensaver: () => {
            update((s) => ({ ...s, enableScreensaver: !s.enableScreensaver }));
        },

        toggleCompleteTimeout: () => {
            update((s) => ({ ...s, enableCompleteTimeout: !s.enableCompleteTimeout }));
        },

        toggleScreensaverPassword: () => {
            update((s) => ({ ...s, screensaverRequiresPassword: !s.screensaverRequiresPassword }));
        },

        setAppLockTimeout: (minutes: number) => {
            update((s) => ({ ...s, appLockTimeoutMinutes: minutes }));
        },

        setScreensaverTimeout: (minutes: number) => {
            update((s) => ({ ...s, screensaverTimeoutMinutes: minutes }));
        },

        setCompleteTimeout: (minutes: number) => {
            update((s) => ({ ...s, completeTimeoutMinutes: minutes }));
        },
    };
}

export const sessionSettings = createSessionSettingsStore();
