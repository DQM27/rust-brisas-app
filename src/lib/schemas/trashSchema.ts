import { z } from 'zod';

export const TrashItemSchema = z
	.object({
		id: z.string().min(1, 'El ID es requerido'),
		deletedAt: z.union([z.string(), z.date()]).optional()
	})
	.passthrough(); // Allow other properties

export const RestoreActionSchema = z.object({
	id: z.string().min(1, 'El ID es requerido')
});

export type TrashItemInput = z.infer<typeof TrashItemSchema>;
export type RestoreActionInput = z.infer<typeof RestoreActionSchema>;
