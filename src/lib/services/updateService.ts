import { check } from '@tauri-apps/plugin-updater';
import { ask, message } from '@tauri-apps/plugin-dialog';
import { relaunch } from '@tauri-apps/plugin-process';
import { openPath } from '@tauri-apps/plugin-opener';
import { invoke } from '@tauri-apps/api/core';

export async function checkAndInstallUpdate(silent = false) {
    try {
        const update = await check();

        if (update) {
            console.log(`Update available: ${update.version}`);

            const shouldUpdate = await ask(
                `Una nueva versión (${update.version}) está disponible.\n\nDescripción:\n${update.body || 'Correcciones y mejoras.'}\n\n¿Quieres descargarla e instalarla ahora?`,
                {
                    title: 'Actualización Disponible',
                    kind: 'info',
                    okLabel: 'Sí, Actualizar',
                    cancelLabel: 'Después'
                }
            );

            if (shouldUpdate) {
                await update.downloadAndInstall((event) => {
                    switch (event.event) {
                        case 'Started':
                            console.log('Download started');
                            break;
                        case 'Progress':
                            console.log(`Downloaded ${event.data.chunkLength} bytes`);
                            break;
                        case 'Finished':
                            console.log('Download finished');
                            break;
                    }
                });

                await message('La actualización se ha instalado correctamente. La aplicación se reiniciará.', { title: 'Actualización Exitosa' });
                await relaunch();
            }
        } else {
            if (!silent) {
                await message('Ya tienes la última versión instalada.', { title: 'Sistema Actualizado' });
            }
        }
    } catch (error) {
        console.error('Error checking for updates:', error);
        if (!silent) {
            await message(`Error al buscar actualizaciones: ${error}`, { title: 'Error', kind: 'error' });
        }
    }
}

// Función simular actualización offline (abrir instalador manual)
// En realidad, esto solo ayuda al usuario a abrir el archivo si lo tiene.
// La mejor manera offline es simplemente correr el instalador.
export async function installUpdateFromFile() {
    // Esto es más complejo porque el binding de updater no soporta "instalar desde archivo arbritrario" directamente
    // de la misma manera que el remoto (que verifica firmas).
    // Sin embargo, podemos permitir al usuario abrir el instalador.
    try {
        // Placeholder para lógica futura si se requiere
        await message('Para actualizar offline, por favor ejecuta el instalador (.exe/.msi) más reciente directamente.', { title: 'Actualización Offline' });
    } catch (e) {
        console.error(e);
    }
}
