import { z } from 'zod';

// ==========================================
// VALIDACIONES BÁSICAS
// ==========================================

const cedulaSchema = z.string()
    .trim()
    .min(5, 'Cédula debe tener al menos 5 caracteres')
    .max(20, 'Cédula no puede exceder 20 caracteres'); // Más flexible para pasaportes etc

const nombreSchema = z.string()
    .trim()
    .min(1, 'Nombre es requerido')
    .max(50, 'Nombre no puede exceder 50 caracteres')
    .regex(/^[a-zA-ZáéíóúÁÉÍÓÚñÑ\s]+$/, 'Solo puede contener letras');

const apellidoSchema = z.string()
    .trim()
    .min(1, 'Apellido es requerido')
    .max(50, 'Apellido no puede exceder 50 caracteres')
    .regex(/^[a-zA-ZáéíóúÁÉÍÓÚñÑ\s]+$/, 'Solo puede contener letras');

const opcionalTextoSchema = z.string()
    .trim()
    .max(50, 'No puede exceder 50 caracteres')
    .optional()
    .or(z.literal(''));

// ==========================================
// SCHEMA PRINCIPAL
// ==========================================

export const VisitaSchema = z.object({
    // Datos de Cita
    fecha: z.string().min(1, "Fecha es requerida"), // YYYY-MM-DD
    hora: z.string().min(1, "Hora es requerida"),   // HH:MM
    anfitrion: z.string().min(1, "Anfitrión es requerido").max(100),
    areaVisitada: z.string().min(1, "Área es requerida").max(100),
    motivo: z.string().max(200).optional().or(z.literal('')),

    // Datos del Visitante
    cedula: cedulaSchema,
    nombre: nombreSchema,
    segundoNombre: opcionalTextoSchema,
    apellido: apellidoSchema,
    segundoApellido: opcionalTextoSchema,
    empresa: z.string().max(100).optional().or(z.literal('')),

    // Vehículo (Opcional por ahora en citas, pero preparado)
    tieneVehiculo: z.boolean().default(false),
    placa: z.string().max(10).optional().or(z.literal('')),
}).superRefine((data, ctx) => {
    // Validaciones extra si fueran necesarias
    // Por ejemplo validar que fecha no sea pasado (aunque permitimos agendar histórico a veces?)
    // Validación de vehiculo si se activa
    if (data.tieneVehiculo && (!data.placa || data.placa.trim() === '')) {
        ctx.addIssue({
            code: z.ZodIssueCode.custom,
            message: "La placa es requerida si tiene vehículo",
            path: ["placa"]
        });
    }
});

export type VisitaForm = z.infer<typeof VisitaSchema>;
