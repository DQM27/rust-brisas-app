import { writable, get, type Writable } from 'svelte/store';
import { browser } from '$app/environment';

// =============================================================================
// TYPES
// =============================================================================

export interface SessionSettings {
    // Enable/disable features
    enableScreensaver: boolean;        // Interactive screensaver mode
    enableCompleteTimeout: boolean;    // Full logout

    // Timeout durations (in minutes)
    screensaverTimeoutMinutes: number; // Default: 20
    completeTimeoutMinutes: number;    // Default: 60

    // Screensaver behavior
    screensaverRequiresPassword: boolean; // Default: true (secure mode)
}

// =============================================================================
// DEFAULTS
// =============================================================================

const DEFAULT_SETTINGS: SessionSettings = {
    // Features enabled by default for security
    enableScreensaver: true,
    enableCompleteTimeout: true,

    // Timeout durations
    screensaverTimeoutMinutes: 20,
    completeTimeoutMinutes: 60,

    // Screensaver requires password by default (secure mode)
    screensaverRequiresPassword: true,
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

    // Ensure timeouts are positive numbers
    validated.screensaverTimeoutMinutes = Math.max(1, Math.floor(settings.screensaverTimeoutMinutes));
    validated.completeTimeoutMinutes = Math.max(1, Math.floor(settings.completeTimeoutMinutes));

    // If both are enabled, ensure complete timeout > screensaver timeout
    if (validated.enableScreensaver && validated.enableCompleteTimeout) {
        if (validated.completeTimeoutMinutes <= validated.screensaverTimeoutMinutes) {
            // Auto-adjust: set complete timeout to screensaver + 10 minutes
            validated.completeTimeoutMinutes = validated.screensaverTimeoutMinutes + 10;
            console.warn(
                `Complete logout timeout adjusted to ${validated.completeTimeoutMinutes} minutes (must be greater than screensaver timeout)`
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
    toggleScreensaver: () => void;
    toggleCompleteTimeout: () => void;
    toggleScreensaverPassword: () => void;
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

        toggleScreensaver: () => {
            update((s) => ({ ...s, enableScreensaver: !s.enableScreensaver }));
        },

        toggleCompleteTimeout: () => {
            update((s) => ({ ...s, enableCompleteTimeout: !s.enableCompleteTimeout }));
        },

        toggleScreensaverPassword: () => {
            update((s) => ({ ...s, screensaverRequiresPassword: !s.screensaverRequiresPassword }));
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
