import { invoke } from '@tauri-apps/api/core';

export interface SysLogEntry {
    id: string;
    terminal_id: string;
    terminal_name: string;
    user_name: string;
    event_type: string;
    duration?: string;
    ip_address?: string;
    details?: string;
    access_date: string;
}

export const auditService = {
    /**
     * Registra un evento del sistema de forma asíncrona ("fire-and-forget").
     */
    log: async (
        event_type: 'LOGIN' | 'LOGOUT' | 'TIMEOUT' | 'FAILED_LOGIN',
        user_name: string,
        details?: string,
        duration?: string
    ): Promise<void> => {
        try {
            // Nota: El backend se encarga de terminal_id y terminal_name desde AppConfig.
            // IP address se pasa como null por ahora (requeriría comando extra)
            await invoke('log_system_event', {
                userName: user_name || 'Desconocido',
                eventType: event_type,
                duration,
                ipAddress: null, // TODO: Implementar obtención de IP
                details
            });
            console.log('[Audit] Event logged:', event_type);
        } catch (error) {
            console.error('[Audit] Failed to log event:', error);
            // No lanzamos error para no interrumpir la experiencia de usuario (es un log secundario)
        }
    },

    /**
     * Obtiene el historial de auditoría paginado.
     */
    getLogs: async (limit = 100, offset = 0): Promise<SysLogEntry[]> => {
        try {
            return await invoke<SysLogEntry[]>('fetch_system_logs', { limit, offset });
        } catch (error) {
            console.error('[Audit] Failed to fetch logs:', error);
            throw error;
        }
    }
};
