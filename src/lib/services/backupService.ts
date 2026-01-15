import { invoke } from '@tauri-apps/api/core';
import { save, open } from '@tauri-apps/plugin-dialog';
import { message, confirm } from '@tauri-apps/plugin-dialog';
import { relaunch } from '@tauri-apps/plugin-process';
import type { BackupEntry, BackupConfig } from '$lib/types/backup';

// ============================================================================
// BACKUP MANUAL
// ============================================================================

/**
 * Inicia el proceso de backup de la base de datos.
 * Abre un diálogo para guardar el archivo.
 */
export async function backupDatabase() {
	try {
		const filePath = await save({
			filters: [
				{
					name: 'Brisas Database Backup',
					extensions: ['db', 'sqlite', 'bak']
				}
			],
			defaultPath: `brisas_backup_${new Date().toISOString().slice(0, 10)}.db`
		});

		if (!filePath) return;

		await invoke('backup_database', { destinationPath: filePath });
		await message('Copia de seguridad creada correctamente.', {
			title: 'Backup Exitoso',
			kind: 'info'
		});
	} catch (error) {
		console.error('Error creating backup:', error);
		await message(`Error al crear backup: ${error}`, { title: 'Error', kind: 'error' });
	}
}

/**
 * Inicia el proceso de restauración de la base de datos.
 * Abre un diálogo para seleccionar el archivo y pide reinicio.
 */
export async function restoreDatabase() {
	try {
		const filePath = await open({
			multiple: false,
			filters: [
				{
					name: 'Brisas Database Backup',
					extensions: ['db', 'sqlite', 'bak']
				}
			]
		});

		if (!filePath) return;

		const confirmed = await confirm(
			'⚠️ ¡ADVERTENCIA!\n\nAl restaurar este backup, se perderán todos los datos actuales y serán reemplazados por los del archivo seleccionado.\n\nLa aplicación se reiniciará automáticamente para aplicar los cambios.\n\n¿Estás seguro de continuar?',
			{ title: 'Confirmar Restauración', kind: 'warning' }
		);

		if (!confirmed) return;

		await invoke('restore_database', { sourcePath: filePath });

		await message(
			'El archivo ha sido preparado correctamente. La aplicación se reiniciará ahora para aplicar los cambios.',
			{ title: 'Reinicio Requerido', kind: 'info' }
		);

		await relaunch();
	} catch (error) {
		console.error('Error restoring database:', error);
		await message(`Error al restaurar backup: ${error}`, { title: 'Error', kind: 'error' });
	}
}

// ============================================================================
// BACKUP AUTOMÁTICO
// ============================================================================

/**
 * Ejecuta un backup automático al directorio configurado.
 * @returns Nombre del archivo generado
 */
export async function backupDatabaseAuto(): Promise<string> {
	return await invoke<string>('backup_database_auto');
}

/**
 * Lista todos los backups existentes en el directorio de backups.
 */
export async function listBackups(): Promise<BackupEntry[]> {
	return await invoke<BackupEntry[]>('list_backups');
}

/**
 * Elimina un backup específico.
 * @param filename Nombre del archivo a eliminar
 */
export async function deleteBackup(filename: string): Promise<void> {
	await invoke('delete_backup', { filename });
}

/**
 * Restaura desde un backup automático.
 * @param filename Nombre del archivo a restaurar
 */
export async function restoreFromAutoBackup(filename: string): Promise<void> {
	const confirmed = await confirm(
		`⚠️ ¡ADVERTENCIA!\n\nAl restaurar "${filename}", se perderán todos los datos actuales.\n\nLa aplicación se reiniciará automáticamente para aplicar los cambios.\n\n¿Estás seguro de continuar?`,
		{ title: 'Confirmar Restauración', kind: 'warning' }
	);

	if (!confirmed) return;

	await invoke('restore_from_auto_backup', { filename });

	await message('El backup ha sido preparado correctamente. La aplicación se reiniciará ahora.', {
		title: 'Reinicio Requerido',
		kind: 'info'
	});

	await relaunch();
}

/**
 * Ejecuta limpieza de backups antiguos según la política de retención.
 * @returns Cantidad de backups eliminados
 */
export async function cleanupOldBackups(): Promise<number> {
	return await invoke<number>('cleanup_old_backups');
}

// ============================================================================
// CONFIGURACIÓN DE BACKUP
// ============================================================================

/**
 * Obtiene la configuración actual de backup.
 */
export async function getBackupConfig(): Promise<BackupConfig> {
	return await invoke<BackupConfig>('get_backup_config');
}

/**
 * Actualiza la configuración de backup automático.
 */
export async function updateBackupConfig(
	enabled: boolean,
	hora: string,
	diasRetencion: number
): Promise<BackupConfig> {
	return await invoke<BackupConfig>('update_backup_config', {
		enabled,
		hora,
		diasRetencion
	});
}
