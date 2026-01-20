import { writable, get, type Writable } from 'svelte/store';
import { browser } from '$app/environment';

export interface SpotlightSettings {
    showActions: boolean;
    showModules: boolean;
    showLinks: boolean;
    enableTantivySearch: boolean;
    tantivyLimit: number;
    showDescriptions: boolean;
    showRecent: boolean;
    recentSearchesLimit: number;
}

const DEFAULT_SPOTLIGHT_SETTINGS: SpotlightSettings = {
    showActions: true,
    showModules: true,
    showLinks: true,
    enableTantivySearch: true,
    tantivyLimit: 10,
    showDescriptions: true,
    showRecent: true,
    recentSearchesLimit: 5
};

const STORAGE_KEY = 'brisas-spotlight-settings';

function loadSettings(): SpotlightSettings {
    if (!browser) return DEFAULT_SPOTLIGHT_SETTINGS;
    try {
        const stored = localStorage.getItem(STORAGE_KEY);
        if (!stored) return DEFAULT_SPOTLIGHT_SETTINGS;
        return { ...DEFAULT_SPOTLIGHT_SETTINGS, ...JSON.parse(stored) };
    } catch {
        return DEFAULT_SPOTLIGHT_SETTINGS;
    }
}

function saveSettings(settings: SpotlightSettings) {
    if (!browser) return;
    localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
}

function createSpotlightSettingsStore() {
    const initial = loadSettings();
    const { subscribe, set, update } = writable<SpotlightSettings>(initial);

    subscribe((value) => {
        saveSettings(value);
    });

    return {
        subscribe,
        set,
        update,
        reset: () => set(DEFAULT_SPOTLIGHT_SETTINGS),
        toggleActions: () => update((s) => ({ ...s, showActions: !s.showActions })),
        toggleModules: () => update((s) => ({ ...s, showModules: !s.showModules })),
        toggleLinks: () => update((s) => ({ ...s, showLinks: !s.showLinks })),
        toggleTantivy: () => update((s) => ({ ...s, enableTantivySearch: !s.enableTantivySearch })),
        toggleDescriptions: () => update((s) => ({ ...s, showDescriptions: !s.showDescriptions })),
        toggleRecent: () => update((s) => ({ ...s, showRecent: !s.showRecent })),
        setTantivyLimit: (limit: number) => update((s) => ({ ...s, tantivyLimit: limit }))
    };
}

export const spotlightSettings = createSpotlightSettingsStore();

// ============================================
// STORE DE ITEMS RECIENTES
// ============================================

const RECENT_ITEMS_KEY = 'brisas-spotlight-recent';

function createRecentSpotlightStore() {
    const initial = browser ? JSON.parse(localStorage.getItem(RECENT_ITEMS_KEY) || '[]') : [];
    const { subscribe, update } = writable<string[]>(initial);

    return {
        subscribe,
        add: (id: string) => {
            if (id.startsWith('tab-') || id.startsWith(' tantivy-')) return; // No guardar tabs ni resultados de búsqueda profunda

            update(items => {
                const limit = get(spotlightSettings).recentSearchesLimit;
                // Mover al principio y eliminar duplicados
                const newItems = [id, ...items.filter(i => i !== id)].slice(0, limit);
                if (browser) localStorage.setItem(RECENT_ITEMS_KEY, JSON.stringify(newItems));
                return newItems;
            });
        },
        clear: () => {
            if (browser) localStorage.removeItem(RECENT_ITEMS_KEY);
            update(() => []);
        }
    };
}

export const recentSpotlightItems = createRecentSpotlightStore();

// ============================================
// STORE DE FAVORITOS
// ============================================

const FAVORITES_KEY = 'brisas-spotlight-favorites';

function createFavoriteSpotlightStore() {
    const initial = browser ? JSON.parse(localStorage.getItem(FAVORITES_KEY) || '[]') : [];
    const { subscribe, update } = writable<string[]>(initial);

    return {
        subscribe,
        toggle: (id: string) => {
            update(items => {
                const newItems = items.includes(id)
                    ? items.filter(i => i !== id)
                    : [...items, id].slice(0, 10);
                if (browser) localStorage.setItem(FAVORITES_KEY, JSON.stringify(newItems));
                return newItems;
            });
        }
    };
}

export const favoriteSpotlightItems = createFavoriteSpotlightStore();
