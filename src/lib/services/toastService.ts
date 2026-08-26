import { toast } from 'svelte-5-french-toast';
import { get } from 'svelte/store';
import { generalSettings } from '../stores/settingsStore';
import CustomToast from '../components/shared/CustomToast.svelte';

/**
 * Servicio centralizado para manejar notificaciones (toasts).
 * Abstrae svelte-5-french-toast e integra las preferencias visuales del usuario.
 */
export const toastService = {
    /**
     * Muestra un mensaje de éxito con el estilo personalizado.
     */
    success(message: string, options: any = {}) {
        const settings = get(generalSettings);

        if (!settings.showToasts) return '';

        return toast.custom(CustomToast as any, {
            ...options,
            position: settings.toastPosition,
            props: {
                toast: { type: 'success', visible: true, ...options },
                message,
                position: settings.toastPosition
            }
        } as any);
    },

    /**
     * Muestra un mensaje de error y reproduce un sonido de alerta si está habilitado en los ajustes.
     */
    error(message: string, options: { playSound?: boolean;[key: string]: any } = { playSound: true }) {
        const settings = get(generalSettings);
        const { playSound: _playSound, ...restOptions } = options;

        if (!settings.showToasts) return '';

        // Limpiar mensajes comunes de Rust/Tauri/SurrealDB
        let cleanMessage = message;
        if (message.includes('error sent from backend: ')) {
            cleanMessage = message.split('error sent from backend: ').pop() || message;
        } else if (message.includes('error: ')) {
            cleanMessage = message.split('error: ').pop() || message;
        }

        return toast.custom(CustomToast as any, {
            ...restOptions,
            props: {
                toast: { type: 'error', visible: true, ...restOptions },
                message: cleanMessage
            }
        } as any);
    },

    /**
     * Muestra un aviso informativo.
     */
    info(message: string, options: any = {}) {
        const settings = get(generalSettings);

        if (!settings.showToasts) return '';

        return toast.custom(CustomToast as any, {
            ...options,
            position: settings.toastPosition,
            props: {
                toast: { type: 'blank', visible: true, ...options },
                message,
                position: settings.toastPosition
            }
        } as any);
    },

    /**
     * Muestra un toast de carga manual.
     */
    loading(message: string, options: any = {}) {
        const settings = get(generalSettings);
        if (!settings.showToasts) return '';

        return toast.custom(CustomToast as any, {
            ...options,
            position: settings.toastPosition,
            props: {
                toast: { type: 'loading', visible: true, ...options },
                message,
                position: settings.toastPosition
            }
        } as any);
    },

    /**
     * Muestra una notificación que sigue el estado de una promesa.
     */
    promise<T>(
        promise: Promise<T>,
        messages: {
            loading: string;
            success: string;
            error: string | ((err: any) => string);
        }
    ) {
        const settings = get(generalSettings);
        if (!settings.showToasts) return promise;

        return toast.promise(promise, messages);
    },

    /**
     * Elimina uno o todos los toasts.
     */
    dismiss(toastId?: string) {
        toast.dismiss(toastId);
    }
};
