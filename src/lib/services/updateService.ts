import { check } from '@tauri-apps/plugin-updater';
import { ask, message } from '@tauri-apps/plugin-dialog';
import { relaunch } from '@tauri-apps/plugin-process';

export async function checkAndInstallUpdate(silent = false) {
    try {
        const update = await check();

        if (update) {
            console.log(`Update available: ${update.version}`);

            const shouldUpdate = await ask(
                `Una nueva version (${update.version}) esta disponible.\n\nDescripcion:\n${update.body || 'Correcciones y mejoras.'}\n\nQuieres descargarla e instalarla ahora?`,
                {
                    title: 'Actualizacion Disponible',
                    kind: 'info',
                    okLabel: 'Si, Actualizar',
                    cancelLabel: 'Despues'
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

                await message('La actualizacion se ha instalado correctamente. La aplicacion se reiniciara.', { title: 'Actualizacion Exitosa' });
                await relaunch();
            }
        } else {
            if (!silent) {
                await message('Ya tienes la ultima version instalada.', { title: 'Sistema Actualizado' });
            }
        }
    } catch (error) {
        console.error('Error checking for updates:', error);
        if (!silent) {
            await message(`Error al buscar actualizaciones: ${error}`, { title: 'Error', kind: 'error' });
        }
    }
}
