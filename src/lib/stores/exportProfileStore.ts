// src/lib/stores/exportProfileStore.ts
import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import type { ExportProfile } from '$lib/types/exportProfile';
import { toast } from 'svelte-5-french-toast';

function createExportProfileStore() {
	const { subscribe, update } = writable<{
		profiles: ExportProfile[];
		loading: boolean;
		error: string | null;
	}>({
		profiles: [],
		loading: false,
		error: null
	});

	return {
		subscribe,

		load: async () => {
			update((s) => ({ ...s, loading: true, error: null }));
			try {
				const profiles = await invoke<ExportProfile[]>('get_export_profiles');
				update((s) => ({ ...s, profiles, loading: false }));
			} catch (err) {
				console.error('Error loading export profiles:', err);
				update((s) => ({
					...s,
					loading: false,
					error: (err as Error).message
				}));
				toast.error('Error cargando perfiles de exportación');
			}
		},

		save: async (profile: ExportProfile) => {
			try {
				await invoke('save_export_profile', { profile });
				toast.success('Perfil guardado');
				// Reload to update list
				const profiles = await invoke<ExportProfile[]>('get_export_profiles');
				update((s) => ({ ...s, profiles }));
				return true;
			} catch (err) {
				console.error('Error saving export profile:', err);
				toast.error('Error al guardar perfil: ' + (err as string));
				return false;
			}
		},

		delete: async (id: string) => {
			try {
				await invoke('delete_export_profile', { id });
				toast.success('Perfil eliminado');
				update((s) => ({
					...s,
					profiles: s.profiles.filter((p) => p.id !== id)
				}));
				return true;
			} catch (err) {
				console.error('Error deleting export profile:', err);
				toast.error('Error al eliminar perfil: ' + (err as string));
				return false;
			}
		},

		setDefault: async (id: string) => {
			try {
				await invoke('set_default_export_profile', { id });
				toast.success('Perfil establecido como predeterminado');
				// Reload to update list
				const profiles = await invoke<ExportProfile[]>('get_export_profiles');
				update((s) => ({ ...s, profiles }));
				return true;
			} catch (err) {
				console.error('Error setting default export profile:', err);
				toast.error('Error al establecer perfil predeterminado: ' + (err as string));
				return false;
			}
		},

		getDefault: () => {
			let defaultProfile: ExportProfile | null = null;
			subscribe((s) => {
				defaultProfile = s.profiles.find((p) => p.isDefault) || null;
			})();
			return defaultProfile;
		}
	};
}

export const exportProfileStore = createExportProfileStore();
