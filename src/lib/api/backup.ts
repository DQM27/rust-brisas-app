import { invoke } from '@tauri-apps/api/core';
import type { BackupEntry, BackupConfig } from '$lib/types/backup';

/**
 * API layer for Backup management
 */
export const backupApi = {
    /**
     * Performs a manual database backup to a specific path
     */
    backupDatabase: (destinationPath: string) =>
        invoke<void>('backup_database', { destinationPath }),

    /**
     * Prepares a database restoration from a specific path
     */
    restoreDatabase: (sourcePath: string) =>
        invoke<void>('restore_database', { sourcePath }),

    /**
     * Performs an automatic encrypted backup
     */
    backupDatabaseAuto: () =>
        invoke<string>('backup_database_auto'),

    /**
     * Lists all available backups
     */
    listBackups: () =>
        invoke<BackupEntry[]>('list_backups'),

    /**
     * Deletes a backup file
     */
    deleteBackup: (filename: string) =>
        invoke<void>('delete_backup', { filename }),

    /**
     * Prepares restoration from an auto-backup file
     */
    restoreFromAutoBackup: (filename: string) =>
        invoke<void>('restore_from_auto_backup', { filename }),

    /**
     * Cleans up old backups based on retention policy
     */
    cleanupOldBackups: () =>
        invoke<number>('cleanup_old_backups'),

    /**
     * Creates a portable encrypted backup with a password
     */
    backupDatabasePortable: (password: string) =>
        invoke<string>('backup_database_portable', { password }),

    /**
     * Prepares restoration from a portable backup file
     */
    restorePortableBackup: (filename: string, password: string) =>
        invoke<void>('restore_portable_backup', { filename, password }),

    /**
     * Gets the current backup configuration
     */
    getBackupConfig: () =>
        invoke<BackupConfig>('get_backup_config'),

    /**
     * Updates the backup configuration
     */
    updateBackupConfig: (enabled: boolean, hora: string, diasRetencion: number) =>
        invoke<BackupConfig>('update_backup_config', { enabled, hora, diasRetencion })
};
