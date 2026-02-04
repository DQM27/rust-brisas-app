import { invoke } from '@tauri-apps/api/core';
import type {
    CredentialStatus,
    Argon2Params,
    Argon2ParamsSafe,
    SetupCredentialsInput,
    SetupResult
} from '$lib/types/keyring';

export const keyringApi = {
    getCredentialStatus: () => invoke<CredentialStatus>('get_credential_status'),
    isAppConfigured: () => invoke<boolean>('is_app_configured'),
    needsSetup: () => invoke<boolean>('needs_setup'),
    setupCredentials: (input: SetupCredentialsInput) => invoke<SetupResult>('setup_credentials', { input }),
    getArgon2Config: () => invoke<Argon2ParamsSafe>('get_argon2_config'),
    updateArgon2Params: (params: Argon2Params) => invoke<void>('update_argon2_params', { params }),
    generateArgon2Secret: () => invoke<string>('generate_argon2_secret'),
    generateRandomSecret: () => invoke<string>('generate_random_secret'),
    resetAllCredentials: (confirm: boolean) => invoke<void>('reset_all_credentials', { confirm }),
    exitApp: () => invoke<void>('exit_app'),
    setWindowDecorations: (decorations: boolean) => invoke<void>('set_window_decorations', { decorations }),
    setWindowSize: (width: number, height: number) => invoke<void>('set_window_size', { width, height }),
    exportMasterKey: (filePath: string, password: string) => invoke<void>('export_master_key_cmd', { filePath, password }),
    importMasterKey: (filePath: string, password: string) => invoke<void>('import_master_key_cmd', { filePath, password }),
    saveSecret: (key: string, value: string) => invoke<void>('save_secret', { key, value }),
    getSecret: (key: string) => invoke<string | null>('get_secret', { key }),
    deleteSecret: (key: string) => invoke<void>('delete_secret', { key }),
    generateRecoveryFragments: () => invoke<string[]>('generate_recovery_fragments'),
    recoverFromFragments: (fragments: string[]) => invoke<void>('recover_from_fragments', { fragments })
};
