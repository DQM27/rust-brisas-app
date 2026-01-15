// ============================================
// src/lib/schemas/roleSchema.ts
// ============================================

import { z } from 'zod';

// ==========================================
// SCHEMAS PRINCIPALES
// ==========================================

export const CreateRoleSchema = z.object({
	name: z
		.string()
		.trim()
		.min(1, 'Nombre es requerido')
		.max(50, 'Nombre no puede exceder 50 caracteres')
		.regex(/^[^:/Local\\]+$/, 'Nombre no puede contener : / \\'),

	description: z
		.string()
		.trim()
		.max(200, 'Descripción no puede exceder 200 caracteres')
		.optional()
		.or(z.literal('')),

	permissions: z.array(z.string()).min(1, 'Debe seleccionar al menos un permiso')
});

export const UpdateRoleSchema = CreateRoleSchema.partial();

// Tipos inferidos
export type CreateRoleForm = z.infer<typeof CreateRoleSchema>;
export type UpdateRoleForm = z.infer<typeof UpdateRoleSchema>;
