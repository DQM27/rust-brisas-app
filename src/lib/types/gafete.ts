import { z } from 'zod';

// ==========================================
// ENUMS
// ==========================================

export const TipoGafeteEnum = z.enum(['contratista', 'proveedor', 'visita', 'otro']);
export type TipoGafete = z.infer<typeof TipoGafeteEnum>;

// ==========================================
// SCHEMAS DE VALIDACIÓN
// ==========================================

export const CreateGafeteSchema = z.object({
    numero: z.union([
        z.number().int().positive("El número debe ser positivo"),
        z.string()
            .min(1, "El número de gafete es requerido")
            .max(50, "El número no puede exceder 50 caracteres")
            .transform(val => parseInt(val.trim(), 10))
    ]).pipe(z.number().int().positive("El número de gafete debe ser un entero positivo")),
    tipo: TipoGafeteEnum
});

export const UpdateGafeteSchema = z.object({
    tipo: TipoGafeteEnum.optional()
});

// ==========================================
// TIPOS INFERIDOS
// ==========================================

export type CreateGafeteInput = z.infer<typeof CreateGafeteSchema>;
export type UpdateGafeteInput = z.infer<typeof UpdateGafeteSchema>;

export interface CreateGafeteRangeInput {
    start: number;
    end: number;
    prefix?: string;
    padding?: number;
    tipo: TipoGafete;
}

export interface UpdateGafeteStatusInput {
    estado: 'activo' | 'danado' | 'extraviado';
}

// ==========================================
// INTERFACES DE RESPUESTA
// ==========================================

export interface GafeteResponse {
    id: string;
    numero: number;
    tipo: TipoGafete;
    tipoDisplay: string;
    estadoFisico: string; // "activo" | "danado" | "extraviado"
    estaDisponible: boolean;
    status: string; // "disponible" | "en_uso" | "perdido" | "danado" | "extraviado"
    // Información de alerta (si está perdido)
    alertaId?: string; // UUID de la alerta
    fechaPerdido?: string;
    quienPerdio?: string;
    alertaResuelta?: boolean;
    reportadoPorNombre?: string;
    resueltoPorNombre?: string;
    fechaResolucion?: string;
    notas?: string;
    // Who has the gafete when "en_uso"
    asignadoA?: string;
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
        danados: number;
        extraviados: number;
        porTipo: {
            contratistas: number;
            proveedores: number;
            visitas: number;
            otros: number;
        };
    };
}