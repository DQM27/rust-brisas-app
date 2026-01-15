// src/lib/stores/network.ts
import { writable } from 'svelte/store';
import { browser } from '$app/environment';

// Store de estado de conexión
export const online = writable<boolean>(true);

/**
 * Inicializa los listeners de conexión
 * Llamar una sola vez en el layout o app principal
 */
export function initNetworkMonitor(): () => void {
	if (!browser) return () => {};

	// Establecer valor inicial
	online.set(navigator.onLine);

	const updateOnlineStatus = () => {
		online.set(navigator.onLine);
	};

	window.addEventListener('online', updateOnlineStatus);
	window.addEventListener('offline', updateOnlineStatus);

	// Retornar función de cleanup
	return () => {
		window.removeEventListener('online', updateOnlineStatus);
		window.removeEventListener('offline', updateOnlineStatus);
	};
}
