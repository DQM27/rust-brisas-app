// src/lib/logic/backup/backupService.ts

import { backupApi } from '$lib/api/backup';
import type { BackupEntry, BackupConfig } from '$lib/types/backup';

export type ServiceResult<T> = { ok: true; data: T } | { ok: false; error: string };

/**
 * Servicio para gestión de backups siguiendo arquitectura limpia
 */

export async function fetchAllBackups(): Promise<ServiceResult<BackupEntry[]>> {
    try {
        const data = await backupApi.listBackups();
        return { ok: true, data };
    } catch (err: any) {
        return { ok: false, error: String(err) };
    }
}

export async function runAutoBackup(): Promise<ServiceResult<string>> {
    try {
        const filename = await backupApi.backupDatabaseAuto();
        return { ok: true, data: filename };
    } catch (err: any) {
        return { ok: false, error: String(err) };
    }
}

export async function createPortableBackup(password: string): Promise<ServiceResult<string>> {
    try {
        const filename = await backupApi.backupDatabasePortable(password);
        return { ok: true, data: filename };
    } catch (err: any) {
        return { ok: false, error: String(err) };
    }
}

export async function deleteBackupFile(filename: string): Promise<ServiceResult<void>> {
    try {
        await backupApi.deleteBackup(filename);
        return { ok: true, data: undefined };
    } catch (err: any) {
        return { ok: false, error: String(err) };
    }
}

export async function prepareAutoRestore(filename: string): Promise<ServiceResult<void>> {
    try {
        await backupApi.restoreFromAutoBackup(filename);
        return { ok: true, data: undefined };
    } catch (err: any) {
        return { ok: false, error: String(err) };
    }
}

export async function preparePortableRestore(filename: string, password: string): Promise<ServiceResult<void>> {
    try {
        await backupApi.restorePortableBackup(filename, password);
        return { ok: true, data: undefined };
    } catch (err: any) {
        return { ok: false, error: String(err) };
    }
}

export async function getConfiguration(): Promise<ServiceResult<BackupConfig>> {
    try {
        const config = await backupApi.getBackupConfig();
        return { ok: true, data: config };
    } catch (err: any) {
        return { ok: false, error: String(err) };
    }
}

export async function updateConfiguration(enabled: boolean, hora: string, diasRetencion: number): Promise<ServiceResult<BackupConfig>> {
    try {
        const config = await backupApi.updateBackupConfig(enabled, hora, diasRetencion);
        return { ok: true, data: config };
    } catch (err: any) {
        return { ok: false, error: String(err) };
    }
}

export async function runManualBackup(destinationPath: string): Promise<ServiceResult<void>> {
    try {
        await backupApi.backupDatabase(destinationPath);
        return { ok: true, data: undefined };
    } catch (err: any) {
        return { ok: false, error: String(err) };
    }
}

export async function runManualRestore(sourcePath: string): Promise<ServiceResult<void>> {
    try {
        await backupApi.restoreDatabase(sourcePath);
        return { ok: true, data: undefined };
    } catch (err: any) {
        return { ok: false, error: String(err) };
    }
}

export async function runCleanup(): Promise<ServiceResult<number>> {
    try {
        const count = await backupApi.cleanupOldBackups();
        return { ok: true, data: count };
    } catch (err: any) {
        return { ok: false, error: String(err) };
    }
}
