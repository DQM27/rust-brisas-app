// ============================================
// src/lib/stores/loginStore.svelte.ts
// ============================================
// Store for login-related persistent state using Svelte 5 runes

import { browser } from '$app/environment';

// ============================================
// STORAGE KEYS
// ============================================

const STORAGE_KEY = 'rememberedEmail';
const PASSWORD_CHECKED_KEY = 'rememberPasswordChecked';

// ============================================
// STORE CLASS (Svelte 5 Runes)
// ============================================

class LoginStore {
    private _rememberedEmail = $state<string>('');
    private _rememberedPassword = $state<string>('');
    private _rememberPasswordChecked = $state<boolean>(false);
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

    get rememberedPassword(): string {
        return this._rememberedPassword;
    }

    get rememberPasswordChecked(): boolean {
        return this._rememberPasswordChecked;
    }

    get hasRememberedEmail(): boolean {
        return this._rememberedEmail.length > 0;
    }

    get hasRememberedPassword(): boolean {
        return this._rememberedPassword.length > 0;
    }

    // ============================================
    // ACTIONS
    // ============================================

    async setRememberedEmail(email: string): Promise<void> {
        this._rememberedEmail = email;
        await this.saveToStorage(email);
    }

    async setRememberedPassword(password: string): Promise<void> {
        this._rememberedPassword = password;
        this._rememberPasswordChecked = true;
        // Solo guardamos si tenemos un email asociado
        if (this._rememberedEmail) {
            await this.savePasswordToKeyring(this._rememberedEmail, password);
            await this.savePersistenceFlags(this._rememberedEmail, true);
        }
    }

    async clearRememberedEmail(): Promise<void> {
        this._rememberedEmail = '';
        await this.removeFromStorage();
        // Si borramos el email, también borramos cualquier password asociado (por seguridad)
        this.clearRememberedPassword();
    }

    async clearRememberedPassword(): Promise<void> {
        this._rememberedPassword = '';
        this._rememberPasswordChecked = false;
        if (this._rememberedEmail) {
            await this.removePasswordFromKeyring(this._rememberedEmail);
        }
    }

    // ============================================
    // PERSISTENCE (Dual Write + Keyring)
    // ============================================

    private loadInitial(): void {
        // Sync load from localStorage for immediate hydration
        const stored = localStorage.getItem(STORAGE_KEY);
        if (stored) {
            this._rememberedEmail = stored;
        }

        // Async load from Tauri Store & Keyring
        this.loadFromTauriStore();
    }

    private async loadFromTauriStore(): Promise<void> {
        if (this._initialized) return;
        this._initialized = true;

        try {
            const { getSetting } = await import('$lib/services/storeService');
            const stored = await getSetting<string>(STORAGE_KEY, '');
            const passChecked = await getSetting<boolean>(PASSWORD_CHECKED_KEY, false);

            if (stored) {
                this._rememberedEmail = stored;
                this._rememberPasswordChecked = passChecked;
                // Sync localStorage with Tauri Store value
                localStorage.setItem(STORAGE_KEY, stored);

                // Try to load password for this email if checked
                if (passChecked) {
                    await this.loadPasswordFromKeyring(stored);
                }
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
    // KEYRING HELPERS
    // ============================================

    private async loadPasswordFromKeyring(email: string): Promise<void> {
        try {
            const { getSecret } = await import('$lib/services/keyringService');
            // SINGLE USER MODE: Usamos una key constante para que solo exista UNA contraseña guardada a la vez
            const key = 'brisas:current_user_password';
            const password = await getSecret(key);

            // Importante: Solo cargamos si tenemos un email (ya validado por el caller)
            if (password && email) {
                this._rememberedPassword = password;
                this._rememberPasswordChecked = true;
            } else {
                this._rememberedPassword = '';
                this._rememberPasswordChecked = false;
            }
        } catch (e) {
            console.error('Error loading password from keyring:', e);
        }
    }

    private async savePersistenceFlags(email: string, passwordChecked: boolean): Promise<void> {
        if (!browser) return;
        try {
            const { setSetting } = await import('$lib/services/storeService');
            await setSetting(STORAGE_KEY, email);
            await setSetting(PASSWORD_CHECKED_KEY, passwordChecked);
        } catch (e) {
            console.error('Error saving persistence flags:', e);
        }
    }

    private async savePasswordToKeyring(email: string, password: string): Promise<void> {
        try {
            const { saveSecret } = await import('$lib/services/keyringService');
            // SINGLE USER MODE: Sobreescribimos siempre la misma key
            const key = 'brisas:current_user_password';
            await saveSecret(key, password);
        } catch (e) {
            console.error('Error saving password to keyring:', e);
        }
    }

    private async removePasswordFromKeyring(email: string): Promise<void> {
        try {
            const { deleteSecret } = await import('$lib/services/keyringService');
            const key = 'brisas:current_user_password';
            await deleteSecret(key);
        } catch (e) {
            console.error('Error removing password from keyring:', e);
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
