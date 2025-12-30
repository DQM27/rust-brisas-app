// src/lib/stores/themeStore.ts
// Migrated to use Tauri Store for persistence with localStorage fallback for sync access
import { writable } from 'svelte/store';
import { browser } from '$app/environment';

type Theme = 'light' | 'dark';

// Sync access for initial render (localStorage as cache)
function getInitialTheme(): Theme {
    if (browser) {
        const stored = localStorage.getItem('theme') as Theme | null;
        if (stored) return stored;
        return 'dark';
    }
    return 'dark';
}

const theme = writable<Theme>(getInitialTheme());

// Load from Tauri Store and sync to localStorage (async, runs after hydration)
async function loadFromTauriStore(): Promise<void> {
    if (!browser) return;
    try {
        const { getSetting } = await import('$lib/services/storeService');
        const storedTheme = await getSetting<Theme>('theme', 'dark');
        theme.set(storedTheme);
        localStorage.setItem('theme', storedTheme); // Keep localStorage in sync
    } catch (e) {
        // Tauri Store not available (dev mode or error), use localStorage
        console.debug('[Theme] Using localStorage fallback');
    }
}

// Apply theme to DOM and persist to both stores
async function applyTheme(newTheme: Theme): Promise<void> {
    if (!browser) return;

    const root = document.documentElement;
    if (newTheme === 'dark') {
        root.classList.add('dark');
    } else {
        root.classList.remove('dark');
    }

    // Persist to localStorage (sync, for next page load)
    localStorage.setItem('theme', newTheme);

    // Persist to Tauri Store (async, for cross-session)
    try {
        const { setSetting } = await import('$lib/services/storeService');
        await setSetting('theme', newTheme);
    } catch {
        // Fallback: localStorage already saved
    }
}

// Initialize theme on DOM
if (browser) {
    const initialTheme = getInitialTheme();
    applyTheme(initialTheme);

    // Load from Tauri Store after hydration
    loadFromTauriStore();
}

// Subscribe to changes
if (browser) {
    theme.subscribe((value) => {
        applyTheme(value);
    });
}

export function toggleTheme(): void {
    theme.update(current => current === 'dark' ? 'light' : 'dark');
}

export const themeStore = theme;
