// src/lib/logic/keyring/keyringService.ts

import { keyringApi } from '$lib/api/keyring';
import type {
    CredentialStatus,
    Argon2Params,
    Argon2ParamsSafe,
    SetupCredentialsInput,
    SetupResult
} from '$lib/types/keyring';

export type ServiceResult<T> = { ok: true; data: T } | { ok: false; error: string };

/**
 * Servicio de Seguridad/Keyring siguiendo arquitectura limpia
 */

export async function getCredentialStatus(): Promise<ServiceResult<CredentialStatus>> {
    try {
        const data = await keyringApi.getCredentialStatus();
        return { ok: true, data };
    } catch (err: any) {
        return { ok: false, error: String(err) };
    }
}

export async function isAppConfigured(): Promise<ServiceResult<boolean>> {
    try {
        const data = await keyringApi.isAppConfigured();
        return { ok: true, data };
    } catch (err: any) {
        return { ok: false, error: String(err) };
    }
}

export async function needsSetup(): Promise<ServiceResult<boolean>> {
    try {
        const data = await keyringApi.needsSetup();
        return { ok: true, data };
    } catch (err: any) {
        return { ok: false, error: String(err) };
    }
}

export async function setupCredentials(input: SetupCredentialsInput): Promise<ServiceResult<SetupResult>> {
    try {
        const data = await keyringApi.setupCredentials(input);
        return { ok: true, data };
    } catch (err: any) {
        return { ok: false, error: String(err) };
    }
}

export async function getArgon2Config(): Promise<ServiceResult<Argon2ParamsSafe>> {
    try {
        const data = await keyringApi.getArgon2Config();
        return { ok: true, data };
    } catch (err: any) {
        return { ok: false, error: String(err) };
    }
}

export async function updateArgon2Params(params: Argon2Params): Promise<ServiceResult<void>> {
    try {
        await keyringApi.updateArgon2Params(params);
        return { ok: true, data: undefined };
    } catch (err: any) {
        return { ok: false, error: String(err) };
    }
}

export async function generateArgon2Secret(): Promise<ServiceResult<string>> {
    try {
        const data = await keyringApi.generateArgon2Secret();
        return { ok: true, data };
    } catch (err: any) {
        return { ok: false, error: String(err) };
    }
}

export async function generateRandomSecret(): Promise<ServiceResult<string>> {
    try {
        const data = await keyringApi.generateRandomSecret();
        return { ok: true, data };
    } catch (err: any) {
        return { ok: false, error: String(err) };
    }
}

export async function resetAllCredentials(confirm: boolean): Promise<ServiceResult<void>> {
    try {
        await keyringApi.resetAllCredentials(confirm);
        return { ok: true, data: undefined };
    } catch (err: any) {
        return { ok: false, error: String(err) };
    }
}

export async function setWindowDecorations(decorations: boolean): Promise<ServiceResult<void>> {
    try {
        await keyringApi.setWindowDecorations(decorations);
        return { ok: true, data: undefined };
    } catch (err: any) {
        return { ok: false, error: String(err) };
    }
}

export async function setWindowSize(width: number, height: number): Promise<ServiceResult<void>> {
    try {
        await keyringApi.setWindowSize(width, height);
        return { ok: true, data: undefined };
    } catch (err: any) {
        return { ok: false, error: String(err) };
    }
}

export async function exportMasterKey(filePath: string, password: string): Promise<ServiceResult<void>> {
    try {
        await keyringApi.exportMasterKey(filePath, password);
        return { ok: true, data: undefined };
    } catch (err: any) {
        return { ok: false, error: String(err) };
    }
}

export async function importMasterKey(filePath: string, password: string): Promise<ServiceResult<void>> {
    try {
        await keyringApi.importMasterKey(filePath, password);
        return { ok: true, data: undefined };
    } catch (err: any) {
        return { ok: false, error: String(err) };
    }
}

export async function saveSecret(key: string, value: string): Promise<ServiceResult<void>> {
    try {
        await keyringApi.saveSecret(key, value);
        return { ok: true, data: undefined };
    } catch (err: any) {
        return { ok: false, error: String(err) };
    }
}

export async function getSecret(key: string): Promise<ServiceResult<string | null>> {
    try {
        const data = await keyringApi.getSecret(key);
        return { ok: true, data };
    } catch (err: any) {
        return { ok: false, error: String(err) };
    }
}

export async function deleteSecret(key: string): Promise<ServiceResult<void>> {
    try {
        await keyringApi.deleteSecret(key);
        return { ok: true, data: undefined };
    } catch (err: any) {
        return { ok: false, error: String(err) };
    }
}

export async function generateRecoveryFragments(): Promise<ServiceResult<string[]>> {
    try {
        const data = await keyringApi.generateRecoveryFragments();
        return { ok: true, data };
    } catch (err: any) {
        return { ok: false, error: String(err) };
    }
}

export async function recoverFromFragments(fragments: string[]): Promise<ServiceResult<void>> {
    try {
        await keyringApi.recoverFromFragments(fragments);
        return { ok: true, data: undefined };
    } catch (err: any) {
        return { ok: false, error: String(err) };
    }
}

export async function exitApp(): Promise<void> {
    await keyringApi.exitApp();
}
