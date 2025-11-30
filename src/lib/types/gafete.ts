import { z } from 'zod';

// ==========================================
// ESQUEMAS (VALIDACIÓN)
// ==========================================

export const TipoGafeteEnum = z.enum(['contratista', 'proveedor', 'visita', 'otro']);

export const GafeteSchema = z.object({
    numero: z.string(),
    tipo: TipoGafeteEnum,
    tipoDisplay: z.string().optional(), // Mapped from tipo_display
    estaDisponible: z.boolean().default(false), // Mapped from esta_disponible
    createdAt: z.string(),
    updatedAt: z.string(),
});

export const CreateGafeteSchema = z.object({
    numero: z.string().min(1, "El número de gafete es requerido").max(50),
    tipo: z.string().refine((val) => ['contratista', 'proveedor', 'visita', 'otro'].includes(val.toLowerCase()), {
        message: "Tipo de gafete inválido. Valores permitidos: contratista, proveedor, visita, otro"
    }),
});

export const UpdateGafeteSchema = z.object({
    tipo: z.string().refine((val) => ['contratista', 'proveedor', 'visita', 'otro'].includes(val.toLowerCase()), {
        message: "Tipo de gafete inválido"
    }).optional(),
});

// ==========================================
// TIPOS INFERIDOS (TYPESCRIPT)
// ==========================================

export type Gafete = z.infer<typeof GafeteSchema>;
export type CreateGafeteInput = z.infer<typeof CreateGafeteSchema>;
export type UpdateGafeteInput = z.infer<typeof UpdateGafeteSchema>;

export interface GafeteResponse {
    numero: string;
    tipo: string; // "contratista" | "proveedor" ...
    tipoDisplay: string;
    estaDisponible: boolean;
    createdAt: string;
    updatedAt: string;
}

export interface GafeteListResponse {
    gafetes: GafeteResponse[];
    total: number;
    stats: {
        total: number;
        disponibles: number;
        enUso: number;
        porTipo: {
            contratistas: number;
            proveedores: number;
            visitas: number;
            otros: number;
        };
    };
}
