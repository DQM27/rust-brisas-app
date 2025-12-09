import { invoke } from "@tauri-apps/api/core";
import { save, open } from "@tauri-apps/plugin-dialog";
import { message, confirm } from "@tauri-apps/plugin-dialog";
import { relaunch } from "@tauri-apps/plugin-process";

/**
 * Inicia el proceso de backup de la base de datos.
 * Abre un diálogo para guardar el archivo.
 */
export async function backupDatabase() {
    try {
        const filePath = await save({
            filters: [{
                name: 'Brisas Database Backup',
                extensions: ['db', 'sqlite', 'bak']
            }],
            defaultPath: `brisas_backup_${new Date().toISOString().slice(0, 10)}.db`
        });

        if (!filePath) return;

        await invoke("backup_database", { destinationPath: filePath });
        await message("Copia de seguridad creada correctamente.", { title: "Backup Exitoso", kind: "info" });
    } catch (error) {
        console.error("Error creating backup:", error);
        await message(`Error al crear backup: ${error}`, { title: "Error", kind: "error" });
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
            filters: [{
                name: 'Brisas Database Backup',
                extensions: ['db', 'sqlite', 'bak']
            }]
        });

        if (!filePath) return;

        const confirmed = await confirm(
            "⚠️ ¡ADVERTENCIA!\n\nAl restaurar este backup, se perderán todos los datos actuales y serán reemplazados por los del archivo seleccionado.\n\nLa aplicación se reiniciará automáticamente para aplicar los cambios.\n\n¿Estás seguro de continuar?",
            { title: "Confirmar Restauración", kind: "warning" }
        );

        if (!confirmed) return;

        await invoke("restore_database", { sourcePath: filePath });

        await message(
            "El archivo ha sido preparado correctamente. La aplicación se reiniciará ahora para aplicar los cambios.",
            { title: "Reinicio Requerido", kind: "info" }
        );

        await relaunch();

    } catch (error) {
        console.error("Error restoring database:", error);
        await message(`Error al restaurar backup: ${error}`, { title: "Error", kind: "error" });
    }
}
