import { writable, type Writable } from 'svelte/store';
import { browser } from '$app/environment';

type StorageType = 'local' | 'session';

export function persisted<T>(key: string, initialValue: T, options: { storage?: StorageType } = { storage: 'local' }): Writable<T> {
    const storage = options.storage === 'session' ? (browser ? sessionStorage : null) : (browser ? localStorage : null);

    // Load initial value from storage if available
    let currentValue = initialValue;
    if (storage) {
        const stored = storage.getItem(key);
        if (stored) {
            try {
                currentValue = JSON.parse(stored);
            } catch (e) {
                console.warn(`Failed to parse stored value for key "${key}"`, e);
            }
        }
    }

    const { subscribe, set, update } = writable<T>(currentValue);

    return {
        subscribe,
        set: (value: T) => {
            if (storage) {
                storage.setItem(key, JSON.stringify(value));
            }
            set(value);
        },
        update: (fn: (value: T) => T) => {
            update((value) => {
                const newValue = fn(value);
                if (storage) {
                    storage.setItem(key, JSON.stringify(newValue));
                }
                return newValue;
            });
        }
    };
}
