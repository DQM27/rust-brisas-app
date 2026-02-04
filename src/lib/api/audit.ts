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

export const auditApi = {
    /**
     * Obtener historial de auditoría
     */
    fetchLogs: (limit: number, offset: number) =>
        invoke<SysLogEntry[]>('fetch_system_logs', { limit, offset }),

    /**
     * Registrar un evento de sistema
     */
    logEvent: (params: {
        userName: string;
        eventType: string;
        duration?: string;
        ipAddress: string;
        details?: string;
    }) => invoke<void>('log_system_event', params),

    /**
     * Obtener IP local
     */
    getLocalIp: () => invoke<string>('get_local_ip')
};
