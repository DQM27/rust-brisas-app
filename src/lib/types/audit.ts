// src/lib/types/audit.ts

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

export type AuditEventType = 'LOGIN' | 'LOGOUT' | 'TIMEOUT' | 'FAILED_LOGIN' | 'PERM_DENIED' | 'GOD_MODE' | 'CONFIG_CHANGE';
