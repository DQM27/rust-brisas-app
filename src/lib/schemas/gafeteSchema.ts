// src/lib/schemas/gafeteSchema.ts
import { z } from 'zod';

export const gafeteSchema = z.object({
    numero: z
        .union([z.string(), z.number()])
        .transform((val) => String(val))
        .pipe(
            z
                .string()
                .min(1, 'El número de gafete es requerido')
                .max(50, 'El número de gafete no puede exceder 50 caracteres')
                .trim()
        ),
    tipo: z.enum(['contratista', 'proveedor', 'visita', 'otro'], {
        message: 'Tipo de gafete inválido'
    })
});

export type GafeteFormData = z.infer<typeof gafeteSchema>;
