/**
 * Tipos para el sistema de backup automático
 */

/**
 * Entrada de backup en el directorio de backups automáticos
 */
export interface BackupEntry {
	/** Nombre del archivo (e.g., "brisas_backup_2026-01-13_02-00.surql") */
	nombre: string;
	/** Ruta completa al archivo */
	ruta: string;
	/** Tamaño en bytes */
	tamano: number;
	/** Fecha de creación en formato ISO 8601 */
	fechaCreacion: string;
	/** Días desde la creación */
	diasAntiguedad: number;
}

/**
 * Configuración de backup automático
 */
export interface BackupConfig {
	/** Backup automático habilitado */
	enabled: boolean;
	/** Hora del backup en formato "HH:MM" (24h) */
	hora: string;
	/** Días de retención antes de eliminar backups antiguos */
	diasRetencion: number;
	/** Directorio donde se almacenan los backups automáticos (null = default) */
	directorio: string | null;
	/** Timestamp ISO del último backup exitoso */
	ultimoBackup: string | null;
}
