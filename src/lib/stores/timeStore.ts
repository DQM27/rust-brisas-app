import { writable, type Readable } from 'svelte/store';

/**
 * Store que proporciona la hora actual y se actualiza cada minuto.
 * Útil para calcular tiempos transcurridos en la UI sin polling al backend.
 */
function createTimeStore() {
    const { subscribe, set } = writable(new Date());

    let interval: NodeJS.Timeout;

    function start() {
        // Actualizar inmediatamente
        set(new Date());

        // Calcular tiempo hasta el próximo minuto para sincronizar
        const now = new Date();
        const msUntilNextMinute = (60 - now.getSeconds()) * 1000 - now.getMilliseconds();

        setTimeout(() => {
            set(new Date());
            // Iniciar intervalo regular
            interval = setInterval(() => {
                set(new Date());
            }, 60000);
        }, msUntilNextMinute);
    }

    function stop() {
        if (interval) clearInterval(interval);
    }

    return {
        subscribe,
        start,
        stop
    };
}

export const currentTime = createTimeStore();

// Iniciar el reloj globalmente (o podría iniciarse en layout si se prefiere)
currentTime.start();
