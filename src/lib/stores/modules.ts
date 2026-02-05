import { writable } from 'svelte/store';
import { toastService } from '$lib/services/toastService';
import { modulesApi } from '$lib/api/modules';
import type { ModuleStatus, ModuleStatusType } from '$lib/types/modules';

// Store principal: Mapa de key -> ModuleStatus
function createModulesStore() {
	const { subscribe, set, update } = writable<Record<string, ModuleStatus>>({});

	return {
		subscribe,

		// Cargar todos los módulos desde el backend
		load: async () => {
			try {
				const modules = await modulesApi.fetchAll();
				const map: Record<string, ModuleStatus> = {};
				modules.forEach((m) => {
					map[m.key] = m;
				});
				set(map);
			} catch (error) {
				console.error('Error cargando módulos:', error);
				toastService.error('Error al cargar configuración de módulos');
			}
		},

		// Actualizar estado (Requiere permisos en Backend)
		updateStatus: async (key: string, status: ModuleStatusType) => {
			try {
				await modulesApi.updateStatus(key, status);

				// Actualización optimista
				update((n) => {
					if (n[key]) n[key].status = status;
					return n;
				});

				toastService.success(`Módulo actualizado a: ${status}`);
				return true;
			} catch (error) {
				console.error(`Error actualizando módulo ${key}:`, error);
				toastService.error(String(error), { duration: 4000 }); // Mostrar error del backend (ej. permisos)
				return false;
			}
		},

		// Helper para obtener estado síncronamente (si ya está cargado)
		getStatus: (key: string, $store: Record<string, ModuleStatus>): ModuleStatusType => {
			return $store[key]?.status || 'active'; // Default active safe
		}
	};
}

export const modulesStore = createModulesStore();

// Derived store para verificar si un módulo específico está activo
// Uso: $isModuleActive('users')
// Nota: Esto es más difícil de hacer como derived store dinámico por la key variable.
// Mejor usar helpers directos en componentes.
