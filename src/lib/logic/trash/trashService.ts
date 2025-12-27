import type { TrashService } from "$lib/types/trash";

// Re-export types from logic for consistency with other modules
export type { TrashService, TrashItem } from "$lib/types/trash";

// Generic helper to format restoration errors if needed
export function formatRestoreError(error: any): string {
    if (typeof error === 'string') return error;
    return error?.message || "Error desconocido al restaurar";
}
