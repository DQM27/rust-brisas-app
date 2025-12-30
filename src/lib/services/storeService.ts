// ============================================
// src/lib/services/storeService.ts
// ============================================
// Tauri Store wrapper for native persistent storage
// Replaces localStorage with cross-platform native storage

import { Store } from '@tauri-apps/plugin-store';

let store: Store | null = null;
const STORE_FILE = 'settings.json';

/**
 * Gets the singleton store instance
 * Lazy-loaded on first access
 */
export async function getStore(): Promise<Store> {
    if (!store) {
        store = await Store.load(STORE_FILE);
    }
    return store;
}

/**
 * Gets a value from the store with a default fallback
 */
export async function getSetting<T>(key: string, defaultValue: T): Promise<T> {
    const s = await getStore();
    const value = await s.get<T>(key);
    return value ?? defaultValue;
}

/**
 * Sets a value in the store
 */
export async function setSetting<T>(key: string, value: T): Promise<void> {
    const s = await getStore();
    await s.set(key, value);
}

/**
 * Deletes a key from the store
 */
export async function deleteSetting(key: string): Promise<void> {
    const s = await getStore();
    await s.delete(key);
}

/**
 * Clears all settings (use with caution)
 */
export async function clearAllSettings(): Promise<void> {
    const s = await getStore();
    await s.clear();
}
