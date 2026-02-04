import { save, open } from '@tauri-apps/plugin-dialog';
import { message, confirm } from '@tauri-apps/plugin-dialog';
import { relaunch } from '@tauri-apps/plugin-process';
import type { BackupEntry, BackupConfig } from '$lib/types/backup';
import { backupApi } from '$lib/api/backup';

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
					extensions: ['db', 'sqlite', 'bak', 'penc', 'enc', 'surql']
				}
			],
			defaultPath: `brisas_backup_${new Date().toISOString().slice(0, 10)}.db`
		});

		if (!filePath) return;

		await backupApi.backupDatabase(filePath);
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
					extensions: ['db', 'sqlite', 'bak', 'penc', 'enc', 'surql']
				}
			]
		});

		if (!filePath) return;

		const confirmed = await confirm(
			'⚠️ ¡ADVERTENCIA!\n\nAl restaurar este backup, se perderán todos los datos actuales y serán reemplazados por los del archivo seleccionado.\n\nLa aplicación se reiniciará automáticamente para aplicar los cambios.\n\n¿Estás seguro de continuar?',
			{ title: 'Confirmar Restauración', kind: 'warning' }
		);

		if (!confirmed) return;

		await backupApi.restoreDatabase(filePath);

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
	return await backupApi.backupDatabaseAuto();
}

/**
 * Lista todos los backups existentes en el directorio de backups.
 */
export async function listBackups(): Promise<BackupEntry[]> {
	return await backupApi.listBackups();
}

/**
 * Elimina un backup específico.
 * @param filename Nombre del archivo a eliminar
 */
export async function deleteBackup(filename: string): Promise<void> {
	await backupApi.deleteBackup(filename);
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

	await backupApi.restoreFromAutoBackup(filename);

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
	return await backupApi.cleanupOldBackups();
}

/**
 * Crea un backup PORTABLE encriptado con contraseña.
 * Puede ser restaurado en cualquier máquina que tenga la contraseña.
 * @param password Contraseña de mínimo 8 caracteres
 * @returns Nombre del archivo generado
 */
export async function backupDatabasePortable(password: string): Promise<string> {
	return await backupApi.backupDatabasePortable(password);
}

/**
 * Restaura desde un backup portable (requiere contraseña).
 * @param filename Nombre del archivo a restaurar
 * @param password Contraseña para desencriptar
 */
export async function restorePortableBackup(filename: string, password: string): Promise<void> {
	const confirmed = await confirm(
		`⚠️ ¡ADVERTENCIA!\n\nAl restaurar "${filename}", se perderán todos los datos actuales.\n\nLa aplicación se reiniciará automáticamente para aplicar los cambios.\n\n¿Estás seguro de continuar?`,
		{ title: 'Confirmar Restauración', kind: 'warning' }
	);

	if (!confirmed) return;

	await backupApi.restorePortableBackup(filename, password);

	await message('El backup ha sido preparado correctamente. La aplicación se reiniciará ahora.', {
		title: 'Reinicio Requerido',
		kind: 'info'
	});

	await relaunch();
}

/**
 * Determina si un backup requiere contraseña para restaurar.
 */
export function requiresPassword(entry: BackupEntry): boolean {
	return entry.encryptionType === 'portable';
}

/**
 * Obtiene un label legible para el tipo de encriptación.
 */
export function getEncryptionLabel(type: string): string {
	switch (type) {
		case 'local':
			return '🔐 Encriptado';
		case 'portable':
			return '🔑 Portable';
		default:
			return '📄 Sin encriptar';
	}
}

// ============================================================================
// CONFIGURACIÓN DE BACKUP
// ============================================================================

/**
 * Obtiene la configuración actual de backup.
 */
export async function getBackupConfig(): Promise<BackupConfig> {
	return await backupApi.getBackupConfig();
}

/**
 * Actualiza la configuración de backup automático.
 */
export async function updateBackupConfig(
	enabled: boolean,
	hora: string,
	diasRetencion: number
): Promise<BackupConfig> {
	return await backupApi.updateBackupConfig(enabled, hora, diasRetencion);
}
