// src/lib/logic/audit/auditService.ts

import { auditApi } from '$lib/api/audit';
import type { SysLogEntry, AuditEventType } from '$lib/types/audit';

export type ServiceResult<T> = { ok: true; data: T } | { ok: false; error: string };

/**
 * Servicio de Auditoría siguiendo arquitectura limpia
 */

export const auditService = {
    /**
     * Registra un evento del sistema
     */
    log: async (
        event_type: AuditEventType,
        user_name: string,
        details?: string,
        duration?: string
    ): Promise<ServiceResult<void>> => {
        try {
            // Obtener IP local
            let ipAddress = 'Unknown';
            try {
                ipAddress = await auditApi.getLocalIp();
            } catch (e) {
                console.warn('[Audit] Failed to get local IP:', e);
            }

            await auditApi.logEvent({
                userName: user_name || 'Desconocido',
                eventType: event_type,
                duration,
                ipAddress,
                details
            });
            return { ok: true, data: undefined };
        } catch (error: any) {
            console.error('[Audit] Failed to log event:', error);
            return { ok: false, error: String(error) };
        }
    },

    /**
     * Obtiene el historial de auditoría paginado
     */
    fetchLogs: async (limit = 100, offset = 0): Promise<ServiceResult<SysLogEntry[]>> => {
        try {
            const logs = await auditApi.fetchLogs(limit, offset);
            return { ok: true, data: logs };
        } catch (error: any) {
            console.error('[Audit] Failed to fetch logs:', error);
            return { ok: false, error: String(error) };
        }
    }
};
