// ============================================
// src/lib/stores/loginStore.svelte.ts
// ============================================
// Store for login-related persistent state using Svelte 5 runes

import { browser } from '$app/environment';

// ============================================
// STORAGE KEYS
// ============================================

const STORAGE_KEY = 'rememberedEmail';

// ============================================
// STORE CLASS (Svelte 5 Runes)
// ============================================

class LoginStore {
    private _rememberedEmail = $state<string>('');
    private _initialized = false;

    constructor() {
        if (browser) {
            this.loadInitial();
        }
    }

    // ============================================
    // GETTERS
    // ============================================

    get rememberedEmail(): string {
        return this._rememberedEmail;
    }

    get hasRememberedEmail(): boolean {
        return this._rememberedEmail.length > 0;
    }

    // ============================================
    // ACTIONS
    // ============================================

    async setRememberedEmail(email: string): Promise<void> {
        this._rememberedEmail = email;
        await this.saveToStorage(email);
    }

    async clearRememberedEmail(): Promise<void> {
        this._rememberedEmail = '';
        await this.removeFromStorage();
    }

    // ============================================
    // PERSISTENCE (Dual Write)
    // ============================================

    private loadInitial(): void {
        // Sync load from localStorage for immediate hydration
        const stored = localStorage.getItem(STORAGE_KEY);
        if (stored) {
            this._rememberedEmail = stored;
        }

        // Async load from Tauri Store
        this.loadFromTauriStore();
    }

    private async loadFromTauriStore(): Promise<void> {
        if (this._initialized) return;
        this._initialized = true;

        try {
            const { getSetting } = await import('$lib/services/storeService');
            const stored = await getSetting<string>(STORAGE_KEY, '');
            if (stored) {
                this._rememberedEmail = stored;
                // Sync localStorage with Tauri Store value
                localStorage.setItem(STORAGE_KEY, stored);
            }
        } catch {
            // Tauri Store not available
        }
    }

    private async saveToStorage(email: string): Promise<void> {
        if (!browser) return;

        // Sync save to localStorage
        localStorage.setItem(STORAGE_KEY, email);

        // Async save to Tauri Store
        try {
            const { setSetting } = await import('$lib/services/storeService');
            await setSetting(STORAGE_KEY, email);
        } catch {
            // localStorage already saved
        }
    }

    private async removeFromStorage(): Promise<void> {
        if (!browser) return;

        localStorage.removeItem(STORAGE_KEY);

        try {
            const { deleteSetting } = await import('$lib/services/storeService');
            await deleteSetting(STORAGE_KEY);
        } catch {
            // localStorage already cleared
        }
    }

    // ============================================
    // RESET (for login form)
    // ============================================

    async reload(): Promise<void> {
        await this.loadFromTauriStore();
    }
}

// ============================================
// SINGLETON EXPORT
// ============================================

export const loginStore = new LoginStore();
