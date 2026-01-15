import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { toast } from 'svelte-5-french-toast';

export type ModuleStatusType = 'active' | 'development' | 'maintenance' | 'hidden';

export interface ModuleStatus {
	key: string;
	name: string;
	status: ModuleStatusType;
}

// Store principal: Mapa de key -> ModuleStatus
function createModulesStore() {
	const { subscribe, set, update } = writable<Record<string, ModuleStatus>>({});

	return {
		subscribe,

		// Cargar todos los módulos desde el backend
		load: async () => {
			try {
				const modules = await invoke<ModuleStatus[]>('get_modules_status');
				const map: Record<string, ModuleStatus> = {};
				modules.forEach((m) => {
					map[m.key] = m;
				});
				set(map);
				console.log('📦 Módulos cargados:', map);
			} catch (error) {
				console.error('Error cargando módulos:', error);
				toast.error('Error al cargar configuración de módulos');
			}
		},

		// Actualizar estado (Requiere permisos en Backend)
		updateStatus: async (key: string, status: ModuleStatusType) => {
			try {
				await invoke('update_module_status', { key, status });

				// Actualización optimista
				update((n) => {
					if (n[key]) n[key].status = status;
					return n;
				});

				toast.success(`Módulo actualizado a: ${status}`);
				return true;
			} catch (error) {
				console.error(`Error actualizando módulo ${key}:`, error);
				toast.error(String(error), { duration: 4000 }); // Mostrar error del backend (ej. permisos)
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
