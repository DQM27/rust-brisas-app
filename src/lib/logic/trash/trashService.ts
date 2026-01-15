import type { TrashService } from '$lib/types/trash';

// Re-export types from logic for consistency with other modules
export type { TrashService, TrashItem } from '$lib/types/trash';

// Generic helper to format restoration errors if needed
export function formatRestoreError(error: unknown): string {
	if (typeof error === 'string') return error;
	if (error instanceof Error) return error.message;
	if (typeof error === 'object' && error !== null) {
		const obj = error as Record<string, unknown>;
		if (obj.message && typeof obj.message === 'string') return obj.message;
	}
	return 'Error desconocido al restaurar';
}
