// src/lib/types/modules.ts

export type ModuleStatusType = 'active' | 'development' | 'maintenance' | 'hidden';

export interface ModuleStatus {
    key: string;
    name: string;
    status: ModuleStatusType;
}
