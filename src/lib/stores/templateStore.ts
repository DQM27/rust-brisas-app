// src/lib/stores/templateStore.ts
import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import type { PdfTemplate } from '$lib/types/template';
import { toast } from 'svelte-5-french-toast';

function createTemplateStore() {
    const { subscribe, set, update } = writable<{
        templates: PdfTemplate[];
        loading: boolean;
        error: string | null;
    }>({
        templates: [],
        loading: false,
        error: null
    });

    return {
        subscribe,

        load: async () => {
            update(s => ({ ...s, loading: true, error: null }));
            try {
                const templates = await invoke<PdfTemplate[]>('get_templates');
                update(s => ({ ...s, templates, loading: false }));
            } catch (err) {
                console.error('Error loading templates:', err);
                update(s => ({
                    ...s,
                    loading: false,
                    error: (err as Error).message
                }));
                toast.error('Error cargando estilos');
            }
        },

        save: async (template: PdfTemplate) => {
            try {
                await invoke('save_template', { template });
                toast.success('Estilo guardado');
                // Reload to update list
                const templates = await invoke<PdfTemplate[]>('get_templates');
                update(s => ({ ...s, templates }));
                return true;
            } catch (err) {
                console.error('Error saving template:', err);
                toast.error('Error al guardar: ' + (err as string));
                return false;
            }
        },

        delete: async (id: string) => {
            try {
                await invoke('delete_template', { id });
                toast.success('Estilo eliminado');
                update(s => ({
                    ...s,
                    templates: s.templates.filter(t => t.id !== id)
                }));
                return true;
            } catch (err) {
                console.error('Error deleting template:', err);
                toast.error('Error al eliminar: ' + (err as string));
                return false;
            }
        }
    };
}

export const templateStore = createTemplateStore();
