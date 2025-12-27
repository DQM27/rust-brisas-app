import type { ColDef } from "@ag-grid-community/core";

export interface TrashItem {
    id: string;
    deletedAt?: string | Date;
    [key: string]: any;
}

export interface TrashService<T extends TrashItem> {
    getArchived: () => Promise<{ ok: boolean; data: T[]; error?: string }>;
    restore: (id: string) => Promise<{ ok: boolean; error?: string }>;
    permanentlyDelete?: (id: string) => Promise<{ ok: boolean; error?: string }>;
}

export interface TrashViewProps<T extends TrashItem> {
    title?: string;
    service: TrashService<T>;
    columnDefs: ColDef<T>[];
    gridId: string;
    onBack: () => void;
    entityName?: string;
    rowIdField?: string;
}
