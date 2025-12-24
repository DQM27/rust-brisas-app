import { z } from 'zod';

// ==========================================
// SCHEMAS BASE
// ==========================================

export const GafeteSchema = z.string()
    .trim()
    .min(1, "El número de gafete es requerido")
    .max(20, "Gafete muy largo");

export const ObservacionesSchema = z.string()
    .trim()
    .max(200, "Observaciones muy largas (máx 200 caracteres)")
    .optional()
    .or(z.literal(''));

export const VehiculoSelectionSchema = z.object({
    tieneVehiculo: z.boolean().default(false),
    vehiculoId: z.string().optional(), // Si selecciona uno existente
    // Si fuera vehículo nuevo (ad-hoc) se agregarían campos aquí
}).refine(data => {
    if (data.tieneVehiculo && !data.vehiculoId) {
        // En este MVP quizás forzamos a seleccionar vehículo registrado o "N/A"
        // O permitimos ingreso con vehículo sin registrar?
        // Por ahora lo dejamos flexible, pero el backend espera vehiculo_id si hay vehículo
        return false;
    }
    return true;
}, {
    message: "Debe seleccionar un vehículo si indica que ingresa con uno",
    path: ["vehiculoId"]
});

// ==========================================
// SCHEMAS ESPECÍFICOS DE INGRESO
// ==========================================

// Input para finalizar el ingreso (Paso CONFIRM)
export const FinalizarIngresoSchema = z.object({
    gafete: GafeteSchema,
    vehiculoId: z.string().optional().nullable(),
    observaciones: ObservacionesSchema,

    // Autorización Excepcional (solo si aplica)
    esExcepcional: z.boolean().default(false),
    autorizadoPor: z.string().optional(),
    motivoExcepcional: z.string().optional(),

    tipoAutorizacion: z.string().default('praind'),
    modoIngreso: z.string().default('caminando'),
}).superRefine((data, ctx) => {
    if (data.esExcepcional) {
        if (!data.autorizadoPor || data.autorizadoPor.length < 3) {
            ctx.addIssue({
                code: z.ZodIssueCode.custom,
                message: "Debe indicar quién autoriza el ingreso excepcional",
                path: ["autorizadoPor"]
            });
        }
        if (!data.motivoExcepcional || data.motivoExcepcional.length < 5) {
            ctx.addIssue({
                code: z.ZodIssueCode.custom,
                message: "Debe justificar el motivo excepcional",
                path: ["motivoExcepcional"]
            });
        }
    }
});

export type FinalizarIngresoForm = z.infer<typeof FinalizarIngresoSchema>;
